pub mod simulated;
pub mod source;

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;

use bevy::prelude::*;

pub use source::{IoSource, Quality, SourceEvent, TagId, TagSnapshot};

const MAX_EVENTS_PER_FRAME: usize = 256;
const UI_REFRESH_INTERVAL: Duration = Duration::from_millis(200);

#[derive(Resource)]
struct IoThread {
    shutdown: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl Drop for IoThread {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

struct IoBridge {
    receiver: Receiver<SourceEvent>,
}

#[derive(Resource, Default)]
pub struct IoTagStore {
    snapshots: HashMap<TagId, TagSnapshot>,
}

impl IoTagStore {
    pub fn get(&self, tag: &TagId) -> Option<TagSnapshot> {
        self.snapshots.get(tag).copied()
    }
}

#[derive(Resource, Default)]
pub struct IoStatus {
    pub connected: bool,
    pub updates_seen: u64,
}

#[derive(Resource)]
struct UiRefreshTimer(Timer);

#[derive(Component, Clone)]
pub struct BindTag {
    pub tag: TagId,
    pub label: String,
    pub last_display: Option<String>,
}

pub struct IoPlugin {
    source: Mutex<Option<Box<dyn IoSource>>>,
}

impl IoPlugin {
    pub fn new(source: impl IoSource) -> Self {
        IoPlugin {
            source: Mutex::new(Some(Box::new(source))),
        }
    }
}

impl Plugin for IoPlugin {
    fn build(&self, app: &mut App) {
        let (sender, receiver) = mpsc::channel::<SourceEvent>();
        let shutdown = Arc::new(AtomicBool::new(false));
        let thread_shutdown = Arc::clone(&shutdown);
        let mut source = self
            .source
            .lock()
            .expect("io source mutex poisoned")
            .take()
            .expect("io source already consumed");
        let handle = std::thread::Builder::new()
            .name("io-source".to_string())
            .spawn(move || source.run(&sender, &thread_shutdown))
            .expect("failed to spawn io source thread");
        app.insert_resource(IoThread {
            shutdown,
            handle: Some(handle),
        });
        app.insert_non_send(IoBridge { receiver });
        app.init_resource::<IoTagStore>();
        app.init_resource::<IoStatus>();
        app.insert_resource(UiRefreshTimer(Timer::new(
            UI_REFRESH_INTERVAL,
            TimerMode::Repeating,
        )));
        app.add_systems(Update, (io_drain, ui_refresh).chain());
    }
}

fn io_drain(
    bridge: NonSend<IoBridge>,
    mut store: ResMut<IoTagStore>,
    mut status: ResMut<IoStatus>,
) {
    let mut received = 0;
    while received < MAX_EVENTS_PER_FRAME {
        match bridge.receiver.try_recv() {
            Ok(SourceEvent::Connected) => status.connected = true,
            Ok(SourceEvent::Disconnected) => status.connected = false,
            Ok(SourceEvent::Update(tag, snapshot)) => {
                store.snapshots.insert(tag, snapshot);
                status.updates_seen += 1;
            }
            Err(TryRecvError::Empty) | Err(TryRecvError::Disconnected) => break,
        }
        received += 1;
    }
}

fn ui_refresh(
    time: Res<Time>,
    mut timer: ResMut<UiRefreshTimer>,
    store: Res<IoTagStore>,
    status: Res<IoStatus>,
    mut bound: Query<(&mut BindTag, &mut Text, &mut TextColor)>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    for (mut bind, mut text, mut color) in &mut bound {
        let display = match store.get(&bind.tag) {
            Some(snapshot) if snapshot.quality == Quality::Good => {
                snapshot.value.to_display()
            }
            Some(_) => "---".to_string(),
            None => "…".to_string(),
        };
        if bind.last_display.as_deref() != Some(display.as_str()) {
            text.0 = format!("{}: {}", bind.label, display);
            bind.last_display = Some(display);
        }
        color.0 = if status.connected {
            Color::srgb(0.75, 0.78, 0.85)
        } else {
            Color::srgb(0.55, 0.30, 0.30)
        };
    }
}

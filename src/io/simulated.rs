use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::thread;
use std::time::{Duration, Instant};

use super::source::{IoSource, Quality, SourceEvent, TagId, TagSnapshot, TagValue};

pub const TAG_LINE1_SPEED: &str = "plc.line1.speed";
pub const TAG_LINE1_TEMP: &str = "plc.line1.temperature";
pub const TAG_LINE2_COUNT: &str = "plc.line2.parts_count";
pub const TAG_TANK_LEVEL: &str = "plc.tank.level";
pub const TAG_PUMP_RUNNING: &str = "plc.pump.running";

pub struct SimulatedSource {
    cycle: Duration,
}

impl SimulatedSource {
    pub fn new(cycle: Duration) -> Self {
        SimulatedSource { cycle }
    }
}

impl Default for SimulatedSource {
    fn default() -> Self {
        SimulatedSource::new(Duration::from_millis(10))
    }
}


impl IoSource for SimulatedSource {
    fn run(&mut self, events: &Sender<SourceEvent>, shutdown: &AtomicBool) {
        let start = Instant::now();
        if events.send(SourceEvent::Connected).is_err() {
            return;
        }
        let mut cycle_count: u64 = 0;
        while !shutdown.load(Ordering::Relaxed) {
            let t = start.elapsed().as_secs_f32();
            let temp_quality = if cycle_count % 1500 >= 1400 {
                Quality::Uncertain
            } else {
                Quality::Good
            };
            let updates = [
                (
                    TagId::new(TAG_LINE1_SPEED),
                    TagValue::F32(35.0 + 15.0 * (t * 0.8).sin()),
                    Quality::Good,
                ),
                (
                    TagId::new(TAG_LINE1_TEMP),
                    TagValue::F32(180.0 + 5.0 * (t * 0.15).sin()),
                    temp_quality,
                ),
                (
                    TagId::new(TAG_LINE2_COUNT),
                    TagValue::U32((t * 9.0) as u32),
                    Quality::Good,
                ),
                (
                    TagId::new(TAG_TANK_LEVEL),
                    TagValue::F32(55.0 + 40.0 * (t * 0.05).sin().abs()),
                    Quality::Good,
                ),
                (
                    TagId::new(TAG_PUMP_RUNNING),
                    TagValue::Bool((t * 0.3).sin() > 0.0),
                    Quality::Good,
                ),
            ];
            for (id, value, quality) in updates {
                let event = SourceEvent::Update(
                    id,
                    TagSnapshot {
                        value,
                        quality,
                    },
                );
                if events.send(event).is_err() {
                    return;
                }
            }
            cycle_count += 1;
            thread::sleep(self.cycle);
        }
        let _ = events.send(SourceEvent::Disconnected);
    }
}

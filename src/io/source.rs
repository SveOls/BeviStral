use std::fmt;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::Sender;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct TagId(pub String);

impl TagId {
    pub fn new(id: impl Into<String>) -> Self {
        TagId(id.into())
    }
}

impl fmt::Display for TagId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Quality {
    Good,
    Uncertain,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TagValue {
    Bool(bool),
    U32(u32),
    F32(f32),
}

impl TagValue {
    pub fn to_display(&self) -> String {
        match self {
            TagValue::Bool(v) => {
                if *v {
                    "ON".to_string()
                } else {
                    "OFF".to_string()
                }
            }
            TagValue::U32(v) => v.to_string(),
            TagValue::F32(v) => format!("{:.2}", v),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TagSnapshot {
    pub value: TagValue,
    pub quality: Quality,
}

pub enum SourceEvent {
    Connected,
    Disconnected,
    Update(TagId, TagSnapshot),
}

pub trait IoSource: Send + Sync + 'static {
    fn run(&mut self, events: &Sender<SourceEvent>, shutdown: &AtomicBool);
}

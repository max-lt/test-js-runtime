use tokio::time::Duration;
use tokio::time::Instant;

pub struct Timer {
    pub(super) id: u32,
    pub(super) timestamp: Instant,
    pub(super) interval: Option<Duration>, // Set if the timer is a repeating timer
    pub(super) callback: v8::Global<v8::Function>,
}

pub struct Timers {
    pub(super) timers: std::collections::HashMap<u32, Timer>,
    next_id: u32,
}

impl Timers {
    pub fn new() -> Timers {
        Timers {
            timers: std::collections::HashMap::new(),
            next_id: 0,
        }
    }

    pub fn create(&mut self, callback: v8::Global<v8::Function>, delay: u64, repeat: bool) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        let duration = std::time::Duration::from_millis(delay);
        let timestamp = Instant::now() + duration;

        let interval = match repeat {
            true => Some(std::time::Duration::from_millis(delay)),
            false => None,
        };

        let timer = Timer {
            id,
            callback,
            interval,
            timestamp,
        };

        self.timers.insert(id, timer);

        id
    }

    pub fn remove(&mut self, id: u32) {
        self.timers.remove(&id);
    }

    pub fn len(&self) -> usize {
        self.timers.len()
    }
}

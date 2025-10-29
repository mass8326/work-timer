use std::time::Instant;

use iced::time::Duration;

#[derive(Default, Debug)]
pub struct Clock {
    elapsed: Duration,
    state: TickState,
}

#[derive(Default, Debug)]
pub enum TickState {
    #[default]
    Idle,
    Ticking {
        last_tick: Instant,
    },
}

impl Clock {
    pub fn get_elapsed(&self) -> &Duration {
        &self.elapsed
    }

    pub fn set_elapsed(&mut self, duration: Duration) {
        self.elapsed = duration;
    }

    pub fn is_ticking(&self) -> bool {
        match self.state {
            TickState::Ticking { .. } => true,
            TickState::Idle => false,
        }
    }

    pub fn tick(&mut self, now: Instant) {
        if let TickState::Ticking { last_tick } = &mut self.state {
            self.elapsed += now - *last_tick;
            *last_tick = now;
        }
    }

    pub fn reset(&mut self) {
        self.elapsed = Duration::default();
    }

    pub fn off(&mut self) {
        if let TickState::Ticking { last_tick } = self.state {
            self.elapsed += last_tick.elapsed();
            self.state = TickState::Idle;
        }
    }

    pub fn on(&mut self) {
        if let TickState::Idle = self.state {
            self.state = TickState::Ticking {
                last_tick: Instant::now(),
            };
        }
    }
}

impl From<Duration> for Clock {
    fn from(duration: Duration) -> Self {
        Self {
            elapsed: duration,
            ..Default::default()
        }
    }
}

use std::time::{Duration, Instant};

pub struct TimeKeeper {
    start_time: Instant,
    before_time: Instant,
    time_threshold: i64,
    end_turn: i64,
    turn: i64,
}

impl TimeKeeper {
    // Create a new instance with the total time limit (in milliseconds) and the maximum number of turns.
    pub fn new(time_threshold: i64, end_turn: i64) -> Self {
        let start_time = Instant::now();
        TimeKeeper {
            start_time,
            before_time: start_time,
            time_threshold,
            end_turn,
            turn: 0,
        }
    }

    // Update the turn and the start time of the turn.
    pub fn set_turn(&mut self, t: i64) {
        self.turn = t;
        self.before_time = Instant::now();
    }

    // Check if the allocated time for each turn has been exceeded.
    pub fn is_time_over(&self) -> bool {
        let now = Instant::now();
        let whole_diff = now.duration_since(self.start_time);
        let whole_count = whole_diff.as_millis() as i64;
        let last_diff = now.duration_since(self.before_time);
        let last_count = last_diff.as_millis() as i64;

        let remaining_time = self.time_threshold - whole_count;
        let now_threshold = remaining_time / (self.end_turn - self.turn);
        last_count >= now_threshold
    }
}

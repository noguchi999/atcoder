use crate::time_keeper::TimeKeeper;
use std::time::Duration;

pub struct State {
    now_answer: Vec<i32>
}

impl State {
    pub fn new() -> Self {
        State {
            now_answer: Vec::new(),
        }
    }

    pub fn hill_climb_with_time_threshold(&mut self, time_threshold: i64) {
        let mut time_keeper = TimeKeeper::new(time_threshold);

        self.now_answer = self.generate_initial_solution();
        let mut now_score = self.calculate_score(&self.now_answer);
        loop {
            time_keeper.set_now_time();
            if time_keeper.is_time_over() {
                break;
            }

            let next_answer = self.generate_next_solution();
            let next_score = self.calculate_score(&next_answer);

            let diff = next_score - now_score;
            if diff > 0 {
                self.now_answer = next_answer;
                now_score = next_score;
            }
        }
    }

    fn generate_initial_solution(&self) -> Vec<i32> {
        vec![]
    }

    fn calculate_score(&self, solution: &Vec<i32>) -> i32 {
        0
    }

    fn generate_next_solution(&self) -> Vec<i32> {
        vec![]
    }
}

use crate::time_keeper::TimeKeeper;
use rand::Rng;
use std::f64::consts::E;

pub struct State {
    now_answer: Vec<i32>,
    actions: Vec<i32>
}

impl State {
    pub fn new() -> Self {
        State {
            now_answer: Vec::new(),
            actions: Vec::new(),
        }
    }

    pub fn simulated_annealing_with_time_threshold(&mut self, time_threshold: i64, start_temp: f64, end_temp: f64) {
        let mut time_keeper = TimeKeeper::new(time_threshold);

        self.now_answer = self.generate_initial_solution();
        let mut now_score = self.calculate_score(&self.now_answer);

        let mut rng = rand::thread_rng();

        loop {
            time_keeper.set_now_time();
            if time_keeper.is_time_over() {
                break;
            }
            let elapsed_time = time_keeper.get_now_time();
            let temp = start_temp + (end_temp - start_temp) * (elapsed_time as f64 / time_threshold as f64);

            let next_answer = self.generate_next_solution();
            let next_score = self.calculate_score(&next_answer);

            let diff = next_score - now_score;

            if (diff as f64 / temp).exp() > rng.gen::<f64>() {
                self.now_answer = next_answer;
                now_score = next_score;
            }
        }
    }

    pub fn beam_search_action_with_time_threshold(
        &self,
        beam_width: usize,
        time_threshold: i64,
    ) -> Vec<i32> {
        let mut time_keeper = TimeKeeper::new(time_threshold);
        let mut best_state = self.clone();

        loop {
            if time_keeper.is_time_over() {
                return best_state.actions.clone();
            }

            let mut next_state = self.clone();
            next_state.actions.push(action);
        }

        best_state.actions.clone()
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

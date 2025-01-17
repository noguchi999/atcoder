pub struct State {
    t: i32,
}

impl State {
    pub fn new() -> Self {
        State { t: 0 }
    }

    pub fn is_done(&self, end_turn: i32) -> bool {
        self.t >= end_turn
    }

    pub fn advance(&mut self, _action: i32) {
        self.t += 1;
    }

    pub fn random_update(&mut self) {
        let parameter = self.generate_random_parameter();
        self.update(parameter);
    }

    pub fn update(&mut self, _parameter: i32) {
    }

    pub fn get_score(&self) -> f64 {
        let score = 0.0;
        score
    }

    fn generate_random_parameter(&self) -> i32 {
        0
    }
}

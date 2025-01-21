pub struct DfsSolver {
    path: Vec<i32>,
    best_path: Vec<i32>,
    best_score: i32,
    score: i32,
    remaining_search_cnt: i32
}

impl DfsSolver {
    pub fn new() -> Self {
        DfsSolver {
            path: Vec::new(),
            best_path: Vec::new(),
            best_score: 0,
            score: 0,
            remaining_search_cnt: 0
        }
    }

    pub fn start(&mut self, search_count: i32, start_coord: i32) {
        self.remaining_search_cnt = search_count;
        self.dfs(start_coord);
    }

    fn dfs(&mut self, coord: i32) {
        self.path.push(coord);
        self.score = self.calculate_score();

        if self.best_score < self.score {
            self.best_score = self.score;
            self.best_path = self.path.clone();
        }

        self.remaining_search_cnt -= 1;
        if self.remaining_search_cnt == 0 {
            return;
        }

        let legal_next_coords = self.get_legal_next_coords(coord);

        for &next_coord in &legal_next_coords {
            self.dfs(next_coord);
            if self.remaining_search_cnt == 0 {
                return;
            }
        }

        self.path.pop();
        self.score = self.calculate_score();
        self.reset_explored_flag(coord);
    }

    fn calculate_score(&self) -> i32 {
        0
    }

    fn get_legal_next_coords(&self, coord: i32) -> Vec<i32> {
        vec![]
    }

    fn reset_explored_flag(&self, coord: i32) {
    }
}

use rand::Rng;
use crate::state::State;
use crate::time_keeper::TimeKeeper;

pub mod montecarlo {
    use super::*;

    static LEGAL_ACTIONS: &[i32] = &[/* ... */];

    pub fn play1turn(state: &State) -> i32 {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..LEGAL_ACTIONS.len());
        LEGAL_ACTIONS[index]
    }

    pub fn playout(mut state: State) -> f64 {
        state.advance(play1turn(&state));
        while !state.is_done(/* END_TURN */) {
            state.random_update();
            state.advance(play1turn(&state));
        }
        state.get_score()
    }

    pub fn primitive_montecarlo(time_keeper: &mut TimeKeeper, base_state: State) -> i32 {
        let mut w = vec![0.0; LEGAL_ACTIONS.len()];
        let mut simulate_cnt = 0;

        loop {
            if time_keeper.is_time_over() {
                break;
            }
            for (d, &action) in LEGAL_ACTIONS.iter().enumerate() {
                let mut state = base_state.clone();
                state.advance(action);
                state.random_update();
                if state.is_done(/* END_TURN */) {
                    w[d] += state.get_score();
                } else {
                    w[d] += playout(state);
                }
            }
            simulate_cnt += 1;
        }

        let mut ret = -1;
        let mut best = -1.0;
        for (d, &score) in w.iter().enumerate() {
            if score > best {
                ret = d as i32;
                best = score;
            }
        }
        ret
    }
}

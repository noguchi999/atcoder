mod state;
mod time_keeper;

use crate::state::State;
use crate::time_keeper::TimeKeeper;
use std::io::{self, Write};

const N: usize = 100;
const K: usize = 10;

struct Board {
}

fn read() -> [Board; N] {
    [Board {}; N]

fn beam_search_action_with_time_threshold(
    state: &State,
    beam_width: usize,
    time_threshold: i64,
) -> Vec<i32> {
    vec![]
}

fn main() {
    let all_boards = read();

    let mut state = State::new();
    for i in 0..K {
        state.boards[i] = all_boards[i];
        print!("{} ", i);
    }
    println!();

    let actions = beam_search_action_with_time_threshold(&state, 64, 3000);
    for action in actions {
        print!("{}", action);
    }
    println!();
}

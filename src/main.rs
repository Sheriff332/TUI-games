use crate::storage::typedef::{TicTacToe, TripleT};

// 1. Declare your "World" (The 3 Folders)
mod io;
mod logic;
mod storage;

fn main() {
    logic::game_run::run_game(TicTacToe::new(), |_| (TripleT::O, 0, 0));
}

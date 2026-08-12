use crate::storage::grid::Grid;
use enum_dispatch::enum_dispatch;
use ratatui::prelude::*;

pub struct App {
    pub exit: bool,
    pub current_game: Option<CurrentGame>,
}

pub struct Simulation<T> {
    pub grid: Grid<T>,
    pub step: usize,
    pub completed: bool,
}

impl<T> Simulation<T> {
    pub fn new(grid: Grid<T>) -> Simulation<T> {
        Simulation {
            grid,
            step: 0,
            completed: false,
        }
    }
}

pub struct Game<T> {
    pub sim: Simulation<T>,
    pub players: usize,
    pub current_player: usize,
}

impl<T> Game<T> {
    pub fn new(sim: Simulation<T>, p: usize) -> Game<T> {
        Game {
            sim,
            players: p,
            current_player: 0,
        }
    }
}

#[enum_dispatch]
pub trait Playable: Simulable {
    type Action;
    fn step_turn(&mut self, action: Self::Action) {
        self.handle_input(action);
        self.step();
    }
    fn handle_input(&mut self, _: Self::Action);
    fn win_condition(&mut self);
    fn winner(&self) -> usize;
}

#[enum_dispatch]
pub trait Simulable {
    fn step(&mut self);
    fn display(&self) -> Text;
    fn is_over(&self) -> bool;
}

#[enum_dispatch(Simulable)]
pub enum CurrentGame {
    TicTacToe(TicTacToe),
}

#[derive(Clone)]
pub enum TripleT {
    X,
    O,
    Empty,
}
pub struct TicTacToe {
    pub name: String,
    pub game: Game<TripleT>,
}

impl TicTacToe {
    pub fn new() -> TicTacToe {
        TicTacToe {
            name: "TicTacToe".to_string(),
            game: Game::new(
                Simulation::new(Grid::from_vec(vec![vec![TripleT::Empty; 3]; 3])),
                2,
            ),
        }
    }
}

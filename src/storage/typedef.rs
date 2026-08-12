use crate::storage::grid::Grid;

pub struct App {
    pub exit: bool,
}

pub struct Simulation<T> {
    pub grid: Grid<T>,
    pub step: usize,
    pub completed: bool,
}

impl<T> Simulation<T> {
    pub fn new(grid: Grid<T>) -> Simulation<T> {
        Simulation {grid, step: 0, completed: false}
    }
}

pub struct Game<T> {
    pub sim: Simulation<T>,
    pub players: usize,
    pub current_player: usize,
}

impl<T> Game<T> {
    pub fn new(sim: Simulation<T>, p: usize) -> Game<T> {
        Game { sim, players: p, current_player: 0 }
    }
}

pub trait Playable: Simulable {
    type Action;
    fn step_turn(&mut self, action: Self::Action) {
        self.apply_action(action);
        self.step();
    }
    fn get_action(&mut self) -> Self::Action;
    fn apply_action(&mut self, _: Self::Action);
    fn win_condition(&mut self);
    fn winner(&self) -> usize;
}

pub trait Simulable {
    fn step(&mut self);
    fn display(&self);
    fn is_over(&self) -> bool;
}

#[derive(Clone)]
pub enum TripleT {
    X,
    O,
    Empty,
}
pub struct TicTacToe {
    pub game: Game<TripleT>,
}

impl TicTacToe {
    pub fn new() -> TicTacToe {
        TicTacToe { game: Game::new(Simulation::new(Grid::from_vec(vec![vec![TripleT::Empty; 3]; 3])), 2) }
    }
}
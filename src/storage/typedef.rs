pub struct Simulation<T> {
    pub grid: Vec<Vec<T>>,
    pub step: usize,
    pub completed: bool,
}

impl<T> Simulation<T> {
    pub fn new(grid: Vec<Vec<T>>) -> Simulation<T> {
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

pub trait Playable {
    type Action;
    fn step_turn(&mut self, _: Self::Action);
    fn win_condition(&mut self);
    fn is_over(&self) -> bool;
    fn winner(&self) -> usize;
}

pub trait Simulable {
    fn step_turn(&mut self);
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
        TicTacToe { game: Game::new(Simulation::new(vec![vec![TripleT::Empty; 3]; 3]), 2) }
    }
}
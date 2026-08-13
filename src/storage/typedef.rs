use crate::storage::grid::Grid;
use crossterm::event::KeyCode;
use enum_dispatch::enum_dispatch;
use ratatui::prelude::*;
use ratatui::widgets::ListState;

pub struct App {
    pub exit: bool,
    pub current_game: Option<CurrentGame>,
    pub selected_game: CurrentGame,
    pub current_menu: CurrentMenu,
    pub list_state: ListState,
    pub keys: Vec<KeyCode>,
}

impl App {
    pub fn next_item(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= NAMES.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn prev_item(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    NAMES.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }
}

#[derive(PartialEq)]
pub enum CurrentMenu {
    Left,
    Right,
    Status,
}

impl CurrentMenu {
    pub fn next(&mut self) {
        *self = {
            match self {
                CurrentMenu::Left => CurrentMenu::Right,
                CurrentMenu::Right => CurrentMenu::Status,
                CurrentMenu::Status => CurrentMenu::Left,
            }
        };
    }
    pub fn previous(&mut self) {
        *self = {
            match self {
                CurrentMenu::Left => CurrentMenu::Status,
                CurrentMenu::Right => CurrentMenu::Left,
                CurrentMenu::Status => CurrentMenu::Right,
            }
        };
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
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
    pub fn cycle_player(&mut self) {
        self.current_player = (self.current_player + 1) % self.players;
    }
}

#[enum_dispatch]
pub trait Playable: Simulable {
    fn step_turn(&mut self, action: &Vec<KeyCode>) -> Result<i32, &str> {
        let inputs = self.handle_input(action);
        if inputs == Vec::new() {
            return Err("Invalid Input");
        } //input checking
        self.win_condition(&inputs);
        if !self.step() {
            let winner = self.winner();
            return Ok(winner as i32);
        }
        Ok(-1)
    }
    fn handle_input(&mut self, _: &Vec<KeyCode>) -> Vec<u32>;
    fn win_condition(&mut self, inputs: &Vec<u32>);
    fn winner(&self) -> usize;
}

#[enum_dispatch]
pub trait Simulable {
    fn step(&mut self) -> bool; //the bool states if the sim is stepping (false if over)
    fn display(&self) -> Text;
    fn dt(&self, row: usize, col: usize) -> String; //display translate, basically match
    fn is_over(&self) -> bool;
}

#[enum_dispatch(Simulable)]
#[enum_dispatch(Playable)]
#[derive(Clone)]
pub enum CurrentGame {
    TicTacToe(TicTacToe),
}

pub const NAMES: [&str; 1] = ["TicTacToe"];

impl CurrentGame {
    pub fn name(&self) -> &str {
        match &self {
            CurrentGame::TicTacToe(_) => NAMES[0],
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum TripleT {
    X,
    O,
    Empty,
}
#[derive(Clone)]
pub struct TicTacToe {
    pub game: Game<TripleT>,
    pub piece: TripleT,
}

impl TicTacToe {
    pub fn new() -> TicTacToe {
        TicTacToe {
            game: Game::new(
                Simulation::new(Grid::from_vec(vec![vec![TripleT::Empty; 3]; 3])),
                2,
            ),
            piece: if rand::random() {
                TripleT::X
            } else {
                TripleT::O
            },
        }
    }
}

use crate::storage::grid::Grid;
use crossterm::event::KeyCode;
use enum_dispatch::enum_dispatch;
use ratatui::prelude::*;
use ratatui::widgets::ListState;

pub struct App {
    pub exit: bool,
    pub current_item: Option<MenuItem>,
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
    pub auto_run: bool,
}

impl<T> Simulation<T> {
    pub fn new(grid: Grid<T>) -> Simulation<T> {
        Simulation {
            grid,
            step: 0,
            completed: false,
            auto_run: false,
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
    fn step_turn(&mut self, action: &[KeyCode]) -> Result<i32, &str> {
        let inputs = self.handle_input(action);
        if inputs.is_empty() {
            return Err("Invalid Input");
        } //input checking
        self.win_condition(&inputs);
        if !self.step() {
            let winner = self.winner();
            return Ok(winner as i32);
        }
        Ok(-1)
    }
    fn win_condition(&mut self, inputs: &[u32]);
    fn winner(&self) -> usize;
    fn current_player(&self) -> Option<usize> {
        None
    }
}

#[enum_dispatch]
pub trait Simulable {
    fn step(&mut self) -> bool; //the bool states if the sim is stepping (false if over)
    fn step_tick(&mut self) -> bool {
        false
    }
    fn handle_input(&mut self, _: &[KeyCode]) -> Vec<u32>; //in Simulable for initial state
    fn display(&self) -> Text<'_>;
    fn help_text(&self) -> &'static str;
    fn dt(&self, row: usize, col: usize) -> String; //display translate, basically match
    fn is_over(&self) -> bool;
    fn reset(&mut self);
}

#[enum_dispatch(Simulable)]
#[enum_dispatch(Playable)]
#[derive(Clone)]
pub enum ActiveGame {
    TicTacToe(TicTacToe),
}

#[enum_dispatch(Simulable)]
pub enum ActiveSim {
    GameOfLife(GameOfLife),
}

pub const NAMES: [&str; 2] = ["TicTacToe", "Game Of Life"];

impl MenuItem {
    pub fn name(&self) -> &str {
        match &self {
            MenuItem::ActiveGame(ActiveGame::TicTacToe(_)) => NAMES[0],
            MenuItem::ActiveSim(ActiveSim::GameOfLife(_)) => NAMES[1],
        }
    }
    pub fn step_tick(&mut self) -> bool {
        match self {
            MenuItem::ActiveGame(game) => game.step_tick(),
            MenuItem::ActiveSim(sim) => sim.step_tick(),
        }
    }
}

impl ActiveSim {
    pub fn is_running(&self) -> bool {
        match self {
            ActiveSim::GameOfLife(s) => s.sim.auto_run,
        }
    }
    pub fn toggle_running(&mut self) {
        match self {
            ActiveSim::GameOfLife(s) => s.sim.auto_run = !s.sim.auto_run,
        }
    }
}

pub fn select_item(index: usize) -> Option<MenuItem> {
    match index {
        0 => Some(MenuItem::ActiveGame(
            ActiveGame::TicTacToe(TicTacToe::new()),
        )),
        1 => Some(MenuItem::ActiveSim(
            ActiveSim::GameOfLife(GameOfLife::new()),
        )),
        _ => None,
    }
}

#[enum_dispatch(Simulable)]
pub enum MenuItem {
    ActiveGame(ActiveGame),
    ActiveSim(ActiveSim),
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

#[derive(Clone, PartialEq)]
pub enum GOLCell {
    Dead,
    Alive,
}
pub struct GameOfLife {
    pub sim: Simulation<GOLCell>,
    pub buf: Simulation<GOLCell>,
    pub current_read: u8,
}

impl GameOfLife {
    pub fn new() -> GameOfLife {
        GameOfLife {
            sim: Simulation::new(Grid::from_vec(vec![vec![GOLCell::Dead; 25]; 25])),
            buf: Simulation::new(Grid::from_vec(vec![vec![GOLCell::Dead; 25]; 25])),
            current_read: 0,
        }
    }
}

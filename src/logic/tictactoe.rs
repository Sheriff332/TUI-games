use crate::storage::typedef::*;
use crossterm::event::KeyCode;
use ratatui::prelude::*;
impl Playable for TicTacToe {
    fn handle_input(&mut self, action: &Vec<KeyCode>) -> Vec<u32> {
        if action.len() == 2
            && let KeyCode::Char(r) = action[0]
            && let Some(row) = r.to_digit(10)
            && let KeyCode::Char(c) = action[1]
            && let Some(col) = c.to_digit(10)
            && self.game.sim.grid.get(row as usize, col as usize) == Some(&TripleT::Empty)
        {
            match self
                .game
                .sim
                .grid
                .set(row as usize, col as usize, self.piece)
            {
                Ok(_) => {}
                Err(_) => return Vec::new(),
            }
            vec![row, col]
        } else {
            Vec::new()
        }
    }

    fn win_condition(&mut self, inputs: &Vec<u32>) {
        let (row, col) = (inputs[0], inputs[1]);
        let condition: bool = {
            if [TripleT::X, TripleT::O]
                .contains(&self.game.sim.grid.get(row as usize, col as usize).unwrap())
                && (self
                    .game
                    .sim
                    .grid
                    .get_row(row as usize)
                    .unwrap()
                    .windows(2)
                    .all(|w| w[0] == w[1])
                    || self
                        .game
                        .sim
                        .grid
                        .get_col(col as usize)
                        .unwrap()
                        .collect::<Vec<_>>()
                        .windows(2)
                        .all(|w| w[0] == w[1])
                    || row == col
                        && self
                            .game
                            .sim
                            .grid
                            .get_diag()
                            .unwrap()
                            .collect::<Vec<_>>()
                            .windows(2)
                            .all(|w| w[0] == w[1])
                    || row + col == 2
                        && self
                            .game
                            .sim
                            .grid
                            .get_anti_diag()
                            .unwrap()
                            .collect::<Vec<_>>()
                            .windows(2)
                            .all(|w| w[0] == w[1]))
            {
                true
            } else {
                false
            }
        };
        if condition {
            self.game.sim.completed = true;
        }
    }

    fn winner(&self) -> usize {
        self.game.current_player
    }
}

impl Simulable for TicTacToe {
    fn step(&mut self) -> bool {
        if self.is_over() {
            return false;
        }
        self.game.cycle_player();
        self.piece = match self.piece {
            TripleT::X => TripleT::O,
            TripleT::O => TripleT::X,
            _ => TripleT::Empty,
        };
        self.game.sim.step += 1;
        true
    }
    fn display(&self) -> Text {
        let s = (0..3)
            .map(|row| {
                (0..3)
                    .map(|col| format!(" {} ", self.dt(row, col)))
                    .collect::<Vec<_>>()
                    .join("|")
            })
            .collect::<Vec<_>>()
            .join("\n---------\n");
        Text::from(s)
    }
    fn dt(&self, row: usize, col: usize) -> String {
        match self.game.sim.grid.get(row, col) {
            Some(TripleT::X) => "X".to_string(),
            Some(TripleT::O) => "O".to_string(),
            Some(TripleT::Empty) => " ".to_string(),
            None => "".to_string(),
        }
    }
    fn is_over(&self) -> bool {
        if self.game.sim.completed || self.game.sim.step == 9 {
            true
        } else {
            false
        }
    }
}

use crate::storage::typedef::*;
use crossterm::event::KeyCode;
use ratatui::prelude::*;
impl Playable for TicTacToe {
    fn win_condition(&mut self, inputs: &[u32]) {
        let (row, col) = (inputs[0], inputs[1]);
        let condition: bool = {
            [TripleT::X, TripleT::O].contains(
                self.game
                    .sim
                    .grid
                    .get(row as usize - 1, col as usize - 1)
                    .unwrap(),
            ) && (self
                .game
                .sim
                .grid
                .get_row(row as usize - 1)
                .unwrap()
                .windows(2)
                .all(|w| w[0] == w[1])
                || self
                    .game
                    .sim
                    .grid
                    .get_col(col as usize - 1)
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
                || row + col == 4
                    && self
                        .game
                        .sim
                        .grid
                        .get_anti_diag()
                        .unwrap()
                        .collect::<Vec<_>>()
                        .windows(2)
                        .all(|w| w[0] == w[1]))
        };
        if condition {
            self.game.sim.completed = true;
        }
    }

    fn winner(&self) -> usize {
        self.game.current_player + 1
    }
    fn current_player(&self) -> Option<usize> {
        Some(self.game.current_player)
    }
}

impl Simulable for TicTacToe {
    fn step_tick(&mut self) {
        if self.is_over() {
            return;
        }
        self.game.sim.ticks += 1;
    }

    fn parse_input(&mut self, action: &mut Vec<KeyCode>) -> Option<Vec<u32>> {
        if action.len() == 1 && action.last().unwrap() == &KeyCode::Enter && self.is_over() {
            action.clear();
            self.reset();
            return None;
        }
        if action.len() == 3
            && action.last().unwrap() == &KeyCode::Enter
            && let KeyCode::Char(r) = action[0]
            && let Some(row) = r.to_digit(10)
            && let KeyCode::Char(c) = action[1]
            && let Some(col) = c.to_digit(10)
            && row > 0
            && col > 0
            && self.game.sim.grid.get(row as usize - 1, col as usize - 1) == Some(&TripleT::Empty)
        {
            Some(vec![row, col])
        } else {
            None
        }
    }

    fn handle_input(&mut self, input: Vec<u32>) {
        let _ = self
            .game
            .sim
            .grid
            .set(input[0] as usize - 1, input[1] as usize - 1, self.piece);
        self.win_condition(&input);

        if !self.is_over() {
            self.game.cycle_player();
            self.piece = match self.piece {
                TripleT::X => TripleT::O,
                TripleT::O => TripleT::X,
                _ => TripleT::X,
            };
            self.game.sim.step += 1;
        }
    }
    fn display(&self) -> Text<'_> {
        let s: String = if !self.is_over() {
            (0..3)
                .map(|row| {
                    (0..3)
                        .map(|col| format!(" {} ", self.dt(row, col)))
                        .collect::<Vec<_>>()
                        .join("|")
                })
                .collect::<Vec<_>>()
                .join("\n---------\n")
        } else {
            if self.game.sim.completed {
                format!(
                    "The winner is Player {}!\nPress Enter to play again or Esc to go back.",
                    self.winner()
                )
            } else {
                "The game is a draw!\nPress Enter to play again or Esc to go back.".to_string()
            }
        };
        Text::from(s)
    }
    fn help_text(&self) -> &'static str {
        "Input: (1-3)column + (1-3)row"
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
        self.game.sim.completed || self.game.sim.step == 9
    }
    fn reset(&mut self) {
        *self = TicTacToe::new();
    }
}

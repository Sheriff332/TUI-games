use crate::storage::typedef::*;
use crossterm::event::KeyCode;
use ratatui::prelude::*;
impl Playable for TicTacToe {
    fn step_turn(&mut self, action: &Vec<KeyCode>) {
        // self.game
        //     .sim
        //     .grid
        //     .set(action.1, action.2, action.0)
        //     .expect("Failed to step turn");
        self.win_condition();
        self.game.sim.step += 1;
    }

    fn handle_input(&mut self, _: &Vec<KeyCode>) {
        todo!()
    }

    fn win_condition(&mut self) {
        if true {
            self.game.sim.completed = true;
        }
    }

    fn winner(&self) -> usize {
        self.game.current_player
    }
}

impl Simulable for TicTacToe {
    fn step(&mut self) {
        todo!()
    }
    fn display(&self) -> Text {
        todo!()
    }
    fn is_over(&self) -> bool {
        if self.game.sim.completed || self.game.sim.step == 9 {
            true
        } else {
            false
        }
    }
}

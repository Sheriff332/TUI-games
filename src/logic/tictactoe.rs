use crate::storage::typedef::*;
use ratatui::prelude::*;
impl Playable for TicTacToe {
    type Action = (TripleT, usize, usize);
    fn step_turn(&mut self, action: Self::Action) {
        self.game
            .sim
            .grid
            .set(action.1, action.2, action.0)
            .expect("Failed to step turn");
        self.win_condition();
        self.game.sim.step += 1;
    }

    fn handle_input(&mut self, _: Self::Action) {
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

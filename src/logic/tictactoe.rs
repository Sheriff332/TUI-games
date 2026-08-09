use crate::storage::typedef::*;
impl Playable for TicTacToe {
    type Action = (TripleT, usize, usize);
    fn step_turn(&mut self, action: Self::Action) {
        self.game.sim.grid[action.1][action.2] = action.0;
        self.win_condition();
        self.game.sim.step+=1;
    }
    fn win_condition(&mut self) {
        if true {
            self.game.sim.completed = true;
        }
    }
    fn is_over(&self) -> bool {
        if self.game.sim.completed || self.game.sim.step==9 {true}
        else {false}
    }
    fn winner(&self) -> usize {
        self.game.current_player
    }
}
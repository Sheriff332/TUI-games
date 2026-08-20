use crate::storage::typedef::{Playable, Simulable, Snake};
use crossterm::event::KeyCode;
use ratatui::prelude::Text;

impl Playable for Snake {
    fn step_turn(&mut self, action: &[KeyCode]) -> Result<i32, &str> {
        todo!()
    }

    fn win_condition(&mut self, inputs: &[u32]) {
        todo!()
    }

    fn winner(&self) -> usize {
        todo!()
    }

    fn current_player(&self) -> Option<usize> {
        todo!()
    }
}

impl Simulable for Snake {
    fn step(&mut self) -> bool {
        todo!()
    }

    fn handle_input(&mut self, _: &[KeyCode]) -> Vec<u32> {
        todo!()
    }

    fn display(&self) -> Text<'_> {
        todo!()
    }

    fn help_text(&self) -> &'static str {
        todo!()
    }

    fn dt(&self, row: usize, col: usize) -> String {
        todo!()
    }

    fn is_over(&self) -> bool {
        todo!()
    }

    fn reset(&mut self) {
        todo!()
    }
}

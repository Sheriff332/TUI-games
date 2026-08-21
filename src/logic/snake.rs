use crate::storage::typedef::{Playable, Simulable, Snake};
use crossterm::event::KeyCode;
use ratatui::prelude::Text;

impl Playable for Snake {
    fn win_condition(&mut self, _inputs: &[u32]) {
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
    fn step_tick(&mut self) {
        todo!()
    }

    fn parse_input(&mut self, _app: &mut Vec<KeyCode>) -> Option<Vec<u32>> {
        todo!()
    }

    fn handle_input(&mut self, _: Vec<u32>) {
        todo!()
    }

    fn display(&self) -> Text<'_> {
        todo!()
    }

    fn help_text(&self) -> &'static str {
        todo!()
    }

    fn dt(&self, _row: usize, _col: usize) -> String {
        todo!()
    }

    fn is_over(&self) -> bool {
        todo!()
    }

    fn reset(&mut self) {
        todo!()
    }
}

use crate::storage::typedef::{GOLCell, GameOfLife, Simulable};
use crossterm::event::KeyCode;
use ratatui::prelude::Text;

impl Simulable for GameOfLife {
    fn step(&mut self) -> bool {
        if !self.sim.auto_run {
            return false; // Toggle is off, do nothing
        }
        if self.is_over() {
            self.reset();
            return false;
        }
        for i in 0..25usize {
            for j in 0..25usize {
                let count: usize = (i.saturating_sub(1)..=(i + 1).clamp(0, 24))
                    .into_iter()
                    .map(|x| {
                        (j.saturating_sub(1)..=(j + 1).clamp(0, 24))
                            .into_iter()
                            .filter(|&y| {
                                (x != i || y != j)
                                    && self.sim.grid.get(x, y).unwrap_or(&GOLCell::Dead)
                                        == &GOLCell::Alive
                            })
                            .count()
                    })
                    .sum();
                if count == 3 {
                    self.buf
                        .grid
                        .set(i, j, GOLCell::Alive)
                        .expect("Failed to set cell");
                } else if self.sim.grid.get(i, j).unwrap_or(&GOLCell::Dead) == &GOLCell::Alive
                    && count == 2
                {
                    self.buf
                        .grid
                        .set(i, j, GOLCell::Alive)
                        .expect("Failed to set cell");
                } else {
                    self.buf
                        .grid
                        .set(i, j, GOLCell::Dead)
                        .expect("Failed to set cell");
                }
            }
        }
        self.sim.step += 1;
        self.buf.step += 1;
        std::mem::swap(&mut self.buf.grid, &mut self.sim.grid);
        true
    }

    fn step_tick(&mut self) -> bool {
        if !self.sim.auto_run {
            return false;
        }
        self.step()
    }

    fn handle_input(&mut self, action: &[KeyCode]) -> Vec<u32> {
        if !self.sim.auto_run {
            if action.is_empty() {
                return vec![0];
            }
            if let Some(s) = action
                .iter()
                .map(|x| {
                    let KeyCode::Char(r) = x else { return None };
                    Some(r)
                })
                .collect::<Option<String>>()
            {
                let mut iter = s
                    .split_whitespace()
                    .map(|x| x.parse::<u32>().unwrap_or(0))
                    .into_iter();
                let a = iter.next().unwrap_or(0);
                let b = iter.next().unwrap_or(0);
                return if iter.next() == None && a > 0 && b > 0 && a <= 25 && b <= 25 {
                    let cell = match self.sim.grid.get(a as usize - 1, b as usize - 1).unwrap() {
                        GOLCell::Dead => GOLCell::Alive,
                        GOLCell::Alive => GOLCell::Dead,
                    };
                    self.sim
                        .grid
                        .set(a as usize - 1, b as usize - 1, cell)
                        .expect("Failed to set cell");
                    vec![a, b]
                } else {
                    Vec::new()
                };
            }
        }
        Vec::new()
    }

    fn display(&self) -> Text<'_> {
        Text::from(
            (0..25)
                .map(|row| {
                    (0..25)
                        .map(|col| format!(" {} ", self.dt(row, col)))
                        .collect::<String>()
                })
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }

    fn help_text(&self) -> &'static str {
        "(1-25) + '<space>' + (1-25) to input a living cell \n Enter to start/stop simulation"
    }

    fn dt(&self, row: usize, col: usize) -> String {
        match self.sim.grid.get(row, col) {
            Some(GOLCell::Alive) => "@".to_string(),
            Some(GOLCell::Dead) => " ".to_string(),
            None => "".to_string(),
        }
    }

    fn is_over(&self) -> bool {
        self.sim.completed
    }

    fn reset(&mut self) {
        *self = GameOfLife::new();
    }
}

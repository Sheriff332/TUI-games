use crate::storage::typedef::{GOLCell, GameOfLife, Simulable};
use crossterm::event::KeyCode;
use ratatui::prelude::Text;

impl Simulable for GameOfLife {
    fn step_tick(&mut self) {
        if !self.sim.auto_run {
            return; // Toggle is off, do nothing
        }
        if self.is_over() {
            self.reset();
            return;
        }
        for i in 0..25usize {
            for j in 0..25usize {
                let count: usize = (i.saturating_sub(1)..=(i + 1).clamp(0, 24))
                    .map(|x| {
                        (j.saturating_sub(1)..=(j + 1).clamp(0, 24))
                            .filter(|&y| {
                                (x != i || y != j)
                                    && self.sim.grid.get(x, y).unwrap_or(&GOLCell::Dead)
                                        == &GOLCell::Alive
                            })
                            .count()
                    })
                    .sum();
                if count == 3
                    || (self.sim.grid.get(i, j).unwrap_or(&GOLCell::Dead) == &GOLCell::Alive
                        && count == 2)
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
        self.sim.ticks += 1;
        self.buf.ticks += 1;
        std::mem::swap(&mut self.buf.grid, &mut self.sim.grid);
    }

    fn parse_input(&mut self, action: &mut Vec<KeyCode>) -> Option<Vec<u32>> {
        if action.len() == 1 && action.last().unwrap() == &KeyCode::Enter {
            self.sim.auto_run = !self.sim.auto_run;
            action.clear();
            return None;
        } else if action.len() >= 4
            && action.len() <= 6
            && action.last().unwrap() == &KeyCode::Enter
        {
            action.pop();
            if let Some(s) = action
                .iter()
                .map(|x| {
                    let KeyCode::Char(r) = x else { return None };
                    Some(r)
                })
                .collect::<Option<String>>()
            {
                let mut iter = s.split_whitespace().map(|x| x.parse::<u32>().unwrap_or(0));
                let a = iter.next().unwrap_or(0);
                let b = iter.next().unwrap_or(0);
                return if iter.next().is_none() && a > 0 && b > 0 && a <= 25 && b <= 25 {
                    Some(vec![a, b])
                } else {
                    None
                };
            }
        }
        None
    }

    fn handle_input(&mut self, action: Vec<u32>) {
        let cell = match self
            .sim
            .grid
            .get(action[0] as usize - 1, action[1] as usize - 1)
            .unwrap()
        {
            GOLCell::Dead => GOLCell::Alive,
            GOLCell::Alive => GOLCell::Dead,
        };
        self.sim
            .grid
            .set(action[0] as usize - 1, action[1] as usize - 1, cell)
            .expect("Failed to set cell");
    }

    fn display(&self) -> Text<'_> {
        let width_chars = 25 * 2 - 1; // 25 columns separated by spaces = 49 chars
        let top_bottom = format!(" {}", "_".repeat(width_chars));

        let grid = (0..25)
            .map(|row| {
                let line = (0..25)
                    .map(|col| self.dt(row, col).to_string())
                    .collect::<Vec<_>>()
                    .join(" ");
                format!("|{}|", line)
            })
            .collect::<Vec<_>>();
        Text::from(format!(
            "{}\n{}\n{}",
            top_bottom,
            grid.join("\n"),
            top_bottom
        ))
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

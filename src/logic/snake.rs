use crate::storage::typedef::{Playable, Simulable, Snake, SnakeCell, SnakeHeadDir};
use crossterm::event::KeyCode;
use rand::RngExt;
use ratatui::prelude::Text;

impl Playable for Snake {
    fn win_condition(&mut self, _inputs: &[u32]) {
        if self.snake.len() == 625 {
            self.game.sim.completed = true;
        }
    }

    fn winner(&self) -> usize {
        0
    }

    fn current_player(&self) -> Option<usize> {
        None
    }
}

impl Simulable for Snake {
    fn step_tick(&mut self) {
        if self.is_over() {
            return;
        }
        let mut head = self.snake.pop_front().unwrap();
        let prev = head;
        match self.facing {
            SnakeHeadDir::Up => {
                if head.0 > 0
                    && *self.game.sim.grid.get(head.0 - 1, head.1).unwrap() != SnakeCell::Body
                {
                    head.0 -= 1;
                } else {
                    self.game.sim.completed = true;
                    return;
                }
            }
            SnakeHeadDir::Down => {
                if head.0 < 24
                    && *self.game.sim.grid.get(head.0 + 1, head.1).unwrap() != SnakeCell::Body
                {
                    head.0 += 1;
                } else {
                    self.game.sim.completed = true;
                    return;
                }
            }
            SnakeHeadDir::Left => {
                if head.1 > 0
                    && *self.game.sim.grid.get(head.0, head.1 - 1).unwrap() != SnakeCell::Body
                {
                    head.1 -= 1;
                } else {
                    self.game.sim.completed = true;
                    return;
                }
            }
            SnakeHeadDir::Right => {
                if head.1 < 24
                    && *self.game.sim.grid.get(head.0, head.1 + 1).unwrap() != SnakeCell::Body
                {
                    head.1 += 1;
                } else {
                    self.game.sim.completed = true;
                    return;
                }
            }
        }
        let mut popped: Option<(usize, usize)> = None;
        if *self.game.sim.grid.get(head.0, head.1).unwrap() == SnakeCell::Apple {
            let mut rng = rand::rng();
            self.apple = (rng.random_range(0..25), rng.random_range(0..25));

            if [
                // 60 -> 50 TPS (Score 2 to 20): Quick initial ramp to get moving
                2, 4, 6, 8, 10, 12, 14, 16, 18, 20,
                // 50 -> 40 TPS (Score 23 to 50): Steady pacing step-down
                23, 26, 29, 32, 35, 38, 41, 44, 47, 50,
                // 40 -> 30 TPS (Score 54 to 90): Mid-game acceleration
                54, 58, 62, 66, 70, 74, 78, 82, 86, 90,
                // 30 -> 20 TPS (Score 95 to 200): Final stretch to maximum speed cap
                95, 100, 106, 112, 118, 125, 135, 150, 170, 200,
                // HIDDEN (goes down to 12 tps i.e. 0.2s per step)
                210, 220, 230, 250, 280, 330, 410, 540,
            ]
            .contains(&self.snake.len())
            {
                self.game.sim.ticks_per_step -= 1;
            }
        } else {
            popped = self.snake.pop_back();
        }
        self.snake.push_front(prev);
        self.game
            .sim
            .grid
            .set(prev.0, prev.1, SnakeCell::Body)
            .expect("Failed to set cell");
        self.snake.push_front(head);
        self.game
            .sim
            .grid
            .set(head.0, head.1, SnakeCell::Head)
            .expect("Failed to set cell");

        self.game
            .sim
            .grid
            .set(self.apple.0, self.apple.1, SnakeCell::Apple)
            .expect("Failed to set cell");
        if let Some(p) = popped {
            self.game
                .sim
                .grid
                .set(p.0, p.1, SnakeCell::Empty)
                .expect("Failed to set cell");
        }
        self.game.sim.ticks += 1;
    }

    fn parse_input(&mut self, action: &mut Vec<KeyCode>) -> Option<Vec<u32>> {
        if action.len() == 1 {
            match action[0] {
                KeyCode::Enter => {
                    if self.is_over() {
                        action.clear();
                        self.reset();
                    }
                }
                KeyCode::Char(c) => match c {
                    'w' => return Some(vec![1]),
                    'a' => return Some(vec![2]),
                    's' => return Some(vec![3]),
                    'd' => return Some(vec![4]),
                    _ => {}
                },
                _ => {}
            }
        }
        None
    }

    fn handle_input(&mut self, input: Vec<u32>) {
        match input[0] {
            1 => self.facing = SnakeHeadDir::Up,
            2 => self.facing = SnakeHeadDir::Left,
            3 => self.facing = SnakeHeadDir::Down,
            4 => self.facing = SnakeHeadDir::Right,
            _ => {}
        }
    }

    fn display(&self) -> Text<'_> {
        let s: String = if !self.is_over() {
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

            format!("{}\n{}\n{}", top_bottom, grid.join("\n"), top_bottom)
        } else {
            format!(
                "Game Over! Your score was: {}\nPress Enter to play again or Esc to go back.",
                self.snake.len()
            )
        };
        Text::from(s)
    }

    fn help_text(&self) -> &'static str {
        "Input: W/A/S/D to move"
    }

    fn dt(&self, row: usize, col: usize) -> String {
        match self.game.sim.grid.get(row, col) {
            Some(SnakeCell::Empty) => " ".to_string(),
            Some(SnakeCell::Apple) => "@".to_string(),
            Some(SnakeCell::Body) => "*".to_string(),
            Some(SnakeCell::Head) => match self.facing {
                SnakeHeadDir::Up => "^".to_string(),
                SnakeHeadDir::Down => "v".to_string(),
                SnakeHeadDir::Left => "<".to_string(),
                SnakeHeadDir::Right => ">".to_string(),
            },
            None => "".to_string(),
        }
    }

    fn is_over(&self) -> bool {
        self.game.sim.completed
    }

    fn reset(&mut self) {
        *self = Snake::new();
    }
}

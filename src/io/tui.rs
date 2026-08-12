use crate::io::inputs::handle_events;
use crate::storage::typedef::Simulable;
use crate::storage::typedef::{App, CurrentGame};
use ratatui::Frame;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Paragraph};

impl App {
    pub fn run(&mut self, term: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
        while !self.exit {
            term.draw(|frame| self.render(frame))?;
            if handle_events()? {
                break;
            }
        }
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        use Constraint::{Length, Min};

        let vertical = Layout::vertical([Min(0), Length(3)]);
        let [main_area, status_area] = vertical.areas(area);
        let horizontal = Layout::horizontal([Length(40), Min(0)]);
        let [left_area, right_area] = horizontal.areas(main_area);
        let status_block = Block::bordered().title("Status Bar");
        let left_block = Block::bordered().title("Left");
        let right_block = Block::bordered().title("Right");
        let status_iarea = status_block.inner(status_area);
        let left_iarea = left_block.inner(left_area);
        let right_iarea = right_block.inner(right_area);
        status_block.render(status_area, buf);
        Line::from("Process Overview").render(status_iarea, buf);
        Line::from("This is the Game List").render(left_iarea, buf);
        Line::from("This is the Game Render").render(right_iarea, buf);
        left_block.render(left_area, buf);
        right_block.render(right_area, buf);
        if let Some(game) = &self.current_game {
            game.render(right_iarea, buf); // Keeps App completely ignorant of how games draw themselves
        }
    }
}

impl Widget for &CurrentGame {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.display())
            .alignment(Alignment::Center)
            .render(area, buf);
    }
}

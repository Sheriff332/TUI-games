use crate::storage::typedef::App;
use ratatui::Frame;
use ratatui::prelude::*;


impl App {
    pub fn run(&mut self, term: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
        while !self.exit {
            term.draw(|frame| self.render(frame))?;
            // if handle_events()? {
            //     break;
            // }
        }
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &App {
    fn render(self, area: Rect , buf: &mut Buffer)
    where
    Self: Sized,{
        Line::from("Process Overview").bold().render(area, buf);
    }
}
use crate::io::inputs::handle_events;
use crate::storage::typedef::App;
use crate::storage::typedef::{CurrentMenu, NAMES, Simulable};
use crate::storage::typedef::{MenuItem, Playable};
use chrono::Local;
use ratatui::layout::Constraint::Length;
use ratatui::prelude::*;
use ratatui::widgets::{Block, ListItem, Paragraph};
use ratatui::widgets::{Clear, List};

impl App {
    pub fn run(&mut self, term: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
        while !self.exit {
            (*term).draw(|frame| {
                frame.render_widget(&mut *self, frame.area());
            })?;
            if handle_events(self)? {
                break;
            }
            if let Some(item) = &mut self.current_item {
                item.step_tick();
            }
        }
        Ok(())
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        use Constraint::{Length, Min};
        let name = if let Some(game) = &self.current_item {
            game.name()
        } else {
            ""
        };

        let vertical = Layout::vertical([Min(0), Length(3)]);
        let [main_area, status_area] = vertical.areas(area);
        let horizontal = Layout::horizontal([Length(40), Min(0)]);
        let [left_area, right_area] = horizontal.areas(main_area);

        let (left_block, right_block, status_block): (Block, Block, Block);
        if self.current_menu == CurrentMenu::Left {
            left_block = Block::bordered()
                .border_style(Color::Rgb(255, 215, 0))
                .title("Game List");
        } else {
            left_block = Block::bordered().title("Game List");
        }
        if self.current_menu == CurrentMenu::Right {
            right_block = Block::bordered()
                .border_style(Color::Rgb(255, 215, 0))
                .title(name);
        } else {
            right_block = Block::bordered().title("Game");
        }
        if self.current_menu == CurrentMenu::Status {
            status_block = Block::bordered()
                .border_style(Color::Rgb(255, 215, 0))
                .title("Status");
        } else {
            status_block = Block::bordered().title("Status");
        }

        let status_iarea = status_block.inner(status_area);
        let left_iarea = left_block.inner(left_area);
        let right_iarea = right_block.inner(right_area);

        status_block.render(status_area, buf);
        Line::from(format!("{}", Local::now().format("%H:%M:%S")))
            .alignment(Alignment::Right)
            .render(status_iarea, buf);
        Line::from(
            self.keys
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join("+")
                .to_string(),
        )
        .alignment(Alignment::Center)
        .render(status_iarea, buf);

        left_block.render(left_area, buf);
        let items: Vec<ListItem> = NAMES.iter().map(|&n| ListItem::new(n)).collect();
        let list = List::new(items).highlight_symbol(">> ");
        StatefulWidget::render(list, left_iarea, buf, &mut self.list_state);

        right_block.render(right_area, buf);
        if let Some(item) = &self.current_item {
            Clear.render(right_iarea, buf);
            item.render(right_iarea, buf);
            Line::from(vec![
                Span::styled("?Help:\n", Style::default().fg(Color::Yellow)),
                Span::raw(item.help_text()),
            ])
            .render(right_iarea, buf);
            match item {
                MenuItem::ActiveSim(sim) => {
                    let str = match sim.is_running() {
                        true => "Running",
                        false => "Paused",
                    };
                    Line::from(format!("Simulation: {}", str)).render(status_iarea, buf);
                }
                MenuItem::ActiveGame(game) => {
                    if let Some(player) = game.current_player() {
                        Line::from(format!("Current player: Player {}", player + 1))
                            .render(status_iarea, buf);
                    }
                }
            }
        } else {
            let text = Text::from(
                "Press Tab/Shift+Tab to jump around\n\
            Press Enter to interact\n\
            Press↑/↓ to traverse the game list\n\
            Press q to quit"
                    .to_string(),
            );
            game_text(text, right_iarea, buf);
        }
    }
}

impl Widget for &MenuItem {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = self.display();
        game_text(text, area, buf);
    }
}
pub fn game_text(text: Text, area: Rect, buf: &mut Buffer) {
    let varea = Layout::vertical([
        Constraint::Fill(1),
        Length(text.height() as u16),
        Constraint::Fill(1),
    ])
    .split(area);
    Paragraph::new(text)
        .alignment(Alignment::Center)
        .render(varea[1], buf);
}

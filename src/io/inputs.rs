use crate::storage::typedef::MenuItem;
use crate::storage::typedef::Simulable;
use crate::storage::typedef::{App, CurrentMenu, select_item};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

//noinspection ALL
pub fn handle_events(app: &mut App) -> std::io::Result<bool> {
    if event::poll(std::time::Duration::ZERO)? {
        let reading = event::read()?;
        if app.current_menu == CurrentMenu::Right {
            match reading {
                Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Esc => {
                        app.current_item = None;
                        app.current_menu = CurrentMenu::Left;
                    }
                    KeyCode::Backspace => {
                        app.keys.pop();
                    }
                    key => {
                        if let Some(item) = app.current_item.as_mut() {
                            app.keys.push(key);
                            if let Some(input) = item.parse_input(&mut app.keys) {
                                match item {
                                    MenuItem::ActiveSim(sim) => {
                                        sim.handle_input(input);
                                    }
                                    MenuItem::ActiveGame(game) => {
                                        game.handle_input(input);
                                    }
                                }
                                app.keys.clear();
                            }
                        }
                    }
                },
                _ => {}
            }
        }
        if app.current_menu == CurrentMenu::Left {
            match reading {
                Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Down => app.list_state.select_next(),
                    KeyCode::Up => app.list_state.select_previous(),
                    KeyCode::Enter => {
                        if let Some(index) = app.list_state.selected() {
                            app.current_item = select_item(index);
                            app.current_menu = CurrentMenu::Right;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        match reading {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Char('q') => return Ok(true),
                KeyCode::Tab => app.current_menu.next(),
                KeyCode::BackTab => app.current_menu.previous(),
                // handle other key events
                _ => {}
            },
            // handle other events
            _ => {}
        }
    }
    Ok(false)
}

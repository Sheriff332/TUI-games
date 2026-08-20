use crate::storage::typedef::{App, CurrentMenu, MenuItem, select_item};
use crate::storage::typedef::{Playable, Simulable};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

//noinspection ALL
pub fn handle_events(app: &mut App) -> std::io::Result<bool> {
    if event::poll(core::time::Duration::from_millis(16))? {
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
                    KeyCode::Enter => {
                        if let Some(item) = app.current_item.as_mut() {
                            match item {
                                MenuItem::ActiveGame(game) => {
                                    if game.is_over() {
                                        game.reset();
                                    } else {
                                        let _ = game.step_turn(&app.keys);
                                    }
                                }
                                MenuItem::ActiveSim(sim) => {
                                    if sim.is_running() {
                                        sim.toggle_running();
                                    } else {
                                        if app.keys.is_empty() {
                                            sim.toggle_running();
                                        } else {
                                            sim.handle_input(&app.keys);
                                        }
                                    }
                                }
                            }
                        }
                        app.keys.clear();
                    }
                    KeyCode::Char(ch) => {
                        app.keys.push(KeyCode::Char(ch));
                    }
                    _ => {}
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

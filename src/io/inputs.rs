use crate::storage::typedef::Playable;
use crate::storage::typedef::{App, CurrentMenu};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

pub fn handle_events(app: &mut App) -> std::io::Result<bool> {
    let reading = event::read()?;
    if app.current_menu == CurrentMenu::Right {
        match reading {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Esc => {
                    app.current_menu = CurrentMenu::Left;
                }
                KeyCode::Backspace => {
                    app.keys.pop();
                }
                KeyCode::Enter => {
                    if let Some(game) = app.current_game.as_mut() {
                        game.step_turn(&app.keys);
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
                    //game_run() thingy
                    app.current_menu = CurrentMenu::Right;
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
    Ok(false)
}

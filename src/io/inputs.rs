use crate::storage::typedef::Playable;
use crate::storage::typedef::{App, CurrentMenu};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

pub fn handle_events(app: &mut App) -> std::io::Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            // KeyCode::Enter => {
            //     match app.current_menu {
            //         CurrentMenu::Left => //run_game() etc etc,
            //         CurrentMenu::Right => //step_turn etc etc,
            //         CurrentMenu::Status => //do nothing, or smth idk,
            //     }
            // }
            KeyCode::Char('q') => return Ok(true),
            KeyCode::Tab => app.current_menu.next(),
            KeyCode::BackTab => app.current_menu.previous(),
            KeyCode::Down => app.list_state.select_next(),
            KeyCode::Up => app.list_state.select_previous(),
            // handle other key events
            _ => {
                if app.current_menu == CurrentMenu::Right {
                    if let Some(game) = app.current_game.as_mut() {
                        game.handle_input(key);
                    }
                }
            }
        },
        // handle other events
        _ => {}
    }
    Ok(false)
}

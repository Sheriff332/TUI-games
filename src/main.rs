use crate::storage::typedef::{App, CurrentGame, CurrentMenu, TicTacToe};
use ratatui::widgets::ListState;

// 1. Declare your "World" (The 3 Folders)
mod io;
mod logic;
mod storage;

fn main() -> std::io::Result<()> {
    let mut term = ratatui::init();

    let mut app = App {
        selected_game: CurrentGame::TicTacToe(TicTacToe::new()),
        current_menu: CurrentMenu::Left,
        current_game: None,
        exit: false,
        list_state: ListState::default().with_selected(Some(0)),
        keys: Vec::new(),
    };
    let app_result = app.run(&mut term);

    ratatui::restore();
    app_result
}

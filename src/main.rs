use crate::storage::typedef::App;

// 1. Declare your "World" (The 3 Folders)
mod io;
mod logic;
mod storage;

fn main() -> std::io::Result<()> {
    let mut term = ratatui::init();

    let mut app = App {
        current_game: None,
        exit: false,
    };
    let app_result = app.run(&mut term);

    ratatui::restore();
    app_result
}

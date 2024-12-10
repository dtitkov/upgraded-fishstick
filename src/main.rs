mod app;

fn main() {
    let terminal = ratatui::init();
    let mut app = app::App::new();
    app.run(terminal).unwrap();
    ratatui::restore()
}


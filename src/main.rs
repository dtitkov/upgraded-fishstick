mod app;

fn main() {
    let terminal = ratatui::init();
    let app = app::App::new();
    app.run(terminal).unwrap();
    ratatui::restore()
}


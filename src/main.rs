mod test_mod;
mod sub_model_test;

fn main()  {
    test_mod::test_func();
    sub_model_test::model::test_func();
}


//
// #[derive(Default)]
// pub struct App {
//     exit: bool
// }
//
// impl App {
//     pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
//         while !self.exit {
//             terminal.draw(|frame| self.draw(frame))?;
//         }
//         Ok(())
//     }
//
//     fn draw(&self, frame: &mut Frame) {
//         frame.render_widget(self, frame.area());
//     }
// }
//
// impl Widget for &App {
//     fn render(self, area: Rect, buf: &mut Buffer) {
//         Layout::default()
//             .direction(Direction::Vertical)
//             .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
//             .split(area);
//     }
// }

//
// fn main() -> io::Result<()> {
//     // Initialize the terminal
//     let _cleanup = ratatui::init();
//     let backend = CrosstermBackend::new(io::stdout());
//     let mut terminal = Terminal::new(backend)?;
//
//     // Enable mouse capture for touchscreen interaction
//     enable_raw_mode()?;
//     crossterm::execute!(io::stdout(), EnableMouseCapture)?;
//
//     // State to track touch interactions
//     let mut touch_message = String::from("Touch the screen!");
//
//     let layout = Layout::default()
//         .direction(Direction::Vertical)
//         .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref());
//
//
//     loop {
//         // Draw the UI
//         terminal.draw(|f| {
//             let chunks = layout.split(f.area());
//
//             let block = Block::default()
//                 .title("Touchscreen Interaction")
//                 .borders(Borders::ALL);
//
//             f.render_widget(block, chunks[0]);
//
//             let button_area = Rect {
//                 x: chunks[0].x+1,
//                 y: chunks[0].y+1,
//                 width: 10,
//                 height: 3,
//             };
//
//             let button_text = Text::raw(" Btn ");
//
//             let button = Paragraph::new(button_text)
//                 .block(Block::default()
//                     .borders(Borders::ALL)
//                     .border_set(symbols::border::ROUNDED)
//                     .style(Style::default()
//                         .fg(Color::White)));
//
//
//             f.render_widget(button, button_area);
//
//             let message = Paragraph::new(touch_message.clone());
//
//             f.render_widget(message, chunks[1]);
//         })?;
//
//         match event::read()? {
//             Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
//                 let _ = exit();
//                 break Ok(());
//             }
//             Event::Mouse(mouse_event) if matches!(mouse_event.kind, MouseEventKind::Down(_)) => {
//                 touch_message = format!(
//                     "Touched at x: {}, y: {}",
//                     mouse_event.column, mouse_event.row
//                 );
//             }
//             Event::Mouse(mouse_event) if is_point_inside_rect(mouse_event.column, mouse_event.row, button_area)
//             _ => {}
//         }
//     }
// }
//
// fn is_point_inside_rect(x:u16, y:u16, rect: Rect) -> bool {
//     x >= rect.x && x <= rect.x + rect.width && y >= rect.y && y <= rect.y + rect.height
// }
//
// fn exit() -> io::Result<()> {
//     let _ = disable_raw_mode();
//     let res = crossterm::execute!(io::stdout(), DisableMouseCapture);
//     ratatui::restore();
//     res
// }

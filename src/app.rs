use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    crossterm,
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
    DefaultTerminal, Frame,
};

/// Represents the main application state.
///
/// The `App` struct contains all the necessary state information
/// required to control the application's lifecycle and runtime behavior.
pub struct App {
    /// Indicates whether the application is currently running.
    ///
    /// When set to `false`, the application will terminate its main loop
    /// and exit gracefully.
    pub running: bool,
}

impl App {
    /// Creates a new instance of the `App` struct.
    ///
    /// The `running` field is initialized to `true`.
    pub fn new() -> Self {
        Self {
            running: true,
        }
    }

    /// Runs the application.
    ///
    /// This function initializes the terminal, enters the main application loop,
    /// and renders the user interface while listening for input events.
    ///
    /// # Parameters
    /// - `terminal`: A [`DefaultTerminal`] instance for handling terminal interactions.
    ///
    /// # Returns
    /// - `Result<()>`: Returns `Ok(())` on successful execution or an error
    ///   wrapped in [`color_eyre::Result`] if something goes wrong.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.running {
            terminal.draw(|f| self.draw(f))?;
            self.handle_crossterm_event()?;
        }
        Ok(())
    }

    /// This function is responsible for rendering the user interface.
    ///
    /// It is called within the main application loop to draw UI elements to the terminal frame.
    /// You can add widgets to customize the user interface in this function.
    ///
    /// # Parameters
    /// - `frame`: A mutable reference to the [`Frame`](https://docs.rs/ratatui/latest/ratatui/struct.Frame.html)
    ///   used to render widgets in the terminal.
    ///
    /// # Example
    /// ```rust
    /// frame.render_widget(
    ///     Paragraph::new("Hello, UI!")
    ///         .block(Block::default().title("Example")),
    ///     frame.area(),
    /// );
    /// ```
    fn draw(&mut self, frame: &mut Frame) {
        let title = Line::from("Raspberry Pi touchscreen UI")
            .bold()
            .blue()
            .centered();
        let text = "Hello, Ratatui!\n\n\
            Created using https://github.com/ratatui/templates\n\
            Press `Esc`, `Ctrl-C` or `q` to stop running.";
        frame.render_widget(
            Paragraph::new(text)
                .block(Block::bordered().title(title))
                .centered(),
            frame.area(),
        )
    }

    /// Handles input events using crossterm's event system.
    ///
    /// This function continuously reads input events and acts upon them,
    /// such as key presses, mouse clicks, or terminal resize events.
    ///
    /// # Parameters
    /// - `event`: A [`crossterm::event::Event`] representing the input event.
    ///
    /// # Behavior
    /// - If the event is a key press, it delegates the handling to `on_key_event`.
    /// - Mouse and resize events are currently ignored.
    fn handle_crossterm_event(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles specific key press events.
    ///
    /// This function handles different key combinations and takes appropriate
    /// actions based on the input.
    ///
    /// # Parameters
    /// - `key`: A [`crossterm::event::KeyEvent`] representing the key press.
    ///
    /// # Behavior
    /// - Quits the application on the following key combinations:
    ///   - `Esc`
    ///   - `q`
    ///   - `Ctrl-C` or `Ctrl-c`
    /// - Can be extended to add more key press handlers.
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            KeyCode::Char('q') => self.quit(),
            // Add other key handlers here.
            _ => {}
        }
    }

    /// Quits the application by setting the running flag to false.
    ///
    /// This function changes the application state to "not running",
    /// which causes the main loop to exit and the application to terminate.
    fn quit(&mut self) {
        self.running = false;
    }
}

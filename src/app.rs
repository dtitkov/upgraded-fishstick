use color_eyre::{
    eyre::{ Context},
    Result
};
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::{ Buffer },
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Tabs, Widget},
    text::{Line, Span},
    style::{ Color },
    Frame,
    DefaultTerminal
};

use strum::{ IntoEnumIterator, EnumIter, FromRepr, Display};


use crate::{
    tabs::{ About },
    theme::THEME,
};

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum Mode {
    #[default]
    Running,
    Quit
}
#[derive(Debug, Clone, Copy, Default, Display, EnumIter, FromRepr, PartialEq, Eq)]
enum Tab {
    #[default]
    About,
}


#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct App {
    mode: Mode,
    tab: Tab,
    about_tab: About,
}

impl App {
    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.is_running() {
            terminal.draw(|f| self.draw(f))
                .wrap_err("Failed to draw terminal")?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.mode != Mode::Quit
    }

    fn draw(&self, frame: &mut Frame){
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1)
        ]);

        let [title_bar, tab, bottom_bar] = vertical.areas(area);

        Block::new().style(THEME.root).render(area, buf);

        self.render_title_bar(title_bar, buf);
        self.render_selected_tab(tab, buf);

        App::render_bottom_bar(bottom_bar, buf);
    }
}

impl App {
    fn render_title_bar(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::horizontal([
            Constraint::Min(0),
            Constraint::Length(43)
        ]);
        let [title, tabs] = layout.areas(area);

        Span::default().content("TUI application").style(THEME.app_title).render(title, buf);

        let titles = Tab::iter().map(Tab::title);

        Tabs::new(titles)
            .style(THEME.tabs)
            .highlight_style(THEME.tabs_selected)
            .select(self.tab as usize)
            .divider("")
            .padding("","")
            .render(tabs, buf);
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.handle_key_press(key),
            _ => {}
        }
        Ok(())
    }

    fn handle_key_press(&mut self, key: KeyEvent)
    {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.mode = Mode::Quit,
            _ => {}
        }
    }

    fn render_selected_tab(&self, area: Rect, buf: &mut Buffer) {
        match self.tab {
            Tab::About => self.about_tab.render(area, buf),
        };
    }

    fn render_bottom_bar(area:Rect, buf: &mut Buffer) {
        let keys = [
            ("Q/ESC", "Quit")
        ];

        let spans: Vec<_> = keys
            .iter()
            .flat_map(|(key, desc)| {
                let key = Span::styled(format!(" {key} "), THEME.key_binding.key);
                let desc = Span::styled(format!(" {desc} "), THEME.key_binding.description);
                [key, desc]
            })
            .collect();

        Line::from(spans)
            .centered()
            .style((Color::Indexed(236), Color::Indexed(232)))
            .render(area, buf);
    }

}

impl Tab {
    fn title(self) -> String {
        match self {
            Self::About => String::new()
        }
    }
}
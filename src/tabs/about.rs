use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Margin, Rect};
use ratatui::prelude::{Alignment, Layout, Widget};
use ratatui::widgets::{Block, Borders, Padding, Paragraph, Wrap};
use crate::theme::THEME;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct About{
    row_index: usize,
}

impl Widget for About {
    fn render(self, area: Rect, buf: &mut Buffer)
    {
        let horizontal = Layout::horizontal([
            Constraint::Length(34),
        ]);

        let [description] = horizontal.areas(area);
        render_crate_description(description, buf);
    }
}

fn render_crate_description(area: Rect, buf: &mut Buffer) {
    let area = area.inner(Margin{
        vertical: 4,
        horizontal: 2,
    });

    Block::new().style(THEME.content).render(area, buf);

    let text = "- cooking up terminal user interfaces -

    Ratatui is a Rust crate that provides widgets (e.g. Paragraph, Table) and draws them to the \
    screen efficiently every frame.";

    Paragraph::new(text)
        .style(THEME.description)
        .block(
            Block::new()
                .title(" Ratatui ")
                .title_alignment(Alignment::Center)
                .borders(Borders::TOP)
                .border_style(THEME.description_title)
                .padding(Padding::new(0,0,0,0)),
        )
        .wrap(Wrap{trim: true})
        .scroll((0, 0))
        .render(area, buf);
}
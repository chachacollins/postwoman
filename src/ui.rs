use ratatui::{
    layout::{self, Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::App::{App, CurrentScreen, CurrentlyEditing};

pub fn ui(frame: &mut Frame, app: &App) {
    fn centered_rect(percentage_x: u16, percentage_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percentage_y) / 2),
                Constraint::Percentage(percentage_y),
                Constraint::Percentage((100 - percentage_y) / 2),
            ])
            .split(r);
        Layout::default()
            .direction(layout::Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percentage_x) / 2),
                Constraint::Percentage(percentage_x),
                Constraint::Percentage((100 - percentage_x) / 2),
            ])
            .split(popup_layout[1])[1]
    }
    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());
    //post woman title
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "POST WOMAN",
        Style::default().fg(ratatui::style::Color::Green),
    ))
    .block(title_block);
    frame.render_widget(title, chunks[0]);
    // list already put key value pairs
    let mut list_items = Vec::<ListItem>::new();

    for key in app.pairs.keys() {
        list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{: <25} : {}", key, app.pairs.get(key).unwrap()),
            Style::default().fg(ratatui::style::Color::Yellow),
        ))));
    }

    let list = List::new(list_items);

    frame.render_widget(list, chunks[1]);
    let current_navigation_text = vec![
        // The first half of the text
        match app.current_screen {
            CurrentScreen::Main => Span::styled("Normal Mode", Style::default().fg(Color::Green)),
            CurrentScreen::Post => Span::styled("Post", Style::default().fg(Color::Yellow)),
            CurrentScreen::Get => Span::styled("Get", Style::default().fg(Color::Yellow)),
            CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
        }
        .to_owned(),
        // A white divider bar to separate the two sections
        Span::styled(" | ", Style::default().fg(Color::White)),
        // The final section of the text, with hints on what the user is editing
        {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Key => {
                        Span::styled("Editing Json Key", Style::default().fg(Color::Green))
                    }
                    CurrentlyEditing::Value => {
                        Span::styled("Editing Json Value", Style::default().fg(Color::LightGreen))
                    }
                    CurrentlyEditing::Url => {
                        Span::styled("Editing URL", Style::default().fg(Color::LightBlue))
                    }
                }
            } else {
                Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray))
            }
        },
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));
    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "(q) quit / (p) post/ (g) get",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Post => Span::styled(
                "(ESC) to cancel/(Tab) to switch boxes/enter to complete",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Get => Span::styled("(ESC) to cancel/", Style::default().fg(Color::Red)),
            CurrentScreen::Exiting => Span::styled("(q) to quit", Style::default().fg(Color::Red)),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));
    let footer_chunks = Layout::default()
        .direction(layout::Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);
    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);

    //ui post
    if let CurrentScreen::Post = app.current_screen {
        if let Some(editing) = &app.currently_editing {
            url_displayer(frame, app);
            let popup_block = Block::default()
                .title("Enter a new key-value pair")
                .borders(Borders::NONE)
                .style(Style::default().bg(Color::DarkGray));

            let area = centered_rect(60, 25, frame.area());
            frame.render_widget(popup_block, area);
            let popup_chunks = Layout::default()
                .direction(layout::Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(area);
            let mut key_block = Block::default().title("Key").borders(Borders::ALL);
            let mut value_block = Block::default().title("Value").borders(Borders::ALL);

            let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

            match editing {
                CurrentlyEditing::Key => key_block = key_block.style(active_style),
                CurrentlyEditing::Value => value_block = value_block.style(active_style),
                CurrentlyEditing::Url => value_block = value_block.style(active_style),
            };

            let key_text = Paragraph::new(app.key_input.clone()).block(key_block);
            frame.render_widget(key_text, popup_chunks[0]);

            let value_text = Paragraph::new(app.value_input.clone()).block(value_block);
            frame.render_widget(value_text, popup_chunks[1]);
        }
    }
    if let CurrentScreen::Exiting = app.current_screen {
        frame.render_widget(Clear, frame.area()); //this clears the entire screen and anything already drawn
        let popup_block = Block::default()
            .title("Y/N")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let exit_text = Text::styled("hello world mf", Style::default().fg(Color::Red));
        // the `trim: false` will stop the text from being cut off when over the edge of the block
        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rect(60, 25, frame.area());
        frame.render_widget(exit_paragraph, area);
    }
    if let CurrentScreen::Get = app.current_screen {
        //url mf
        url_displayer(frame, app);
    }
}
fn url_displayer(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());
    let url_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let url_items = &app.url;
    let url = Paragraph::new(Text::styled(
        url_items,
        Style::default().fg(ratatui::style::Color::LightCyan),
    ))
    .block(url_block);
    frame.render_widget(url, chunks[1]);
}

// Logs tab rendering
use crate::app::App;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table, TableState},
    Frame,
};
use std::io;

// Render the logs tab
pub fn render_logs_tab(f: &mut Frame<CrosstermBackend<io::Stdout>>, app: &App, area: Rect) {
    // Split the area into header, search bar (optionally shown), and log content
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3), // Header with instructions
                Constraint::Length(
                    if app.log_search_active
                        || !app.log_search_query.is_empty()
                        || app.log_filter_level.is_some()
                    {
                        3
                    } else {
                        0
                    },
                ), // Search bar (optional)
                Constraint::Min(3),    // Logs content
            ]
            .as_ref(),
        )
        .margin(1)
        .split(area);

    // Determine if search/filter bar should be shown
    let show_search_bar =
        app.log_search_active || !app.log_search_query.is_empty() || app.log_filter_level.is_some();

    // Render header with instructions
    let mut header_text = vec![
        Line::from(vec![Span::styled(
            "Execution and System Logs",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::styled("↑/↓", Style::default().fg(Color::Cyan)),
            Span::raw(" or "),
            Span::styled("j/k", Style::default().fg(Color::Cyan)),
            Span::raw(": Navigate logs/matches   "),
            Span::styled("s", Style::default().fg(Color::Cyan)),
            Span::raw(": Search   "),
            Span::styled("f", Style::default().fg(Color::Cyan)),
            Span::raw(": Filter   "),
            Span::styled("Tab", Style::default().fg(Color::Cyan)),
            Span::raw(": Switch tabs"),
        ]),
    ];

    if show_search_bar {
        header_text.push(Line::from(vec![
            Span::styled("Enter", Style::default().fg(Color::Cyan)),
            Span::raw(": Apply search   "),
            Span::styled("Esc", Style::default().fg(Color::Cyan)),
            Span::raw(": Clear search   "),
            Span::styled("c", Style::default().fg(Color::Cyan)),
            Span::raw(": Clear all filters"),
        ]));
    }

    let header = Paragraph::new(header_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .alignment(Alignment::Center);

    f.render_widget(header, chunks[0]);

    // Render search bar if active or has content
    if show_search_bar {
        let search_text = if app.log_search_active {
            format!("Search: {}█", app.log_search_query)
        } else {
            format!("Search: {}", app.log_search_query)
        };

        let filter_text = match &app.log_filter_level {
            Some(level) => format!("Filter: {}", level.to_string()),
            None => "No filter".to_string(),
        };

        let match_info = if !app.log_search_matches.is_empty() {
            format!(
                "Matches: {}/{}",
                app.log_search_match_idx + 1,
                app.log_search_matches.len()
            )
        } else if !app.log_search_query.is_empty() {
            "No matches".to_string()
        } else {
            "".to_string()
        };

        let search_info = Line::from(vec![
            Span::raw(search_text),
            Span::raw("   "),
            Span::styled(
                filter_text,
                Style::default().fg(match &app.log_filter_level {
                    Some(crate::models::LogFilterLevel::Error) => Color::Red,
                    Some(crate::models::LogFilterLevel::Warning) => Color::Yellow,
                    Some(crate::models::LogFilterLevel::Info) => Color::Cyan,
                    Some(crate::models::LogFilterLevel::Success) => Color::Green,
                    Some(crate::models::LogFilterLevel::Trigger) => Color::Magenta,
                    Some(crate::models::LogFilterLevel::All) | None => Color::Gray,
                }),
            ),
            Span::raw("   "),
            Span::styled(match_info, Style::default().fg(Color::Magenta)),
        ]);

        let search_block = Paragraph::new(search_info)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title(Span::styled(
                        " Search & Filter ",
                        Style::default().fg(Color::Yellow),
                    )),
            )
            .alignment(Alignment::Left);

        f.render_widget(search_block, chunks[1]);
    }

    // Use processed logs from background thread instead of processing on every frame
    let filtered_logs = &app.processed_logs;

    // Create a table for logs for better organization
    let header_cells = ["Time", "Type", "Message"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)));

    let header = Row::new(header_cells)
        .style(Style::default().add_modifier(Modifier::BOLD))
        .height(1);

    // Convert processed logs to table rows - this is now very fast since logs are pre-processed
    let rows = filtered_logs
        .iter()
        .map(|processed_log| processed_log.to_row());

    let content_idx = if show_search_bar { 2 } else { 1 };

    let log_table = Table::new(rows)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(Span::styled(
                    format!(
                        " Logs ({}/{}) ",
                        if filtered_logs.is_empty() {
                            0
                        } else {
                            app.log_scroll + 1
                        },
                        filtered_logs.len()
                    ),
                    Style::default().fg(Color::Yellow),
                )),
        )
        .highlight_style(Style::default().bg(Color::DarkGray))
        .widths(&[
            Constraint::Length(10),     // Timestamp column
            Constraint::Length(7),      // Log type column
            Constraint::Percentage(80), // Message column
        ]);

    // We need to convert log_scroll index to a TableState
    let mut log_table_state = TableState::default();

    if !filtered_logs.is_empty() {
        // If we have search matches, use the match index as the selected row
        if !app.log_search_matches.is_empty() {
            // Make sure we're within bounds
            let _match_index = app
                .log_search_match_idx
                .min(app.log_search_matches.len() - 1);

            // This would involve more complex logic to go from search matches to the filtered logs
            // For simplicity in this placeholder, we'll just use the scroll position
            log_table_state.select(Some(app.log_scroll.min(filtered_logs.len() - 1)));
        } else {
            // No search matches, use regular scroll position
            log_table_state.select(Some(app.log_scroll.min(filtered_logs.len() - 1)));
        }
    }

    f.render_stateful_widget(log_table, chunks[content_idx], &mut log_table_state);
}

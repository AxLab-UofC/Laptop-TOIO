use std::error::Error;
use std::io::stdout;

use crossterm::{
    cursor::Show,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};

pub type ToioUI = Option<Terminal<CrosstermBackend<std::io::Stdout>>>;

pub fn ui(
    toio_info: Vec<(String, String, String, String, String, bool)>,
    filter: Option<Vec<usize>>,
) -> impl Fn(&mut Frame) {
    return move |frame| {
        let area = frame.size();

        let rows: Vec<Row> = toio_info
            .iter()
            .enumerate()
            .map(|(i, val)| {
                let connected_color = match val.5 {
                    true => Style::new().white(),
                    false => Style::new().red(),
                };

                let battery = val.2.clone();
                let battery_color = if let Ok(level) = battery.parse::<i32>() {
                    if level == 10 || connected_color == Style::new().red() {
                        Style::new().red()
                    } else if level < 50 {
                        Style::new().yellow()
                    } else {
                        Style::new().green()
                    }
                } else if connected_color == Style::new().red() {
                    Style::new().red()
                } else {
                    Style::new().white()
                };

                Row::new(vec![
                    Span::raw(format!("{}", i)).style(connected_color),
                    Span::raw(val.0.clone()).style(connected_color),
                    Span::raw(val.1.clone()).style(connected_color),
                    Span::raw(battery).style(battery_color),
                    Span::raw(val.3.clone()).style(connected_color),
                    Span::raw(val.4.clone()).style(connected_color),
                ])
            })
            .collect();

        let instructions = Title::from(Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]));

        let widths = [
            Constraint::Length(2),
            Constraint::Length(4),
            Constraint::Length(3),
            Constraint::Length(7),
            Constraint::Length(12),
            Constraint::Length(12),
        ];

        let table = Table::new(rows, widths)
            .column_spacing(7)
            .header(
                Row::new(vec![
                    "",
                    "Name",
                    "ID",
                    "Battery",
                    "Last Update",
                    "Last Command",
                ])
                .style(Style::new().bold()),
            )
            .highlight_style(Style::new().reversed())
            .highlight_symbol(">>");

        let connected_ids: Vec<String> = toio_info.iter().map(|val| val.1.clone()).collect();
        let filter_list = match &filter {
            Some(toio_filter) => {
                let spans: Vec<Span> = toio_filter
                    .iter()
                    .enumerate()
                    .map(|(i, x)| {
                        let id_string = x.to_string();
                        let formatted_id = match i == 0 {
                            true => format!(" {} ", id_string),
                            false => format!("{} ", id_string),
                        };

                        match connected_ids.contains(&id_string) {
                            true => Span::from(formatted_id).style(Style::new().white()),
                            false => Span::from(formatted_id).style(Style::new().green()),
                        }
                    })
                    .collect();
                Title::from(spans)
            }
            None => Title::from(Line::from(" Search Mode ").style(Style::new().green())),
        };

        frame.render_widget(
            table.block(
                Block::default()
                    .title(" Laptop Toio ")
                    .title(filter_list)
                    .title(
                        instructions
                            .alignment(Alignment::Center)
                            .position(Position::Bottom),
                    )
                    .borders(Borders::ALL),
            ),
            area,
        );
    };
}

pub fn setup_terminal() -> Result<ToioUI, Box<dyn Error>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    Ok(Some(terminal))
}

pub fn exit_terminal() -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    stdout().execute(Show)?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

use std::error::Error;
use std::io::stdout;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

use crossterm::{
    cursor::Show,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};

use crate::toio::{Toio, ToioLocation};

pub type ToioUI = Option<Terminal<CrosstermBackend<std::io::Stdout>>>;

#[derive(Clone)]
pub struct ToioInfo {
    pub name: String,
    pub location_string: String,
    pub id: String,
    pub battery_string: String,
    pub last_update_string: String,
    pub last_command_string: String,
    pub connected: bool,
}

pub async fn create_toio_info(toio_guard: &Arc<RwLock<Toio>>) -> ToioInfo {
    // Read toio state
    let toio = toio_guard.read().await;
    let state = toio.state.read().await;
    let connected = toio.connected;

    // get battery level
    let battery_string = if let Some(level) = state.battery {
        format!("{}", level)
    } else {
        "N/A".to_string()
    };

    // get time of last update
    let last_update_string = if let Some(last) = state.last_update {
        if let Ok(time) = last.elapsed() {
            format_elapsed_time(time)
        } else {
            "N/A".to_string()
        }
    } else {
        "N/A".to_string()
    };

    // get time of last command
    let last_command_string = if let Some(last) = state.last_command {
        if let Ok(time) = last.elapsed() {
            format_elapsed_time(time)
        } else {
            "N/A".to_string()
        }
    } else {
        "N/A".to_string()
    };

    let location_string = if let Some(ToioLocation { x, y }) = state.curr_loc {
        format!("({}, {})", x, y).to_string()
    } else {
        "N/A".to_string()
    };

    ToioInfo {
        name: toio.name.clone(),
        id: toio.id.clone(),
        location_string: location_string,
        battery_string: battery_string,
        last_update_string: last_update_string,
        last_command_string: last_command_string,
        connected: connected,
    }
}

// Helper function to format elapsed time
fn format_elapsed_time(time: Duration) -> String {
    if time.as_millis() < 50 {
        "<50ms".to_string()
    } else if time.as_secs() < 1 {
        format!(">{}ms", time.as_millis() - (time.as_millis() % 100))
    } else {
        format!("{}s", time.as_secs())
    }
}

pub fn create_toio_ui(toio_info: Vec<ToioInfo>, filter: Option<Vec<usize>>) -> impl Fn(&mut Frame) {
    move |frame| {
        let area = frame.size();

        let rows: Vec<Row> = toio_info
            .iter()
            .enumerate()
            .map(|(i, val)| {
                let info = val.clone();
                let connected_color = match info.connected {
                    true => Style::new().white(),
                    false => Style::new().red(),
                };

                let battery = info.battery_string;
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
                    Span::raw(info.id).style(connected_color),
                    Span::raw(info.name).style(connected_color),
                    Span::raw(battery).style(battery_color),
                    Span::raw(info.location_string).style(connected_color),
                    Span::raw(info.last_update_string).style(connected_color),
                    Span::raw(info.last_command_string).style(connected_color),
                ])
            })
            .collect();

        let instructions = Title::from(Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]));

        let widths = [
            Constraint::Length(2),  // id
            Constraint::Length(3),  // id
            Constraint::Length(4),  // name
            Constraint::Length(7),  // battery
            Constraint::Length(10), // location
            Constraint::Length(12), // last update
            Constraint::Length(12), // last command
        ];

        let table = Table::new(rows, widths)
            .column_spacing(7)
            .header(
                Row::new(vec![
                    "",
                    "ID",
                    "Name",
                    "Battery",
                    "Pos",
                    "Last Update",
                    "Last Command",
                ])
                .style(Style::new().bold()),
            )
            .highlight_style(Style::new().reversed())
            .highlight_symbol(">>");

        let connected_ids: Vec<String> = toio_info.iter().map(|val| val.id.clone()).collect();
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
    }
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

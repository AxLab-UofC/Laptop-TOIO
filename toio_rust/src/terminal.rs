use crate::toio::{Toio, ToioLocation};
use crossterm::{
    cursor::Show,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};
use std::io::stdout;
use std::sync::Arc;
use std::time::Duration;
use std::{error::Error, time::SystemTime};
use tokio::sync::RwLock;

pub type ToioUI = Option<Terminal<CrosstermBackend<std::io::Stdout>>>;

const TOIO_LAYOUT: [(u16, &str); 7] = [
    (2, ""),              // toio number
    (3, "ID"),            // toio id
    (4, "Name"),          // name
    (7, "Battery"),       // battery
    (10, "Pos"),          // location
    (12, "Last Update"),  // last update
    (12, "Last Command"), // last command
];

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

    // get battery level, location, last update timestamp, and last command timestamp
    let battery_string = or_na!(Some(level) = state.battery => format!("{}", level));
    let location_string =
        or_na!(Some(ToioLocation { x, y }) = state.curr_loc => format!("({}, {})", x, y));
    let last_update_string = format_time(state.last_update);
    let last_command_string = format_time(state.last_command);

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

pub fn create_toio_ui(toio_info: Vec<ToioInfo>, filter: Option<Vec<usize>>) -> impl Fn(&mut Frame) {
    move |frame| {
        let area = frame.size();

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

        let table = Table::new(rows, TOIO_LAYOUT.map(|x| Constraint::Length(x.0)))
            .column_spacing(7)
            .header(Row::new(TOIO_LAYOUT.map(|x| x.1)).style(Style::new().bold()))
            .highlight_style(Style::new().reversed())
            .highlight_symbol(">>");

        let instructions = Title::from(Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]))
            .alignment(Alignment::Center)
            .position(Position::Bottom);

        frame.render_widget(
            table.block(
                Block::default()
                    .title(" Laptop Toio ")
                    .title(filter_list)
                    .title(instructions)
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

fn format_time(time: Option<SystemTime>) -> String {
    or_na!(Some(Ok(elapsed)) = time.map(|t| t.elapsed()) => format_elapsed_time(elapsed))
}

fn format_elapsed_time(time: Duration) -> String {
    if time.as_millis() < 50 {
        "<50ms".to_string()
    } else if time.as_secs() < 1 {
        format!(">{}ms", time.as_millis() - (time.as_millis() % 100))
    } else {
        format!("{}s", time.as_secs())
    }
}

use ratatui::{
    backend::CrosstermBackend, style::{Color, Style}, symbols, text::Line, widgets::{Axis, BarChart, Block, Borders, Chart, Dataset, GraphType}, Terminal
};
use sysinfo::System;
use std::{collections::VecDeque, thread};
use std::time::Duration;
use color_eyre::Result;
use std::io;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};

fn main() -> Result<(), io::Error> {
    let mut sys = System::new_all();
    sys.refresh_all();

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let window_size = 60;
    let cpu_count = sys.cpus().len();
    let mut cpu_history: Vec<VecDeque<f64>> = vec![VecDeque::with_capacity(window_size); cpu_count];
    for history in &mut cpu_history {
        history.extend(vec![0.0; window_size]);
    }

    loop {
        sys.refresh_cpu_usage();

        for (i, cpu) in sys.cpus().iter().enumerate() {
            cpu_history[i].pop_front();
            cpu_history[i].push_back(cpu.cpu_usage() as f64);
        }

        terminal.draw(|f| {
            let size = f.area();
            let mut cpu_data: Vec<Vec<(f64, f64)>> = Vec::new();
            for history in &cpu_history {
                let data: Vec<(f64, f64)> = history
                .iter()
                .enumerate()
                .map(|(x, &y)| (x as f64, y))
                .collect();
                cpu_data.push(data);
            }
            let datasets: Vec<Dataset> = cpu_history
                .iter()
                .enumerate()
                .map(|(i, _)| {
                    Dataset::default()
                        .name(format!("CPU{}", i))
                        .marker(symbols::Marker::Braille)
                        .graph_type(GraphType::Line)
                        .style(Style::default().fg(match i % 4 {
                            0 => Color::Yellow,
                            1 => Color::Green,
                            2 => Color::Blue,
                            3 => Color::Red,
                            _ => Color::White,
                        }))
                        .data(&cpu_data[i]) // Borrow from cpu_data
                })
                .collect();

            let chart = Chart::new(datasets)
            .block(Block::default().title("CPU Usage....").borders(Borders::ALL))
            .x_axis(
                Axis::default()
                .title("Time (s)")
                .bounds([0.0, window_size as f64])
                .labels(vec![Line::from("-60"), Line::from("0")])
            )
            .y_axis(
                Axis::default()
                .title("%")
                .bounds([0.0, 100.0])
                .labels(vec![Line::from("0"), Line::from("50"), Line::from("100")])
            );

            f.render_widget(chart, size);

        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        thread::sleep(Duration::from_secs(1));
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, thread, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

mod models;

fn main() -> Result<(), io::Error> {
    // let resp = reqwest::get("http://localhost:8000/users")
    //     .await?
    //     .json::<Vec<User>>()
    //     .await?;
    // println!("{:#?}", resp);
    //Set up terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        // let size = f.size();
        // let block = Block::default()
        //     .title("Example")
        //     .borders(Borders::ALL);
        // f.render_widget(block, size);
        ui(f);
    })?;

    thread::sleep(Duration::from_millis(5000));

    //Restore terminal
    terminal::disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(f.size());

    let block = Block::default().title("Open Tickets").borders(Borders::ALL);

    f.render_widget(block, chunks[0]);

    let paragraph = Paragraph::new("Plop with TUI")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title("Block 2")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL),
        );

    f.render_widget(paragraph, chunks[1]);

    let block = Block::default().title("Block 3").borders(Borders::ALL);

    f.render_widget(block, chunks[2]);
}

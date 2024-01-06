use std::io::{self, stdout};
use crossterm::{
    event::{self, Event, KeyCode},
    ExecutableCommand,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use ratatui::{prelude::*, widgets::*};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    let mut input_text = String::new(); 
    while !should_quit {
        terminal.draw(|frame| ui(frame, &input_text))?; 
        should_quit = handle_events(&mut input_text)?; 
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events(input_text: &mut String) -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true)
            } else if key.kind == event::KeyEventKind::Release {
                match key.code {
                    KeyCode::Char(c) => {
                        input_text.push(c);
                    }
                    KeyCode::Backspace => {
                        input_text.pop();
                    }
                    _ => {
                        return Ok(false);
                    }
                }
            }
       }
    }
    Ok(false)
}

fn ui(frame: &mut Frame, input_text: &str) { // принимаем поле с текстом как аргумент
    frame.render_widget(
        Paragraph::new(input_text) // используем поле с текстом для отображения
            .block(Block::default().title("Greeting").borders(Borders::ALL)),
        frame.size(),
    );
}

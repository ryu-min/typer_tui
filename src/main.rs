use std::io::{self, stdout};
use crossterm::{
    event::{self, Event, KeyCode},
    ExecutableCommand,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use ratatui::{prelude::*, widgets::*};
use tui_textarea::{Input, Key, TextArea};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    let mut input_text = String::new();

    let mut textarea = TextArea::default();
    textarea.set_cursor_line_style(Style::default());
    textarea.set_placeholder_text("Enter a valid float (e.g. 1.56)");

    while !should_quit {
        terminal.draw(|frame| ui(frame, &mut textarea))?;
        should_quit = handle_events(&mut textarea)?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events(text_area: &mut TextArea) -> io::Result<bool> {
    // if event::poll(std::time::Duration::from_millis(50))? {
    //     if let Event::Key(key) = event::read()? {
    //         if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
    //             return Ok(true)
    //         } else if key.kind == event::KeyEventKind::Release {
    //             match key.code {
    //                 KeyCode::Char(c) => {
    //                     text_area.input(key);
    //                 }
    //                 _ => {
    //                     return Ok(false);
    //                 }
    //             }
    //         }
    //    }
    // }

    match crossterm::event::read()?.into() {
        Input { key: Key::Esc, .. } => return Ok(true),
        input => {
            text_area.input(input);
        }
    }
    Ok(false)
}

fn ui(frame: &mut Frame, text_area: &mut TextArea) { // принимаем поле с текстом как аргумент
    frame.render_widget(
        text_area.widget(),
        frame.size(),
    );
}

use color_eyre::eyre::Result;
use crossterm::event;
use crossterm::event::Event;
use datoteka::csv_v_vektor;
use ratatui::widgets::ListState;
use ratatui::widgets::TableState;
use crate::app::Screen;
use crate::app::AppState;

mod app;
mod datoteka;
mod input;
mod knjiga;
mod ui;

/*
TODO LIST:
- [ ] UI-(ratatui)
- [X] rewrite datoteka.rs
- [ ] tracking branja glede na teden, mesec ...
- [X] exeptions
*/

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut terminal = ratatui::init();
    let mut app = AppState {
        vrstica: 0,
        screen: Screen::HomePage,
	knjige: csv_v_vektor(),
	exited: false,
	list_state: ListState::default().with_selected(Some(0)),
	table_state: TableState::default().with_selected(Some(0)),
    };
    
    loop {
	terminal.draw(|f| ui::render(f, &mut app)).unwrap();
	
	if app.exited {
	    break;
	}

	if let Event::Key(key) = event::read()? {
	    match key.code {
		event::KeyCode::Backspace => app.screen = Screen::HomePage,
		event::KeyCode::Char('q') => app.exited = true,
		event::KeyCode::Esc => app.exited = true,
		_ => app.handle_input(key.code)
	    }
	}
    }

    ratatui::restore();
    Ok(())
}

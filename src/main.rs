use color_eyre::eyre::Result;
use crossterm::event;
use crossterm::event::Event;
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
    };
    
    loop {
	terminal.draw(|f| ui::render(f, &app)).unwrap();

	if let Event::Key(key) = event::read()? {
	    if key.code == event::KeyCode::Esc {
		break;
	    }
            app.handle_input(key.code);
	}
    }

    ratatui::restore();
    Ok(())
}

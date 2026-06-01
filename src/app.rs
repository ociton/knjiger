use color_eyre::eyre::Result;
use ratatui::crossterm::event::{self, Event, KeyCode};
use ratatui::{DefaultTerminal, Frame, widgets::Paragraph};

pub struct AppState {
    pub vrstica: usize,
    pub screen: Screen,
}

pub enum Screen {
    HomePage,
    DodajKnjigoPage,
    SpremeniKnjigoPage,
    IzpisiKnjigePage,
    VnesiBranjePage,
}

pub const MAIN_MENU_VSEBINA :[&str; 3] = [
    "Dodaj knjigo",
    "Izbrisi knjigo",
    "Exit",
];

impl AppState {
    pub fn handle_input(&mut self, key: KeyCode) {
        match self.screen {
            Screen::HomePage => self.input_home_page(key),
            Screen::DodajKnjigoPage => self.input_dodaj_knjigo(key),
            Screen::SpremeniKnjigoPage => self.input_spremeni_knjigo(key),
            Screen::IzpisiKnjigePage => self.input_izpisi_knjige(key),
            Screen::VnesiBranjePage => self.input_vnesi_branje(key),
        }
    }

    fn input_home_page(&mut self, key: KeyCode) {
	match key {
            KeyCode::Down => {
                self.vrstica = (self.vrstica + 1) % MAIN_MENU_VSEBINA.len();
            }
            KeyCode::Up => {
                if self.vrstica == 0 {
                    self.vrstica = MAIN_MENU_VSEBINA.len() - 1;
                } else {
                    self.vrstica -= 1;
                }
            }
	    KeyCode::Enter => {
		if self.vrstica == 0 {
		    self.screen = Screen::DodajKnjigoPage
		} else if self.vrstica == 1 {
		    self.screen = Screen::IzpisiKnjigePage
		} else if self.vrstica == 2 {
		}},
            _ => {}
        }
    }

    fn input_dodaj_knjigo(&mut self, key: KeyCode) {}

    fn input_spremeni_knjigo(&mut self, key: KeyCode) {}

    fn input_izpisi_knjige(&mut self, key: KeyCode) {}

    fn input_vnesi_branje(&mut self, key: KeyCode) {}
}

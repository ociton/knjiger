use color_eyre::eyre::Result;
use ratatui::crossterm::event::{self, Event, KeyCode};
use ratatui::{DefaultTerminal, Frame, widgets::Paragraph};

use crate::datoteka;
use crate::knjiga::Knjiga;

pub struct AppState {
    pub vrstica: usize,
    pub screen: Screen,
    pub knjige: Vec<Knjiga>,
    pub exited: bool,
}

pub enum Screen {
    HomePage,
    DodajKnjigoPage,
    IzbrisiKnjigoPage,
    SpremeniKnjigoPage,
    IzpisiKnjigePage,
    VnesiBranjePage,
}

pub const HOME_PAGE_VSEBINA: [&str; 6] = [
    "Dodaj knjigo",
    "Izbrisi knjigo",
    "Spremeni knjgo",
    "Izpisi knjige",
    "Vnesi branje",
    "Exit",
];

impl AppState {
    pub fn handle_input(&mut self, key: KeyCode) {
        match self.screen {
            Screen::HomePage => self.input_home_page(key),
            Screen::DodajKnjigoPage => self.input_dodaj_knjigo(key),
            Screen::IzbrisiKnjigoPage => self.input_izbrisi_knjigo(key),
            Screen::SpremeniKnjigoPage => self.input_spremeni_knjigo(key),
            Screen::IzpisiKnjigePage => self.input_izpisi_knjige(key),
            Screen::VnesiBranjePage => self.input_vnesi_branje(key),
        }
    }

    fn input_home_page(&mut self, key: KeyCode) {
        match key {
            KeyCode::Down => {
                self.vrstica = (self.vrstica + 1) % HOME_PAGE_VSEBINA.len();
            }
            KeyCode::Up => {
                if self.vrstica == 0 {
                    self.vrstica = HOME_PAGE_VSEBINA.len() - 1;
                } else {
                    self.vrstica -= 1;
                }
            }
            KeyCode::Enter => match self.vrstica {
                0 => self.screen = Screen::DodajKnjigoPage,
                1 => self.screen = Screen::IzbrisiKnjigoPage,
                2 => self.screen = Screen::SpremeniKnjigoPage,
                3 => self.screen = Screen::IzpisiKnjigePage,
                4 => self.screen = Screen::VnesiBranjePage,
                5 => self.exited = true,
		_ => {}
            },
            _ => {}
        }
    }

    fn input_izbrisi_knjigo(&mut self, key: KeyCode) {}

    fn input_dodaj_knjigo(&mut self, key: KeyCode) {}

    fn input_spremeni_knjigo(&mut self, key: KeyCode) {}

    fn input_izpisi_knjige(&mut self, key: KeyCode) {}

    fn input_vnesi_branje(&mut self, key: KeyCode) {}

    fn shrani_knjigo(&mut self, knjiga: Knjiga) {
        self.knjige.push(knjiga);
        datoteka::shrani_knjige(&self.knjige)
    }

    fn zbrisi_knjigo(&mut self, index: usize) {
        self.knjige.remove(index);
        datoteka::shrani_knjige(&self.knjige);
    }
    fn posodobi_knjigo(&mut self, index: usize, knjiga: Knjiga) {
        self.knjige[index] = knjiga;
        datoteka::shrani_knjige(&self.knjige);
    }
    fn vnesi_branje(&mut self, index: usize, prebrane_strani: u32) {
        self.knjige[index].prebrano += prebrane_strani;
        datoteka::shrani_knjige(&self.knjige);
    }
}

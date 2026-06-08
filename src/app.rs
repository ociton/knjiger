use color_eyre::eyre::Result;
use ratatui::crossterm::event::{self, Event, KeyCode};
use ratatui::widgets::{ListState, TableState};
use ratatui::{DefaultTerminal, Frame, widgets::Paragraph};

use crate::{datoteka, knjiga};
use crate::knjiga::Knjiga;

#[derive(PartialEq)]
pub enum PopupPoljeDodaja {
    Naslov,
    PrebraneStrani,
    VseStrani,
}

pub struct DodajKnjigoPopup {
    pub polje: PopupPoljeDodaja,
    pub naslov: String,
    pub prebrano: String,
    pub vse: String,
    pub napaka: Option<String>,
}

pub struct AppState {
    pub vrstica: usize,
    pub screen: Screen,
    pub knjige: Vec<Knjiga>,
    pub exited: bool,
    pub list_state: ListState,
    pub table_state: TableState,
    pub dodaj_popup: DodajKnjigoPopup,
    pub dodaj_popup_viden: bool,
}

pub enum Screen {
    HomePage,
    IzpisiKnjigePage,
    VnesiBranjePage,
}

pub const HOME_PAGE_VSEBINA: [&str; 3] = [
    "Izpisi knjige",
    "Vnesi branje",
    "Exit",
];

impl DodajKnjigoPopup {
    
    pub fn new() -> Self {
	Self {
	    napaka: None,
	    naslov: String::new(),
	    polje: PopupPoljeDodaja::Naslov,
	    prebrano: String::new(),
	    vse: String::new()
	}
    }

    pub fn handle_char(&mut self, c: char) {
	self.napaka = None;
	match self.polje {
	    PopupPoljeDodaja::Naslov => self.naslov.push(c),
	    PopupPoljeDodaja::PrebraneStrani => if c.is_ascii_digit() {
		self.prebrano.push(c)
	    }
	    PopupPoljeDodaja::VseStrani => if c.is_ascii_digit() {
		self.vse.push(c)
	    }
	}
    }
    
    pub fn handle_backspace(&mut self) {
	self.napaka = None;

	match self.polje {
	    PopupPoljeDodaja::Naslov => {self.naslov.pop();}
	    PopupPoljeDodaja::PrebraneStrani => {self.prebrano.pop();}
	    PopupPoljeDodaja::VseStrani => {self.vse.pop();}
	}
    }
    
    pub fn menjaj_polje(&mut self) {
	self.napaka = None;
	self.polje = match self.polje {
	    PopupPoljeDodaja::Naslov => PopupPoljeDodaja::PrebraneStrani,
	    PopupPoljeDodaja::PrebraneStrani => PopupPoljeDodaja::VseStrani,
	    PopupPoljeDodaja::VseStrani => PopupPoljeDodaja::Naslov,
	}
    }
    
    pub fn submit(&mut self) -> Option<Knjiga> {
        if self.naslov.is_empty() {
            self.napaka = Some("Naslov ne sme biti prazen!".into());
            return None;
        }
        let prebrane = match self.prebrano.parse::<u32>() {
            Ok(n) => n,
            Err(_) => {
                self.napaka = Some("Prebrane strani mora biti število!".into());
                return None;
            }
        };
        let vse = match self.vse.parse::<u32>() {
            Ok(n) => n,
            Err(_) => {
                self.napaka = Some("Vse strani mora biti število!".into());
                return None;
            }
        };
        if prebrane > vse {
            self.napaka = Some("Prebrane strani ne morejo biti večje od vseh!".into());
            return None;
        }
        Some(Knjiga {
	    naslov: self.naslov.clone(),
	    prebrano: prebrane,
	    vse: vse,
        })
    }
    
    pub fn reset(&mut self) {
        *self = DodajKnjigoPopup::new();
    }
}
	

impl AppState {
    pub fn handle_input(&mut self, key: KeyCode) {
        match self.screen {
            Screen::HomePage => self.input_home_page(key),
            Screen::IzpisiKnjigePage => self.input_izpisi_knjige(key),
            Screen::VnesiBranjePage => self.input_vnesi_branje(key),
        }
    }

    fn input_home_page(&mut self, key: KeyCode) {
        match key {
	    KeyCode::Char('j') | KeyCode::Down => self.list_state.select_next(),
	    KeyCode::Char('k') | KeyCode::Up => self.list_state.select_previous(),
	    KeyCode::Char('q') | KeyCode::Backspace => self.exited = true, 

            KeyCode::Enter =>  match self.list_state.selected() {
                Some(0) => self.screen = Screen::IzpisiKnjigePage,
                Some(1) => self.screen = Screen::VnesiBranjePage,
                Some(2) => self.exited = true,
		_ => {}
            },
            _ => {}
        }
    }

    fn input_izpisi_knjige(&mut self, key: KeyCode) {
	if self.dodaj_popup_viden {
	    match key {
		KeyCode::Tab => self.dodaj_popup.menjaj_polje(),
		KeyCode::Backspace => self.dodaj_popup.handle_backspace(),
		KeyCode::Char(c) => self.dodaj_popup.handle_char(c),
		KeyCode::Enter => {
		    if let Some(knjiga) = self.dodaj_popup.submit() {
			self.shrani_knjigo(knjiga);
			self.dodaj_popup_viden = false;
			self.dodaj_popup.reset();
		    }
		}
		KeyCode::Esc => {
		    self.dodaj_popup_viden = false;
		    self.dodaj_popup.reset();
		}
		_ => {}
	    }
	    return;
	}
        match key {
	    KeyCode::Char('j') | KeyCode::Down => self.table_state.select_next(),
	    KeyCode::Char('k') | KeyCode::Up => self.table_state.select_previous(),
	    KeyCode::Char('l') | KeyCode::Right => self.table_state.select_next_column(),
	    KeyCode::Char('h') | KeyCode::Left => self.table_state.select_previous_column(),
	    KeyCode::Char('g') => self.table_state.select_first(),
	    KeyCode::Char('G') => self.table_state.select_last(),
	    KeyCode::Char('a') => self.dodaj_popup_viden = true,
	    KeyCode::Backspace => self.screen = Screen::HomePage,
	    KeyCode::Char('q') => self.exited = true, 
	    _ => {}
	}
    }

    
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

use color_eyre::eyre::Result;
use ratatui::crossterm::event::{self, Event, KeyCode};
use ratatui::layout::{self, Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{List, ListDirection, ListItem, ListState, Row, Table, TableState};
use ratatui::{DefaultTerminal, Frame, widgets::Paragraph};

use crate::app::AppState;
use crate::app::Screen;
use crate::datoteka::*;
use crate::input::*;
use crate::knjiga::*;
use crate::{app::*, knjiga};

pub fn render(frame: &mut Frame, app: &mut AppState) {
    match app.screen {
        Screen::HomePage => rander_home_page(frame, app),
        Screen::DodajKnjigoPage => rander_dodaj_knjigo(frame, app),
        Screen::IzbrisiKnjigoPage => rander_izbrisi_knjigo(frame, app),
        Screen::IzpisiKnjigePage => rander_izpisi_knjige(frame, app),
        Screen::SpremeniKnjigoPage => rander_spremeni_knjigo(frame, app),
        Screen::VnesiBranjePage => rander_vnesi_branje(frame, app),
    }
}

fn rander_home_page(frame: &mut Frame, app: &mut AppState) {
    let constraints = [Constraint::Length(1), Constraint::Fill(1)];
    let layout = Layout::vertical(constraints).spacing(1);
    let [top, main] = frame.area().layout(&layout);

    let title = Line::from_iter([Span::from("Knjiger").bold()]);
    let items = HOME_PAGE_VSEBINA.to_vec();

    frame.render_widget(title.centered(), top);
    rander_list(frame, app, main, items);
}

fn rander_dodaj_knjigo(frame: &mut Frame, app: &AppState) {}

fn rander_izbrisi_knjigo(frame: &mut Frame, app: &AppState) {}

fn rander_izpisi_knjige(frame: &mut Frame, app: &mut AppState) {
    let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let [top, main] = frame.area().layout(&layout);

    let title = Line::from_iter([Span::from("Izpisi knjige").bold()]);
    frame.render_widget(title.centered(), top);

    render_table(frame, main, app);
}

pub fn render_table(frame: &mut Frame, area: Rect, app: &mut AppState) {
    let header = Row::new(["Naslov", "Prebrane strani", "Vse strani"])
        .style(Style::new().bold())
        .bottom_margin(1);

    let mut rows: Vec<Row> = vec![];
    for knjiga in &app.knjige {
        rows.push(Row::new([
            knjiga.naslov.clone(),
            knjiga.prebrano.to_string(),
            knjiga.vse.to_string(),
        ]))
    }

    let widths = [
        Constraint::Percentage(60),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
    ];
    let table = Table::new(rows, widths)
        .header(header)
        .column_spacing(1)
        .style(Color::White)
        .row_highlight_style(Style::new().on_black().bold())
        .column_highlight_style(Color::Gray)
        .cell_highlight_style(Style::new().reversed().yellow())
        .highlight_symbol("> ");

    frame.render_stateful_widget(table, area, &mut app.table_state);
}

fn rander_list(frame: &mut Frame, app: &mut AppState, area: Rect, items: Vec<&str>) {
    let list = List::new(items)
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, area, &mut app.list_state);
}

//     let mut text = String::new();
//     for (i, knjiga) in app.knjige.iter().enumerate() {
// 	if i == app.vrstica {
// 	    text.push_str(&format!("> {} ~ {}/{}\n",  knjiga.naslov, knjiga.prebrano, knjiga.vse))
// 	} else {
// 	    text.push_str(&format!("  {} ~ {}/{}\n",  knjiga.naslov, knjiga.prebrano, knjiga.vse))
// 	}
//     }

//     frame.render_widget(
// 	Paragraph::new(text),
// 	frame.area(),
//     );

fn rander_spremeni_knjigo(frame: &mut Frame, app: &AppState) {}

fn rander_vnesi_branje(frame: &mut Frame, app: &AppState) {}

// pub fn home_page() {
//     let mut knjige: Vec<Knjiga> = csv_v_vektor();
//     loop {
//         println!("****************** Dobrodošli v beležniku branja ******************\n");
//         println!(" 1. Dodaj knjigo");
//         println!(" 2. Zbriši knjigo");
//         println!(" 3. Spremeni knjigo");
//         println!(" 4. Prebrane knjige");
//         println!(" 5. Vnesi branje");
//         println!(" 6. Exit");
//         print!("\n");

//         let izbira = vnesi_st();

//         match izbira {
//             1 => dodaj_knjigo_page(&mut knjige),
//             2 => izbrisi_knjigo_page(&mut knjige),
//             3 => spremeni_knjigo_page(&mut knjige),
//             4 => preberi_knjige_page(&knjige),
//             5 => belezi_branje_page(&mut knjige),
//             6 => {
//                 println!("*************************** Nasvidenje! ***************************");
//                 shrani_knjige(&mut knjige);
//                 break;
//             }
//             _ => println!(" Neprimeren vnos, poskusite ponovno:"),
//         }
//     }
// }

// fn dodaj_knjigo_page(knjige: &mut Vec<Knjiga>) {
//     println!("*************************** Dodaj Knjigo **************************\n");
//     println!(" Naslov knjige:");
//     let naslov = vnesi_string();
//     println!(" Število strani:");
//     let vse_strani = vnesi_st();
//     println!(" Število prebranih strani:");
//     let prebrane_strani = vnesi_prebrane_strani(vse_strani, 0);
//     println!();
//     let dodana_knjiga = Knjiga {
//         naslov: naslov,
//         prebrano: prebrane_strani,
//         vse: vse_strani,
//     };
//     knjige.push(dodana_knjiga.clone());
//     shrani_knjige(knjige);
//     println!(
//         " Naslov: {} \n Število prebranih strani: {} \n Število strani knjige: {} \n",
//         dodana_knjiga.naslov, dodana_knjiga.prebrano, dodana_knjiga.vse
//     );
// }

// fn preberi_knjige_page(knjige: &Vec<Knjiga>) {
//     println!("************************* Prebrane knjige *************************");

//     for knjiga in knjige {
//         println!(" Naslov: {}", knjiga.naslov);
//         println!(" Prebrane strani: {}", knjiga.prebrano);
//         println!(" Vse strani: {}", knjiga.vse);
//         println!("-------------------------------------------------------------------");
//     }
// }

// fn izbrisi_knjigo_page(knjige: &mut Vec<Knjiga>) {
//     println!("************************* Izbriši knjigo **************************\n");

//     if knjige.len() != 0 {
//         let stevilka_za_zbrisat = izberi_knjigo(knjige, "Številka knjige, ki jo želite zbrisati");

//         let zbrisana_knjiga = knjige.remove(stevilka_za_zbrisat as usize);
//         shrani_knjige(knjige);

//         println!("Zbrisana knjiga: {}", zbrisana_knjiga.naslov);
//         println!();
//     } else {
//         println!(" Ni knjig za izbrisati");
//         println!();
//     }
// }

// fn spremeni_knjigo_page(knjige: &mut Vec<Knjiga>) {
//     println!("************************* Spremeni knjigo *************************");

//     if knjige.len() != 0 {
//         let stevilka_knjige = izberi_knjigo(knjige, "Številka knjige, ki jo želite spremeniti");
//         let knjiga_stara: &Knjiga = &knjige[stevilka_knjige as usize];

//         println!(" Kaj želite spremeniti?");
//         println!(" 1. Naslov knjige ({})", knjiga_stara.naslov);
//         println!(" 2. Prebrane strani ({})", knjiga_stara.prebrano);
//         println!(" 3. Vse strani knjige ({})", knjiga_stara.vse);

//         loop {
//             let izbira = vnesi_st();
//             match izbira {
//                 1 => {
//                     println!(" Vnesite nov naslov ({}):", knjiga_stara.naslov);
//                     let nov_naslov = vnesi_string();
//                     knjige[stevilka_knjige as usize].naslov = nov_naslov;
//                     break;
//                 },
//                 2 => {
//                     println!(" Vnesite prebrane strani ({}):", knjiga_stara.prebrano);
//                     let nov_prebrane = vnesi_prebrane_strani(knjiga_stara.vse, 0);
//                     knjige[stevilka_knjige as usize].prebrano = nov_prebrane;
//                     break;
//                 },
//                 3 => {
//                     println!(" Vnesite vse strani knjige ({}):", knjiga_stara.vse);
//                     let nov_vse = vnesi_vse_strani(knjiga_stara.prebrano);
//                     knjige[stevilka_knjige as usize].vse = nov_vse;
//                     break;
//                 },
//                 _ => {
//                     println!("Neprimeren vnos, poskusite ponovno:")
//                 }
//             }
//         }

//         shrani_knjige(knjige)

//     } else {
//         println!();
//         println!(" Ni knjig, za jih spremeniti");
//         println!();
//     }
// }

// fn belezi_branje_page(knjige: &mut Vec<Knjiga>) {
//     println!("************************** Beleži branje ***************************\n");
//     if knjige.len() != 0 {
//         let prebrana_knjiga = izberi_knjigo(knjige, "Knjiga, ste jo brali:");
//         println!("Koliko strani ste prebrali");
//         let st_prebranih_strani = vnesi_prebrane_strani(
//             knjige[prebrana_knjiga as usize].vse,
//             knjige[prebrana_knjiga as usize].prebrano,
//         );

//         knjige[prebrana_knjiga as usize].prebrano += st_prebranih_strani;
//         shrani_knjige(knjige)
//     } else {
//         println!(" Ni knjig za beležiti branje");
//         println!();
//     }
// }

use color_eyre::eyre::Result;
use crossterm::event::poll;
use ratatui::crossterm::event::{self, Event, KeyCode};
use ratatui::layout::Alignment;
use ratatui::layout::{self, Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{
    Block, BorderType, Borders, Clear, List, ListDirection, ListItem, ListState, Row, Table,
    TableState,
};
use ratatui::{DefaultTerminal, Frame, widgets::Paragraph};

use crate::app::AppState;
use crate::app::Screen;
use crate::datoteka::*;
use crate::input::*;
use crate::knjiga::*;
use crate::{app::*, knjiga};

pub fn render(frame: &mut Frame, app: &mut AppState) {
    match app.screen {
        Screen::HomePage => render_home_page(frame, app),
        Screen::IzpisiKnjigePage => rander_izpisi_knjige(frame, app),
        Screen::VnesiBranjePage => rander_vnesi_branje(frame, app),
    }
}

fn render_home_page(frame: &mut Frame, app: &mut AppState) {
    let area = centered_rect(20, 5, frame.area());

    let title = Line::from_iter([Span::from("Knjiger").bold()]);

    let block = Block::default()
        .title(title)
        .title_alignment(Alignment::Center)
        // .title_bottom(" j/k - gor/dol | <enter> - izberi | q/<backspace> - zapri ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan));

    frame.render_widget(block, area);

    let inner = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .inner(area);

    let items = HOME_PAGE_VSEBINA.to_vec();
    rander_list(frame, app, inner, items);
}

fn rander_izpisi_knjige(frame: &mut Frame, app: &mut AppState) {
    let title = Line::from_iter([Span::from("Izpisi knjige").bold()]);

    let area = centered_rect(90, 40, frame.area());

    let block = Block::default()
        .title(title)
        .title_alignment(Alignment::Center)
        .title_bottom(" a - dodaj knjigo | d - zbrisi knjigo | e - uredi | j/k - gor/dol | <backspace> - nazaj ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan));

    frame.render_widget(block, area);

    let inner = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .inner(area);

    render_table(frame, inner, app);

    if app.dodaj_popup_viden {
        render_dodaj_popup(frame, app)
    } else if app.izbrisi_popup_viden {
        render_izbrisi_popup(frame, app)
    } else if app.spremeni_popup_viden {
        render_spremeni_popup(frame, app)
    }
}

fn render_spremeni_popup(frame: &mut Frame, app: &AppState) {
    let title = Line::from_iter([Span::from("Spremeni knjigo").bold()]);

    let area = bottom_rect(90, 3, frame.area());

    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(title)
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan));

    frame.render_widget(block, area);

    let inner = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan))
        .inner(area);

    let spodaj = if let Some(napaka) = &app.spremeni_popup.napaka {
        Paragraph::new(napaka.as_str())
            .style(Style::default().fg(Color::Red))
            .alignment(Alignment::Right)
    } else {
        Paragraph::new("")
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Right)
    };
    frame.render_widget(spodaj, inner);

    match app.spremeni_popup.oznaceno_polje {
        SpremeneljivkeKnjige::Naslov => render_popup_polje(
            frame,
            inner,
            "Naslov        ",
            &app.spremeni_popup.naslov,
            app.spremeni_popup.oznaceno_polje == SpremeneljivkeKnjige::Naslov,
        ),
        SpremeneljivkeKnjige::PrebraneStrani => render_popup_polje(
            frame,
            inner,
            "Prebrane strani  ",
            &app.spremeni_popup.prebrano,
            app.spremeni_popup.oznaceno_polje == SpremeneljivkeKnjige::PrebraneStrani,
        ),
        SpremeneljivkeKnjige::VseStrani => render_popup_polje(
            frame,
            inner,
            "Vse strani    ",
            &app.spremeni_popup.vse,
            app.spremeni_popup.oznaceno_polje == SpremeneljivkeKnjige::VseStrani,
        ),
    }
}

fn render_izbrisi_popup(frame: &mut Frame, app: &AppState) {
    let title = Line::from_iter([Span::from("Izbrisi knjige").bold()]);

    let area = bottom_rect(90, 3, frame.area());

    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(title)
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan));

    frame.render_widget(block, area);

    let inner = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan))
        .inner(area);

    let vsebina = Line::from(vec![
        Span::styled(
            " Za potrditev izbrisa knjige pritisnite ",
            Style::default().fg(Color::White),
        ),
        Span::styled("y", Style::default().fg(Color::Cyan).bold()),
        Span::styled(" / ", Style::default().fg(Color::White)),
        Span::styled("n", Style::default().fg(Color::Cyan).bold()),
    ])
    .left_aligned();

    frame.render_widget(vsebina, inner)
}

pub fn render_dodaj_popup(frame: &mut Frame, app: &AppState) {
    let area = centered_rect(50, 7, frame.area());

    frame.render_widget(Clear, area);

    let block = Block::default()
        .title(" Dodaj Knjigo ")
        .title_alignment(Alignment::Center)
        .title_bottom(Span::styled(
            " <tab> - naslednje | <enter> - shrani | <esc> - zapri ",
            Style::default(),
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan));
    frame.render_widget(block, area);

    let chunks = Layout::vertical([
        Constraint::Length(1), // naslov
        Constraint::Length(1), // prebrano
        Constraint::Length(1), // vsestrani
        Constraint::Length(1), // napaka
    ])
    .margin(1)
    .split(area);

    let popup = &app.dodaj_popup;

    render_popup_polje(
        frame,
        chunks[0],
        "Naslov          ",
        &popup.naslov,
        popup.polje == PopupPoljeDodaja::Naslov,
    );
    render_popup_polje(
        frame,
        chunks[1],
        "Prebrane strani ",
        &popup.prebrano,
        popup.polje == PopupPoljeDodaja::PrebraneStrani,
    );
    render_popup_polje(
        frame,
        chunks[2],
        "Vse strani      ",
        &popup.vse,
        popup.polje == PopupPoljeDodaja::VseStrani,
    );

    let spodaj = if let Some(napaka) = &popup.napaka {
        Paragraph::new(napaka.as_str())
            .style(Style::default().fg(Color::Red))
            .alignment(Alignment::Center)
    } else {
        Paragraph::new("")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center)
    };
    frame.render_widget(spodaj, chunks[3]);
}

fn render_popup_polje(frame: &mut Frame, area: Rect, prompt: &str, vsebina: &str, active: bool) {
    let prompt_style = if active {
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    let line = Line::from(vec![
        Span::styled(format!("{}: ", prompt), prompt_style),
        Span::styled(vsebina.to_string(), Style::default().fg(Color::White)),
        if active {
            Span::styled("█", Style::default().fg(Color::White))
        } else {
            Span::raw("")
        },
    ]);

    frame.render_widget(Paragraph::new(line), area);
}

fn bottom_rect(percent_x: u16, height: u16, r: Rect) -> Rect {
    let x = r.x + (r.width.saturating_sub(r.width * percent_x / 100)) / 2;
    let w = r.width * percent_x / 100;
    let y = r.y + r.height.saturating_sub(height);
    Rect::new(x, y, w, height.min(r.height))
}

fn centered_rect(percent_x: u16, height: u16, r: Rect) -> Rect {
    let x = r.x + (r.width.saturating_sub(r.width * percent_x / 100)) / 2;
    let w = r.width * percent_x / 100;
    let y = r.y + r.height.saturating_sub(height) / 2;
    Rect::new(x, y, w, height.min(r.height))
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
        .cell_highlight_style(Style::new().reversed().light_cyan())
        .highlight_symbol("> ");

    frame.render_stateful_widget(table, area, &mut app.table_state);
}

fn rander_list(frame: &mut Frame, app: &mut AppState, area: Rect, items: Vec<&str>) {
    let list = List::new(items)
        .style(Color::White)
        .highlight_style(Modifier::BOLD)
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, area, &mut app.list_state);
}

fn rander_vnesi_branje(frame: &mut Frame, app: &AppState) {}

use std::fs;

use crate::input::*;
use crate::knjiga::*;
use crate::app::AppState;

pub fn csv_v_vektor() -> Vec<Knjiga> {
    let vsebina: String = fs::read_to_string("knjige.csv").unwrap_or_default();

    let mut knjige = Vec::new();
    for vrstica in vsebina.lines() {
        let podatki: Vec<&str> = vrstica.split("|").collect();
        let knjiga = Knjiga {
            naslov: podatki[0].to_string(),
            prebrano: podatki[1].parse().expect("Napaka pri branju"),
            vse: podatki[2].parse().expect("Napaka pri branju"),
        };
        knjige.push(knjiga)
    }

    return knjige;
}

pub fn shrani_knjige(knjige: &Vec<Knjiga>) {
    let mut vsebina = String::new();
    for knjiga in knjige {
        vsebina.push_str(&format!(
            "{}|{}|{}\n",
            knjiga.naslov, knjiga.prebrano, knjiga.vse
        ));
    }
    fs::write("knjige.csv", vsebina).expect("Napaka pri pisanju v datoteko");
}

// pub fn izberi_knjigo(knjige: &mut Vec<Knjiga>, prompt: &str) -> u32 {
//     for (index, knjiga) in knjige.iter().enumerate() {
//         println!(
//             " {}. {} ~ {}/{}",
//             index, knjiga.naslov, knjiga.prebrano, knjiga.vse
//         );
//     }
//     println!();
//     println!("{}", prompt);
//     println!();
//     loop {
//         let izbrnana_knjiga = vnesi_st();
//         if izbrnana_knjiga as i64 <= knjige.len() as i64 - 1 {
//             return izbrnana_knjiga;
//         } else {
//             println!("Vnesite primerno številko:")
//         }
//     }
// }

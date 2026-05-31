use crate::datoteka::*;
use crate::input::*;
use crate::knjiga::*;

pub fn home_page() {
    let mut knjige: Vec<Knjiga> = csv_v_vektor();
    loop {
        println!("****************** Dobrodošli v beležniku branja ******************\n");
        println!(" 1. Dodaj knjigo");
        println!(" 2. Zbriši knjigo");
        println!(" 3. Spremeni knjigo");
        println!(" 4. Prebrane knjige");
        println!(" 5. Vnesi branje");
        println!(" 6. Exit");
        print!("\n");

        let izbira = vnesi_st();

        match izbira {
            1 => dodaj_knjigo_page(&mut knjige),
            2 => izbrisi_knjigo_page(&mut knjige),
            3 => spremeni_knjigo_page(&mut knjige),
            4 => preberi_knjige_page(&knjige),
            5 => belezi_branje_page(&mut knjige),
            6 => {
                println!("*************************** Nasvidenje! ***************************");
                shrani_knjige(&mut knjige);
                break;
            }
            _ => println!(" Neprimeren vnos, poskusite ponovno:"),
        }
    }
}

fn dodaj_knjigo_page(knjige: &mut Vec<Knjiga>) {
    println!("*************************** Dodaj Knjigo **************************\n");
    println!(" Naslov knjige:");
    let naslov = vnesi_string();
    println!(" Število strani:");
    let vse_strani = vnesi_st();
    println!(" Število prebranih strani:");
    let prebrane_strani = vnesi_prebrane_strani(vse_strani, 0);
    println!();
    let dodana_knjiga = Knjiga {
        naslov: naslov,
        prebrano: prebrane_strani,
        vse: vse_strani,
    };
    knjige.push(dodana_knjiga.clone());
    shrani_knjige(knjige);
    println!(
        " Naslov: {} \n Število prebranih strani: {} \n Število strani knjige: {} \n",
        dodana_knjiga.naslov, dodana_knjiga.prebrano, dodana_knjiga.vse
    );
}

fn preberi_knjige_page(knjige: &Vec<Knjiga>) {
    println!("************************* Prebrane knjige *************************");

    for knjiga in knjige {
        println!(" Naslov: {}", knjiga.naslov);
        println!(" Prebrane strani: {}", knjiga.prebrano);
        println!(" Vse strani: {}", knjiga.vse);
        println!("-------------------------------------------------------------------");
    }
}

fn izbrisi_knjigo_page(knjige: &mut Vec<Knjiga>) {
    println!("************************* Izbriši knjigo **************************\n");

    if knjige.len() != 0 {
        let stevilka_za_zbrisat = izberi_knjigo(knjige, "Številka knjige, ki jo želite zbrisati");

        let zbrisana_knjiga = knjige.remove(stevilka_za_zbrisat as usize);
        shrani_knjige(knjige);

        println!("Zbrisana knjiga: {}", zbrisana_knjiga.naslov);
        println!();
    } else {
        println!(" Ni knjig za izbrisati");
        println!();
    }
}

fn spremeni_knjigo_page(knjige: &mut Vec<Knjiga>) {
    println!("************************* Spremeni knjigo *************************");

    if knjige.len() != 0 {
        let stevilka_knjige = izberi_knjigo(knjige, "Številka knjige, ki jo želite spremeniti");
        let knjiga_stara: &Knjiga = &knjige[stevilka_knjige as usize];

        println!(" Vnesite nov naslov (prejšni: {})", knjiga_stara.naslov);
        let naslov_nov = vnesi_string();
        println!(" Vnesite število vseh strani (prej: {})", knjiga_stara.vse);
        let vse_strani_nov = vnesi_st();
        println!(
            " Vnesite število prebranih strani (prej: {})",
            knjiga_stara.prebrano
        );
        let prebrane_nov = vnesi_prebrane_strani(vse_strani_nov, 0);

        let knjiga_nov = Knjiga {
            naslov: naslov_nov,
            prebrano: prebrane_nov,
            vse: vse_strani_nov,
        };

        knjige[stevilka_knjige as usize] = knjiga_nov;
        shrani_knjige(knjige)
    } else {
        println!();
        println!(" Ni knjig, za jih spremeniti");
        println!();
    }
}

fn belezi_branje_page(knjige: &mut Vec<Knjiga>) {
    println!("************************** Beleži branje ***************************\n");
    if knjige.len() != 0 {
        let prebrana_knjiga = izberi_knjigo(knjige, "Knjiga, ste jo brali:");
        println!("Koliko strani ste prebrali");
        let st_prebranih_strani = vnesi_prebrane_strani(
            knjige[prebrana_knjiga as usize].vse,
            knjige[prebrana_knjiga as usize].prebrano,
        );

        knjige[prebrana_knjiga as usize].prebrano += st_prebranih_strani;
        shrani_knjige(knjige)
    } else {
        println!(" Ni knjig za beležiti branje");
        println!();
    }
}

use std::io;

pub fn vnesi_st() -> u32 {
    loop {
        let mut x = String::new();
        io::stdin().read_line(&mut x).expect("Faled to input");
        match x.trim().parse() {
            Ok(x) => return x,
            Err(_) => println!(" Sprejemljiv vnos le številk\n Poskusi znova:"),
        }
    }
}

pub fn vnesi_string() -> String {
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Faled to input");
    return x.trim().to_string();
}

pub fn vnesi_prebrane_strani(vse_strani: u32, prej_prebrane_strani: u32) -> u32 {
    loop {
        let mut nove_prebrane_strani = String::new();
        io::stdin()
            .read_line(&mut nove_prebrane_strani)
            .expect("Faled to input");
        match nove_prebrane_strani.trim().parse() {
            Ok(nove_prebrane_strani) => {
                if nove_prebrane_strani + prej_prebrane_strani <= vse_strani {
                    return nove_prebrane_strani;
                } else {
                    println!(
                        "Stevilo prebranih strani nesme biti večje od vseh strani knjige! \nPoiskusi še enkrat:"
                    );
                }
            }
            Err(_) => {
                println!(" Sprejemljive le številke,\n Poskusi še enkrat:")
            }
        }
    }
}

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

pub fn vnesi_vse_strani(prebrane_strani: u32) -> u32 {
    loop {
        let mut nove_vse_strani = String::new();
        io::stdin()
            .read_line(&mut nove_vse_strani)
            .expect("Faled to input");
        match nove_vse_strani.trim().parse() {
            Ok(nove_vse_strani) => {
                if nove_vse_strani >= prebrane_strani {
                    return nove_vse_strani;
                } else {
                    println!(
                        "Stevilo vseh strani, nesme biti manjše od prebranih strani ({}) \nPoiskusi še enkrat:", prebrane_strani);
                }
            }
            Err(_) => {
                println!(" Sprejemljive le številke,\n Poskusi še enkrat:")
            }
        }
    }
}

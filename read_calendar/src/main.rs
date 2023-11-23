use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "src/calendar.txt";
    
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Errore durante l'apertura del file: {}", e);
            return;
        }
    };

    let reader = BufReader::new(file);
    let mut hours = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let parts = line.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
                hours.push(parts);
            }
            Err(e) => {
                eprintln!("Errore durante la lettura della riga: {}", e);
            }
        }
    }

    let bounds = hours.first().unwrap_or(&vec![]).to_vec();
    let schedule = hours.iter().skip(1).cloned().collect::<Vec<Vec<String>>>();

    println!("Bounds:");
    println!("{:?}", bounds);
    println!("Schedule:");
    println!("{:?}", schedule);
}

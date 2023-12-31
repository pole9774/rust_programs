use binary_search::{find, find_iter, find_iter_g};
use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    keyval: i32,
    #[arg(short, long)]
    filename: String,

    #[arg(short, long, default_value_t = 2)]
    count: u8,
}

fn main() {

    let val = find_iter_g(&[1, 3, 4, 6, 8, 9, 11], 6).map_or(-1i32, |x| (x) as i32);
    println!("{}", val);

    let args = Args::parse();

    let file_name = args.filename;

    let file = match File::open(file_name) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Errore durante l'apertura del file: {}", e);
            return;
        }
    };

    let reader = BufReader::new(file);

    let mut numbers = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                // Prova a convertire la riga in un intero
                match line.trim().parse::<i32>() {
                    Ok(number) => {
                        // Se la conversione ha successo, aggiungi l'intero al vettore
                        numbers.push(number);
                    }
                    Err(e) => {
                        eprintln!("Errore durante la conversione della riga in un intero: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Errore durante la lettura della riga: {}", e);
            }
        }
    }

    let key = args.keyval;
    println!("{}", find(&numbers, key).map_or(-1i32, |x| (x) as i32));
    
}


// cargo run -- --keyval 12 --filename "src/array.txt"
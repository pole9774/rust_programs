use minesweeper::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    rows: usize,
    #[arg(short, long)]
    pcols: usize,
    #[arg(short, long)]
    minefield: String,
    #[arg(short, long, default_value_t = 3)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    if args.minefield.len() != args.rows * args.pcols {
        panic!("Wrong dimensions!");
    }

    let mut s = Vec::new();
    let mut rs = Vec::<&str>::with_capacity(s.len());
    
    for i in 0..args.rows {
        let mut tmp = String::new();
        for j in 0..args.pcols {
            tmp.push(args.minefield.chars().nth(i*args.pcols + j).unwrap());
        }
        s.push(tmp);
    }

    for r in &s {
        rs.push(r);
    }

    println!("{:?}", annotate(&rs));
}

// cargo run -- --rows=3 --pcols=3 --minefield="*  *  *  "
use clock::Clock;

fn main() {
    let mut c = Clock::new(0, -84);
    println!("{}", c);
    c = c.add_minutes(-30);
    println!("{}", c);

    let clock = Clock::new(2, 20).add_minutes(-3000);
    println!("{}", clock);

    if c == clock {
        println!("Uguali");
    } else {
        println!("Diversi");
    }

    let c1 = c - 30;
    println!("{}", c1);

    let c2 = clock + 30;
    println!("{}", c2);
}
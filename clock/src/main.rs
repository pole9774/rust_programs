use clock::Clock;

fn main() {
    let c = Clock::new(2, 32);
    let c1 = Clock::new(3, 11);

    println!("{}", c + 2271);
    println!("{}", c1.add_minutes(11));
}
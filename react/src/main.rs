use react::*;

fn main() {
    let mut reactor = Reactor::new();
    let input = reactor.create_input(10);
    println!("{:?}", reactor.value(CellId::Input(input)));
    reactor.set_value(input, 4);
    println!("{:?}", reactor.value(CellId::Input(input)));
}
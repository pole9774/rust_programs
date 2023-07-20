fn make_box(a: i32) -> Box<(i32, i32)> {
    let r = Box::new((a, 1));
    return r;
}

fn main() {
    let v = 123;
    let mut w = v;
    let t = (7.5, true);
    let mut c = (9.0, 1.3);
    w = 42;
    println!("{}", w);
    println!("{}", v);
    println!("{:?}", t);
    c.0 = 5.42;
    c.1 = 7.77;
    println!("{:?}", c);

    let mut b = Box::new((1, 3));
    println!("{:?}", b);
    b.1 = 5;
    println!("{:?}", b);

    let m = make_box(51);
    println!("{:?}", m);
}

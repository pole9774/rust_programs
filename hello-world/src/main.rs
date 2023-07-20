fn make_box(a: i32) -> Box<(i32, i32)> {
    let r = Box::new((a, 1));
    return r;
}

fn add_numbers(x: i32, y: i32) -> i32 {
    x + y
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

    let a = [1,2,3,4,5];
    let a1 = [0;5];

    println!("{:?} --- {:?}", a, a1);
    println!("{}", a.len());
    println!("{}", a[3]);

    let s1 = &a[0..2];
    let s2 = &a[2..];
    println!("{:?}", s1);
    println!("{:?}", s2);

    let mut vec = Vec::new();
    vec.push(2);
    vec.push(4);
    println!("{:?}", vec);
    println!("{}", vec[1]);

    println!("{}", add_numbers(42, 11));
    println!("Hello world");
}

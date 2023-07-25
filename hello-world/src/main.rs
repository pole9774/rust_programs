fn test(v: &[&str]) {
    for a in v {
        println!("{}", a);
    }
}

fn main() {
    let mut ground = Vec::new();
    let s = "aaaabbbbcccc".to_string();

    for a in 0..3 {
        let tmp = &s[a*4..(a+1)*4];
        ground.push(tmp);
    }
    test(&ground);
}

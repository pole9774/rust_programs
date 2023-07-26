use std::collections::HashMap;

fn test(v: &[&str]) {
    for a in v {
        println!("{}", a);
    }
}

fn generator(prefix: &str) -> impl FnMut() -> String {
    let mut i = 0;
    let b = prefix.to_string();
    move || {i+=1; format!("{}{}",b,i)}
}

fn main() {
    let mut ground = Vec::new();
    let s = "aaaabbbbcccc".to_string();

    for a in 0..3 {
        let tmp = &s[a*4..(a+1)*4];
        ground.push(tmp);
    }
    test(&ground);


    let s1 = "098 928 0982";
    let sep = ' ';

    let a1 = s1.chars().filter(|c| { *c == sep }).count();
    println!("{}", a1);

    let b1 = s1.chars().filter(|c| { *c == sep }).count();
    println!("{}", b1);


    let mut f = generator("id_");

    for _ in 1..6 {
        println!("{}",f());
    }


    let v = vec![5, 10, 14, 12, 15];
    let mut names = vec!["Aria".to_string(), "Prova".to_string(), "Terra".to_string()];

    /*
    let mut is_ok = true;

    for x in v {
        if x < 6 || x > 15 {
            is_ok = false;
        }
    }
    */

    // let is_ok = v.iter().all(|x| { *x >= 6 && *x <= 15 });

    let is_ok = v.iter().any(|x| { *x >= 6 && *x <= 15 });

    if is_ok {
        println!("Ok");
    } else {
        eprintln!("Not ok");
    }

    // let v2: Vec<(&i32, &&str)> = v.iter().zip(names.iter()).collect();
    let v2 = v.iter().zip(names.iter()).collect::<Vec<(_,_)>>();

    for elem in v2 {
        println!("{} - {}", elem.0, elem.1);
    }

    for s in names.iter_mut() {
        s.push_str("-1");
    }

    for s in names.into_iter() {
        println!("{}", s);
    }


    let v3 = vec![1, 2, 3, 4, 5];

    // let v4 = v3.iter().map(|x| {(x * 2, x - 1)}).collect::<Vec<(i32,i32)>>();
    // let v4: Vec<(i32,i32)> = v3.iter().map(|x| {(x * 2, x - 1)}).collect();
    // let v4: Vec<_> = v3.iter().filter(|x| {*x % 2 == 0}).collect();

    // let v4: Vec<_> = v3.iter().filter_map(|x| { if x % 2 == 0 { Some(x*x) } else { None } }).collect();
    let v4 = v3.iter().filter_map(|x| { if x % 2 == 0 { Some(x*x) } else { None } }).collect::<Vec<_>>();

    println!("{:?}", v4);

}

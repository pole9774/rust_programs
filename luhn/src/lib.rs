pub fn is_valid(code: &str) -> bool {
    let mut s = String::new();
    let mut sum = 0;

    for c in code.chars() {
        match c {
            c if c.is_digit(10) => s.push(c),
            ' ' => (),
            _ => return false
        }
    }

    if s.len() < 2 {
        return false;
    }

    for (i, c) in s.chars().rev().enumerate() {
        let n = c.to_digit(10).unwrap();

        sum += if i % 2 == 0 {
            n
        } else if n * 2 > 9 {
            n * 2 - 9
        } else {
            n * 2
        };
    }

    sum % 10 == 0
}

// cargo test
// cargo test -- --ignored
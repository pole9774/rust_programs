// Determines whether the supplied string is a valid ISBN number

/*
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut t = 0;
    let mut w = 10;
    for ch in isbn.chars() {
        match ch {
            '-' => continue,
            'X' if w==1 => t+=10,
            '0'..='9' => t += ch.to_digit(10).unwrap() * w,
            _ => return false
        }
        if w == 0 { return false; }
        w -= 1;
    }
    t % 11 == 0 && w == 0
} 
*/

pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut t = 0;
    let mut w = 10;
    let s = isbn.replace("-", "");
    if s.chars().count() != 10 {
        return false;
    }
    for ch in isbn.chars() {
        match ch {
            '-' => continue,
            'X' if w==1 => t+=10,
            '0'..='9' => t += ch.to_digit(10).unwrap() * w,
            _ => return false
        }
        w -= 1;
    }
    t % 11 == 0
} 

// cargo test
// cargo test -- --ignored

use binary_search::{find, find_iter};

fn main() {
    let val = find(&[1, 3, 4, 6, 8, 9, 11], 9).map_or(-1i32, |x| (x) as i32);
    println!("{}", val);
}
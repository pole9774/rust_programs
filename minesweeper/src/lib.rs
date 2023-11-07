fn count_ad(minefield: &[&str], row: usize, col: usize) -> u32 {
    let r = minefield.len();
    let c = minefield[0].len();
    let mut res = 0;

    let mut start_row = 0;
    let mut start_col = 0;

    if row > 0 {
        start_row = row - 1;
    }
    if col > 0 {
        start_col = col - 1;
    }

    for i in start_row ..= (row+1).min(r-1) {
        for j in start_col ..= (col+1).min(c-1) {
            if i != row || j != col {
                if minefield[i].as_bytes()[j] == '*' as u8 {
                    res += 1;
                }
            }
        }
    }
    res
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let r = minefield.len();
    
    if r == 0 {
        return Vec::new();
    }

    let c = minefield[0].len();
    let mut res = Vec::new();

    if c == 0 {
        res.push("".to_string());
        return res;
    }

    for (i, row) in minefield.iter().enumerate() {
        let mut new_row = String::new();
        for (j, ch) in row.as_bytes().iter().enumerate() {
            if *ch == '*' as u8 {
                new_row.push('*');
            } else {
                let n = count_ad(minefield, i, j);
                match n {
                    0 => new_row.push(' '),
                    ad @ 1..=8 => new_row.push_str(&format!("{}", ad)),
                    _ => panic!("Invalid count")
                }
            }
        }
        res.push(new_row);
    }
    res
}

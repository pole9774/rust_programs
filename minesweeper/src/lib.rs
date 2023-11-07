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
                if minefield[i].chars().nth(j).unwrap() == '*' {
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
        for (j, ch) in row.chars().enumerate() {
            let n = count_ad(minefield, i, j);
            let tmp = if ch == ' ' && n > 0 {
                n.to_string().chars().nth(0).unwrap()
            } else {
                ch
            };
            new_row.push(tmp);
        }
        res.push(new_row);
    }
    res
}

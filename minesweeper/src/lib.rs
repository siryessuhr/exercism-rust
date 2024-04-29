pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '*' => '*',
                    _ => {
                        let mut count = 0;
                        for x in -1..=1 {
                            for y in -1..=1 {
                                if x == 0 && y == 0 {
                                    continue;
                                }
                                let i = i as i32 + x;
                                let j = j as i32 + y;
                                if i >= 0
                                    && i < minefield.len() as i32
                                    && j >= 0
                                    && j < row.len() as i32
                                    && minefield[i as usize].chars().nth(j as usize) == Some('*')
                                {
                                    count += 1;
                                }
                            }
                        }
                        if count > 0 {
                            std::char::from_digit(count, 10).unwrap()
                        } else {
                            ' '
                        }
                    }
                })
                .collect()
        })
        .collect()
}

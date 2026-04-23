use std::collections::HashMap;

struct Tile {
    count: i32,
    is_flower: bool,
}

pub fn annotate(garden: &[&str]) -> Vec<String> {
    let mut counts = HashMap::new();

    for (i, row) in garden.iter().enumerate() {
        for (j, c) in row.chars().enumerate() {
            let is_flower = match c {
                ' ' => false,
                '*' => true,
                _ => false,
            };

            let central = counts.entry((i as isize, j as isize)).or_insert(Tile {
                count: 0,
                is_flower,
            });

            central.is_flower = is_flower;

            if is_flower {
                for k in -1isize..=1 {
                    for s in -1isize..=1 {
                        let update =
                            counts
                                .entry((i as isize + k, j as isize + s))
                                .or_insert(Tile {
                                    count: 0,
                                    is_flower: false,
                                });
                        update.count += 1
                    }
                }
            }
        }
    }

    let mut results: Vec<String> = Vec::new();

    if garden.is_empty() {
        return results;
    }

    for i in 0..garden.len() {
        let mut row_result = String::new();
        for j in 0..garden[0].len() {
            let tile = counts.get(&(i as isize, j as isize));
            if let Some(t) = tile {
                if t.is_flower {
                    row_result.push('*');
                } else if t.count == 0 {
                    row_result.push(' ');
                } else {
                    row_result.push_str(&t.count.to_string())
                }
            }
        }
        results.push(row_result);
    }

    results
}

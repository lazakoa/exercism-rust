/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut values = Vec::new();

    for c in code.chars() {
        match c {
            '0' => values.push(0),
            '1' => values.push(1),
            '2' => values.push(2),
            '3' => values.push(3),
            '4' => values.push(4),
            '5' => values.push(5),
            '6' => values.push(6),
            '7' => values.push(7),
            '8' => values.push(8),
            '9' => values.push(9),
            ' ' => (),
            _ => return false,
        }
    }

    if values.len() < 2 {
        return false;
    }

    values.reverse();

    let mut doubling_flag = false;

    for value in values.iter_mut() {
        if doubling_flag {
            *value *= 2;
            if *value > 9 {
                *value -= 9
            }
        }
        doubling_flag = !doubling_flag;
    }

    let sum: i32 = values.into_iter().sum();

    sum % 10 == 0
}

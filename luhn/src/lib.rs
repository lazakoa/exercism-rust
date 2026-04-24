/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if !code.chars().all(|x| x.is_numeric() || x == ' ') {
        return false;
    }

    let values: Vec<_> = code.chars().filter_map(|x| x.to_digit(10)).collect();

    if values.len() < 2 {
        return false;
    }

    let mut doubling_flag = false;

    let sum: u32 = values
        .iter()
        .rev()
        .map(|x| {
            let mut temp = *x;
            if doubling_flag {
                temp *= 2;
                if temp > 9 {
                    temp -= 9
                }
            }
            doubling_flag = !doubling_flag;
            temp
        })
        .sum();

    sum.is_multiple_of(10)
}

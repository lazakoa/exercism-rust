pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<_> = num
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();

    let power = digits.len();

    let result: u32 = digits.into_iter().map(|x| u32::pow(x, power as u32)).sum();

    result == num
}

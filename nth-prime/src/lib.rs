pub fn nth(n: u32) -> u32 {
    let mut index = 0;
    let mut number = 0;
    let mut count = 0;

    loop {
        if is_prime(index) {
            number = index;
            count += 1;
        }

        if count == n + 1 {
            break;
        }

        index += 1;
    }

    number
}

fn is_prime(n: u32) -> bool {
    if n == 0 || n == 1 {
        return false;
    }

    for i in 2..f64::from(n).sqrt() as u32 + 1 {
        if n.is_multiple_of(i) {
            return false;
        }
    }

    true
}

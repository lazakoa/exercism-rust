use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut result_set: HashSet<u32> = HashSet::new();
    for factor in factors {
        result_set.extend(multiples(limit, *factor));
    }

    result_set.into_iter().sum()
}

fn multiples(limit: u32, factor: u32) -> HashSet<u32> {
    let mut set = HashSet::new();

    for i in 0..=limit {
        let multiple = factor * i;

        if multiple < limit {
            set.insert(multiple);
        } else {
            break;
        }
    }

    set
}

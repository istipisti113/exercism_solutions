pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples: Vec<Vec<u32>> = Vec::new();
    for i in factors {
        if i == &0 {
            continue;
        }
        let mut a = Vec::new();
        for j in (*i..*&limit).step_by(*i as usize) {
            a.push(j);
        }
        multiples.push(a);
    }
    let concatenated = multiples.concat();
    let mut unique = Vec::new();
    concatenated.iter().for_each(|i| {
        if !unique.contains(i) {
            unique.push(*i);
        }
    });
    unique.iter().sum()
}

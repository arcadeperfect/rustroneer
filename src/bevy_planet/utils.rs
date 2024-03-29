pub fn doubler<T: Clone>(input: Vec<T>) -> Option<Vec<(T, T)>> {
    if input.len() < 2 {
        return None;
    }

    let tuples: Vec<(T, T)> = input
        .windows(2)
        .map(|window| (window[0].clone(), window[1].clone()))
        .collect();

    Some(tuples)
}
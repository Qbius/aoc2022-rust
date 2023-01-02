fn parse(input: &String) -> Vec<(i32, i32)> {
    input
        .split('\n').map(|line| line.split(' ').map(|s| s.chars().nth(0).unwrap()).collect::<Vec<char>>())
        .map(|vec| ((vec[0] as i32) - ('A' as i32), (vec[1] as i32) - ('X' as i32)))
        .collect::<Vec<(i32, i32)>>()
}

pub fn first(input: &String) -> Option<String> {
    let games = parse(input);
    let sum = games.iter().map(|(op, me)| (me + 1) + (((me - op) + 1).rem_euclid(3) * 3)).sum::<i32>();
    Some(sum.to_string())
}

pub fn second(input: &String) -> Option<String> {
    let games = parse(input);
    let sum = games.iter().map(|(op, me)| ((op + me - 1).rem_euclid(3) + 1) + me * 3).sum::<i32>();
    Some(sum.to_string())  
}
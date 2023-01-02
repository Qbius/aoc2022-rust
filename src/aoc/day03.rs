
fn priority(letter: char) -> i32 {
    match letter {
        'a'..='z' => (letter as i32) - ('a' as i32) + 1,
        'A'..='Z' => (letter as i32) - ('A' as i32) + 1 + 26,
        _ => panic!("wtf"),
    }
}


fn handle_row(row: &str) -> i32 {
    let part_len = row.len() / 2;
    let second = row.chars().skip(part_len).take(part_len).collect::<Vec<char>>();
    let mut dups = row.chars().take(part_len).filter(|c| second.contains(c)).collect::<Vec<char>>();
    dups.dedup();
    dups.into_iter().map(priority).sum::<i32>()
}

pub fn first(input: &String) -> Option<String> {
    let sum: i32 = input.split('\n').map(handle_row).sum();
    Some(sum.to_string())
}

pub fn second(input: &String) -> Option<String> {
    let lines = input.split('\n').map(|rs| rs.to_string()).collect::<Vec<String>>();
    let groupcount = lines.len() / 3;
    let sum: i32 = (0..groupcount).map(|i| {
        let first = &lines[3 * i + 0];
        let secnd = &lines[3 * i + 1];
        let third = &lines[3 * i + 2];
        let mut dups = first.chars().filter(|c| secnd.contains(*c) && third.contains(*c)).collect::<Vec<char>>();
        dups.dedup();
        dups.into_iter().map(priority).sum::<i32>()
    }).sum();
    Some(sum.to_string())
}
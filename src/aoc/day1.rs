
pub fn first(input: &String) -> Option<String> {
    let max_elf: i32 = input.trim().split("\n\n").map(|elflines| elflines.trim().split('\n').map(|food| food.trim().parse::<i32>().unwrap()).sum()).max().unwrap();
    Some(max_elf.to_string())
}

pub fn second(input: &String) -> Option<String> {
    let mut elves: Vec<i32> = input.trim().split("\n\n").map(|elflines| elflines.trim().split('\n').map(|food| food.trim().parse::<i32>().unwrap()).sum()).collect();
    elves.sort();
    Some(elves.iter().rev().take(3).sum::<i32>().to_string())
}
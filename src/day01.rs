pub(crate) fn run() {
    let data: &str = include_str!("../inputs/day01.txt").trim();

    let mut elves: Vec<usize> = data
        .split("\n\n")
        .map(|elf| elf.split("\n").map(|e| e.parse::<usize>().unwrap()).sum())
        .collect();

    elves.sort();
    elves.reverse();

    println!("Part I: {}", elves[0]);

    println!("Part II: {}", elves.iter().take(3).sum::<usize>());
}

// use std::fs;
//
// pub fn run() {
//     let input = fs::read_to_string("inputs/day01.txt").unwrap();
//     let mut sorted_calories_by_elf: Vec<u32> = input
//         .split("\n\n")
//         .map(|s| s.lines().map(|l| l.parse::<u32>().unwrap()).sum())
//         .collect();
//
//     sorted_calories_by_elf.sort();
//
//     let ans1 = sorted_calories_by_elf.last().unwrap();
//     println!("{}", ans1);
//
//     let ans2 = sorted_calories_by_elf.iter().rev().take(3).sum::<u32>();
//     println!("{}", ans2);
// }

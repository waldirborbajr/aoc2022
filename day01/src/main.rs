
fn main() {
    // let filepath = "day01.txt";
    // let content = fs::read_to_string(filepath)
    //     .expect("Cannot open file");

    // Part I
    // let result: u32 = content
    //     .split("\n\n")
    //     .map(|block|
    //         block
    //         .split("\n")
    //         .filter(|item| !item.is_empty())
    //         .map(|item| item.parse::<u32>().unwrap())
    //         .sum::<u32>()
    //     )
    //     .max()
    //     .unwrap();

        // .collect();

    // Part II
    // let mut result: Vec<u32> = content
    //     .split("\n\n")
    //     .map(|block|
    //         block
    //         .split("\n")
    //         .filter(|item| !item.is_empty())
    //         .map(|item| item.parse::<u32>().unwrap())
    //         .sum::<u32>()
    //     )
    //     .collect();
    //
    //     result.sort_by(|a,b| b.cmp(a));
    //
    // let sum: u32 = result
    //     .iter()
    //     .take(3)
    //     .sum();

    let data: &str = include_str!("../day01.txt").trim();

    let mut elves: Vec<usize> = data.split("\n\n")
        .map(|elf| elf.split("\n")
            .map(|e| e.parse::<usize>().unwrap())
            .sum()
        )
    .collect();

    elves.sort();
    elves.reverse();

    println!("Part I: {}",
        elves[0]);

    println!("Part II: {}",
        elves.iter().take(3).sum::<usize>()
    );
}

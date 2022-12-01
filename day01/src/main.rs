use std::fs;

fn main() {
    let filepath = "day01.txt";
    let content = fs::read_to_string(filepath)
        .expect("Cannot open file");

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
    let mut result: Vec<u32> = content
        .split("\n\n")
        .map(|block|
            block
            .split("\n")
            .filter(|item| !item.is_empty())
            .map(|item| item.parse::<u32>().unwrap())
            .sum::<u32>()
        )
        .collect();

        result.sort_by(|a,b| b.cmp(a));

    let sum: u32 = result
        .iter()
        .take(3)
        .sum();

    println!("{:?}", sum);


}

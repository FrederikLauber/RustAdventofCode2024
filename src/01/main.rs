use std::collections::HashMap;
use std::iter::zip;

fn main() {
    let content = include_str!("input");
    let mut l1 = Vec::new();
    let mut l2 = Vec::new();

    let mut rate = HashMap::new();

    for line in content.lines(){
        let (mut entry1, mut entry2) = line.split_once(" ").unwrap();
        entry1 = entry1.trim();
        entry2 = entry2.trim();

        let entry1_int = entry1.parse::<i32>().expect("Failed to parse entry 1");
        let entry2_int = entry2.parse::<i32>().expect("Failed to parse entry 2");

        l1.push(entry1_int);
        l2.push(entry2_int);
        *rate.entry(entry2_int).or_insert(0) += 1;
    }

    l1.sort();
    l2.sort();

    let diff: i32 = zip(&l1, &l2).map(|(a, b)| (a - b).abs()).sum();

    let diff2: i32 = l1
        .iter()
        .map(|a| a * rate.get(a).unwrap_or(&0))
        .sum();

    println!("{}", diff);
    println!("{}", diff2);


}

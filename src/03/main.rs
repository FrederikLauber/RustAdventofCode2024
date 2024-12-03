use regex::Regex;

fn main() {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let data = include_str!("input");

    let mut res = 0;
    for (_, [a, b]) in re.captures_iter(data).map(|c| c.extract()) {
        res += a.parse::<i32>().expect("A not parsed") * b.parse::<i32>().expect("B not parsed");
    }
    println!("{}", res);
}
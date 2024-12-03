use regex::Regex;

fn main() {
    let re = Regex::new(r"do\(()()\)|don't\(()()\)|mul\((\d+),(\d+)\)").unwrap();
    let data = include_str!("input");

    let mut res = 0;
    let mut enabled = true;
    for (main, [a, b]) in re.captures_iter(data).map(|c| c.extract()) {
        match main {
            "do()" => {enabled = true; continue},
            "don't()" => {enabled = false; continue},
            _ => {}
        }
        if enabled{
            res += a.parse::<i32>().expect("A not parsed") * b.parse::<i32>().expect("B not parsed");
        }
    }
    println!("{}", res);
}
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let data = include_str!("input");

    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut lines = data.lines();
    let mut res1 = 0;
    let mut res2 = 0;


    for line in lines.by_ref(){
        let line_strip = line.trim();
        if line_strip == "" {
            break;
        }
        let (a, b) = line_strip.split_once("|").expect("Line should have the form integer|integer");
        let key = a.parse::<u32>().expect("Should be integer");
        let value =  b.parse::<u32>().expect("Should be integer");
        rules.entry(key).or_insert(Vec::new()).push(value);
    }

    for line in lines{
        let mut previous_pages = HashSet::new();
        let line_strip = line.trim();
        let mut valid = true;
        let mut page_numbers:VecDeque<u32> = VecDeque::new();

        for page_str in line_strip.split(","){
            let page = page_str.parse::<u32>().expect("Should be integer");
            let tmp_rule = rules.entry(page).or_default();

            for rule in tmp_rule.iter(){
                if previous_pages.contains(rule){
                    valid = false;
                    break;
                }
            }

            // cannot be used for second task
            // if !valid {
            //    break;
            //}

            page_numbers.push_back(page);
            previous_pages.insert(page);
        }

        if valid{
            res1 += page_numbers.get(page_numbers.len() / 2).expect("sadsd");
        } else {
            // need to order

            let mut correct_page_numbers:VecDeque<u32> = VecDeque::new();
            while page_numbers.len() > 0 {
                let mut forbidden_pages = HashSet::new();
                for page in &page_numbers{
                    let tmp_rule2 = rules.entry(*page).or_default();
                    for rule in tmp_rule2.iter(){
                        forbidden_pages.insert(*rule);
                    }
                }
                for (i, page) in page_numbers.iter().enumerate(){
                    if forbidden_pages.contains(page){
                        continue;
                    }
                    correct_page_numbers.push_back(*page);
                    page_numbers.remove(i);
                    break;
                }
            }

            res2 += correct_page_numbers.get(correct_page_numbers.len() / 2).expect("sadsd");
        }
    }
    println!("{}", res1);
    println!("{}", res2);
}

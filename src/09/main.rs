use std::collections::{HashMap, HashSet};

fn main(){
    let data = include_str!("example");
    let mut files: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut files_defragmented: HashMap<u32, HashSet<usize>> = HashMap::new();

    let mut block_id = 0;
    let mut free_blocks : HashSet<u32> = HashSet::new();
    for (index, char) in data.chars().enumerate(){
        let val = char.to_digit(10).expect("Format");

        if index % 2 == 1{
            for _ in 0..val{
                free_blocks.insert(block_id);
                block_id += 1;
            }
        } else {
            let file_index = (index /2) as u32;
            for _ in 0..val{
                files.entry(file_index).or_default().insert(block_id);
                block_id += 1;
            }
        }
    }

    println!("{:?}", files);
    let biggest_file = *files.keys().max().expect("Foo");

    'outer: for file_index in biggest_file..0{
        while files[file_index].len() > 0 {
            if free_blocks.len() == 0{
                break 'outer;
            }
            let max_block = files[file_index].max();



        }

    }



}
use std::cmp::min;
use std::collections::{BTreeMap, HashMap, HashSet};
use itertools::Itertools;

fn main(){
    let data = include_str!("example");
    let mut files: BTreeMap<u32, HashSet<u32>> = BTreeMap::new();
    let mut files_defragmented: BTreeMap<u32, HashSet<u32>> = BTreeMap::new();
    let mut block2fileid: BTreeMap<u32, u32> = BTreeMap::new();
    let mut block2fileid_start: BTreeMap<u32, String> = BTreeMap::new();


    let mut block_id = 0;
    let mut free_blocks : BTreeMap<u32, HashSet<u32>> = BTreeMap::new();

    for (index, char) in data.chars().enumerate(){
        let val = char.to_digit(10).expect("Format");

        if index % 2 == 1{
            for _ in 0..val{
                free_blocks.entry(block_id).or_default().insert(block_id);
                block2fileid_start.insert(block_id, ".".parse().unwrap());
                block_id += 1;
            }
        } else {
            let file_index = (index / 2) as u32;
            for _ in 0..val{
                files.entry(file_index).or_default().insert(block_id);
                block2fileid_start.insert(block_id, file_index.to_string());
                block_id += 1;
            }
        }
    }

    let biggest_file = *files.keys().max().expect("Foo");

    'outer: for file_index in (0..=biggest_file).rev() {
        let mut file_blocks = &files[&file_index];
        let file_length = file_blocks.len();
        for free_block in free_blocks.values_mut() {
            if free_block.len() <= file_length {
                for _ in 0..file_length {
                    let tmp = *free_block.iter().min().expect("sd");
                    free_block.remove(&tmp);
                    file_blocks.insert(tmp);
                }
                continue 'outer;
            }
        }

        files_defragmented.entry(file_index).or_default().extend(file_blocks);
    }

    let mut file_blocks = files.get_mut(&file_index).expect("sd");
            file_blocks.remove(&max_block);
            files_defragmented.entry(file_index).or_default().insert(new_block);
            block2fileid.insert(new_block, file_index);
        }
    }
    let mut res:u128 = 0;
    for (file_index, blocks) in files_defragmented.iter(){
        for block in blocks.iter(){
            res = res + (file_index * block) as u128;
        }
    }

    println!("Checksum: {}", res);
}
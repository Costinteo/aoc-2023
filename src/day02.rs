use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};

use crate::util;
use crate::INPUT_FILEPATH;

const MAX_RED_CNT: u32 = 12;
const MAX_GREEN_CNT: u32 = 13;
const MAX_BLUE_CNT: u32 = 14;

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    let input_filename = format!("{}{}.txt", INPUT_FILEPATH, util::get_input_filename(file!()));

    let file = fs::File::open(&input_filename).expect(&format!("File {} not found!", input_filename));
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|res| res.expect("Unable to read lines!")).collect();
    let lines_ref = &lines;

    // --==[ Task 1 ]==--
    let mut id_sum: u32 = 0;
    
    for line in lines_ref {
        let mut legit_game = true;
        //println!("line: {:?}", line);
        let id: u32 = line.split_whitespace().nth(1).expect("Lines don't have at least two tokens").replace(":", "").parse()?;
        let reveals = line.split_once(":").unwrap().1;
        //println!("id {}: {:?}", id, reveals);
        for rev_round in reveals.split(";") {
            let mut cnt_map = HashMap::from([
                ("red", 0),
                ("green", 0),
                ("blue", 0)
            ]);  
            rev_round.split(",")
                .for_each(|cnt_color| { 
                    cnt_map
                    .entry(cnt_color.split_whitespace().nth(1).unwrap())
                    .and_modify(|x| *x += cnt_color.split_whitespace().nth(0).unwrap().parse::<u32>().unwrap());
                }
                );
            if cnt_map.get("red").unwrap() > &MAX_RED_CNT ||
               cnt_map.get("green").unwrap() > &MAX_GREEN_CNT ||
               cnt_map.get("blue").unwrap() > &MAX_BLUE_CNT {
                   legit_game = false;
                   break;
            }
            //println!("{:?}", cnt_map);

        }
        if legit_game {
            id_sum += id;
        }
        //println!("{:?}", cnt_map);
        //break;
    }

    println!("Sum of IDs from possible games is: {}", id_sum);

    // --==[ Task 2 ]==--
    let mut power_sum: u32 = 0;
    
    for line in lines_ref {
        let mut max_map: HashMap<&str, u32> = HashMap::from([
                ("red", 0),
                ("green", 0),
                ("blue", 0)
        ]);  

        let reveals = line.split_once(":").unwrap().1;
        //println!("id {}: {:?}", id, reveals);
        for rev_round in reveals.split(";") {
            let mut cnt_map = HashMap::from([
                ("red", 0),
                ("green", 0),
                ("blue", 0)
            ]);  
            rev_round.split(",")
                .for_each(|cnt_color| { 
                    cnt_map
                    .entry(cnt_color.split_whitespace().nth(1).unwrap())
                    .and_modify(|x| *x += cnt_color.split_whitespace().nth(0).unwrap().parse::<u32>().unwrap());
                }
                );
            
            let keys: Vec<&str> = max_map.keys().cloned().collect();
            for k in keys {
                let curr_cnt = *cnt_map.get(k).unwrap();
                max_map.entry(k).and_modify(|val| *val = std::cmp::max(curr_cnt, *val));
            };
            //println!("{:?}", max_map)

        }
        power_sum += max_map.into_values().product::<u32>()
    }

    println!("Power sum is: {}", power_sum);

    Ok(())
}

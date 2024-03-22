// use conv::prelude::*;

use std::collections::HashMap;

pub fn calculate_median(mut int_list: Vec<u8>) -> u8 {
    int_list.sort();
    let length = int_list.len() as f64; // .value_as::<f64>().unwrap();
    let middle_index_f64 = length/2.0;
    let middle_index = middle_index_f64.floor() as usize;

    int_list[middle_index]
}

pub fn calculate_mode(int_list: &Vec<u8>) -> u8 {
    let mut int_map: HashMap<u8, u8> = HashMap::new();

    for num in int_list {
        let count = int_map.entry(*num).or_insert(1);
        *count += 1;
    }

    *int_map.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap().0
}
use rand::prelude::*;

pub fn get_rand_num(list: Vec<i16>) -> i128 {
    let sum = get_list_sum(list);
    let mut rng:ThreadRng = thread_rng();

    rng.gen_range(0..sum - 1)
}

fn get_list_sum(list: Vec<i16>) -> i128 {
    let mut sum: i128 = 0;
    for i in 0..list.len() {
        sum += (list[i] as i128) + 20000;
    }

    sum
}

pub fn get_selection_num(bits_list: &Vec<[u8; 24]>, output_list: &Vec<i16>, rand_num: i128) -> ([u8; 24], i16) {
    let mut top: i128 = 0;
    for n in 0..output_list.len() {
        top += output_list[n] as i128 + 20000;
        if top > rand_num {
            return (bits_list[n].clone(), output_list[n]);
        }
    }

    (bits_list[bits_list.len() - 1].clone(), output_list[output_list.len() - 1])
}
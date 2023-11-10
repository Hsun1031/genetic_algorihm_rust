use rand::prelude::*;

pub fn get_rand_num(list: Vec<i32>) -> i128 {
    let sum = get_list_sum(list);
    let mut rng:ThreadRng = thread_rng();

    rng.gen_range(0..sum - 1)
}

fn get_list_sum(list: Vec<i32>) -> i128 {
    let mut sum: i128 = 0;
    for i in 0..list.len() {
        sum += (list[i] as i128) + 20000;
    }

    sum
}

pub fn get_selection_num(bits_list: &Vec<Vec<Vec<u8>>>, output_list: &Vec<i32>, rand_num: i128) -> (Vec<Vec<u8>>, i32) {
    let mut top: i128 = 0;
    for n in 0..output_list.len() {
        top += (output_list[n] + 20000) as i128;
        if top > rand_num {
            return (bits_list[n].clone(), output_list[n]);
        }
    }

    (bits_list[bits_list.len() - 1].clone(), output_list[output_list.len() - 1])
}
mod init_module;
mod selection_module;

use std::time::Instant;

fn main() {
    let len = 100;
    let (bits_list, output_list) = init_population(len);
    let (new_bits_list, new_output_list) = selection(&bits_list, &output_list);

    // println!("output_list: {:?}", output_list);
    // println!("new_output_list: {:?}", new_output_list);
}

fn init_population(len: i32) -> (Vec<Vec<Vec<u8>>>, Vec<i32>) {
    let start_time = Instant::now();

    let mut bits_list: Vec<Vec<Vec<u8>>> = Vec::new();
    let mut output_list: Vec<i32> = Vec::new();

    // Get 100 bits lists
    for _ in 1..=len {
        bits_list.push(init_module::get_bits_list(3));
    }

    // Get 100 outputs
    for i in 0..bits_list.len() {
        output_list.push(init_module::get_output(bits_list[i].clone()));
    }

    let elapsed_time = start_time.elapsed(); 
    println!("init_population: {:?}", elapsed_time);

    // println!("{:?}", output_list);
    // println!("{:?}", bits_list);

    (bits_list, output_list)
}

fn selection(bits_list: &Vec<Vec<Vec<u8>>>, output_list: &Vec<i32>) -> (Vec<Vec<Vec<u8>>>, Vec<i32>){
    let start_time = Instant::now();

    let mut new_bits_list: Vec<Vec<Vec<u8>>> = Vec::new();
    let mut new_output_list: Vec<i32> = Vec::new();

    for _ in 1..=output_list.len() {
        // Get random number in range of sum of output_list
        let rand_num = selection_module::get_rand_num(output_list.clone());

        // Get new bits and output by random number
        let (new_bits, new_output)= selection_module::get_selection_num(&bits_list, &output_list, rand_num);
        new_bits_list.push(new_bits);       // Add new bits to new_bits_list
        new_output_list.push(new_output);   // Add new output to new_output_list
    }

    let elapsed_time = start_time.elapsed(); 
    println!("selection: {:?}", elapsed_time);

    // println!("{:?}", new_output_list);
    // println!("{:?}", new_bits_list);

    (new_bits_list, new_output_list)
}

// fn crossover() {

// }

// fn mutation() {

// }

// fn reporting() {

// }

mod init_module;
mod selection_module;
mod crossover_module;
mod mutation_module;

use std::fs;
use std::iter::Iterator;

fn main() {
    let len = 1000;
    let mut max_list:Vec<i16> = Vec::new();
    let mut avg_list:Vec<f32> = Vec::new();
    let mut std_deviation_list:Vec<f32> = Vec::new();

    let (mut bits_list, mut output_list ) = init_population(len);

    for _ in 0..=1000 {
        let (new_bits_list, new_output_list)            = selection(&bits_list, &output_list);
        let (cross_bits_list, cross_output_list)        = crossover(&new_bits_list);
        let (mutation_bits_list, mutation_output_list)  = mutation(&cross_bits_list);

        // find max and avg
        let (max, avg, std_deviation) = get_output(&mutation_output_list);
        max_list.push(max);
        avg_list.push(avg);
        std_deviation_list.push(std_deviation);

        bits_list   = mutation_bits_list.  clone();
        output_list = mutation_output_list.clone();
    }
    // println!("max_list: {:?}", max_list);
    // println!("avg_list: {:?}", avg_list);
    let max_list_str = max_list.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", ");
    let avg_list_str = avg_list.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", ");
    let std_deviation_list_str = std_deviation_list.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", ");

    // Write the strings to files
    fs::write("data.js", format!("const max_list = [{}];\nconst avg_list = [{}];\nconst std_deviation_list = [{}]", max_list_str, avg_list_str, std_deviation_list_str)).expect("Unable to write file");

    println!("-------------------------------------------------------");
    println!("Done!");
}

fn init_population(len: i16) -> (Vec<[u8; 24]>, Vec<i16>) {
    // let start_time = Instant::now();

    let mut bits_list: Vec<[u8; 24]> = Vec::new();
    let mut output_list: Vec<i16> = Vec::new();

    // Get 100 bits lists
    for _ in 1..=len {
        bits_list.push(init_module::get_bits_list());
    }

    // Get 100 outputs
    for i in 0..bits_list.len() {
        // output_list.push(init_module::get_output( &bits_list[i] ));
        match bits_list.get(i) {
            Some(bits) => output_list.push(init_module::get_output( bits )),
            None => println!("Error: bits_list.get({}) is None", i),
        }
    }

    // let elapsed_time = start_time.elapsed();
    // println!("init_population: {:?}", elapsed_time);

    // println!("{:?}", output_list);
    // println!("{:?}", bits_list);

    (bits_list, output_list)
}

fn selection(bits_list: &Vec<[u8; 24]>, output_list: &Vec<i16>) -> (Vec<[u8; 24]>, Vec<i16>){
    // let start_time = Instant::now();

    let mut new_bits_list: Vec<[u8; 24]> = Vec::new();
    let mut new_output_list: Vec<i16> = Vec::new();

    for _ in 1..=output_list.len() {
        // Get random number in range of sum of output_list
        let rand_num = selection_module::get_rand_num(output_list.clone());

        // Get new bits and output by random number
        let (new_bits, new_output)= selection_module::get_selection_num(&bits_list, &output_list, rand_num);
        new_bits_list.push(new_bits);       // Add new bits to new_bits_list
        new_output_list.push(new_output);   // Add new output to new_output_list
    }

    // let elapsed_time = start_time.elapsed(); 
    // println!("selection: {:?}", elapsed_time);

    // println!("{:?}", new_output_list);
    // println!("{:?}", new_bits_list);

    (new_bits_list, new_output_list)
}

fn crossover(bits_list: &Vec<[u8; 24]>) -> (Vec<[u8; 24]>, Vec<i16>) {
    let mut new_output_list: Vec<i16> = Vec::new();
    let new_bits_list = crossover_module::two_points_crossover(bits_list);
    
    // Get 100 outputs
    for i in 0..new_bits_list.len() {
        // output_list.push(init_module::get_output( &bits_list[i] ));
        match new_bits_list.get(i) {
            Some(bits) => new_output_list.push(init_module::get_output( bits )),
            None => println!("Error: bits_list.get({}) is None", i),
        }
    }

    (new_bits_list, new_output_list)
}

fn mutation(bits_list: &Vec<[u8; 24]>) -> (Vec<[u8; 24]>, Vec<i16>) {
    let mut new_output_list: Vec<i16> = Vec::new();
    let new_bits_list = mutation_module::gene(bits_list);

    // Get 100 outputs
    for i in 0..new_bits_list.len() {
        // output_list.push(init_module::get_output( &bits_list[i] ));
        match new_bits_list.get(i) {
            Some(bits) => new_output_list.push(init_module::get_output( bits )),
            None => println!("Error: bits_list.get({}) is None", i),
        }
    }

    (new_bits_list, new_output_list)
}

fn mean(data: &[i16]) -> Option<f32> {
    let sum = data.iter().map(|&value| value as i128).sum::<i128>() as f32;
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    }
}

fn std_deviation_fn(data: &[i16]) -> Option<f32> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data.iter().map(|value| {
                let diff = data_mean - (*value as f32);

                diff * diff
            }).sum::<f32>() / count as f32;

            Some(variance.sqrt())
        },
        _ => None
    }
}

fn get_output(output_list: &[i16]) -> (i16, f32, f32) {
    // Option<f32> to f32
    let _max = output_list.iter().max().unwrap_or(&0);
    let _mean = mean(output_list).unwrap_or(0.0);
    let _std_deviation = std_deviation_fn(output_list).unwrap_or(0.0);
    (*_max, _mean , _std_deviation)
}

// // fn reporting() {

// // }

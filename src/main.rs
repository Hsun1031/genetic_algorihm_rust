mod init_module;
mod selection_module;
mod crossover_module;
mod mutation_module;

use std::fs;

// use std::time::Instant;

fn main() {
    let len = 1000;
    let mut max_list:Vec<i16> = Vec::new();
    let mut avg_list:Vec<f64> = Vec::new();

    let (mut bits_list, mut output_list ) = init_population(len);

    for _ in 0..=1000 {
        let (new_bits_list, new_output_list)            = selection(&bits_list, &output_list);
        let (cross_bits_list, cross_output_list)        = crossover(&new_bits_list);
        let (mutation_bits_list, mutation_output_list)  = mutation(&cross_bits_list);

        // find max and avg
        let (max, avg) = get_max_and_avg(&mutation_output_list);
        max_list.push(max);
        avg_list.push(avg);

        bits_list   = mutation_bits_list.  clone();
        output_list = mutation_output_list.clone();
    }
    // println!("max_list: {:?}", max_list);
    // println!("avg_list: {:?}", avg_list);
    let max_list_str = max_list.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", ");
    let avg_list_str = avg_list.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", ");

    // Write the strings to files
    fs::write("data.js", format!("const max_list = [{}];\nconst avg_list = [{}];", max_list_str, avg_list_str)).expect("Unable to write file");

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

fn get_max_and_avg(output_list: &Vec<i16>) -> (i16, f64) {
    let mut total: i32 = 0;

    let mut max = 0;

    for i in 0..output_list.len() {
        match output_list.get(i) {
            Some(output) => {
                if max < *output {
                    max = *output;
                }
                total += *output as i32;
            },
            None => println!("Error: output_list.get({}) is None", i),
        }
    }

    let avg: f64 = ( total as f64/ (output_list.len() as f64) ) as f64;

    (max, avg)
}

// // fn reporting() {

// // }

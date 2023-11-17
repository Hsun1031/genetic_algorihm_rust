use rand::prelude::*;

pub fn one_point_crossover(bits_list: &Vec<[u8; 24]>) {

}

pub fn two_points_crossover(bits_list: &Vec<[u8; 24]>) -> Vec<[u8; 24]> {
    let mut new_bits_list = Vec::new();
    for i in (0..bits_list.len()).step_by(2)  {
        match bits_list.get(i) {
            Some(bits) => {
                let (num1, num2) = get_two_points_rand(bits.len());
                match bits_list.get(i + 1) {
                    Some(bits2) => {
                        let (swap_bits_list1, swap_bits_list2) = two_points_swap(&bits, &bits2, num1, num2);
                        new_bits_list.push(swap_bits_list1);
                        new_bits_list.push(swap_bits_list2);
                        // println!("num1: {}, num2: {}", num1, num2);
                        // println!("    Array title: [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2 ]");  
                        // println!("     bits_list1: {:?}", bits);
                        // println!("     bits_list2: {:?}", bits2);
                        // println!("swap_bits_list1: {:?}", swap_bits_list1);
                        // println!("swap_bits_list2: {:?}", swap_bits_list2)
                    },
                    None => println!("Error: bits_list.get({}) is None", i + 1),
                    
                }
            },
            None => println!("Error: bits_list.get({}) is None", i),
        }
    }

    new_bits_list
}

fn get_two_points_rand(bits_list_len: usize) -> (usize, usize){
    // println!("bits_list_len: {}", bits_list_len);
    let mut rng:ThreadRng = thread_rng();

    let num1: usize = rng.gen_range(0..bits_list_len - 2);
    let mut num2: usize = rng.gen_range(0..bits_list_len - 2);

    if num1 == num2 {
        if num1 == bits_list_len - 2 {
            num2 = 1;
        } else {
            num2 += 1;
        }
    }

    if num2 < num1 {
        return (num2, num1);
    }

    (num1, num2)
}

fn two_points_swap(bits_list1: &[u8; 24], bits_list2: &[u8; 24], num1: usize, num2: usize) -> ([u8; 24], [u8; 24]){
    let mut swap_bits_list1: [u8; 24] = [0; 24];
    let mut swap_bits_list2: [u8; 24] = [0; 24];

    let mut is_swap = false;

    for i in 0..bits_list1.len() {
        if is_swap {
            swap_bits_list1[i] = bits_list2[i];
            swap_bits_list2[i] = bits_list1[i];
        } else {
            swap_bits_list1[i] = bits_list1[i];
            swap_bits_list2[i] = bits_list2[i];
        }
        
        if i == num1 {
            is_swap = true;
        }

        if i == num2 {
            is_swap = false;
        }
    }

    (swap_bits_list1, swap_bits_list2)
}

pub fn rand_points_crossover(bits_list: &Vec<[u8; 24]>) -> Vec<[u8; 24]> { 
    let mut new_bits_list = Vec::new();

    for i in (0..bits_list.len()).step_by(2)  {
        match bits_list.get(i) {
            Some(bits) => {
                match bits_list.get(i + 1) {
                    Some(bits2) => {
                        let (swap_bits_list1, swap_bits_list2) = rand_points_swap(&bits, &bits2);
                        new_bits_list.push(swap_bits_list1);
                        new_bits_list.push(swap_bits_list2);
                    },
                    None => println!("Error: bits_list.get({}) is None", i + 1),
                    
                }
            },
            None => println!("Error: bits_list.get({}) is None", i),
        }
    }

    new_bits_list
}

fn rand_points_swap(bits_list1: &[u8; 24], bits_list2: &[u8; 24]) -> ([u8; 24], [u8; 24]){
    let mut rng:ThreadRng = thread_rng();
    let mut swap_bits_list1: [u8; 24] = [0; 24];
    let mut swap_bits_list2: [u8; 24] = [0; 24];

    for i in 0..bits_list1.len() {
        let num1: usize = rng.gen_range(0..1);

        if num1 == 0 {
            swap_bits_list1[i] = bits_list2[i];
            swap_bits_list2[i] = bits_list1[i];
        } else {
            swap_bits_list1[i] = bits_list1[i];
            swap_bits_list2[i] = bits_list2[i];
        }
    }

    (swap_bits_list1, swap_bits_list2)
}
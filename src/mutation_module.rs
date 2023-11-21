use rand::prelude::*;

pub fn chromosome() {

}

pub fn gene(bits_list: &Vec<[u8; 24]>) -> Vec<[u8; 24]> {
    let mut rng:ThreadRng = thread_rng();
    let my_probability = 0.03 / (bits_list.len() as f64);

    let mut new_bits_list = bits_list.clone();

    for i in 0..bits_list.len() {
        let mut new_bits = bits_list[i].clone();
        match bits_list.get(i) {
            Some(bits) => {
                for i in 0..bits.len() {
                    let num: f64 = rng.gen(); // generates a float between 0 and 1

                    if num < my_probability {
                        // println!("i: {}, num: {}, my_probability: {}", i, num, my_probability);
                        new_bits[i] = if bits[i] == 0 { 1 } else { 0 };
                        new_bits_list[i] = new_bits;
                    };
                
                }
            },
            None => println!("Error: bits_list.get({}) is None", i),
        }
        // println!("   bits_list: {:?}", bits_list[i]);
        // println!("new_bits_list:{:?}", new_bits_list);
    }

    new_bits_list
}
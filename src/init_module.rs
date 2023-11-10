use rand::prelude::*;

pub fn get_output(bits_list: Vec<Vec<u8>>) -> i32 {
    let mut _list: Vec<i8> = Vec::new();
    for i in 0..bits_list.len() {
        let _hex: i16 = get_hex_by_bits(bits_list[i].clone()) as i16;
        _list.push(
            if _hex > 228 {
                100
            } else if _hex < 28 {
                -100
            } else {
                (_hex - 128) as i8
            }
        );
    }

    ( (_list[0] as i32).pow(2) ) - ( (_list[1] as i32).pow(2) ) + ( (_list[2] as i32).pow(2) )
}

fn get_hex_by_bits(bits: Vec<u8>) -> u8 {
    let mut _hex: u8 = 0;

    for i in 0..bits.len() {
        _hex += (bits[i] as u8) << (bits.len() - 1 - i);
    }

    _hex
}

pub fn get_bits_list(len: u32) -> Vec<Vec<u8>>{
    let mut _bits_list: Vec<Vec<u8>> = Vec::new();

    for _ in 1..=len {
        let _bits: Vec<u8> = get_bits(8);
        _bits_list.push(_bits);
    }

    _bits_list
}

fn get_bits(len: u8) -> Vec<u8> {
    let mut _rng:ThreadRng = thread_rng();
    let mut _bits: Vec<u8> = Vec::new();

    for _ in 1..=len {
        _bits.push(_rng.gen_range(0..2));
    }

    _bits
}
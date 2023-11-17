use rand::prelude::*;

pub fn get_output(bits_list: &[u8]) -> i16 {
    let mut _list: [i8; 3] = [0; 3];

    for i in ( 0..bits_list.len() ).step_by(8) {
        let _hex: u16 = get_hex_by_bits( &bits_list[i..i+8] );
        _list[i/8] = {
            if _hex > 228 {
                100i8
            } else if _hex < 28 {
                -100i8
            } else {
                (_hex as i32 - 128 ) as i8
            }
        };
    }

    ( _list[0] as i16 ).pow(2) - ( _list[1] as i16 ).pow(2) + ( _list[2] as i16 ).pow(2) // -10000 ~ 20000
}

fn get_hex_by_bits(bits: &[u8]) -> u16 {
    let mut _hex: u16 = 0;

    for i in 0..bits.len() {
        _hex += ( bits[i] << (bits.len() - 1 - i) ) as u16;
    }

    _hex
}

pub fn get_bits_list() -> [u8; 24] {
    let mut _rng:ThreadRng = thread_rng();
    let mut _bits_list:[u8; 24] = [0; 24];

    for i in 0.._bits_list.len() {
        _bits_list[i] = _rng.gen_range(0..2);
    }

    _bits_list
}

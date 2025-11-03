#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use crate_batch_1::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 150 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut num_ops = _to_u8(GLOBAL_DATA, 0);
        while num_ops > 0 {
            match _to_u8(GLOBAL_DATA, 1) % 3 {
                0 => {
                    let len_1 = _to_u8(GLOBAL_DATA, 2) as usize;
                    let str_1 = _to_str(GLOBAL_DATA, 3, 3 + len_1);
                    let len_2 = _to_u8(GLOBAL_DATA, 3 + len_1) as usize;
                    let str_2 = _to_str(GLOBAL_DATA, 3 + len_1 + 1, 3 + len_1 + 1 + len_2);
                    crate_batch_1::benchmark_string(str_1, str_2);
                }
                1 => {
                    let vec_len = _to_u8(GLOBAL_DATA, 4) as usize % 65;
                    let vec: Vec<u8> = (0..vec_len).map(|i| _to_u8(GLOBAL_DATA, 5 + i)).collect();
                    let array: [u8; 64] = _to_array(GLOBAL_DATA, 6 + vec_len);
                    let u64_val = _to_u64(GLOBAL_DATA, 70 + vec_len);
                    crate_batch_1::benchmark_vec_u8(&vec, u64_val, &array);
                }
                2 => {
                    let test_key = _to_array_u64(GLOBAL_DATA, 90);
                    let len_str_1 = _to_u8(GLOBAL_DATA, 98) as usize;
                    let test_string = _to_str(GLOBAL_DATA, 99, 99 + len_str_1);
                    let len_str_2 = _to_u8(GLOBAL_DATA, 99 + len_str_1) as usize;
                    let test_string2 = _to_str(GLOBAL_DATA, 99 + len_str_1 + 1, 99 + len_str_1 + 1 + len_str_2);

                    let vec_len = _to_u8(GLOBAL_DATA, 200) as usize % 65;
                    let test_vec_u8: Vec<u8> = (0..vec_len).map(|i| _to_u8(GLOBAL_DATA, 201 + i)).collect();
                    let test_u64 = _to_u64(GLOBAL_DATA, 266);

                    let benchmark_data = crate_batch_1::BenchmarkData {
                        testKey: test_key,
                        testString: String::from(test_string),
                        testString2: String::from(test_string2),
                        testVecU8: test_vec_u8,
                        testU64: test_u64,
                    };
                    crate_batch_1::benchmark(&benchmark_data);
                }
                _ => (),
            }
            num_ops -= 1;
        }
    });
}

fn _to_array(data: &[u8], start: usize) -> [u8; 64] {
    let mut array = [0; 64];
    let slice = &data[start..start + 64];
    array.copy_from_slice(slice);
    array
}

fn _to_array_u64(data: &[u8], start: usize) -> [u8; 64] {
    let mut array = [0; 64];
    let slice = &data[start..start + 64];
    array.copy_from_slice(slice);
    array
}

fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}

fn _to_u16(data:&[u8], index:usize)->u16 {
    let data0 = _to_u8(data, index) as u16;
    let data1 = _to_u8(data, index+1) as u16;
    data0 << 8 | data1
}

fn _to_u32(data:&[u8], index:usize)->u32 {
    let data0 = _to_u16(data, index) as u32;
    let data1 = _to_u16(data, index+2) as u32;
    data0 << 16 | data1
}

fn _to_u64(data:&[u8], index:usize)->u64 {
    let data0 = _to_u32(data, index) as u64;
    let data1 = _to_u32(data, index+4) as u64;
    data0 << 32 | data1
}

fn _to_u128(data:&[u8], index:usize)->u128 {
    let data0 = _to_u64(data, index) as u128;
    let data1 = _to_u64(data, index+8) as u128;
    data0 << 64 | data1
}

fn _to_usize(data:&[u8], index:usize)->usize {
    _to_u64(data, index) as usize
}

fn _to_i8(data:&[u8], index:usize)->i8 {    
    data[index] as i8
}

fn _to_i16(data:&[u8], index:usize)->i16 {
    let data0 = _to_i8(data, index) as i16;
    let data1 = _to_i8(data, index+1) as i16;
    data0 << 8 | data1
}

fn _to_i32(data:&[u8], index:usize)->i32 {
    let data0 = _to_i16(data, index) as i32;
    let data1 = _to_i16(data, index+2) as i32;
    data0 << 16 | data1
}

fn _to_i64(data:&[u8], index:usize)->i64 {
    let data0 = _to_i32(data, index) as i64;
    let data1 = _to_i32(data, index+4) as i64;
    data0 << 32 | data1
}

fn _to_i128(data:&[u8], index:usize)->i128 {
    let data0 = _to_i64(data, index) as i128;
    let data1 = _to_i64(data, index+8) as i128;
    data0 << 64 | data1
}

fn _to_isize(data:&[u8], index:usize)->isize {
    _to_i64(data, index) as isize
}

fn _to_f32(data:&[u8], index: usize) -> f32 {
    let data_slice = &data[index..index+4];
    use std::convert::TryInto;
    let data_array:[u8;4] = data_slice.try_into().expect("slice with incorrect length");
    f32::from_le_bytes(data_array)
}

fn _to_f64(data:&[u8], index: usize) -> f64 {
    let data_slice = &data[index..index+8];
    use std::convert::TryInto;
    let data_array:[u8;8] = data_slice.try_into().expect("slice with incorrect length");
    f64::from_le_bytes(data_array)
}

fn _to_char(data:&[u8], index: usize)->char {
    let char_value = _to_u32(data,index);
    match char::from_u32(char_value) {
        Some(c)=>c,
        None=>{
            std::process::exit(0);
        }
    }
}

fn _to_bool(data:&[u8], index: usize)->bool {
    let bool_value = _to_u8(data, index);
    if bool_value %2 == 0 {
        true
    } else {
        false
    }
}

fn _to_str(data:&[u8], start_index: usize, end_index: usize)->&str {
    let data_slice = &data[start_index..end_index];
    use std::str;
    match str::from_utf8(data_slice) {
        Ok(s)=>s,
        Err(_)=>{
            std::process::exit(0);
        }
    }
}

fn _unwrap_option<T>(opt: Option<T>) -> T {
    match opt {
        Some(_t) => _t,
        None => {
            std::process::exit(0);
        }
    }
}

fn _unwrap_result<T, E>(_res: std::result::Result<T, E>) -> T {
    match _res {
        Ok(_t) => _t,
        Err(_) => {
            std::process::exit(0);
        },
    }
}
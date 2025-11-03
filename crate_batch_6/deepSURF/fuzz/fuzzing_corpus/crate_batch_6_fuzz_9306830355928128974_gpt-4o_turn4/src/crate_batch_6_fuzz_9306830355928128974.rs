#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use crate_batch_6::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 20 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let iterations = data[0] as usize % 16;
        let main_iterations = data[1] as usize % 5;

        for _ in 0..iterations {
            let enter_condition = data[2] % 2 == 0;
            crate_batch_6::benchmark_misc();

            if enter_condition {
                crate_batch_6::benchmark_vec_u8(GLOBAL_DATA);
            }

            for _ in 0..main_iterations {
                crate_batch_6::main();
            }

            let arg_types = crate_batch_6::get_arg_types(GLOBAL_DATA);
            let option_arg_types = _unwrap_option(arg_types);

            let bench_data_constructor_index = data[3] as usize % 2;
            let bench_data = match bench_data_constructor_index {
                0 => crate_batch_6::BenchmarkData {
                    testKey: [data[4], data[5], data[6], data[7], data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15], 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    testString: _to_str(data, 5, 10).to_string(),
                    testString2: _to_str(data, 10, 15).to_string(),
                    testVecU8: vec![data[16]; 8],
                    testU64: data[18] as u64,
                },
                1 => crate_batch_6::BenchmarkData {
                    testKey: [data[5], data[6], data[7], data[8], data[9], data[10], data[11], data[12], data[13], data[14], data[15], data[16], 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    testString: _to_str(data, 6, 11).to_string(),
                    testString2: _to_str(data, 11, 16).to_string(),
                    testVecU8: vec![data[17]; 8],
                    testU64: data[19] as u64,
                },
                _ => unreachable!(),
            };

            let bench_data_ref = &bench_data;

            if data[3] % 2 == 0 {
                crate_batch_6::benchmark(bench_data_ref);
            }

            let template_index = global_data.first_half.len() % 5;
            let template = match template_index {
                0 => "first_template",
                1 => "second_template",
                2 => "third_template",
                3 => "fourth_template",
                _ => "fifth_template",
            };

            let str1 = _to_str(data, 2, 2 + data[2] as usize);
            let str2 = _to_str(data, 2 + data[2] as usize, 3 + data[2] as usize * 2);
            let count = _to_u64(data, data.len() - 8);

            crate_batch_6::benchmark_template_and_strings(str1, str2, count);
        }
    });
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
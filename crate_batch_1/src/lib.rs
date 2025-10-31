use alloy_json_abi::JsonAbi;
use std::io::{BufReader, Cursor};
use der::Decodable;
// pub struct BenchmarkData {
//     pub run1_abi_json: String,
//     pub run3_password: String,
//     pub run3_hash: String,
//     pub run4_input: Vec<u8>,
//     pub run5_buffer: Vec<u8>,
//     pub run5_crash: Vec<u8>,
//     pub run6_rfc2822: String,
//     pub run6_days_to_add: u64,
//     pub run7_cookie_data: String,
//     pub run7_key: [u8; 64],
//     pub run8_program: Vec<u8>,
//     pub run9_css_color: String,
//     pub run10_css_input: String,
//     pub run11_bytes: Vec<u8>,
//     pub run12_der_data: Vec<u8>,
//     pub run13_input: Vec<u8>,
//     pub run14_expression: String,
//     pub run15_fatfs_data: Vec<u8>,
//     pub run16_flac_stream: Vec<u8>,
//     pub run17_input: Vec<u8>,
// }

pub struct BenchmarkData {
    pub testString: String,
    pub testString2: String,
    pub testVecU8: Vec<u8>,
    pub testU64: u64,
    pub testKey: [u8; 64],
}

impl Default for BenchmarkData{
    fn default() -> Self {
        Self {
            testString: "Hello, Benchmark!".to_owned(),
            testString2: "Another test string".to_owned(),
            testVecU8: vec![1, 2, 3, 4, 5],
            testU64: 42,
            testKey: [0u8; 64],
        }
    }
}

// impl Default for BenchmarkData {
//     fn default() -> Self {
//         Self {
//             run1_abi_json: r#"
//     {
//         "functions": [
//             {
//                 "name": "transfer",
//                 "inputs": [
//                     {"name": "to", "type": "address"},
//                     {"name": "value", "type": "uint256"}
//                 ],
//                 "outputs": []
//             }
//         ]
//     }
//     "#.to_owned(),
//             run3_password: "password".to_owned(),
//             run3_hash: "$2y$12$XZ6J8vZc6Q1jz2X1Z5Q5eOe5eOe5eOe5eOe5eOe5eOe5eOe5eOe5eOe".to_owned(),
//             run4_input: vec![1, 2, 3, 4],
//             run5_buffer: vec![0u8; 128],
//             run5_crash: b"\n\x00\x00\x00\x04\x00\x00\x00\x00\x00".to_vec(),
//             run6_rfc2822: "31 DEC 262143 23:59 -2359".to_owned(),
//             run6_days_to_add: 1,
//             run7_cookie_data: "x=yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy£".to_owned(),
//             run7_key: [0u8; 64],
//             run8_program: b"; Test the division legalizttions.\ntest legalizer\n; See also legalize-div-traps.clif.\nset avoid_div_traps=0\ntarget x86_64\n\n; regex: V=v\\d+\n; regex: EBB=ebb\\d+\n\nfunction %udiv(i64, i64) -> i64 {\nebb0(v0: i64, v1: i64):\n    ; check: ebb0(\n    v2 = udiv v0, v1\n    ; nextln: $(hi=$V) = iconst.i64 0\n    ; nextln: $(d=$V), $(r=$V) = x86_udivmodx v0, $hi, v1\n    return v2\n    ; 28, i64) -> i64 {\nebb0(v0: i64, v1: i64):\n    ; check: ebb0(\n    v2 = srem v0, v1\n    ; nextln: $(fm1=$V) = ifcmp_imm v1, -1\n    ; nextln: brif eq $fm1, $(m1=$EBB)\n    ; check: $(hi=$V) = sshr_imm\n    ; nextln: $(d=$V), $(r=$V) = x86_sdivmodx v0, $hi, v1\n    ; nextln: jump $(done=$EBB)($r)\n    ; check: $m1:\n    ; nextln: $(zero=$V) = iconst.i64 0\n    ; nextln: jump $(done=$ x86_udivmodx v ; check: $done(v2: i64):\n    return v2\n    ; nextln: return v2\n}\n".to_vec(),
//             run9_css_color: "#123456".to_owned(),
//             run10_css_input: "color: red;".to_owned(),
//             run11_bytes: vec![194, 133, 45, 127, 29, 127, 127],
//             run12_der_data: (&[0x02, 0x01, 0x01]).to_vec(),
//             run13_input: b"\x03\x00\x00kk\x00\x00\x00\x00\x00\x00\x00.\x00\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff;\xff\xff\xff\xff\xff\xff\xff\xff\xff\x01\x00\x00\x00\xff\x0a\xff".to_vec(),
//             run14_expression: "3 + 4 * 2".to_owned(),
//             run15_fatfs_data: b"\x00\xfe\xf7\xf7\xf7\xf7\xf7\xf7\xb7\xf7\x00\x00\x02\x10\x00\xfc\x01\x00\x00\x00\x00\x00\x00\x00\x002222\x01\x00\x00\x0022222\xfc\x00\x00\x00\x00\x00\x00\x00222[[[[[21[[[[[[[[[[[[[[[[[[[[[[[222[[2222\x91\x91\x91\x91\x00\x00\x00\x00\xaa\xaa\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xb2\xb2\xb2\xb2\xb2\xb2\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xbe&\x00\x00\x00\x00\x00\x00\x00\xbez\x00\x01\x00\xd0\x00-\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x012\x94\x94\x94\x94\x94\x9422222222222222222222222222\x01\x00\x00\x0022222\xe1222222222221[[[[[[[[[[[[K\x1b[[[[[[[[[[[[\x00\x00\x00\x00\x00\x00\x00\x00\x012\x94\x94\x94\x94\x94\x94\xf7\xf7\xf7\xf7\xf73\x00\xaa\xaa\x11\x03\x00\x00\x002222222222222222222222\x00\x00\x00\x00\xaa\xaa\xe7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\x00\x00\x00\x00\x00\x00\x00\x11\x00\x00\x00\x00\x00\x00\x00\xaa\xaa\xaa\x02\x00\x98\x00\x00\x00\x002222\x01\x00\x00\x0022222\xfc\x00\x00\x00\x00\x00\x00\x00222[[[[[21[[[[[[[[[[[[[[[[[[[[[[[222[[2222\x91\x91\x91\x91\x91\x91\xd4\x91\x91\x91\x91t\x912222222222222222\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\xaa\xaa\xaa\xaa\xaa\xaa\xaa\x00\x00\x01\x00\x00\x00\xaa\xaa\x9cU\xaa\xaa\x01\xaa2\xaa\xff\xff\xff\xff\xff\xff\xff\x02]\x00\x01\xff".to_vec(),
//             run16_flac_stream: b"fLaC\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00H\x01\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xff\xff\xff\xff\\".to_vec(),
//             run17_input: vec![
//                 102, 103, 98, 3, 102, 103, 98, 0, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39,
//                 219, 216, 216, 216, 216, 216, 216, 216, 39, 39, 39, 39, 39, 32, 39, 39, 39, 39, 39,
//                 39, 39, 39, 39, 10, 169, 247, 247, 247, 247,
//             ],
//         }
//     }
// }

pub unsafe fn main() {
    println!("crate batch 1 benchmark starting");
    let input = vec![1, 2, 3, 4, 5];
    let data = BenchmarkData::default();
    benchmark(&data);
}

pub unsafe fn benchmark(data: &BenchmarkData) {
    benchmark_string(&data.testString, &data.testString2);
    benchmark_vec_u8(&data.testVecU8, data.testU64, &data.testKey);
}


pub unsafe fn benchmark_string(str: &str, str2: &str) {
    println!("Benchmarking with input string: {}", str);

     // --- run 1 ---------------------------------------------------------------
    {
        let abi_lines = str.lines();

        match JsonAbi::parse(abi_lines) {
            Ok(parsed_abi) => println!("Parsed ABI: {:?}", parsed_abi),
            Err(err) => eprintln!("Error parsing ABI: {}", err),
        }
    }


     // --- run 3 ---------------------------------------------------------------
    {
        match bcrypt::verify(str.clone(), str2) {
            Ok(matched) => {
                if matched {
                    println!("Password matches the hash");
                } else {
                    println!("Password does not match the hash");
                }
            }
            Err(err) => eprintln!("Error verifying password: {}", err),
        };
    }

    // --- run 6_part1 ---------------------------------------------------------------
    {
        print!("chrono_16");
        let _ = chrono_16::DateTime::parse_from_rfc2822(str);
       
    }

    // --- run 9 ---------------------------------------------------------------
    {
        let _ = csscolorparser::parse(str);
    }

    // --- run 10 --------------------------------------------------------------
    {
        let mut parser_input = cssparser::ParserInput::new(str2);
        let mut parser = cssparser::Parser::new(&mut parser_input);

        match parser.next_including_whitespace_and_comments() {
            Ok(token) => println!("Parsed token: {:?}", token),
            Err(err) => println!("Parsing error: {:?}", err),
        }
    }

    // --- run 14 --------------------------------------------------------------
    {
        let _ = exmex::parse_with_default_ops::<f64>(str).unwrap();
    }


}


pub unsafe fn benchmark_vec_u8(data: &Vec<u8>, num: u64, key: &[u8; 64]) {
    println!("Benchmarking with input vec u8: {:?}", data);

    // --- run 4 ---------------------------------------------------------------
    {
        let input = data.as_slice();

        match bincode_6::decode_from_slice::<u32, _>(
            input,
            bincode_6::config::Configuration::standard(),
        ) {
            Ok((value, _)) => println!("Decoded value from bincode_6: {}", value),
            Err(err) => eprintln!("Error decoding from bincode_6: {}", err),
        }

        match bincode_7::decode_from_slice::<u32, _>(
            input,
            bincode_7::config::Configuration::standard(),
        ) {
            Ok((value, _)) => println!("Decoded value from bincode_7: {}", value),
            Err(err) => eprintln!("Error decoding from bincode_7: {}", err),
        }
    }

     // --- run 5 ---------------------------------------------------------------
    {
        let mut cursor1 = Cursor::new(data.as_slice());
        let mut cursor2 = Cursor::new(data.as_slice());
        let _ = bson_10::decode_document(&mut cursor1);
        let _ = bson_11::decode_document(&mut cursor2);

        let mut reader = Cursor::new(data.clone());
        let _ = bson_12::Document::from_reader(&mut reader);
    }

    // --- run 6_part2 ---------------------------------------------------------------
    {
         let _ = chrono_17::DateTime::checked_add_days(
            chrono_17::Utc::now(),
            chrono_17::Days::new(num),
        );
    }

       // --- run 7 ---------------------------------------------------------------
    {
        let _cookie = cookie::Cookie::parse("test".clone()).expect("failed to parse cookie");

        let key = cookie::Key::from(key);

        let mut jar = cookie::CookieJar::new();

        let _signed = jar.signed_mut(&key);
    }

    // --- run 8 ---------------------------------------------------------------
    {
        let s_str = match std::str::from_utf8(data.as_slice()) {
            Ok(value) => value,
            Err(err) => {
                eprintln!("Invalid UTF-8 sequence: {}", err);
                ""
            }
        };
        if !s_str.is_empty() {
            let _ = cranelift_reader::parse_test(s_str);
        }
    }

    // --- run 11 --------------------------------------------------------------
    {
        let str_result = std::str::from_utf8(data.as_slice());
        if let Ok(s) = str_result {
            let mut siv = cursive::default();

            siv.add_layer(
                cursive::views::Dialog::around(cursive::views::TextView::new(s))
                    .title("Cursive")
                    .button("Quit", |s| s.quit()),
            );

            siv.run();
        } else {
            println!("not valid utf8");
        }
    }

    // --- run 12 --------------------------------------------------------------
    {
        match der::Decoder::new(data.as_slice()) {
            Ok(mut decoder) => match der::asn1::Any::decode(&mut decoder) {
                Ok(decoded) => println!("Decoded successfully: {:?}", decoded),
                Err(err) => eprintln!("Error decoding: {}", err),
            },
            Err(err) => eprintln!("Failed to create Decoder: {}", err),
        }
    }

    // --- run 13 --------------------------------------------------------------
    {
        let _ = der_parser::parse_der(data.as_slice());
    }

    // --- run 15 --------------------------------------------------------------
    {
        let storage = Cursor::new(data.clone());
        let _ = fatfs::FileSystem::new(storage, fatfs::FsOptions::new());
    }

    // --- run 16 --------------------------------------------------------------
    {
        if let Ok(mut stream) = flac::Stream::<flac::ByteStream>::from_buffer(data) {
            let _ = stream.info();
            let _ = stream.metadata();
            let mut iter = stream.iter::<i8>();
            while iter.next().is_some() {}
        }
    }

    // --- run 17 --------------------------------------------------------------
    {
        let mut buf_reader = BufReader::new(Cursor::new(data.clone()));

        match flatgeobuf::FgbReader::open(&mut buf_reader) {
            Ok(mut reader) => {
                let _ = reader.header();
            }
            Err(err) => eprintln!("Failed to open FgbReader: {}", err),
        }
    }
}

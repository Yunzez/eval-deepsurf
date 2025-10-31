// use alloy_json_abi::JsonAbi;
// use std::io::BufReader;
// use std::io::Cursor;
// use std::time::Duration;

// // *** main start
// pub fn main() {
//     println!("crate batch 1 starting");

//     run_1();

//     run_3();
//     run_4();
//     run_5();
//     run_6();
//     run_7();
//     run_8();
//     run_9();
//     run_10();
//     run_11();
//     run_12();
//     run_13();
//     run_14();
//     run_15();
//     run_16();
//     run_17();
// }
// // *** main end

// fn run_1() {
//     let abi_json = r#"
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
//     "#;

//     // Convert the string to an iterator (by splitting into lines)
//     let abi_lines = abi_json.lines();

//     // Call `parse` with an iterator
//     match JsonAbi::parse(abi_lines) {
//         Ok(parsed_abi) => println!("Parsed ABI: {:?}", parsed_abi),
//         Err(err) => eprintln!("Error parsing ABI: {}", err),
//     }
// }

// // fn run_2() {
// //     let data = b"R0VUIGh0dHBzOi8vZXhhbXBsZS5jb20gSFRUUC8xLjEKSG9zdDrIgw0KDQo=";
// //     let mut cursor = Cursor::new(data.as_ref());
// //     async_h1::server::decode(&mut cursor);
// // }

// fn run_3() {
//     // bcrypt::verify
//     match bcrypt::verify(
//         "password",
//         "$2y$12$XZ6J8vZc6Q1jz2X1Z5Q5eOe5eOe5eOe5eOe5eOe5eOe5eOe5eOe5eOe",
//     ) {
//         Ok(matched) => {
//             if matched {
//                 println!("Password matches the hash");
//             } else {
//                 println!("Password does not match the hash");
//             }
//         }
//         Err(err) => eprintln!("Error verifying password: {}", err),
//     };
// }

// fn run_4() {
//     match bincode_6::decode_from_slice::<u32, _>(
//         &[1, 2, 3, 4],
//         bincode_6::config::Configuration::standard(),
//     ) {
//         Ok((value, _)) => println!("Decoded value from bincode_6: {}", value),
//         Err(err) => eprintln!("Error decoding from bincode_6: {}", err),
//     }

//     match bincode_7::decode_from_slice::<u32, _>(
//         &[1, 2, 3, 4],
//         bincode_7::config::Configuration::standard(),
//     ) {
//         Ok((value, _)) => println!("Decoded value from bincode_7: {}", value),
//         Err(err) => eprintln!("Error decoding from bincode_7: {}", err),
//     }
// }

// fn run_5() {
//     let buf = vec![0u8; 128]; // Example buffer, replace with actual data
//     bson_10::decode_document(&mut Cursor::new(&buf[..]));
//     bson_11::decode_document(&mut Cursor::new(&buf[..]));

//     let crash = b"\n\x00\x00\x00\x04\x00\x00\x00\x00\x00";
//     let mut reader = std::io::Cursor::new(crash);
//     let _ = bson_12::Document::from_reader(&mut reader);
// }
// fn run_6() {
//     print!("chrono_16");
//     chrono_16::DateTime::parse_from_rfc2822("31 DEC 262143 23:59 -2359");
//     chrono_17::DateTime::checked_add_days(chrono_17::Utc::now(), chrono_17::Days::new(1));
// }
// fn run_7() {
//     let data = "x=yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy£";

//     let c = cookie::Cookie::parse(data).expect("failed to parse cookie");

//     let key = cookie::Key::from(&[0u8; 64]); // Fully qualified path

//     let mut jar = cookie::CookieJar::new();

//     let signed = jar.signed_mut(&key); // Fully qualified path
// }

// #[macro_use]
// extern crate cranelift_reader;
// use std::str;

// fn run_8() {
//     let s = b"; Test the division legalizttions.\ntest legalizer\n; See also legalize-div-traps.clif.\nset avoid_div_traps=0\ntarget x86_64\n\n; regex: V=v\\d+\n; regex: EBB=ebb\\d+\n\nfunction %udiv(i64, i64) -> i64 {\nebb0(v0: i64, v1: i64):\n    ; check: ebb0(\n    v2 = udiv v0, v1\n    ; nextln: $(hi=$V) = iconst.i64 0\n    ; nextln: $(d=$V), $(r=$V) = x86_udivmodx v0, $hi, v1\n    return v2\n    ; 28, i64) -> i64 {\nebb0(v0: i64, v1: i64):\n    ; check: ebb0(\n    v2 = srem v0, v1\n    ; nextln: $(fm1=$V) = ifcmp_imm v1, -1\n    ; nextln: brif eq $fm1, $(m1=$EBB)\n    ; check: $(hi=$V) = sshr_imm\n    ; nextln: $(d=$V), $(r=$V) = x86_sdivmodx v0, $hi, v1\n    ; nextln: jump $(done=$EBB)($r)\n    ; check: $m1:\n    ; nextln: $(zero=$V) = iconst.i64 0\n    ; nextln: jump $(done=$ x86_udivmodx v ; check: $done(v2: i64):\n    return v2\n    ; nextln: return v2\n}\n";

//     let s_str = std::str::from_utf8(s).expect("Invalid UTF-8 sequence");
//     cranelift_reader::parse_test(s_str);
// }

// fn run_9() {
//     // ! parsehex is private, parse gets to parsehex
//     csscolorparser::parse("#123456");
// }
// fn run_10() {
//     let input = "color: red;";

//     let mut parser_input = cssparser::ParserInput::new(input);
//     let mut parser = cssparser::Parser::new(&mut parser_input);

//     // Handle the result properly
//     match parser.next_including_whitespace_and_comments() {
//         Ok(token) => println!("Parsed token: {:?}", token),
//         Err(err) => println!("Parsing error: {:?}", err),
//     }
// }

// use cursive::views::{Dialog, TextView};
// fn run_11() {
//     let d: Vec<u8> = vec![194, 133, 45, 127, 29, 127, 127];
//     let str = str::from_utf8(&d);
//     if let Ok(s) = str {
//         // Creates the cursive root - required for every application.
//         let mut siv = cursive::default();

//         // Creates a dialog with a single "Quit" button
//         siv.add_layer(
//             Dialog::around(TextView::new(s))
//                 .title("Cursive")
//                 .button("Quit", |s| s.quit()),
//         );

//         siv.run();
//     } else {
//         println!("not valid utf8");
//     }
// }

// use der::Decodable;
// fn run_12() {
//     // ! decode_to_array is the function yet is private, we use decode instead
//     let der_data = &[0x02, 0x01, 0x01]; // DER encoding of an integer (1)

//     // Handle the `Result` correctly
//     let mut decoder = match der::Decoder::new(der_data) {
//         Ok(decoder) => decoder, // Successfully created Decoder
//         Err(err) => {
//             eprintln!("Failed to create Decoder: {}", err);
//             return;
//         }
//     };
//     match der::asn1::Any::decode(&mut decoder) {
//         Ok(decoded) => println!("Decoded successfully: {:?}", decoded),
//         Err(err) => eprintln!("Error decoding: {}", err),
//     }
// }

// fn run_13() {
//     let data =
//         b"\x03\x00\x00kk\x00\x00\x00\x00\x00\x00\x00.\x00\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff;\xff\xff\xff\xff\xff\xff\xff\xff\xff\x01\x00\x00\x00\xff\x0a\xff";
//     let _ = der_parser::parse_der(data);
// }

// fn run_14() {
//     let s = "3 + 4 * 2";
//     let _ = exmex::parse_with_default_ops::<f64>(s).unwrap();
// }
// fn run_15() {
//     // ! fatfs
//     let data = b"\x00\xfe\xf7\xf7\xf7\xf7\xf7\xf7\xb7\xf7\x00\x00\x02\x10\x00\xfc\x01\x00\x00\x00\x00\x00\x00\x00\x002222\x01\x00\x00\x0022222\xfc\x00\x00\x00\x00\x00\x00\x00222[[[[[21[[[[[[[[[[[[[[[[[[[[[[[222[[2222\x91\x91\x91\x91\x00\x00\x00\x00\xaa\xaa\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xb2\xb2\xb2\xb2\xb2\xb2\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xbe&\x00\x00\x00\x00\x00\x00\x00\xbez\x00\x01\x00\xd0\x00-\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x012\x94\x94\x94\x94\x94\x9422222222222222222222222222\x01\x00\x00\x0022222\xe1222222222221[[[[[[[[[[[[K\x1b[[[[[[[[[[[[\x00\x00\x00\x00\x00\x00\x00\x00\x012\x94\x94\x94\x94\x94\x94\xf7\xf7\xf7\xf7\xf73\x00\xaa\xaa\x11\x03\x00\x00\x002222222222222222222222\x00\x00\x00\x00\xaa\xaa\xe7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\xf7\x00\x00\x00\x00\x00\x00\x00\x11\x00\x00\x00\x00\x00\x00\x00\xaa\xaa\xaa\x02\x00\x98\x00\x00\x00\x002222\x01\x00\x00\x0022222\xfc\x00\x00\x00\x00\x00\x00\x00222[[[[[21[[[[[[[[[[[[[[[[[[[[[[[222[[2222\x91\x91\x91\x91\x91\x91\xd4\x91\x91\x91\x91t\x912222222222222222\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\xaa\xaa\xaa\xaa\xaa\xaa\xaa\x00\x00\x01\x00\x00\x00\xaa\xaa\x9cU\xaa\xaa\x01\xaa2\xaa\xff\xff\xff\xff\xff\xff\xff\x02]\x00\x01\xff";

//     let storage = std::io::Cursor::new(data.to_vec());
//     fatfs::FileSystem::new(storage, fatfs::FsOptions::new());
// }

// use flac::{ByteStream, Stream};

// fn run_16() {
//     // ! flac
//     let s = Stream::<ByteStream>::from_buffer(b"fLaC\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00H\x01\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xff\xff\xff\xff\\");
//     if let Ok(mut stream) = s {
//         let _ = stream.info();
//         let _ = stream.metadata();
//         let mut iter = stream.iter::<i8>();
//         while iter.next().is_some() {}
//     }
// }

// use flatgeobuf::*;
// fn run_17() {
//     let mut input: &[u8] = &[
//         102, 103, 98, 3, 102, 103, 98, 0, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 219,
//         216, 216, 216, 216, 216, 216, 216, 39, 39, 39, 39, 39, 32, 39, 39, 39, 39, 39, 39, 39, 39,
//         39, 10, 169, 247, 247, 247, 247,
//     ];
//     let mut buf_reader = BufReader::new(Cursor::new(input));

//     // ! spreedsheet line 36
//     let mut fgb = match FgbReader::open(&mut buf_reader) {
//         Ok(n) => n,
//         Err(_) => return,
//     };
//     // ! spreedsheet line 37
//     let _ = fgb.header();
// }

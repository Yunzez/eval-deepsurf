// // *** main start
// pub fn main() {
//     println!("crate batch 5 starting");

//     run_1();
//     // run_2();
//     run_4();
//     run_3();
//     run_5();
//     run_6();
//     run_7();
//     run_8();
//     run_9();
//     run_11();
//     run_12();
//     run_13();
//     run_14();
//     run_15();
//     run_16();
//     run_17();
//     run_18();
//     run_19();
//     run_20();
//     println!("crate batch 5 ending");
// }
// // *** main end

// fn run_1() {
//     // ! crashing input
//     let data = [
//         24, 19, 43, 53, 49, 54, 49, 53, 32, 32, 48, 53, 50, 52, 48, 57, 52, 48, 50, 48, 90,
//     ];

//     // Attempt to decode, handle errors
//     let value = match rasn::der::decode::<rasn::types::Open>(&data) {
//         Ok(v) => v,
//         Err(e) => {
//             eprintln!("Decoding failed: {:?}", e);
//             return; // Stop execution here if decoding fails
//         }
//     };

//     // Try encoding & decoding again
//     let encoded = rasn::der::encode(&value).unwrap();

//     match rasn::der::decode::<rasn::types::Open>(&encoded) {
//         Ok(_) => println!("Decoding succeeded!"),
//         Err(e) => eprintln!("Decoding failed after encoding: {:?}", e),
//     }
// }

// fn run_2() {}

// fn run_3() {
//     println!("running line 152");
//     // Attempt to parse deeply nested input safely
//     // ! crashing result
//     //  let result: Result<ron::Value, _> = ron::from_str(&"{".repeat(10_000));
//     let result: Result<ron::Value, _> = ron::from_str(&"{}".repeat(1));
//     match result {
//         Ok(value) => println!("Parsed successfully: {:?}", value),
//         Err(err) => eprintln!("Failed to parse: {}", err),
//     }

//     println!("running line 153");
//     // Safe parsing with known good input
//     let input = "{}";
//     let v: ron::Value = ron::from_str(input).expect("Valid input should not fail");
//     println!("{:?}", v);

//     let ser = ron::to_string(&v).unwrap();
//     println!("{:?}", ser);
// }

// fn run_4() {
//     ini::Ini::read_from(&mut std::io::Cursor::new(String::from(""))).unwrap();
// }

// fn run_5() {
//     if let Err(e) = std::panic::catch_unwind(|| {
//         rustc_demangle::demangle("_ZN2222222222222222222222EE"); // should panic
//     }) {
//         eprintln!("Caught panic in run 5: {:?}", e);
//     }
// }
// fn run_6() {
//     if let Err(e) = std::panic::catch_unwind(|| {
//         semver::VersionReq::parse("8.*.7").unwrap(); // should panic
//     }) {
//         eprintln!("Caught panic in run 6: {:?}", e);
//     }
// }

// use serde_yaml::Number;
// fn run_7() {
//     // ? line 159

//     let data = b"invalid yaml data";
//     match serde_yaml::from_slice::<serde_yaml::Value>(data) {
//         Ok(value) => println!("Parsed YAML successfully: {:?}", value),
//         Err(err) => eprintln!("Failed to parse YAML: {}", err),
//     }

//     // ? line 160
//     // Step 1: Start with a YAML string containing "50."
//     let yaml_str = "50.";
//     let deserialized: Number = serde_yaml::from_str(yaml_str).unwrap();

//     println!("Deserialized from YAML (50.): {:?}", deserialized);

//     // Step 2: Serialize it back to YAML
//     let serialized_yaml = serde_yaml::to_string(&deserialized).unwrap();
//     println!("Serialized to YAML: {}", serialized_yaml);

//     // Step 3: Deserialize again and check type
//     let roundtrip: Number = serde_yaml::from_str(&serialized_yaml).unwrap();
//     println!("Deserialized again: {:?}", roundtrip);

//     assert_eq!(deserialized, roundtrip, "Roundtrip failed!");
// }

// fn run_8() {
//     let data = &[0x30, 0x0a, 0x02, 0x01, 0x01, 0x02, 0x01, 0x02, 0x02, 0x01];

//     if let Err(e) = std::panic::catch_unwind(|| {
//         let _ = simple_asn1::from_der(data); // should panic
//     }) {
//         eprintln!("Caught panic in run 8: {:?}", e);
//     }
// }

// use snap::Decoder;
// fn run_9() {
//     let crashing_input: &[u8] = &[
//         0x82, 0x04, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01, 0x00, 0x00, 0x00,
//     ];
//     let mut decoder = Decoder::new();

//     match decoder.decompress_vec(crashing_input) {
//         Ok(_) => println!("Decompression succeeded."),
//         Err(e) => eprintln!("Decompression failed: {:?}", e),
//     }
// }

// use core::cmp::{min, Ordering};
// use soroban_env_common::xdr::ScVec;
// use soroban_env_common::Symbol;
// use soroban_env_host::budget::Budget;
// use soroban_env_host::TryFromVal;
// use soroban_env_host::{Compare, Host};
// fn run_11() {
//     println!("run 11");
//     // ? line 172
//     let v1 = ScVec::try_from((0, 1)).unwrap();
//     let v2 = ScVec::try_from((0, 0, 2)).unwrap();
//     let expected_cmp = Ordering::Greater;
//     let budget = Budget::default();
//     let actual_cmp = budget.compare(&v1, &v2).unwrap();

//     // ? line 174
//     let host = Host::default();
//     let s = "#";
//     let s = Symbol::try_from_val(&host, &s);
//     match s {
//         Ok(_) => println!("Unexpected success in symbol conversion"),
//         Err(_) => println!("Symbol conversion failed as expected"),
//     }
// }

// use sqlparser::dialect::GenericDialect;
// use sqlparser::parser::Parser;

// fn run_12() {
//     println!("run 12");
//     // ! crashing input
//     // let sql = "(".repeat(1024);
//     let sql = "(".repeat(1);
//     let dialect = GenericDialect {};

//     if let Err(e) = std::panic::catch_unwind(|| {
//         Parser::parse_sql(&dialect, &sql);
//     }) {
//         eprintln!("Caught panic in run 12: {:?}", e);
//     }
// }

// fn run_13() {
//     println!("run 13");
//     let data =
//         "-----BEGIN OPENSSH PRIVATE KEY------END OPENSSH PRIVATE KEY-----ENSSH PRIVAPRIVATE KEY-----\x00\x00\x00\x01\x00";

//     if let Err(e) = std::panic::catch_unwind(|| {
//         ssh_keys::openssh::parse_private_key(data);
//     }) {
//         eprintln!("Caught panic in run 13: {:?}", e);
//     }
// }

// fn run_14() {
//     println!("run 14");

//     let data = b"\x00\x00\x00\x00\x00\x00\x00\x00";

//     if let Err(e) = std::panic::catch_unwind(|| {
//         let _ = ssh_parser::parse_ssh_packet(data);
//     }) {
//         eprintln!("Caught panic in run 14: {:?}", e);
//     }
// }

// use swf_parser::streaming::movie;
// fn run_15() {
//     let bytes = b"CWSCCCACCGCCC";
//     if let Err(e) = std::panic::catch_unwind(|| {
//         let _ = movie::parse_movie(&bytes[..]);
//     }) {
//         eprintln!("Caught panic in run 15: {:?}", e);
//     }
// }

// use symbolic::common::ByteView;
// // use symbolic::minidump::processor::ProcessState;
// fn run_16() {
//     println!("run 16");
//     // ? line 184
//     symbolic::demangle::demangle(
//         "_ZUlzjjlZZL1zStUlSt7j_Z3kjIIjIjL1vfIIEEEjzjjfjzSt7j_Z3kjIIjfjzL4t3kjIIjfjtUlSt7j_Z3kjIIjIjL1vfIIEEEjzjjfjzSt7j_Z3kjIIjfjzL4t3kjIIjfjzL4t7IjIjjzjjzSt7j_Z3kjIIjfjzStfjzSt7j_ZA3kjIIjIjL1vfIIEEEjzjjfjzSt7j_Z3kjIIjIjL1vfIIEEEjzjjfjzSt7j_Z3kjIIjfjzL4t3kjIIjzL4t7IjIjjzjjzSt7j_Z3kjIIjfjzStfjzSt7j_ZA3kjIIjIjL1vfIIEEEjzjjfjzSt7j_Z3kjIIjIjL1vfIIEEEjzjjfjzSt7j_Z3kjIIjfjzL4t3kjIIjfjzL4t7IjIjL1vfIIEEEjzjjSI"
//     );

//     let data =
//         b"MDMP\x93\xa7\x00\x00\r\x00\x00\x00 \xff\xff\xff\xff\xff\xff\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
//     let bv = ByteView::from_slice(data);
//     // ProcessState::from_minidump(&bv, None);

//     // ? line 186

//     let data =
//         b"MDMP\x93\xa7\x00\x00\r\x00\x00\x00 \xff\xff\xff\xff\xff\xff\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
//     let _ = symbolic::unreal::Unreal4Crash::parse_with_limit(data, 1024 * 1024);
// }

// use std::io::Cursor;
// use symphonia::core::codecs::{CodecParameters, DecoderOptions};
// use symphonia::core::errors;
// use symphonia::core::formats::{FormatOptions, FormatReader};
// use symphonia::core::io::MediaSourceStream;
// use symphonia::core::meta::MetadataOptions;
// use symphonia::core::probe::Hint;
// use symphonia::default::get_probe;

// fn test_decode(data: Vec<u8>) -> symphonia::core::errors::Result<()> {
//     let data = Cursor::new(data);
//     let mss = MediaSourceStream::new(Box::new(data), Default::default());

//     // Probing format instead of manually creating an `AdtsReader`
//     let probed = get_probe().format(
//         &Hint::new(),
//         mss,
//         &FormatOptions::default(),
//         &MetadataOptions::default(),
//     )?;
//     let mut reader = probed.format;

//     let track = reader
//         .default_track()
//         .ok_or(errors::Error::DecodeError("No default track found"))?;
//     let params = track.codec_params.clone();

//     println!("creating decoder");
//     // Correctly initialize an AAC decoder
//     let mut decoder = match std::panic::catch_unwind(|| {
//         symphonia::default::get_codecs().make(&params, &DecoderOptions::default())
//     }) {
//         Ok(Ok(decoder)) => decoder,
//         Ok(Err(e)) => {
//             return Err(e);
//         }
//         Err(_) => {
//             return Err(errors::Error::DecodeError(
//                 "Panic occurred while creating decoder",
//             ));
//         }
//     };
//     println!("entering while decode loop");
//     while let Ok(packet) = reader.next_packet() {
//         let _ = decoder.decode(&packet);
//     }

//     Ok(())
// }

// fn run_17() {
//     println!("run 17");
//     let file = vec![
//         0xff, 0xf1, 0xaf, 0xce, 0x02, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xfb,
//         0xaf,
//     ];
//     let err = test_decode(file).unwrap_err();
// }

// use syn_188::Expr;

// fn run_18() {
//     // ? same bug function for 188 and 189
//     println!("run 18");
//     let s = "6E--5458";
//     if let Err(e) = std::panic::catch_unwind(|| {
//         let _ = syn_188::parse_str::<Expr>(s);
//     }) {
//         eprintln!("Caught panic in run 18: {:?}", e);
//     }
// }

// fn run_19() {
//     // ? line 190 191, same bug function
//     let context = tera_190::Context::new();
//     if let Err(e) = std::panic::catch_unwind(|| {
//         let _ = tera_190::Tera::one_off("{{1/0}}", &context, true);
//     }) {
//         eprintln!("Caught panic in run 19: {:?}", e);
//     }
// }

// use sqlformat::FormatOptions as SqlFormatOptions;
// use sqlformat::*;
// fn run_20() {
//     if let Err(e) = std::panic::catch_unwind(|| {
//         let _ = format(
//             "?62666666121266666612",
//             &QueryParams::None,
//             SqlFormatOptions::default(),
//         );
//     }) {
//         eprintln!("Caught panic in run 20: {:?}", e);
//     }
// }

// use raven_uxn::{Backend, EmptyDevice, Uxn, UxnRam};

// fn run_update() {
//     let mut ram_v = UxnRam::new();
//     let mut vm_v = Uxn::new(&mut ram_v, Backend::Interpreter);

//     // Don't load any programs that require auxiliary memory
//     let data: &[u8] = &[0x00, 0x01, 0x02, 0x03]; // Random data
//     if !vm_v.reset(data).is_empty() {
//         return;
//     }
// }

// use unified_diff::diff;
// fn run_update_2() {
//     let test_cases = vec![
//         (vec![b'a', b'b', b'c'], vec![b'x', b'y', b'z'], 3),
//         (vec![b'1', b'2', b'3'], vec![b'4', b'5', b'6'], 2),
//         (vec![b'!', b'@', b'#'], vec![b'$', b'%', b'^'], 1),
//     ];

//     for (from, to, context) in test_cases {
//         if let Ok(s) = String::from_utf8(from.clone()) {
//             if !s.is_ascii() {
//                 continue;
//             }
//             if s.find(|x| x < ' ' && x != '\n').is_some() {
//                 continue;
//             }
//         } else {
//             continue;
//         }
//         if let Ok(s) = String::from_utf8(to.clone()) {
//             if !s.is_ascii() {
//                 continue;
//             }
//             if s.find(|x| x < ' ' && x != '\n').is_some() {
//                 continue;
//             }
//         } else {
//             continue;
//         }
//         let diff_result = diff(
//             &from,
//             "a/fuzz.file",
//             &to,
//             "target/fuzz.file",
//             context as usize,
//         );
//         println!("Diff result: {:?}", diff_result);
//     }
// }

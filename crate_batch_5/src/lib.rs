use core::cmp::Ordering;
use ini::Ini;
use rasn::der::{decode, encode};
use rasn::types::Open;
use ron;
use semver::VersionReq;
use serde_yaml::{self, Number};
use simple_asn1;
use snap::Decoder;
use soroban_env_common::xdr::ScVec;
use soroban_env_common::Symbol;
use soroban_env_host::budget::Budget;
use soroban_env_host::{Compare, Host, TryFromVal};
// use sqlformat::{format, QueryParams};
// use sqlformat::FormatOptions as SqlFormatOptions;
// use sqlparser::dialect::GenericDialect;
// use sqlparser::parser::Parser;
use ssh_keys;
use ssh_parser;
use std::io::Cursor;
use std::panic;
use swf_parser::streaming::movie;
// use symbolic::common::ByteView; // disabled: symbolic requires newer Rust toolchain
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::errors;
use symphonia::core::errors::Result as SymphoniaResult;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::default::get_probe;
use tera_190;

/*pub struct BenchmarkData {
    pub run1_der: Vec<u8>,
    pub run3_repeat: usize,
    pub run4_ini: String,
    pub run5_symbol: String,
    pub run6_version_req: String,
    pub run7_invalid_yaml: Vec<u8>,
    pub run7_number: String,
    pub run8_asn1: Vec<u8>,
    pub run9_snappy: Vec<u8>,
    pub run12_sql_repeat: usize,
    pub run13_private_key: String,
    pub run14_packet: Vec<u8>,
    pub run15_bytes: Vec<u8>,
    pub run16_demangle: String,
    pub run16_minidump: Vec<u8>,
    pub run17_file: Vec<u8>,
    pub run18_syn_input: String,
    pub run19_template: String,
    pub run20_sql: String,
}

impl Default for BenchmarkData {
    fn default() -> Self {
        Self {
            run1_der: vec![
                24, 19, 43, 53, 49, 54, 49, 53, 32, 32, 48, 53, 50, 52, 48, 57, 52, 48, 50, 48, 90,
            ],
            run3_repeat: 1,
            run4_ini: String::new(),
            run5_symbol: "_ZN2222222222222222222222EE".to_owned(),
            run6_version_req: "8.*.7".to_owned(),
            run7_invalid_yaml: b"invalid yaml data".to_vec(),
            run7_number: "50.".to_owned(),
            run8_asn1: (&[0x30, 0x0a, 0x02, 0x01, 0x01, 0x02, 0x01, 0x02, 0x02, 0x01]).to_vec(),
            run9_snappy: vec![
                0x82, 0x04, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01, 0x00, 0x00,
                0x00,
            ],
            run12_sql_repeat: 1,
            run13_private_key:
                "-----BEGIN OPENSSH PRIVATE KEY------END OPENSSH PRIVATE KEY-----ENSSH PRIVAPRIVATE KEY-----\x00\x00\x00\x01\x00".to_owned(),
            run14_packet: b"\x00\x00\x00\x00\x00\x00\x00\x00".to_vec(),
            run15_bytes: b"CWSCCCACCGCCC".to_vec(),
            run16_demangle: "_ZUlzjjlZZL1zStUlSt7j_Z3kjIIjIjL1vfIIEEEjzjjfjzSt7j_Z3kjIIjfjzL4t3kjIIjfjtUlSt7j_Z3kjIIjIjL1vfIIEEEjzjjfjzSt7j_Z3kjIIjfjzL4t3kjIIjfjzL4t7IjIjjzjjzSt7j_Z3kjIIjfjzStfjzSt7j_ZA3kjIIjIjL1vfIIEEEjzjjfjzSt7j_Z3kjIIjIjL1vfIIEEEjzjjfjzSt7j_Z3kjIIjfjzL4t3kjIIjzL4t7IjIjjzjjzSt7j_Z3kjIIjfjzStfjzSt7j_ZA3kjIIjIjL1vfIIEEEjzjjfjzSt7j_Z3kjIIjIjL1vfIIEEEjzjjfjzSt7j_Z3kjIIjfjzL4t3kjIIjfjzL4t7IjIjL1vfIIEEEjzjjSI".to_owned(),
            run16_minidump: b"MDMP\x93\xa7\x00\x00\r\x00\x00\x00 \xff\xff\xff\xff\xff\xff\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00".to_vec(),
            run17_file: vec![0xff, 0xf1, 0xaf, 0xce, 0x02, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xfb, 0xaf],
            run18_syn_input: "6E--5458".to_owned(),
            run19_template: "{{1/0}}".to_owned(),
            run20_sql: "?62666666121266666612".to_owned(),
        }
    }
}*/

pub struct BenchmarkData {
    pub testString: String,
    pub testString2: String,
    pub testVecU8: Vec<u8>,
    pub testU64: u64,
    pub testKey: [u8; 64],
}

impl Default for BenchmarkData {
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

pub fn main() {
    println!("crate batch 5 benchmark starting");
    let data = BenchmarkData::default();
    benchmark(&data);
    println!("crate batch 5 ending");
}

pub fn test_decode(file: &[u8]) -> SymphoniaResult<()> {
    unsafe {
        let data = Cursor::new(file.to_vec());
        let mss = MediaSourceStream::new(Box::new(data), Default::default());

        let probed = get_probe().format(
            &Hint::new(),
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )?;
        let mut reader = probed.format;

        let track = reader
            .default_track()
            .ok_or(errors::Error::DecodeError("No default track found"))?;
        let params = track.codec_params.clone();

        let mut decoder = match panic::catch_unwind(|| {
            symphonia::default::get_codecs().make(&params, &DecoderOptions::default())
        }) {
            Ok(Ok(decoder)) => decoder,
            Ok(Err(e)) => return Err(e),
            Err(_) => return Err(errors::Error::DecodeError("Panic occurred while creating decoder")),
        };

        while let Ok(packet) = reader.next_packet() {
            let _ = decoder.decode(&packet);
        }

        Ok(())
    }
}

pub fn benchmark(data: &BenchmarkData) {
    benchmark_vec_u8(&data.testVecU8, data.testU64);
    benchmark_numeric(data.testU64);
    benchmark_string_ops(&data.testString, &data.testString2, &data.testVecU8);
    benchmark_misc();
}
use syn_188::Expr;

pub fn benchmark_vec_u8(bytes: &[u8], num: u64) {
    unsafe {
        // --- run 1 ---------------------------------------------------------------
        {
            let decoded = match decode::<Open>(bytes) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Decoding failed: {:?}", e);
                    return;
                }
            };

            let encoded = encode(&decoded).unwrap();

            match decode::<Open>(&encoded) {
                Ok(_) => println!("Decoding succeeded!"),
                Err(e) => eprintln!("Decoding failed after encoding: {:?}", e),
            }
        }

        // --- run 7 ---------------------------------------------------------------
        {
            let sample_bytes = if bytes.is_empty() {
                vec![b'i', b'n']
            } else {
                bytes.to_vec()
            };
            match serde_yaml::from_slice::<serde_yaml::Value>(&sample_bytes) {
                Ok(value) => println!("Parsed YAML successfully: {:?}", value),
                Err(err) => eprintln!("Failed to parse YAML: {}", err),
            }

            let number_input = num.to_string();
            let deserialized: Number = serde_yaml::from_str(&number_input).unwrap();
            let serialized_yaml = serde_yaml::to_string(&deserialized).unwrap();
            let roundtrip: Number = serde_yaml::from_str(&serialized_yaml).unwrap();

            println!("Deserialized from YAML (50.): {:?}", deserialized);
            println!("Serialized to YAML: {}", serialized_yaml);
            println!("Deserialized again: {:?}", roundtrip);

            assert_eq!(deserialized, roundtrip, "Roundtrip failed!");
        }

        // --- run 8 ---------------------------------------------------------------
        {
            if let Err(e) = panic::catch_unwind(|| simple_asn1::from_der(bytes)) {
                eprintln!("Caught panic in run 8: {:?}", e);
            }
        }

        // --- run 9 ---------------------------------------------------------------
        {
            let mut decoder = Decoder::new();
            match decoder.decompress_vec(bytes) {
                Ok(_) => println!("Decompression succeeded."),
                Err(e) => eprintln!("Decompression failed: {:?}", e),
            }
        }

        // --- run 14 --------------------------------------------------------------
        {
            println!("run 14");
            if let Err(e) = panic::catch_unwind(|| ssh_parser::parse_ssh_packet(bytes)) {
                eprintln!("Caught panic in run 14: {:?}", e);
            }
        }

        // --- run 15 --------------------------------------------------------------
        {
            if let Err(e) = panic::catch_unwind(|| movie::parse_movie(bytes)) {
                eprintln!("Caught panic in run 15: {:?}", e);
            }
        }

        // --- run 17 --------------------------------------------------------------
        {
            println!("run 17");
            let mut file_bytes = bytes.to_vec();
            file_bytes.extend_from_slice(bytes);
            match test_decode(&file_bytes) {
                Ok(_) => println!("Decoding succeeded unexpectedly"),
                Err(err) => println!("Decoding error: {:?}", err),
            }
        }
    }
}

pub fn benchmark_numeric(num: u64) {
    unsafe {
        // --- run 3 ---------------------------------------------------------------
        {
            println!("running line 152");
            let repeat = (num as usize).min(16);
            let nested = "{}".repeat(repeat);
            let result: Result<ron::Value, _> = ron::from_str(&nested);
            match result {
                Ok(value) => println!("Parsed successfully: {:?}", value),
                Err(err) => eprintln!("Failed to parse: {}", err),
            }

            println!("running line 153");
            let input = "{}";
            let value: ron::Value = ron::from_str(input).expect("Valid input should not fail");
            println!("{:?}", value);

            let serialized = ron::to_string(&value).unwrap();
            println!("{:?}", serialized);
        }

        // --- run 12 --------------------------------------------------------------
        {
            println!("run 12 disabled (sqlparser requires newer Rust)");
            let _ = num;
        }
    }
}

pub fn benchmark_string_ops(str1: &str, str2: &str, bytes: &[u8]) {
    unsafe {
        // --- run 4 ---------------------------------------------------------------
        {
            let mut cursor = Cursor::new(str1.to_owned());
            let _ = Ini::read_from(&mut cursor).unwrap();
        }

        // --- run 5 ---------------------------------------------------------------
        {
            if let Err(e) = panic::catch_unwind(|| rustc_demangle::demangle(str1)) {
                eprintln!("Caught panic in run 5: {:?}", e);
            }
        }

        // --- run 6 ---------------------------------------------------------------
        {
            if let Err(e) = panic::catch_unwind(|| VersionReq::parse(str2).unwrap()) {
                eprintln!("Caught panic in run 6: {:?}", e);
            }
        }

        // --- run 13 --------------------------------------------------------------
        {
            println!("run 13");
            if let Err(e) = panic::catch_unwind(|| ssh_keys::openssh::parse_private_key(str1)) {
                eprintln!("Caught panic in run 13: {:?}", e);
            }
        }

        // --- run 16 --------------------------------------------------------------
        {
        println!("run 16 disabled (symbolic/minidump requires newer Rust)");
        }

        // --- run 18 --------------------------------------------------------------
        {
            println!("run 18");
            if let Err(e) = panic::catch_unwind(|| syn_188::parse_str::<Expr>(str1)) {
                eprintln!("Caught panic in run 18: {:?}", e);
            }
        }

        // --- run 19 --------------------------------------------------------------
        {
        let _ = (str1, str2);
        println!("run 19 disabled (tera dependency requires newer toolchain)");
        }

        // --- run 20 --------------------------------------------------------------
        {
        println!("run 20 disabled (sqlformat/sqlparser require newer toolchain)");
        }
    }
}

pub fn benchmark_misc() {
    unsafe {
        // --- run 11 --------------------------------------------------------------
        {
            println!("run 11");
            let v1 = ScVec::try_from((0, 1)).unwrap();
            let v2 = ScVec::try_from((0, 0, 2)).unwrap();
            let budget = Budget::default();
            let actual_cmp = budget.compare(&v1, &v2).unwrap();
            assert_eq!(actual_cmp, Ordering::Greater);

            let host = Host::default();
            let symbol = Symbol::try_from_val(&host, &"#");
            match symbol {
                Ok(_) => println!("Unexpected success in symbol conversion"),
                Err(_) => println!("Symbol conversion failed as expected"),
            }
        }
    }
}

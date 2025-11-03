#![allow(clippy::needless_return)]

#[macro_use]
extern crate nom;
extern crate npy;
#[macro_use]
extern crate npy_derive;

use base64;
use libflate::deflate::Decoder as DeflateDecoder;
use lopdf::Document;
use lz4_flex::block::compress::compress_prepend_size;
use lz4_flex::block::decompress::decompress_size_prepended;
use lz_fear::framed::LZ4FrameReader;
use mp4ameta::Tag;
// use naga::front::wgsl::Parser; // disabled: naga pulls dependencies needing Rust >=1.82
use npy::NpyData;
use ntfs::Ntfs;
// use rust_minidump_85::Minidump as Minidump85;
// use rust_minidump_86::{
//     Minidump,
//     MinidumpAssertion,
//     MinidumpBreakpadInfo,
//     MinidumpCrashpadInfo,
//     MinidumpException,
//     MinidumpLinuxCpuInfo,
//     MinidumpLinuxEnviron,
//     MinidumpLinuxLsbRelease,
//     MinidumpLinuxMaps,
//     MinidumpLinuxProcStatus,
//     MinidumpMacCrashInfo,
//     MinidumpMemoryInfoList,
//     MinidumpMemoryList,
//     MinidumpMiscInfo,
//     MinidumpModuleList,
//     MinidumpSystemInfo,
//     MinidumpThreadList,
//     MinidumpThreadNames,
//     MinidumpUnloadedModuleList,
// };
use std::error::Error;
use std::io::{self, Cursor, Read};

#[derive(NpyData, Debug)]
struct Array {
    a: i32,
}

// pub struct BenchmarkData {
//     pub run1_input: Vec<u8>,
//     pub run2_pdf: Vec<u8>,
//     pub run3_data: Vec<u8>,
//     pub run4_text: Vec<u8>,
//     pub run5_panic_b64: String,
//     pub run5_data_b64: String,
//     pub run9_data: Vec<u8>,
//     pub run10_data: Vec<u8>,
//     pub run12_data: Vec<u8>,
//     pub run13_data: Vec<u8>,
//     pub run14_data: Vec<u8>,
//     pub run14_mutdata: Vec<u8>,
// }
//
// impl Default for BenchmarkData {
//     fn default() -> Self {
//         Self {
//             run1_input: b"\x04\x04\x04\x04:\x1az*\xfc\x06\x01\x90\x01\x06\x01".to_vec(),
//             run2_pdf: b"%PDF-1.5\n\
//     000000028100 000 n \n\
//     0000000338 00000 n \n\
//     %%EOF".to_vec(),
//             run3_data: b"\x04\x22\x4D\x18\x64\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00".to_vec(),
//             run4_text: b"Some data to compress and decompress using lz4_flex".to_vec(),
//             run5_panic_b64: "TURNUJOnAAAA/2ZmZFlmZmZmZkAKCmZwCrv///8K/wo=".to_owned(),
//             run5_data_b64: "U29tZSB2YWxpZCBkYXRhIHN0cmluZw==".to_owned(),
//             run9_data: include_bytes!("run9_data.bin").to_vec(),
//             run10_data: include_bytes!("run10_data.bin").to_vec(),
//             run12_data: vec![0u8; 1024],
//             run13_data: include_bytes!("run13_data.bin").to_vec(),
//             run14_data: include_bytes!("run14_data.bin").to_vec(),
//             run14_mutdata: include_bytes!("run14_mutdata.bin").to_vec(),
//         }
//     }
// }

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
    println!("crate batch 3 benchmark starting");
    let data = BenchmarkData::default();
    benchmark(&data);
    println!("crate batch 3 benchmark ending");
}

pub fn benchmark(data: &BenchmarkData) {
    benchmark_vec_u8(&data.testVecU8);
    benchmark_string_operations(&data.testString2, &data.testVecU8);
}

pub fn benchmark_vec_u8(bytes: &[u8]) {
    unsafe {
        // --- run 1 ---------------------------------------------------------------
        {
            let mut decoder = DeflateDecoder::new(Cursor::new(bytes));
            let _ = io::copy(&mut decoder, &mut io::sink());
        }

        // --- run 2 ---------------------------------------------------------------
        {
            let _ = Document::load_mem(bytes);
        }

        // --- run 3 ---------------------------------------------------------------
        {
            let input = Cursor::new(bytes.to_vec());
            let mut output = Vec::new();

            if let Ok(reader) = LZ4FrameReader::new(input) {
                let _ = reader.into_read().read_to_end(&mut output);
            }
        }

        // --- run 4 ---------------------------------------------------------------
        {
            let compressed = compress_prepend_size(bytes);
            let _ = decompress_size_prepended(&compressed);
        }

        // --- run 8 ---------------------------------------------------------------
        {
            // Original implementation returns immediately; nothing to execute here.
        }

        // --- run 9 ---------------------------------------------------------------
        {
            let mut cursor = Cursor::new(bytes.to_vec());
            let _ = Tag::read_from(&mut cursor);
        }

    // --- run 10 disabled: naga requires a newer Rust compiler --------------
    // {
    //     let data_str = std::str::from_utf8(bytes).unwrap_or("");
    //     let _ = Parser::new().parse(data_str);
    // }

        // --- run 12 --------------------------------------------------------------
        {
            named!(parser01<&[u8], ()>,
                do_parse!(
                    hdr: take!(1) >>
                    data: take!(1023) >>
                    ( () )
                )
            );

            let mut buffer = bytes.to_vec();
            if buffer.len() < 1024 {
                buffer.resize(1024, 0);
            }
            let _ = parser01(&buffer);
        }

        // --- run 13 --------------------------------------------------------------
        {
            let _ = npy::from_bytes::<Array>(bytes);
        }

        // --- run 14 --------------------------------------------------------------
        {
            let mut cursor = Cursor::new(bytes.to_vec());
            let _ = Ntfs::new(&mut cursor);

            let mut mutated = bytes.to_vec();
            mutated.extend_from_slice(bytes);
            let mut cursor_mut = Cursor::new(mutated);
            if let Ok(mut fs) = Ntfs::new(&mut cursor_mut) {
                if let Err(e) = fs.read_upcase_table(&mut cursor_mut) {
                    eprintln!("Failed to read upcase table: {}", e);
                }
            } else {
                eprintln!("Failed to create NTFS filesystem");
            }
        }

        // --- run 15 --------------------------------------------------------------
        {
            // Intentionally left blank as in source.
        }
    }
}

pub fn benchmark_string_operations(str_data: &str, bytes: &[u8]) {
    unsafe {
        // --- run 5 ---------------------------------------------------------------
        {
        // Disabled: rust-minidump requires newer Rust toolchains.
        // let panic_b64 = base64::encode(bytes);
        // let panic_data = base64::decode(panic_b64).unwrap_or_default();
        //
        // let encoded_payload = base64::encode(str_data.as_bytes());
        // let decoded = base64::decode(&encoded_payload)
        //     .unwrap_or_else(|_| str_data.as_bytes().to_vec());
        // Minidump85::read(decoded.clone());
        //
        // if let Ok(dump) = Minidump::read(decoded.clone()) {
        //     let _ = dump.get_stream::<MinidumpAssertion>();
        //     let _ = dump.get_stream::<MinidumpBreakpadInfo>();
        //     let _ = dump.get_stream::<MinidumpCrashpadInfo>();
        //     let _ = dump.get_stream::<MinidumpException>();
        //     let _ = dump.get_stream::<MinidumpLinuxCpuInfo>();
        //     let _ = dump.get_stream::<MinidumpLinuxEnviron>();
        //     let _ = dump.get_stream::<MinidumpLinuxLsbRelease>();
        //     let _ = dump.get_stream::<MinidumpLinuxMaps>();
        //     let _ = dump.get_stream::<MinidumpLinuxProcStatus>();
        //     let _ = dump.get_stream::<MinidumpMacCrashInfo>();
        //     let _ = dump.get_stream::<MinidumpMemoryInfoList>();
        //     let _ = dump.get_stream::<MinidumpMemoryList>();
        //     let _ = dump.get_stream::<MinidumpMiscInfo>();
        //     let _ = dump.get_stream::<MinidumpModuleList>();
        //     let _ = dump.get_stream::<MinidumpSystemInfo>();
        //     let _ = dump.get_stream::<MinidumpThreadNames>();
        //     let _ = dump.get_stream::<MinidumpThreadList>();
        //     let _ = dump.get_stream::<MinidumpUnloadedModuleList>();
        // }
        //
        // match Minidump::read(&decoded[..]) {
        //     Ok(f) => {
        //         let e = f.get_stream::<MinidumpLinuxMaps>().unwrap_err();
        //         assert_eq!(e.to_string(), "Data error");
        //     }
        //     Err(e) => {
        //         println!("Expected to parse the header, got {:?}", e);
        //     }
        // }
        //
        // let _ = panic_data;
        }
    }
}

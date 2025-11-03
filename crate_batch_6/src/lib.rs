use std::io::Read;
use std::mem;
use std::os::unix::io::RawFd;
use std::panic;
use std::slice;

use tinytemplate::TinyTemplate;
use todotxt::Task;
// use tokei::{Config, LanguageType};
use toml;
use ttf_parser;
use ubyte::ByteUnit;
use unicode_segmentation::UnicodeSegmentation;
use url::Url;
use uuid::Uuid;
use vial;
use vobsub;
use wayland_commons::wire::{ArgumentType, Message};
use ws;
use yaxpeax_x86;
use zip;

/*pub struct BenchmarkData {
    pub run1_template: String,
    pub run2_task: Vec<u8>,
    pub run3_input: Vec<u8>,
    pub run4_first: String,
    pub run4_second: String,
    pub run4_brackets: usize,
    pub run5_font: Vec<u8>,
    pub run6_byte_unit: String,
    pub run7_grapheme: String,
    pub run7_word_bounds: String,
    pub run9_url: String,
    pub run9_edge: String,
    pub run10_uuid: String,
    pub run12_data: Vec<u8>,
    pub run13_message_data: Vec<u8>,
    pub run14_bytes: Vec<u8>,
    pub run15_bytes: Vec<u8>,
    pub run16_zip: Vec<u8>,
}

impl Default for BenchmarkData {
    fn default() -> Self {
        Self {
            run1_template: "{}".to_owned(),
            run2_task: b"2021-01-01 This is a task +project @context".to_vec(),
            run3_input: b"<script>\ra </script>".to_vec(),
            run4_first: r#"
         q = "\u000B"
    "#.to_owned(),
            run4_second: r#"
        "\n" = 5
    "#.to_owned(),
            run4_brackets: 2,
            run5_font: b"\x00\x01\x00\x00\x00\x0f\x00\x10\x00PTT-W\x002h\xd7\x81x\x00\"".to_vec(),
            run6_byte_unit: "1 KB".to_owned(),
            run7_grapheme: "\u{1F938}\u{1F3FE}\u{1F3FE}".to_owned(),
            run7_word_bounds: "j\u{FFFD}jjjjjjjjjjj\u{0489}\u{200D}\u{2764}jjjjjjjjj\u{0489}j\u{FFFD}\u{FFFD}\u{FFFD}\"jjjjjj\"jjD\u{0409}\u{0489}0\\f\u{FFFD}".to_owned(),
            run9_url: "www.example.com".to_owned(),
            run9_edge: "p:/.//:/".to_owned(),
            run10_uuid: "F9168C5E-CEB2F4faaFB6BFF329BF39FA1E4".to_owned(),
            run12_data: b"".to_vec(),
            run13_message_data: vec![
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, // RawFd data
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, // ArgumentType data
                0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, // Message data
            ],
            run14_bytes: b"\xff\xfe\xfe\xfe\xfe\xfe\xfe\xfe\xfe\xfe\xfe\xfe\xfe\xfe\xfe\xfe".to_vec(),
            run15_bytes: (&[98, 98, 101, 10]).to_vec(),
            run16_zip: b"PK\x03\x04\n\x00\x00\x00\x00\x00\xe9p\xdaJ\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x04\x00\x1c\x00zip/UT\t\x00\x03\xf5\xf8PY\"\xf9PYux\x0b\x00\x01\x04\xf5\x01\x00\x00\x04\x14\x00\x00\x00PK\x03\x04\x14\x00\x00\x00\x08\x00\xe9p\xdaJ\xf6\xe3\xf0\xa6\xd6\x00\x00\x00\x04\x18\x00\x00\r\x00\x1c\x00zip/.".to_vec(),
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
    println!("running crate batch 6 benchmark");
    let data = BenchmarkData::default();
    benchmark(&data);
    println!("ending crate batch 6 benchmark");
}

struct Builder;

impl ttf_parser::OutlineBuilder for Builder {
    #[inline]
    fn move_to(&mut self, _: f32, _: f32) {
        panic!();
    }

    #[inline]
    fn line_to(&mut self, _: f32, _: f32) {
        panic!();
    }

    #[inline]
    fn quad_to(&mut self, _: f32, _: f32, _: f32, _: f32) {
        panic!();
    }

    #[inline]
    fn curve_to(&mut self, _: f32, _: f32, _: f32, _: f32, _: f32, _: f32) {
        panic!();
    }

    #[inline]
    fn close(&mut self) {
        panic!();
    }
}

pub fn convert_slice<T: Sized>(data: &[u8]) -> Option<&[T]> {
    unsafe {
        let n = mem::size_of::<T>();

        if n == 0 || data.as_ptr().align_offset(n) != 0 || data.len() % n != 0 {
            return None;
        }

        Some(slice::from_raw_parts(
            data.as_ptr() as *const T,
            data.len() / n,
        ))
    }
}

pub fn get_arg_types(data: &[u8]) -> Option<[ArgumentType; 16]> {
    unsafe {
        use ArgumentType::*;

        if data.len() != 16 {
            return None;
        }

        let mut res = [Int; 16];
        for i in 0..16 {
            res[i] = match data[i] & 0b111 {
                0 => Int,
                1 => Uint,
                2 => Fixed,
                3 => Str,
                4 => Object,
                5 => NewId,
                6 => Array,
                7 => Fd,
                _ => return None,
            };
        }
        Some(res)
    }
}

pub fn benchmark(data: &BenchmarkData) {
    benchmark_template_and_strings(&data.testString, &data.testString2, data.testU64);
    benchmark_vec_u8(&data.testVecU8);
    benchmark_misc();
}

pub fn benchmark_template_and_strings(str1: &str, str2: &str, num: u64) {
    unsafe {
        // --- run 1 ---------------------------------------------------------------
        {
            println!("run 1");
            let mut tpl = TinyTemplate::new();
            let _ = tpl.add_template("template", str1);
        }

        // --- run 4 ---------------------------------------------------------------
        {
            println!("run 4");
            let first_input = format!("key = \"{}\"", str1);
            let value_first: toml::Value = toml::from_str(&first_input).unwrap();
            println!("{:?}", value_first);
            println!("{}", toml::to_string(&value_first).unwrap());

            let second_input = format!("other = \"{}\"", str2);
            let value_second = toml::from_str::<toml::Value>(&second_input)
                .unwrap_or_else(|_| toml::Value::String(str2.to_owned()));
            println!("{:?}", value_second);
            match toml::to_string(&value_second) {
                Ok(serialized) => println!("{}", serialized),
                Err(e) => eprintln!("Error serializing TOML: {}", e),
            }

            let bracket_count = ((num as usize) % 8).max(1);
            let brackets = "[".repeat(bracket_count);
            let input_string = format!("x={}", &brackets);
            let _: Result<toml::Value, _> = toml::from_str(&input_string);
        }

        // --- run 6 ---------------------------------------------------------------
        {
            println!("run 6");
            let byte_unit_input = if str1.contains('B') {
                str1.to_owned()
            } else {
                format!("{} B", num.max(1))
            };
            match byte_unit_input.parse::<ByteUnit>() {
                Ok(byte_unit) => println!("Parsed byte unit: {:?}", byte_unit),
                Err(e) => eprintln!("Error parsing byte unit: {:?}", e),
            }
        }

        // --- run 7 ---------------------------------------------------------------
        {
            println!("run 7");
            let grapheme_text = if str1.is_empty() {
                str2.to_owned()
            } else {
                str1.to_owned()
            };
            let forward = UnicodeSegmentation::graphemes(grapheme_text.as_str(), true).collect::<Vec<_>>();
            let forward_reversed = forward.clone().into_iter().rev().collect::<Vec<_>>();
            let reverse = UnicodeSegmentation::graphemes(grapheme_text.as_str(), true)
                .rev()
                .collect::<Vec<_>>();
            assert_eq!(forward_reversed, reverse);

            let word_bounds_text = format!("{} {}", str1, str2);
            let forward = word_bounds_text
                .split_word_bounds()
                .collect::<Vec<_>>();
            let forward_reversed = forward.clone().into_iter().rev().collect::<Vec<_>>();
            let reverse = word_bounds_text.split_word_bounds().rev().collect::<Vec<_>>();
            assert_eq!(forward_reversed, reverse);
        }

        // --- run 9 ---------------------------------------------------------------
        {
            let base_url = format!("http://{}", str1.replace(' ', ""));
            let _ = Url::parse(&base_url);

            let edge_candidate = if str2.contains("://") {
                str2.to_owned()
            } else {
                "p://:/".to_string()
            };
            match Url::parse(&edge_candidate) {
                Ok(u) => {
                    let s = u.as_str();
                    println!("Parsed edge URL: {}", s);
                    match Url::parse("p://:/") {
                        Ok(parsed_url) => println!("Parsed URL: {}", parsed_url),
                        Err(e) => eprintln!("Error parsing URL: {:?}", e),
                    }
                }
                Err(e) => eprintln!("Error parsing URL: {:?}", e),
            }
        }

        // --- run 10 --------------------------------------------------------------
        {
            println!("run 10");
            let _ = panic::catch_unwind(|| Uuid::parse_str(str1).unwrap());
        }
    }
}

pub fn benchmark_vec_u8(bytes: &[u8]) {
    unsafe {
        // --- run 2 ---------------------------------------------------------------
        {
            println!("run 2");
            if let Ok(line) = std::str::from_utf8(bytes) {
                let _: Result<Task, _> = line.parse();
            }
        }

        // --- run 3 ---------------------------------------------------------------
        {
            println!("run 3 disabled (tokei depends on globset with edition2024)");
            let _ = bytes;
        }

        // --- run 5 ---------------------------------------------------------------
        {
            println!("run 5");
            match ttf_parser::Face::from_slice(bytes, 0) {
                Ok(face) => {
                    let _ = face.outline_glyph(ttf_parser::GlyphId(0), &mut Builder);
                }
                Err(e) => eprintln!("Error parsing font: {:?}", e),
            }
        }

        // --- run 12 --------------------------------------------------------------
        {
            println!("run 12");
            let mut subtitle_bytes = bytes.to_vec();
            subtitle_bytes.push(0);
            for _ in vobsub::subtitles(&subtitle_bytes) {
                // iterate
            }
        }

        // --- run 13 --------------------------------------------------------------
        {
            println!("run 13");
            let mut message_data = bytes.to_vec();
            if message_data.len() < 48 {
                message_data.resize(48, 0);
            }
            let slice = message_data.as_slice();
            let fds = convert_slice::<RawFd>(&slice[..16]);
            let args = get_arg_types(&slice[16..32]);
            let rest = convert_slice::<u32>(&slice[32..]);

            if let (Some(fds), Some(args), Some(payload)) = (fds, args, rest) {
                let result = panic::catch_unwind(|| Message::from_raw(payload, &args, fds));
                match result {
                    Ok(_) => println!("✅ Message parsing succeeded."),
                    Err(_) => eprintln!("❌ Caught panic in `Message::from_raw`"),
                }
            } else {
                eprintln!("❌ Invalid input detected, skipping Message::from_raw");
            }
        }

        // --- run 14 --------------------------------------------------------------
        {
            println!("run 14");
            let mut cursor = std::io::Cursor::new(bytes.to_vec());
            let _ = ws::Frame::parse(&mut cursor);
        }

        // --- run 15 --------------------------------------------------------------
        {
            let decoder = yaxpeax_x86::amd64::InstDecoder::default();
            drop(decoder.decode_slice(bytes));
        }

        // --- run 16 --------------------------------------------------------------
        {
            println!("running run 16");
            let mut zip_bytes = bytes.to_vec();
            if zip_bytes.len() < 4 {
                zip_bytes.extend_from_slice(&[0u8; 4]);
            }
            let reader = std::io::Cursor::new(zip_bytes);
            let mut archive = if let Ok(x) = zip::ZipArchive::new(reader) {
                x
            } else {
                return;
            };

            for i in 0..archive.len() {
                let file = archive.by_index(i).unwrap();
                let _size = file.bytes().count();
            }
        }
    }
}

pub fn benchmark_misc() {
    unsafe {
        // --- run 11 --------------------------------------------------------------
        {
            println!("run 11");
            let request = vial::Request::from_reader(std::io::empty());
            match request {
                Ok(req) => {
                    vial::util::percent_decode(req.path());
                }
                Err(e) => eprintln!("Error creating request: {:?}", e),
            }
        }
    }
}

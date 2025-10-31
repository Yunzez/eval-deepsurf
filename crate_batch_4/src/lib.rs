use glob::glob;
use pcapng::block::parse_block;
use pdf_112::file::File as Pdf112File;
use pdf_115::file::File as Pdf115File;
use phonenumber;
use pgp;
use plist;
use prettytable::{Cell, Row, Table};
use pulldown_cmark_128;
use pulldown_cmark_131;
use pulldown_cmark_133;
use quick_xml::reader::Reader;
use std::ffi::CString;
use std::io::{self, Cursor};
use std::str;

// pub struct BenchmarkData {
//     pub run2_block: Vec<u8>,
//     pub run3_number: String,
//     pub run4_pdf_data: Vec<u8>,
//     pub run5_xref_input: Vec<u8>,
//     pub run6_signature: [u8; 8],
//     pub run7_plist: Vec<u8>,
//     pub run8_png: Vec<u8>,
//     pub run9_table: Vec<Vec<(String, u8)>>,
//     pub run10_text: String,
//     pub run10_crashing: String,
//     pub run11_bytes: Vec<u8>,
//     pub run11_doc: String,
// }
//
// impl Default for BenchmarkData {
//     fn default() -> Self {
//         Self {
//             run2_block: b"\x0a\x0b\x0c\x0d\x0e\x0f\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19".to_vec(),
//             run3_number: "2067736026".to_owned(),
//             run4_pdf_data: b"%PDF-startxref>".to_vec(),
//             run5_xref_input: b"0 1\n0000000000 65535 f\n".to_vec(),
//             run6_signature: [5, 2, 2, 11, 0, 2, 0, 0],
//             run7_plist: b"bplist00\x10\x01".to_vec(),
//             run8_png: b"\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x00\x01\x00\x00\x00\x01\x08\x06\x00\x00\x00\x1f\x15\xc4\x89\x00\x00\x00\nIDATx\xdac\xf8\x0f\x00\x01\x01\x01\x00\x18\xdd\x03\x1a\x00\x00\x00\x00IEND\xaeB`\x82".to_vec(),
//             run9_table: vec![
//                 vec![
//                     ("Cell1".to_string(), 0),
//                     ("Cell2".to_string(), 1),
//                     ("Cell3".to_string(), 2),
//                 ],
//                 vec![
//                     ("Cell4".to_string(), 0),
//                     ("Cell5".to_string(), 1),
//                     ("Cell6".to_string(), 2),
//                 ],
//             ],
//             run10_text: "01010101".to_owned(),
//             run10_crashing: r"[]([]([]([](\u{1b}]([](|AN\u{b}|||[](&&&#0000000000000000000000\u{1}\u{0}\u{0}\u{0}[]([]([]([](|||||||[](&&&#0000000000000000000\u{1000000000}[]([]([](|||||||[](&&&#0018446744073709551([](|||||||[](&&&#001844674407370955161615\u{1}\u{0}\u{0}\u{0}[]([]\u{112}\u{32}\u{00372}\u{008}\u{005156194}&#&#&#&".to_owned(),
//             run11_bytes: b"\x00".to_vec(),
//             run11_doc: "<!D>".to_owned(),
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

pub struct TableCellData {
    pub text: String,
    pub alignment: u8,
}

pub struct TableRowData {
    pub cells: Vec<TableCellData>,
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

pub unsafe fn main() {
    println!("crate batch 4 benchmark starting");
    let data = BenchmarkData::default();
    benchmark(&data);
    println!("crate batch 4 benchmark ending");
}

pub unsafe fn align_from_u8(x: u8) -> prettytable::format::Alignment {
    match x {
        0 => prettytable::format::Alignment::LEFT,
        1 => prettytable::format::Alignment::CENTER,
        _ => prettytable::format::Alignment::RIGHT,
    }
}

pub unsafe fn run_table(data: &[TableRowData]) {
    let mut pt = Table::new();
    for row in data {
        let cells = row
            .cells
            .iter()
            .map(|cell| Cell::new_align(&cell.text, align_from_u8(cell.alignment)))
            .collect();
        pt.add_row(Row::new(cells));
    }

    let _ = pt.print(&mut io::sink());
}

pub unsafe fn benchmark(data: &BenchmarkData) {
    benchmark_misc();
    benchmark_vec_u8(&data.testVecU8, &data.testKey);
    benchmark_strings(&data.testString, &data.testString2, &data.testVecU8);
}

unsafe fn benchmark_misc() {
    // --- run 1 ---------------------------------------------------------------
    {
        // Original code is commented out.
        let _ = CString::new("noop").unwrap();
    }
}

pub unsafe fn benchmark_vec_u8(bytes: &[u8], key: &[u8]) {
    // --- run 2 ---------------------------------------------------------------
    {
        let _ = parse_block(bytes);
    }

    // --- run 4 ---------------------------------------------------------------
    {
        println!("running pdf_112");
        Pdf112File::from_data(bytes);
    }

    // --- run 5 ---------------------------------------------------------------
    {
        println!("running pdf_115");
        let mut lexer = pdf_115::parser::Lexer::new(bytes);
        let resolve = pdf_115::object::NoResolve;

        match pdf_115::parser::parse_xref_stream_and_trailer(&mut lexer, &resolve) {
            Ok((xref_sections, dictionary)) => {
                println!("Parsed xref sections: {:?}", xref_sections);
                println!("Parsed dictionary: {:?}", dictionary);
            }
            Err(e) => println!("Failed to parse xref stream and trailer: {:?}", e),
        }

        for entry in glob("invalid/*.pdf").expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    let path_str = match path.to_str() {
                        Some(p) => p,
                        None => continue,
                    };
                    println!("\n\n == Now testing `{}` ==\n", path_str);

                    match Pdf115File::<Vec<u8>>::open(path_str) {
                        Ok(file) => {
                            for i in 0..file.num_pages() {
                                let _ = file.get_page(i);
                            }
                        }
                        Err(_) => continue,
                    }
                }
                Err(e) => panic!("error when reading glob patterns: {:?}", e),
            }
        }
    }

    // --- run 6 ---------------------------------------------------------------
    {
        println!("running pgp");
        let mut signature = [0u8; 8];
        let copy_len = key.len().min(signature.len());
        if copy_len > 0 {
            signature[..copy_len].copy_from_slice(&key[..copy_len]);
        }
        let _ = pgp::Signature::from_slice(pgp::types::Version::New, &signature);
    }

    // --- run 7 ---------------------------------------------------------------
    {
        println!("running plist");
        let cursor = Cursor::new(bytes.to_vec());
        let _ = plist::Value::from_reader(cursor);
    }

    // --- run 8 ---------------------------------------------------------------
    {
        let _ = decode_png(bytes);
    }
}

pub unsafe fn benchmark_strings(str: &str, str2: &str, bytes: &[u8]) {
    // --- run 3 ---------------------------------------------------------------
    {
        let digits: String = str.chars().filter(|c| c.is_ascii_digit()).collect();
        let input = if digits.is_empty() {
            String::from("1234567890")
        } else {
            digits
        };
        let _ = phonenumber::parse(None, input);
    }

    // --- run 9 ---------------------------------------------------------------
    {
        let table_data = [
            TableRowData {
                cells: vec![TableCellData {
                    text: str.to_owned(),
                    alignment: 0u8,
                }],
            },
            TableRowData {
                cells: vec![TableCellData {
                    text: str2.to_owned(),
                    alignment: 1u8,
                }],
            },
        ];
        run_table(&table_data);
    }

    // --- run 10 --------------------------------------------------------------
    {
        let text = str.to_owned();
        let _ = pulldown_cmark_128::Parser::new(&text);

        let mut output = String::new();
        let parser = pulldown_cmark_131::Parser::new(&text);
        pulldown_cmark_131::html::push_html(&mut output, parser);

        let mut opts = pulldown_cmark_133::Options::empty();
        opts.insert(pulldown_cmark_133::Options::ENABLE_HEADING_ATTRIBUTES);

        for _ in pulldown_cmark_133::Parser::new_ext(&text, opts) {}

        let _ = str2.to_owned();
    }

    // --- run 11 --------------------------------------------------------------
    {
        let cursor = Cursor::new(bytes.to_vec());
        let mut reader = Reader::from_reader(cursor);
        reader.trim_text(true);
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(quick_xml::events::Event::Eof) | Err(_) => break,
                _ => buf.clear(),
            }
        }

        let mut reader = Reader::from_str(str);
        let mut buf = Vec::new();
        let _ = reader.read_event(&mut buf);
        let _ = reader.read_event(&mut buf);
    }
}

pub unsafe fn decode_png(data: &[u8]) -> io::Result<Vec<u8>> {
    let limits = png::Limits { bytes: 1 << 16 };
    let decoder = png::Decoder::new_with_limits(data, limits);
    let (info, mut reader) = decoder.read_info()?;

    if info.buffer_size() > 5_000_000 {
        return Err(io::Error::new(io::ErrorKind::Other, "memory limit exceeded"));
    }

    let mut buffer = vec![0u8; info.buffer_size()];
    reader.next_frame(&mut buffer)?;

    Ok(buffer)
}

// use std::io::{self, BufReader, Cursor, Read};

// // *** main start
// pub fn main() {
//     println!("crate batch 4 starting");
//     run_1();
//     run_2();
//     run_4();
//     run_5();
//     run_6();
//     run_7();
//     run_8();
//     run_9();
//     run_10();
//     run_11();
//     println!("crate batch 4 ending");
// }
// // *** main end

// use std::ffi::{CString, NulError};
// use std::str;

// fn run_1() {
//     // ! for pancurses, not solved
//     // FIXME: make pancurses work
//     // let null: Vec<u8> = vec![0];
//     // let null_str = str::from_utf8(&null).unwrap();

//     // match CString::new(null_str) {
//     //     Ok(c_string) => println!("CString created successfully: {:?}", c_string),
//     //     Err(e) => println!("Failed to create CString: {:?}", e),
//     // }
// }

// fn run_2() {
//     // ! crashing output:
//     // let data = b"h;\x00\x00\x00\x00\x00\x00\x00\x00\x07/\x8a";

//     let data = b"\x0a\x0b\x0c\x0d\x0e\x0f\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19";
//     let _ = pcapng::block::parse_block(data);
// }

// fn run_3() {
//     // ! test for phonenumber
//     let data = "  2 22#:";
//     let data = "2067736026";
//     phonenumber::parse(None, data);
// }

// fn run_4() {
//     //?pdf with hash 316168c
//     println!("running pdf_112");
//     // info: line 112 & 113 & 114, the are all the same
//     pdf_112::file::File::from_data(b"%PDF-startxref>".as_ref());
//     // pdf_112::file::File::from_data(b"%PDF>".as_ref());
// }

// use glob::glob;
// use pdf_115::file::File;
// fn run_5() {
//     //? pdf with hash 99e70cd
//     println!("running pdf_115");
//     // info: line 115 & 117
//     let mut lexer = pdf_115::parser::Lexer::new(b"0 1\n0000000000 65535 f\n");
//     let resolve = pdf_115::object::NoResolve;

//     match pdf_115::parser::parse_xref_stream_and_trailer(&mut lexer, &resolve) {
//         Ok((xref_sections, dictionary)) => {
//             println!("Parsed xref sections: {:?}", xref_sections);
//             println!("Parsed dictionary: {:?}", dictionary);
//         }
//         Err(e) => println!("Failed to parse xref stream and trailer: {:?}", e),
//     }

//     // info: line 116

//     for entry in glob("invalid/*.pdf").expect("Failed to read glob pattern") {
//         match entry {
//             Ok(path) => {
//                 let path = path.to_str().unwrap();
//                 println!("\n\n == Now testing `{}` ==\n", path);

//                 match File::<Vec<u8>>::open(path) {
//                     Ok(file) => {
//                         for i in 0..file.num_pages() {
//                             let _ = file.get_page(i);
//                         }
//                     }
//                     Err(_) => {
//                         continue;
//                     }
//                 }
//             }
//             Err(e) => panic!("error when reading glob patterns: {:?}", e),
//         }
//     }
// }

// // use pgp::de::Deserialize;
// fn run_6() {
//     // info: pgp
//     println!("running pgp");
//     let data = [5, 2, 2, 11, 0, 2, 0, 0];
//     let _ = pgp::Signature::from_slice(pgp::types::Version::New, &data);
// }

// use plist::Date;
// use std::time::Duration;
// fn run_7() {
//     // info: plist
//     println!("running plist");

//     // ! crashing data
//     // let data = b"bplist00\x10\x01\x00\x00\x00\x00\x00\x003~\x00\x00\x00\x00\x00\x00\x00\x01\x02\x00\x00\x00\x00\x00\x00\x00\n\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x00\x00\x00\x00\x00";

//     let data = b"bplist00\x10\x01";

//     let cursor = Cursor::new(data);
//     plist::Value::from_reader(cursor);
// }

// extern crate png;

// fn decode_png(data: &[u8]) -> io::Result<Vec<u8>> {
//     let limits = png::Limits { bytes: 1 << 16 };
//     let decoder = png::Decoder::new_with_limits(data, limits);
//     let (info, mut reader) = decoder.read_info()?;

//     if info.buffer_size() > 5_000_000 {
//         return Err(std::io::Error::new(
//             std::io::ErrorKind::Other,
//             "memory limit exceeded",
//         ));
//     }

//     let mut data = vec![0u8; info.buffer_size()];
//     reader.next_frame(&mut data)?;

//     Ok(data)
// }

// fn run_8() {
//     let data: &[u8] = b"\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x00\x01\x00\x00\x00\x01\x08\x06\x00\x00\x00\x1f\x15\xc4\x89\x00\x00\x00\nIDATx\xdac\xf8\x0f\x00\x01\x01\x01\x00\x18\xdd\x03\x1a\x00\x00\x00\x00IEND\xaeB`\x82";
//     decode_png(data);
// }

// use prettytable::format::TableFormat;
// use prettytable::{csv::ReaderBuilder, format, Cell, Row, Table};
// fn align_from_u8(x: u8) -> prettytable::format::Alignment {
//     match x {
//         0 => prettytable::format::Alignment::LEFT,
//         1 => prettytable::format::Alignment::CENTER,
//         _ => prettytable::format::Alignment::RIGHT,
//     }
// }

// fn run_table(data: Vec<Vec<(String, u8)>>) {
//     let mut pt = prettytable::Table::new();
//     for row in data {
//         let cells = row
//             .into_iter()
//             .map(|x| Cell::new_align(&x.0, align_from_u8(x.1)))
//             .collect();
//         pt.add_row(Row::new(cells));
//     }

//     let _ = pt.print(&mut std::io::sink());
// }

// fn run_9() {
//     // ! crashing input
//     // let data: Vec<u8> = vec![125, 27, 27, 27, 91, 27, 91, 125, 91, 27, 10, 9];

//     // let data: Vec<u8> = vec![120, 21, 27, 27, 91, 27, 91, 125, 91, 27, 10];
//     // let reader = Box::new(Cursor::new(data));

//     // let csv_reader = &mut ReaderBuilder::new()
//     //     .delimiter('\t' as u8)
//     //     .has_headers(false)
//     //     .from_reader(reader);

//     // let mut table = Table::init(
//     //     csv_reader
//     //         .records()
//     //         .map(|row|
//     //             Row::new(
//     //                 row
//     //                     .unwrap()
//     //                     .into_iter()
//     //                     .map(|cell| Cell::new(&cell))
//     //                     .collect()
//     //             )
//     //         )
//     //         .collect()
//     // );
//     // table.set_format(format::TableFormat::new());
//     // table.printstd();

//     let data: Vec<Vec<(String, u8)>> = vec![
//         vec![
//             ("Cell1".to_string(), 0),
//             ("Cell2".to_string(), 1),
//             ("Cell3".to_string(), 2),
//         ],
//         vec![
//             ("Cell4".to_string(), 0),
//             ("Cell5".to_string(), 1),
//             ("Cell6".to_string(), 2),
//         ],
//     ];
//     run_table(data);
// }

// fn run_10() {
//     // ! crashing input
//     let crashing =
//         "[]([]([]([](\x1b]([](|AN\x0b|||[](&&&#0000000000000000000000\x01\x00\x00\x00[]([]([]([](|||||||[](&&&#0000000000000000000\x1000000000[]([]([](|||||||[](&&&#0018446744073709551([](|||||||[](&&&#001844674407370955161615\x01\x00\x00\x00[]([]\x112\x00372\x008\x005156194&#&#&#&";

//     // ? for line 128 - 130, and 132, they are the same function
//     let data = "01010101";
//     pulldown_cmark_128::Parser::new(data);

//     // ? line 131
//     let mut output = String::new();
//     let parser = pulldown_cmark_131::Parser::new(data);
//     pulldown_cmark_131::html::push_html(&mut output, parser);

//     // ? for line 133
//     let mut opts = pulldown_cmark_133::Options::empty();

//     opts.insert(pulldown_cmark_133::Options::ENABLE_HEADING_ATTRIBUTES);

//     // ! crashing input
//     // let data = "[[][({][}\n-";

//     for _ in pulldown_cmark_133::Parser::new_ext(data, opts) {}
// }

// use quick_xml::reader::Reader;
// fn run_11() {
//     // ! crashing input
//     // let data : &[u8] = b"\xe9\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\n(\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00<>\x00\x08\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00<<\x00\x00\x00";

//     // ? line 134， 135, 136
//     let data: &[u8] = b"\x00";
//     let cursor = Cursor::new(data);
//     let mut reader = Reader::from_reader(cursor);
//     reader.trim_text(true); // ! this is for line 136, it only takes one line
//     let mut buf = vec![];
//     loop {
//         match reader.read_event(&mut buf) {
//             Ok(quick_xml::events::Event::Eof) | Err(..) => {
//                 break;
//             }
//             _ => buf.clear(),
//         }
//     }

//     // ? line 137
//     let doc = "<!D>";
//     let mut reader = Reader::from_str(doc);
//     let mut buf = Vec::new();
//     reader.read_event(&mut buf);
//     reader.read_event(&mut buf);
// }

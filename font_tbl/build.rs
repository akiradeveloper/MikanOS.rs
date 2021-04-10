use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::io::BufRead;
use std::io::Write;
use regex::*;

fn main() {
    // println!("cargo:rerun-if-changed=hankaku.txt");
    let f = File::open("hankaku.txt").unwrap();
    let mut f = BufReader::new(f);

    let mut xss = vec![];

    let header_matcher = Regex::new("char 0x.+").unwrap();
    let mut buf = String::new();
    loop {
        buf.clear();
        let r = f.read_line(&mut buf);
        match r {
            Ok(0) => break,
            Ok(_) => {
                if header_matcher.is_match(&buf) {
                    // dbg!(&buf);
                    let mut xs = [0;16];
                    for i in 0..16 {
                        buf.clear();
                        f.read_line(&mut buf);
                        let mut x = 0;
                        for j in 0..8 {
                            if buf.chars().nth(j).unwrap() == '*' {
                                x |= 1 << (7-j);
                            }
                        }
                        xs[i] = x;
                    }
                    xss.push(xs);
                }
            },
            Err(_) => panic!(),
        }
    }
    // dbg!(&xss);
    assert_eq!(xss.len(), 256);

    let out = OpenOptions::new().write(true).truncate(true).create(true).open("src/font_tbl.rs").unwrap();
    let mut out = BufWriter::new(out);
    out.write(b"pub static font_tbl: [[u8; 16]; 256] = [");
    for i in 0..256 {
        let xs = &xss[i];
        out.write(b"[");
        for j in 0..16 {
            let x = format!("{}", xs[j]);
            out.write(x.as_bytes());
            if j < 15 {
                out.write(b",");
            }
        }
        out.write(b"]");
        if i < 255 {
            out.write(b",");
        }
    }
    out.write(b"];");
}
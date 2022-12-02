use std::env;
use std::fs;
use std::io::{Read, Write};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use sha1::{Sha1, Digest};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "init" => {
            fs::create_dir(".git").unwrap();
            fs::create_dir(".git/objects").unwrap();
            fs::create_dir(".git/refs").unwrap();
            fs::write(".git/HEAD", "ref: refs/heads/master\n").unwrap();
        },
        "cat-file" => {
            let sha1_code = args[3].as_str();
            let subdir = &sha1_code[0..2];
            let filename = &sha1_code[2..];

            let object = fs::read(format!(".git/objects/{}/{}", subdir, filename)).unwrap();
            let mut decoder = ZlibDecoder::new(&object[..]);
            let mut store = String::new();
            decoder.read_to_string(&mut store).unwrap();

            let content = store.splitn(2, '\0').collect::<Vec<&str>>()[1];
            print!("{}", content);
        },
        "hash-object" => {
            let filename = args[3].as_str();
            let content = fs::read_to_string(filename).unwrap();
            let object = format!("blob {}\0{}", content.len(), content);
            let mut hasher = Sha1::new();
            hasher.update(object.as_bytes());
            let sha1 = format!("{:x}", hasher.finalize());

            let mut encoder =
                ZlibEncoder::new(Vec::new(), flate2::Compression::default());
            encoder.write_all(object.as_bytes()).unwrap();
            let compress = encoder.finish().unwrap();

            let subdir = &sha1[0..2];
            let filename = &sha1[2..];
            fs::create_dir(format!(".git/objects/{}", subdir)).unwrap();
            fs::write(format!(".git/objects/{}/{}", subdir, filename), compress).unwrap();
            println!("{}", sha1);
        },
        _ => {
            println!("unknown command: {}", args[1])
        }
    }
}

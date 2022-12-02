use std::env;
use std::fs;
use flate2::read::ZlibDecoder;
use std::io::Read;

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
        _ => {
            println!("unknown command: {}", args[1])
        }
    }
}

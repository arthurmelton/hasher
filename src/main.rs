use clap::*;

fn main() {
    let matches = App::new("Hasher")
        .version("0.1")
        .about("Hasher")
        .arg(Arg::with_name("Crc")
            .long("crc")
            .help("sets the crc type 16,32,64")
            .required(false)
            .takes_value(true))
        .arg(Arg::with_name("md")
            .long("md")
            .help("sets the md type 2,4,5")
            .required(false)
            .takes_value(true))
        .arg(Arg::with_name("INPUT")
            .help("what to hash")
            .required(true)
            .index(1))
        .get_matches();
    if matches.is_present("Crc") && matches.value_of("Crc").unwrap().to_string() == "32" {
        use crc::{Crc, CRC_32_ISO_HDLC};
        println!("{}", Crc::<u32>::new(&CRC_32_ISO_HDLC).checksum(matches.value_of("INPUT").unwrap().to_string().as_ref()));
        std::process::exit(1);
    }
    if matches.is_present("Crc") && matches.value_of("Crc").unwrap().to_string() == "16" {
        use crc::{Crc, CRC_16_ARC};
        println!("{}", Crc::<u16>::new(&CRC_16_ARC).checksum(matches.value_of("INPUT").unwrap().to_string().as_ref()));
        std::process::exit(1);
    }
    if matches.is_present("Crc") && matches.value_of("Crc").unwrap().to_string() == "64" {
        use crc::{Crc, CRC_64_ECMA_182};
        println!("{}", Crc::<u64>::new(&CRC_64_ECMA_182).checksum(matches.value_of("INPUT").unwrap().to_string().as_ref()));
        std::process::exit(1);
    }
    if matches.is_present("md") && matches.value_of("md").unwrap().to_string() == "5" {
        println!("{}", format!("{:x}", md5::compute(matches.value_of("INPUT").unwrap().to_string())));
        std::process::exit(1);
    }
    if matches.is_present("md") && matches.value_of("md").unwrap().to_string() == "2" {
        use md2::{Md2, Digest};
        let mut hasher = Md2::new();
        hasher.update(matches.value_of("INPUT").unwrap().to_string());
        println!("{:x}", hasher.finalize());
        std::process::exit(1);
    }
    if matches.is_present("md") && matches.value_of("md").unwrap().to_string() == "4" {
        use md4::{Md4, Digest};
        let mut hasher = Md4::new();
        hasher.update(matches.value_of("INPUT").unwrap().to_string());
        println!("{:x}", hasher.finalize());
        std::process::exit(1);
    }
    println!("What ever you just tried to do failed");
}

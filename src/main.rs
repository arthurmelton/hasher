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
        .arg(Arg::with_name("sha2")
            .long("sha2")
            .help("sets the md type 224,256,384,512")
            .required(false)
            .takes_value(true))
        .arg(Arg::with_name("sha3")
            .long("sha3")
            .help("sets the md type 224,256,384,512")
            .required(false)
            .takes_value(true))
        .arg(Arg::with_name("fsb")
            .long("fsb")
            .help("sets the md type 160,224,256,384,512")
            .required(false)
            .takes_value(true))
        .arg(Arg::with_name("groestl")
            .long("groestl")
            .help("sets the md type 224,256,384,512")
            .required(false)
            .takes_value(true))
        .arg(Arg::with_name("ripemd")
            .long("ripemd")
            .help("sets the md type 160,256,320")
            .required(false)
            .takes_value(true))
        .arg(Arg::with_name("blake2")
            .long("blake2")
            .help("uses the blake hash")
            .required(false)
            .takes_value(false))
        .arg(Arg::with_name("gost94")
            .long("gost94")
            .help("uses the gost94 hash")
            .required(false)
            .takes_value(false))
        .arg(Arg::with_name("INPUT")
            .help("what to hash")
            .required(true)
            .index(1))
        .get_matches();
    if matches.is_present("Crc") && matches.value_of("Crc").unwrap().to_string() == "32" {
        use crc::{Crc, CRC_32_ISO_HDLC};
        println!("{:x}", Crc::<u32>::new(&CRC_32_ISO_HDLC).checksum(matches.value_of("INPUT").unwrap().to_string().as_ref()));
        std::process::exit(1);
    }
    if matches.is_present("Crc") && matches.value_of("Crc").unwrap().to_string() == "16" {
        use crc::{Crc, CRC_16_ARC};
        println!("{:x}", Crc::<u16>::new(&CRC_16_ARC).checksum(matches.value_of("INPUT").unwrap().to_string().as_ref()));
        std::process::exit(1);
    }
    if matches.is_present("Crc") && matches.value_of("Crc").unwrap().to_string() == "64" {
        use crc::{Crc, CRC_64_ECMA_182};
        println!("{:x}", Crc::<u64>::new(&CRC_64_ECMA_182).checksum(matches.value_of("INPUT").unwrap().to_string().as_ref()));
        std::process::exit(1);
    }
    if matches.is_present("md") && matches.value_of("md").unwrap().to_string() == "5" {
        println!("{:x}", md5::compute(matches.value_of("INPUT").unwrap().to_string()));
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
    if matches.is_present("blake2") {
        use blake2::{Blake2b, Digest};
        let mut hasher = Blake2b::new();
        hasher.update(matches.value_of("INPUT").unwrap().to_string());
        println!("{:x}", hasher.finalize());
        std::process::exit(1);
    }
    if matches.is_present("fsb") && matches.value_of("fsb").unwrap().to_string() == "160"  {
        use fsb::{Fsb160, Digest};
        let mut hasher = Fsb160::new();
        hasher.update(matches.value_of("INPUT").unwrap().to_string());
        println!("{:x}", hasher.finalize());
        std::process::exit(1);
    }
    if matches.is_present("fsb") && matches.value_of("fsb").unwrap().to_string() == "224"  {
        use fsb::{Fsb224, Digest};
        let mut hasher = Fsb224::new();
        hasher.update(matches.value_of("INPUT").unwrap().to_string());
        println!("{:x}", hasher.finalize());
        std::process::exit(1);
    }
    if matches.is_present("fsb") && matches.value_of("fsb").unwrap().to_string() == "256"  {
        use fsb::{Fsb256, Digest};
        let mut hasher = Fsb256::new();
        hasher.update(matches.value_of("INPUT").unwrap().to_string());
        println!("{:x}", hasher.finalize());
        std::process::exit(1);
    }
    if matches.is_present("fsb") && matches.value_of("fsb").unwrap().to_string() == "384"  {
        use fsb::{Fsb384, Digest};
        let mut hasher = Fsb384::new();
        hasher.update(matches.value_of("INPUT").unwrap().to_string());
        println!("{:x}", hasher.finalize());
        std::process::exit(1);
    }
    if matches.is_present("fsb") && matches.value_of("fsb").unwrap().to_string() == "512"  {
        use fsb::{Fsb512, Digest};
        let mut hasher = Fsb512::new();
        hasher.update(matches.value_of("INPUT").unwrap().to_string());
        println!("{:x}", hasher.finalize());
        std::process::exit(1);
    }
    if matches.is_present("gost94") {
        use gost94::{Gost94Test, Digest};
        let mut hasher = Gost94Test::new();
        hasher.update(matches.value_of("INPUT").unwrap().to_string());
        println!("{:x}", hasher.finalize());
        std::process::exit(1);
    }
    if matches.is_present("groestl") && matches.value_of("groestl").unwrap().to_string() == "224"  {
        use groestl::{Digest, Groestl224};
        let mut hasher = Groestl224::new();
        hasher.update(matches.value_of("INPUT").unwrap().to_string());
        println!("{:x}", hasher.finalize());
        std::process::exit(1);
    }
    if matches.is_present("groestl") && matches.value_of("groestl").unwrap().to_string() == "256"  {
        use groestl::{Digest, Groestl256};
        let mut hasher = Groestl256::new();
        hasher.update(matches.value_of("INPUT").unwrap().to_string());
        println!("{:x}", hasher.finalize());
        std::process::exit(1);
    }
    if matches.is_present("groestl") && matches.value_of("groestl").unwrap().to_string() == "384"  {
        use groestl::{Digest, Groestl384};
        let mut hasher = Groestl384::new();
        hasher.update(matches.value_of("INPUT").unwrap().to_string());
        println!("{:x}", hasher.finalize());
        std::process::exit(1);
    }
    if matches.is_present("ripemd") && matches.value_of("ripemd").unwrap().to_string() == "160"  {
        use ripemd160::{Ripemd160, Digest};
        let mut hasher = Ripemd160::new();
        hasher.update(matches.value_of("INPUT").unwrap().to_string());
        println!("{:x}", hasher.finalize());
        std::process::exit(1);
    }
    if matches.is_present("ripemd") && matches.value_of("ripemd").unwrap().to_string() == "256"  {
        use ripemd256::{Ripemd256, Digest};
        let mut hasher = Ripemd256::new();
        hasher.update(matches.value_of("INPUT").unwrap().to_string());
        println!("{:x}", hasher.finalize());
        std::process::exit(1);
    }
    if matches.is_present("ripemd") && matches.value_of("ripemd").unwrap().to_string() == "320"  {
        use ripemd320::{Ripemd320, Digest};
        let mut hasher = Ripemd320::new();
        hasher.update(matches.value_of("INPUT").unwrap().to_string());
        println!("{:x}", hasher.finalize());
        std::process::exit(1);
    }
    if matches.is_present("sha2") && matches.value_of("sha2").unwrap().to_string() == "224"  {
        use sha2::{Sha224, Digest};
        let mut hasher = Sha224::new();
        hasher.update(matches.value_of("INPUT").unwrap().to_string());
        println!("{:x}", hasher.finalize());
        std::process::exit(1);
    }
    if matches.is_present("sha2") && matches.value_of("sha2").unwrap().to_string() == "256"  {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(matches.value_of("INPUT").unwrap().to_string());
        println!("{:x}", hasher.finalize());
        std::process::exit(1);
    }
    if matches.is_present("sha2") && matches.value_of("sha2").unwrap().to_string() == "384"  {
        use sha2::{Sha384, Digest};
        let mut hasher = Sha384::new();
        hasher.update(matches.value_of("INPUT").unwrap().to_string());
        println!("{:x}", hasher.finalize());
        std::process::exit(1);
    }
    if matches.is_present("sha2") && matches.value_of("sha2").unwrap().to_string() == "512"  {
        use sha2::{Sha512, Digest};
        let mut hasher = Sha512::new();
        hasher.update(matches.value_of("INPUT").unwrap().to_string());
        println!("{:x}", hasher.finalize());
        std::process::exit(1);
    }
    if matches.is_present("sha3") && matches.value_of("sha3").unwrap().to_string() == "224"  {
        use sha3::{Sha3_224, Digest};
        let mut hasher = Sha3_224::new();
        hasher.update(matches.value_of("INPUT").unwrap().to_string());
        println!("{:x}", hasher.finalize());
        std::process::exit(1);
    }
    if matches.is_present("sha3") && matches.value_of("sha3").unwrap().to_string() == "256"  {
        use sha3::{Sha3_256, Digest};
        let mut hasher = Sha3_256::new();
        hasher.update(matches.value_of("INPUT").unwrap().to_string());
        println!("{:x}", hasher.finalize());
        std::process::exit(1);
    }
    if matches.is_present("sha3") && matches.value_of("sha3").unwrap().to_string() == "384"  {
        use sha3::{Sha3_384, Digest};
        let mut hasher = Sha3_384::new();
        hasher.update(matches.value_of("INPUT").unwrap().to_string());
        println!("{:x}", hasher.finalize());
        std::process::exit(1);
    }
    if matches.is_present("sha3") && matches.value_of("sha3").unwrap().to_string() == "512"  {
        use sha3::{Sha3_512, Digest};
        let mut hasher = Sha3_512::new();
        hasher.update(matches.value_of("INPUT").unwrap().to_string());
        println!("{:x}", hasher.finalize());
        std::process::exit(1);
    }
    println!("What ever you just tried to do failed");
}

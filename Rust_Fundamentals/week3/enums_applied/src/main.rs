use std::env;

enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
    Terabytes(u64),
}

// #[derive(Debug)]
struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
    terabytes: String,
}

fn db(x: u64) -> f64 {10.0*f64::log10(x as f64)}

fn num_raw_to_num(num: u64, unit: &str) -> u64 {
    let num_units = match unit {
        "B" => num,
        "KB" => num * 1000,
        "MB" => num * 1_000_000,
        "GB" => num * 1_000_000_000,
        "TB" => num * 1_000_000_000_000,
        _ => num,
    };
    num_units
}

fn format_size_db(size: u64) -> String {

    let filesize_db = match db(size) as i32 {
        0..=29   => FileSize::Bytes(size),
        30..=59  => FileSize::Kilobytes(size / 1000),
        60..=89  => FileSize::Megabytes(size / 1_000_000),
        90..=120 => FileSize::Gigabytes(size / 1_000_000_000),
        _ => FileSize::Terabytes(size / 1_000_000_000_000),
    };

    match filesize_db {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.1} KB", kb as f64),
        FileSize::Megabytes(mb) => format!("{:.1} MB", mb as f64),
        FileSize::Gigabytes(gb) => format!("{:.1} GB", gb as f64),
        FileSize::Terabytes(tb) => format!("{:.1} TB", tb as f64),
    }
}

fn format_size(num_raw: u64, units: &str) -> String {
    let filesize = match units {
        "B" => num_raw,
        "KB" => num_raw / 1000,
        "MB" => num_raw / 1_000_000,
        "GB" => num_raw / 1_000_000_000,
        "TB" => num_raw / 1_000_000_000_000,
        _ => num_raw,
    };
    return filesize.to_string();
}


fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    let num_raw = args[1].parse::<u64>().unwrap();
    let unit = &args[2].to_uppercase();
    let num = num_raw_to_num(num_raw, unit);
    println!("raw num -> num: {}", num);
    // let num = 25_888_837_399_765;
    // let num = 25_888_837;
    // println!("{:.1} dB ", db(num) as i32);
    let result = format_size_db(num);
    println!("{}", result);
    // let test : u64 = 6_888_837_399/1_000_000_000;
    // println!("test: {}", test);
    let sizes = Sizes {
        bytes: format_size(num, "B"),
        kilobytes: format_size(num, "KB"),
        megabytes: format_size(num, "MB"),
        gigabytes: format_size(num, "GB"),
        terabytes: format_size(num, "TB"),
    };
    println!("Sizes {{ bytes: {:?}, kilobytes: {:?}, megabytes: {:?}, gigabytes: {:?}, terabytes: {:?} }}", 
            sizes.bytes, sizes.kilobytes, sizes.megabytes, sizes.gigabytes, sizes.terabytes);
    // println!("Sizes {{ bytes: {:?}}}", sizes.bytes);
}

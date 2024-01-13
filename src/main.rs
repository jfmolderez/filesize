use std::env;

#[derive(Debug)]
struct FSize {
    size: u64,
    unit: String,
}

enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
    Terabytes(u64),
}

impl FileSize {
    fn format_size(&self) -> String {
        match self {
            FileSize::Bytes(bytes) => convert(*bytes),
            FileSize::Kilobytes(kb) => convert(*kb * 1000),
            FileSize::Megabytes(mb) => convert(*mb * 1000 * 1000),
            FileSize::Gigabytes(gb) => convert(*gb * 1000 * 1000 * 1000),
            FileSize::Terabytes(tb) => convert(*tb * 1000 * 1000 * 1000 * 1000),
        }
    }
}

fn convert(size_in_bytes: u64) -> String {
    match size_in_bytes as u64 {
        0..=999 => format!("{} bytes", size_in_bytes as f64),
        1000..=999_999 => format!("{:.2} KB", size_in_bytes as f64 / 1000.0),
        1_000_000..=999_999_999 => format!("{:.2} MB", size_in_bytes as f64 / 1_000_000.0),
        1_000_000_000..=999_999_999_999 => format!("{:.2} GB", size_in_bytes as f64 / 1_000_000_000.0),
        _ => format!("{:.2} TB", size_in_bytes as f64 / 1_000_000_000_000.0),
    }   
}



fn main() {
    let args: Vec<String> = env::args().collect();

    // The first argument is the size and the second is the unit
    if args.len() != 3 {
        println!("args.len() is {}", args.len());
        println!("Please provide a valid size : <positive integer> <unit> (unit is either 'kb', 'mb', 'gb' otr 'tb' case insensitive)");
        return ;
    } 

    println!("Size is {}", args[1]);
    println!("Unit is {}", args[2]);

    let size = 251234567890000;
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size / 1000),
        1_000_000..=999_999_999 => FileSize::Megabytes(size / 1_000_000),
        1_000_000_000..=999_999_999_999 => FileSize::Gigabytes(size / 1_000_000_000),
        _ => FileSize::Terabytes(size / 1_000_000_000_000),
    };

    println!("File size: {}", filesize.format_size());

    println!("File size: {}", convert(size));
}


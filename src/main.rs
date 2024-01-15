use std::{env, fs};

#[derive(Debug)]
enum Units {
    // Bytes,
    Kilobytes,
    Megabytes,
    Gigabytes,
    //Terabytes,
}

/* 
impl Units {
    fn to_string(&self) -> String {
        match self {
            //Units::Bytes => String::from("bytes"),
            Units::Kilobytes => String::from("kilobytes"),
            Units::Megabytes => String::from("megabytes"),
            Units::Gigabytes => String::from("megabytes"),
            //Units::Terabytes => String::from("terabytes"),
        }
    }
}
*/

#[derive(Debug)]
struct FSize {
    size: u64,
    units: Units,
}

impl FSize {
    fn to_bytes(&self) -> f64 {
        let FSize {size, units} = self;
        let nb = *size as f64;
        let bytes = match units {
            // Units::Bytes => nb,
            Units::Kilobytes => nb * 1000f64,
            Units::Megabytes => nb * 1_000_000f64,
            Units::Gigabytes => nb * 1_000_000_000f64,
            // Units::Terabytes => nb * 1_000_000_000_000f64, 
        };
        bytes
    }

    fn to_sizes(&self) -> Vec<String> {
        let powers: Vec<(f64, String)> = vec![
            (1., String::from("bytes")),
            (1_000., String::from("kb")),
            (1_000_000., String::from("mb")),
            (1_000_000_000., String::from("gb")),
        ];
        let bytes = self.to_bytes();
        let res: Vec<String> = powers.into_iter().map(|(p, u)| { 
            let sz = bytes / p;
            format!("{:.2} {}", sz, u)
        }).collect();
        res
    }

    fn to_fsizes(&self) -> FSizes {
        let sizes = self.to_sizes();
        FSizes { 
            bytes: String::from(&sizes[0]), 
            kilobytes: String::from(&sizes[1]), 
            megabytes: String::from(&sizes[2]), 
            gigabytes: String::from(&sizes[3]) 
        }
    }
}


#[derive(Debug)]
struct FSizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
    // terabytes: String,
}


fn main() {
    let args: Vec<String> = env::args().collect();

    // The first argument is the size and the second is the unit
    if args.len() != 3 {
        println!("args.len() is {}", args.len());
        println!("Please provide a valid size : <positive integer> <unit> (unit is either 'kb', 'mb', or 'gb' (case insensitive)");
        return ;
    } 

    let size = String::from(&args[1]).parse::<u64>().unwrap();
    let units_str = String::from(&args[2]);
    let units = match units_str[0..2].to_lowercase().as_str() {
        "kb" => Units::Kilobytes,
        "mb" => Units::Megabytes,
        "gb" => Units::Gigabytes,
        _ => panic!("Valid units are either kb, mb or gb"),
    };

    let fsize = FSize {size, units};
    println!("{:?}", fsize);
    println!("{:?}", fsize.to_fsizes());
}


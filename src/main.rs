enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

#[derive(Debug)]
struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}

impl Sizes {
    fn new(size: f64) -> Sizes{
        Sizes {
            bytes: format!("{:.0} bytes", size),
            kilobytes: format!("{:.0} kilobytes", size as f64 / 1024 as f64),
            megabytes: format!("{:.0} megabytes", size as f64 / (1024 * 1024) as f64),
            gigabytes: format!("{:.0} gigabytes", size as f64 / (1024 * 1024 * 1024) as f64),
        }
    }
}

fn format_size(size: FileSize) -> Sizes {
    
    let filesize = match size {
        FileSize::Bytes(bytes) => bytes as f64,
        FileSize::Kilobytes(kb) => kb * 1024 as f64,
        FileSize::Megabytes(mb) => mb * (1024 * 1024) as f64,
        FileSize::Gigabytes(gb) => gb * (1024 * 1024 * 1024) as f64,
    };
    
    Sizes::new(filesize)   
}

fn parse_input(string: &String) -> Result<FileSize, &str> {
    println!("User inputed {}", string);

    let words: Vec<&str> = string.split_whitespace().collect();

    if words.len() != 2 {
        println!("Bad number of args");
        return Err("Bad input");
    }

    let size: Result<f64, _> = words[0].parse();

    let filesize = match size {
        Ok(x) => {            
            x
        }
        Err(e) => {
            println!("Error: {}", e);
            return Err("Bad input");
        }
    };

    println!("Size: {}", filesize);

    match words[1].to_lowercase().as_str() {
        "bytes" => Ok(FileSize::Bytes(filesize as u64)),
        "kb" => Ok(FileSize::Kilobytes(filesize)),
        "mb" => Ok(FileSize::Megabytes(filesize)),
        "gb" => Ok(FileSize::Gigabytes(filesize)),
        _ => {
            println!("Can not parse!");
            Err("Bad input")
        }
    }
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
        
    if args.len() < 2 {
        println!("FMSIZE: Parameter format not correct\n");
        std::process::exit(1);
    }
    
    let result = match parse_input(&args[1]) {
        Ok(result) => result,
        Err(_) => std::process::exit(1),
    };

    let result = format_size(result);

    println!("{:?}", result);
}

// Sizes { bytes: "24000000 bytes", kilobytes: "24000 kilobytes", megabytes: "24 megabytes", gigabytes: "0 gigabytes" }
use std::{env, process};
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;

use rand::Rng;

use config::Config;

mod config;

fn main() -> Result<(), Box<dyn Error>> {
    let config = match Config::parse_config(env::args().collect()) {
        Ok(config) => config,
        Err(why) => {
            println!("Failed to parse command line arguments: {}", why);
            process::exit(1);
        }
    };
    println!("{:?}", config);

    let path = Path::new(&config.output_filename);
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path);

    let file = match file {
        Ok(file) => file,
        Err(why) => {
            println!("Failed to open output file: {}", why);
            process::exit(1);
        }
    };
    let mut writer = BufWriter::new(file);
    let mut rng = rand::thread_rng();

    for _row in 0..config.row_count {
        for _col in 0..config.col_count {
            if config.custom_range {
                write!(writer, "{}", rng.gen_range(config.range_from..config.range_to))?;
            } else {
                write!(writer, "{}", rng.gen::<i64>())?;
            }

            if config.spaces {
                write!(writer, " ")?;
            }
        }

        write!(writer, "\n")?;
    }

    writer.flush()?;

    Ok(())
}

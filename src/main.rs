use std::{env, process};
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;

use rand::Rng;

use config::Config;

mod config;

        Ok(Config {row_count, col_count, spaces, output_filename})
    }
}

fn main() {
    let config = Config::parse_config(env::args().collect());
    println!("{:?}", config);
}

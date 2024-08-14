use clap::Parser;
use crate::config;
use std::error::Error;

const INSTR_PER_SECOND_DEFAULT: u32 = 700;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Path to the file of the emulator
    #[arg(short, long)]
    file_path: String,

    // Speed of the emulator, default 700/60 instr_per_second
    #[arg(short, long, default_value_t = INSTR_PER_SECOND_DEFAULT)]
    instr_per_second: u32,

    // determines if it uses the legacy version of certain opcodes or not
    #[arg(short, long, default_value_t = false)]
    legacy: bool,

    #[arg(short, long, default_value_t = false)]
    bxnn_quirk: bool,

    #[arg(short, long)]
    scale_factor: Option<f32>,

    #[arg(long)]
    frame_rate: Option<f32>,
}


pub fn pass_args(config: &mut config::Config) -> Result<(), ArgsError> {
    let args = Args::parse();
        
    let f = std::fs::File::open(args.file_path);

    if let Err(e) = f {
        panic!("Error opening file: {}", e);
    }
    config.file_path = Ok(f.unwrap());

    println!("{}", args.instr_per_second);
    if args.instr_per_second < 60 {
        return Err(ArgsError::BadFlag);
    }
    config.instr_per_second = args.instr_per_second / 60;

    config.legacy = args.legacy;
    
    config.bxnn_quirk = args.bxnn_quirk;

    if let Some(scale_factor) = args.scale_factor {
        config.scale_factor = scale_factor;
    }

    if let Some(frame_rate) = args.frame_rate {
        config.frame_rate = frame_rate;
    }

    Ok(())
}


#[derive(Debug)]
pub enum ArgsError {
    BadFlag,
}

impl std::fmt::Display for ArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::BadFlag => write!(f, "ERROR: -i flag set to value less than 60, this will cause the emulator to run 0 instructions per second, please choose a value of at least 60"),
        }
    }
}

impl Error for ArgsError {}

use clap::ArgMatches;
use std::fs;
use std::path::Path;
use std::process;

pub fn exec_playas(sub_matches:  &ArgMatches) {
    let _rom_arg = Path::new(sub_matches.value_of("rom").unwrap_or_default());
    let _output_arg = Path::new(sub_matches.value_of("output").unwrap_or_default());
    let manifest_arg = Path::new(sub_matches.value_of("manifest").unwrap());
    let input_arg = Path::new(sub_matches.value_of("input").unwrap());
    let bank_arg = Path::new(sub_matches.value_of("bank").unwrap());
    let _zobj_arg = sub_matches.is_present("zobj");

    let _convert_zobj_file = match fs::read(input_arg) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Issue opening input file {0:?}: {1:?}", input_arg.file_name().unwrap_or_default(), error.kind());
            process::exit(error.raw_os_error().unwrap_or(1));
        }
    };

    let _manifest_file = match fs::read(manifest_arg) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Issue opening manifest file {0:?}: {1:?}", manifest_arg.file_name().unwrap_or_default(), error.kind());
            process::exit(error.raw_os_error().unwrap_or(1));
        }
    };

    let _bank_file = match fs::read(bank_arg) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Issue opening bank file {0:?}: {1:?}", manifest_arg.file_name().unwrap_or_default(), error.kind());
            process::exit(error.raw_os_error().unwrap_or(1));
        }
    };

    let _playas_zobj_file: Vec<u8> = Vec::new();
}
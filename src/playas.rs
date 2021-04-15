#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

extern crate clap;
extern crate regex;
use clap::ArgMatches;
use regex::Regex;
use std::{fs, process};
use std::path::Path;

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

    _manifest_parse(String::from_utf8(_manifest_file).unwrap_or_default());

    let _playas_zobj_file: Vec<u8> = Vec::new();
}

#[derive(Debug)]
enum DictPointerTypes {
    POINTER,
    ACTOR,
    EFFECT,
    OBJECT
}

struct DictPointer {
    ptr_type: DictPointerTypes,
    val: usize
}

struct DictListing {
    definition: String,
    pointer: DictPointer,
    name: Option<String>
}

struct ObjectCommands {
    command: String,
    args: Vec<String>
}

struct ObjectListing {
    label: String,
    subcommands: Vec<ObjectCommands>
}

struct RepointListing {
    command: String,
    arg: String
}

struct ObjectPool {
    pool_addr: usize,
    pool_size: usize,
    commands: Vec<ObjectListing>
}

struct Manifest {
    dictionary: Vec<DictListing>,
    object: ObjectPool,
    repoint: Vec<RepointListing>
}

fn _manifest_parse(file: String) {
    let mut manifest_file = file;
    let mut manifest = Manifest {
        dictionary: Vec::new(),
        object: ObjectPool {
            pool_addr: 0,
            pool_size: 0,
            commands: Vec::new()
        },
        repoint: Vec::new()
    };

    manifest_file = Regex::new(r"//[^\n]*\n").unwrap().replace_all(manifest_file.as_str(), "\n").to_string();
    manifest_file = Regex::new(r"\n[\s]*\n").unwrap().replace_all(manifest_file.as_str(), "\n").to_string();

    lazy_static! {
        static ref DICT_INNER_RE: Regex = Regex::new(r#"^\s*(?:(VROM_CODE|VROM_OBJ|TABLE_OBJECT|TABLE_ACTOR|TABLE_PARTICLE|HIERARCHY_CODE|SEG)|(DL|VROM)_([^\s]+)?|([^\s]+))\s+(?:([AEO])_)?(0x)?([0-9a-fA-F]+)(?:\s+"([^"]+)")?\s*$"#).unwrap();
        static ref OBJECT_RE: Regex = Regex::new(r"(?s)OBJECT(?: POOL=0x([0-9a-fA-F]+),0x([0-9a-fA-F]+))?\n(.*?)\nEND(?-s)").unwrap();
    }

    let dict_captures = Regex::new(r"(?s)DICTIONARY\n(.*?)\nEND(?-s)").unwrap().captures(manifest_file.as_str()).unwrap().get(1).unwrap().as_str();
    if let Some(object_pool_addr) = OBJECT_RE.captures(manifest_file.as_str()).unwrap().get(1) {
        manifest.object.pool_addr = usize::from_str_radix(object_pool_addr.as_str(), 16).unwrap_or_default();
    }
    if let Some(object_pool_size) = OBJECT_RE.captures(manifest_file.as_str()).unwrap().get(2) {
        manifest.object.pool_size = usize::from_str_radix(object_pool_size.as_str(), 16).unwrap_or_default();
    }
    let _object_captures = OBJECT_RE.captures(manifest_file.as_str()).unwrap().get(3).unwrap().as_str();
    let _repoint_captures = Regex::new(r"(?s)REPOINT\n(.*?)\nEND(?-s)").unwrap().captures(manifest_file.as_str()).unwrap().get(1).unwrap().as_str();

    let dict_split = Regex::new(r"\n").unwrap();
    let dict_split = dict_split.split(dict_captures);
    for _splits in dict_split {
        let _dict_re = Regex::new(r#"^\s*(?:(VROM_CODE|VROM_OBJ|TABLE_OBJECT|TABLE_ACTOR|TABLE_PARTICLE|HIERARCHY_CODE|SEG)|(DL|VROM)_([^\s]+)?|([^\s]+))\s+(?:([AEO])_)?(0x)?([0-9a-fA-F]+)(?:\s+"([^"]+)")?\s*$"#).unwrap();
        let matches = _dict_re.captures(_splits).unwrap();

        let _definition = matches.get(1).unwrap().as_str().to_string();

        let _ptr_type = match matches.get(2) {
            Some(n) => match n.as_str() {
                "A" => DictPointerTypes::ACTOR,
                "E" => DictPointerTypes::EFFECT,
                "O" => DictPointerTypes::OBJECT,
                _ => DictPointerTypes::POINTER
            },
            None => DictPointerTypes::POINTER
        };
        
        // let _hex = if let None = matches.get(3) {
        //     true
        // } else {
        //     false
        // };

        let _hex = matches.get(3).is_some();

        let _val = usize::from_str_radix(matches.get(4).unwrap().as_str(), if _hex {16} else {10}).unwrap_or_default();

        let _name = if let Some(s) = matches.get(5) {
            Some(s.as_str().to_string())
        } else {
            None
        };

        manifest.dictionary.push(DictListing {
            definition: _definition,
            pointer: DictPointer {
                val: _val,
                ptr_type: _ptr_type
            },
            name: _name
        });
    }

    for dict_vec in manifest.dictionary {
        println!("[");
        println!("  {},", dict_vec.definition);
        println!("  {{");
        println!("    {:X},", dict_vec.pointer.val);
        println!("    {:?},", dict_vec.pointer.ptr_type);
        println!("  }},");
        // println!("  {},", dict_vec.1);
        if let Some(s) = dict_vec.name {
            println!("  {},", s);
        }
        println!("],");
    }

    // println!("{}", dict_captures);
}


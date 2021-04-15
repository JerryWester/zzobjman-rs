#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
use clap::App;
mod playas;

fn main() {
    #[cfg(not(debug_assertions))]
    panic::set_hook(Box::new(|panic_info| {
        if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            println!("panic occurred: {:?}", s);
        } else {
            println!("panic occurred");
        }
    }));
    let yaml = load_yaml!("cli.yml");
    let mut app = App::from_yaml(yaml);
    
    let mut help = Vec::new();
    app.write_long_help(&mut help).unwrap_or(());
    
    let matches = app.get_matches();

    let help_str = String::from_utf8(help).unwrap_or_default();

    match matches.subcommand_name() {
        Some("playas") => {
            if let Some(sub_matches) = matches.subcommand_matches("playas") {
                // playas::exec_playas(sub_matches);
                playas::exec_playas(sub_matches);
            }
        }

        _ => println!("{}", help_str)
    }

    // let matches = App::from_yaml(yaml).get_matches();
    // if matches.args.is_empty() {
    //     println!("Error: no commands\nUse the -h flag for usage options");
    // }
}

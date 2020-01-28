
mod interface;

#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from(yaml).get_matches();

    match matches.subcommand()
    {
        ("list", Some(sub_m))     => {
            if let Some(u) = sub_m.value_of("user") {
                interface::perform_my_query(u)
                    .map_err(|err| println!("{:?}", err))
                    .ok();
            }
        },
        
        ("favorite", Some(_)) => {println!("Display favorite titles")},
        _                         => {},
    }
}

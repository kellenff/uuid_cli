use clap::{App, Arg};

pub fn get_app<'a, 'b: 'a>() -> App<'a, 'b> {
    App::new("UUID")
        .version("0.1.1")
        .author("Kellen Frodelius-Fujimoto <kellen@kellenfujimoto.com>")
        .about("A command line utility for genrating UUIDs")
        .arg(Arg::with_name("count")
             .short("c")
             .long("count")
             .takes_value(true)
             .help("The number of UUIDs to generate, defaults to 1")
             )
}

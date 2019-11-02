use uuid::Uuid;
use uuid_cli::{format_uuids, get_app, OutputFormat, UuidFormat};

fn main() {
    let app = get_app();
    let matches = app.get_matches();

    let count = matches
        .value_of("count")
        // `count` arg has a default value
        .unwrap()
        .parse::<usize>()
        .unwrap_or(1);

    let output_format = matches
        .value_of("format")
        // `format` arg has a default value
        .unwrap()
        .parse::<OutputFormat>()
        .unwrap_or(OutputFormat::Unix);

    let uuid_format = if matches.is_present("upper") {
        UuidFormat::Upper
    } else {
        UuidFormat::Lower
    };

    let uuids = std::iter::repeat_with(Uuid::new_v4).take(count).collect();

    println!("{}", format_uuids(uuids, output_format, uuid_format));
}

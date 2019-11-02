use clap::{App, Arg};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputFormat {
    Unix,
    Json,
}

impl FromStr for OutputFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "unix" => Ok(Self::Unix),
            "json" => Ok(Self::Json),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UuidFormat {
    Lower,
    Upper,
}

impl Default for UuidFormat {
    fn default() -> Self {
        Self::Lower
    }
}

pub fn format_uuids(uuids: Vec<Uuid>, format: OutputFormat, uuid_format: UuidFormat) -> String {
    let uuid_strings = uuids.iter().map(|u| format_uuid(u, uuid_format));

    match format {
        OutputFormat::Unix => uuid_strings
            .map(|s| s + "\n")
            .collect::<String>()
            .trim_end()
            .to_string(),
        OutputFormat::Json => {
            serde_json::to_string(uuid_strings.collect::<Vec<String>>().as_slice())
                .expect("Failed to format UUIDs as JSON :(")
        }
    }
}

fn format_uuid(id: &Uuid, uuid_format: UuidFormat) -> String {
    let formatted = id.to_string();

    match uuid_format {
        UuidFormat::Lower => formatted,
        UuidFormat::Upper => formatted.to_uppercase(),
    }
}

pub fn get_app<'a, 'b: 'a>() -> App<'a, 'b> {
    App::new("UUID")
        .version("0.1.1")
        .author("Kellen Frodelius-Fujimoto <kellen@kellenfujimoto.com>")
        .about("A command line utility for genrating UUIDs")
        .arg(
            Arg::with_name("count")
                .short("c")
                .long("count")
                .takes_value(true)
                .help("The number of UUIDs to generate")
                .default_value("1"),
        )
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .takes_value(true)
                .help("The output format of the UUIDs. Defaults to `unix`")
                .possible_values(&["unix", "json"])
                .default_value("unix"),
        )
        .arg(
            Arg::with_name("upper")
                .long("upper")
                .help("Formats UUIDs in uppercase")
                .conflicts_with("lower"),
        )
        .arg(
            Arg::with_name("lower")
                .long("lower")
                .help("Formats UUIDs in lowercase (default)")
                .conflicts_with("upper"),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_uuid_lowercase() {
        let expected = "00c2f061-0913-489b-b6c6-cf7596cf1669";

        let id = expected.parse::<Uuid>().unwrap();

        let formatted = format_uuid(&id, UuidFormat::Lower);

        assert_eq!(&formatted, expected);
    }

    #[test]
    fn format_uuid_uppercase() {
        let given = "5f8505a9-7d0a-40a0-9caf-badca9c17d84";
        let expected = "5F8505A9-7D0A-40A0-9CAF-BADCA9C17D84";

        let id = given.parse::<Uuid>().unwrap();

        let formatted = format_uuid(&id, UuidFormat::Upper);

        assert_eq!(&formatted, expected);
    }
}

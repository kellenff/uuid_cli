use clap::{Args, Parser, ValueEnum};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, ValueEnum)]
pub enum OutputFormat {
    Unix,
    Json,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UuidFormat {
    Lower,
    Upper,
}

#[derive(Parser, Debug)]
#[command(
    name = "uuid",
    version,
    author,
    about = "A command line utility for generating UUIDs"
)]
pub struct Cli {
    /// The number of UUIDs to generate
    #[arg(short, long, default_value = "1")]
    pub count: usize,

    /// The output format of the UUIDs
    #[arg(short, long, default_value = "unix")]
    pub format: OutputFormat,

    #[command(flatten)]
    pub case: CaseArgs,
}

#[derive(Args, Debug)]
#[group(multiple = false)]
pub struct CaseArgs {
    /// Format UUIDs in uppercase
    #[arg(long)]
    pub upper: bool,

    /// Format UUIDs in lowercase (default)
    #[arg(long)]
    pub lower: bool,
}

impl CaseArgs {
    pub fn uuid_format(&self) -> UuidFormat {
        if self.upper {
            UuidFormat::Upper
        } else {
            UuidFormat::Lower
        }
    }
}

pub fn format_uuids(uuids: Vec<Uuid>, format: OutputFormat, uuid_format: UuidFormat) -> String {
    let uuid_strings: Vec<String> = uuids.iter().map(|u| format_uuid(u, uuid_format)).collect();

    match format {
        OutputFormat::Unix => uuid_strings.join("\n"),
        OutputFormat::Json => {
            serde_json::to_string(&uuid_strings).expect("Failed to format UUIDs as JSON")
        }
    }
}

fn format_uuid(id: &Uuid, uuid_format: UuidFormat) -> String {
    match uuid_format {
        UuidFormat::Lower => id.to_string(),
        UuidFormat::Upper => id.to_string().to_uppercase(),
    }
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

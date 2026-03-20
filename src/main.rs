use clap::Parser;
use uuid::Uuid;
use uuid_cli::{format_uuids, Cli};

fn main() {
    let cli = Cli::parse();

    let uuids: Vec<Uuid> = std::iter::repeat_with(Uuid::new_v4)
        .take(cli.count)
        .collect();

    println!(
        "{}",
        format_uuids(uuids, cli.format, cli.case.uuid_format())
    );
}

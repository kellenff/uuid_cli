use uuid::Uuid;
use uuid_cli::get_app;

fn main() {
    let app = get_app();
    let matches = app.get_matches();

    let count = matches
        .value_of("count")
        .unwrap_or("1")
        .parse::<isize>()
        .unwrap_or(1);

    for _ in 0..count {
        let generated = Uuid::new_v4();

        println!("{}", generated);
    }
}

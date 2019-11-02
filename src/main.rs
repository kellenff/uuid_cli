use uuid::Uuid;

fn main() {
    let generated = Uuid::new_v4();

    println!("{}", generated);
}

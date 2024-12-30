use dotnet_aot_rust::{add, init};

fn main() {
    init();

    println!("1 + 2 = {}", add(1, 2));
}

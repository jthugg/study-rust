fn main() {
    println!("{}", add(128, 256));
    println!("{}", {
        128 + 256
    })
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

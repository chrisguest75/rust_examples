mod fibonacci;

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn main() {
    println!("Hello, world!");

    let a = 3;
    let b = 6;    
    let x = add(a, b);

    println!(
        "{} = {} + {}",
        x, a, b
    );

    // run for 40 generations
    let sequence = fibonacci::fibonacci(40);
    for value in sequence {
        println!("{}", value);
    }

}

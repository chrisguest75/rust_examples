fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn fibonacci(generations: i32) -> vec {
    let mut sequence = vec![1, 1];

    while generations > 0 {
        let a = sequence[-2];
        let b = sequence[-1];
        let c: i32 = a + b;
        sequence.push(c)    
    }
    return sequence;
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

    let sequence = fibonacci(20);
    println!(sequence)

}

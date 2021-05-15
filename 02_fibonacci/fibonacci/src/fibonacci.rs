

pub fn fibonacci(mut generations: i32) -> Vec<i32> {
    let mut sequence = vec![1, 1];
    let mut a = 1;
    let mut b = 1;

    while generations > 0 {
        let c: i32 = a + b;
        sequence.push(c);   
        a = b;
        b = c;
        generations-=1; 
    };
    sequence
}

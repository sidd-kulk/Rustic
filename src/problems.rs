use std::str::FromStr;
use std::io::Write;

fn main(){
    let mut numbers = Vec::new();
    for arg in std::env::args().skip(1) {
        numbers.push(arg.parse::<usize>().unwrap());
    }

    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "pass parameters to function call <number 1> <number 2> ... ").unwrap();
        std::process::exit(1);
    }

    let mut final_result = numbers[0];
    for n in &numbers[1..] {
        final_result = gcd(final_result, *n);
    }

    println!("GCD of {:?} is {}", numbers, final_result);
}

fn gcd(n: usize, m: usize) -> usize {
    if n == 0 {return m};

    gcd(m%n, n)
}

#[test]
fn test_gcd(){
    assert_eq!(gcd(15,10), 5);
    assert_eq!(gcd(3,10), 1);
}

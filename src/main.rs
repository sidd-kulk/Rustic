use std::str::FromStr;
use std::io::Write;

fn main(){
    println!("Starting...");
    let mut numbers = Vec::new();
    for arg in std::env::args().skip(1) {

        numbers.push(usize::from_str(&arg)
            .expect("error parsing argument"));
    }

    if numbers.len() != 2 {
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
    }

    println!("{:?}", numbers);

    println!("{}", gcd(numbers[0], numbers[1]));
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
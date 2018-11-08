fn main(){
    println!("Starting...");
    for arg in std::env::args() {
        println!("{}", arg);
    }
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


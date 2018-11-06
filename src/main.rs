fn gcd(n: u32, m: u32) -> u32 {
    if n == 0 {return m};

    gcd(m%n, n)

}

#[test]
fn test_gcd(){
    assert_eq!(gcd(15,10), 5);
    assert_eq!(gcd(3,10), 1)
}
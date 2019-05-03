fn prime_factors(i: i32) -> Vec<i32> {
    let mut n = i;
    let mut factors = Vec::new();
    let mut d = 2;
    while(n > 1){
        while(n%d == 0){
            factors.push(d);
            n = n/d;
        }
        d = d + 1;
    }
    if(n>1){
        factors.push(n);
    }

    factors
}


#[test]
fn test_prime_factors(){
    assert_eq!(vec![2], prime_factors(2));
    assert_eq!(vec![2,2], prime_factors(4));
    assert_eq!(vec![2,3], prime_factors(6));
    assert_eq!(vec![3,3,5,31], prime_factors(1395));
}

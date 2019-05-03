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

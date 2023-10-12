use std::str::FromStr;
use std::env;

fn main() {
    println!("Hello, world!");
    println!("{}", gcd(49,200)); // 1
    // println!("{}", gcd(0,200)); // thread 'main' panicked at 'assertion failed: n != 0 && m != 0'

    let mut numbers = Vec::new();

    for arg in env::args().skip(1){
        numbers.push(u64::from_str(&arg)
                    .expect("error parsing argument"));
    }

    if numbers.len()==0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..]{ // &演算子はベクタの2番目以降の要素への参照を借用
        d = gcd(d, *m); // *m:mをさんん章解決する演算子で、参照されている値を返す
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

fn gcd(mut n:u64, mut m:u64) -> u64 {
    assert!(n!=0 && m != 0); // どちらも0でないことを確認
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m %n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14,15),1);
    assert_eq!(gcd(2*3*5*11*17,
                   3*7*11*13*19),
            3*11);
}
// running 1 test
// test test_gcd ... ok
// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


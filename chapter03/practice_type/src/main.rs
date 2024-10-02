fn main() {
    println!("Hello, world!");
    let build_vector_val = build_vector();
    println!("{:?}", build_vector_val);

    // println!("{}", (-4).abs());
    println!("{}", (-4_i32).abs());
    println!("{}", i32::abs(-4));

    let mut i: i32 = 1;
    loop {
        // i *= 10; // attempt to multiply with overflow
        i = i.checked_mul(10).expect("multiplication overflowed");
    }


}

fn build_vector() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    // v.push(10i16);
    // v.push(20i16);

    // 型推論のおかげで以下のように書ける
    v.push(10);
    v.push(20);
    v
}


#[test]
fn tests() {
    assert_eq!(10_i8 as u16, 10_u16);// in range
    assert_eq!(2525_u16 as i16, 2525_i16);// in range

    assert_eq!(-1_i16 as i32, -1_i32);    // sign_extended
    assert_eq!(65535_u16 as i32, 65535_i32); // zero_extended

    assert_eq!(1000_i16 as u8, 232_u8); // when out of range, the original modulo 2^N    

    assert_eq!(65535_u32 as i16, -1_i16);

    assert_eq!(-1_i8 as u8, 255_u8);
    assert_eq!(255_u8 as i8, -1_i8);

    assert_eq!(2_u16.pow(4), 16);
    assert_eq!((-4_i32).abs(), 4);
    assert_eq!(0b101101_u8.count_ones(), 4); // population count（ビットカウント）

    assert_eq!(10_u8.checked_add(20), Some(30));
    assert_eq!(100_u8.checked_add(200), None);

    // let sum = x.checked_add(y).unwrap(); // 加算を行い、オーバーフローしてたらパニックを発生させる

    assert_eq!((-128_i8).checked_div(-1), None);
    
    assert_eq!(100_u16.wrapping_mul(200), 20000);
    assert_eq!(500_u16.wrapping_mul(500), 53392);

    assert_eq!(500_i16.wrapping_mul(500), -12144);

}


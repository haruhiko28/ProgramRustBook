fn main() {
    println!("Hello, world!");
    let build_vector_val = build_vector();
    println!("{:?}", build_vector_val);
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
}


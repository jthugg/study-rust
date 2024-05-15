fn main() {
    let x = get_value();
    println!("{}", x);
    // println!("{}", void_fn()); // compile error
    println!("{:?}", void_fn()); // {:?} -> debug print
}

fn get_value() -> i32 { // i32가 리턴 타입
    8 // 리턴 타입에는 세미콜론이 없다.
    // 8; // 세미콜론을 붙이면 "()"가 반환된다.
    // "()"는 empty tuple이고 unit type (void)
}

fn void_fn() {
    // 리턴을 주지 않으면 empty tuple을 자동으로 반환한다.
}

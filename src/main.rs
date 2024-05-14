use std::mem::size_of;

fn main() {
    println!("Hello, world!");

    // 정수형 변수 선언
    let var00 = 1;
    println!("정수형 변수 선언 var00: {}", var00);

    // let 변수는 기본 불변, 재할당 불가
    // const 키워드로 상수 선언 가능, 재할당 가능한 변수로 만들 수 없음.
    //var00 = 2;

    // mut 키워드로 재할당 가능한 변수로 선언 할 수 있음
    let mut var01 = 2;
    println!("정수형 변수 선언 var01: {}", var01);
    var01 = 3;
    println!("변수 재할당 var01: {}", var01);

    // Shadowing
    // 변수 셰도잉
    // 블록(세그먼트)이 끝날때 까지 새로 선언된 변수를 본다.
    println!("셰도잉 전 var00: {}", var00);
    let var00 = 3;
    println!("셰도잉 후 var00: {}", var00);

    {
        println!("새로운 코드블록 진입 var00: {}", var00);
        let var00 = 4;
        println!("새로운 셰도잉 시작 var00: {}", var00);
    }

    println!("직전 셰도잉 블록 종료 var00: {}", var00);

    // 세부 데이터 타입 지정
    // 만약 정수형 타입에서 별도로 명시하지 않으면 기본 i32 할당.

    // 부호가 있는 정수형
    let var02: i8 = 1; // 8 bit
    let var02: i16 = 1; // 16 bit
    let var02: i32 = 1; // 21 bit
    let var02: i64 = 1; // 64 bit
    let var02: i128 = 1; // 128 bit
    println!("var02: {}", var02);

    // 부호 없는 정수형
    let var03: u8 = 1; // unsigned 8 bit
    let var03: u16 = 1; // unsigned 16 bit
    let var03: u32 = 1; // unsigned 32 bit
    let var03: u64 = 1; // unsigned 64 bit
    let var03: u128 = 1; // unsigned 128 bit
    println!("var03: {}", var03);

    // let var03: u32 = -1; // compile error!

    // 특징적으로 Rust는 사용하지 않는 변수가 있다면 Warning 뱉는다.

    // 실수형
    // 만약 실수형 타입에서 별도로 명시하지 않으면 기본 f64 할당.
    let var04 = 1.0;
    let var04: f32 = 1.0;
    let var04: f64 = 1.0;
    println!("var04: {}", var04);

    // 문자형
    // 4 Bytes
    let var05 = 'a';
    let var05: char = 'a';
    println!("var05: {}", var05);

    // boolean
    let var06 = true;
    let var06: bool = false;
    println!("var06: {}", var06);

    // 참고: size_of
    println!("size of: {} Bytes", size_of::<char>());

    // 기본 타입 간 타입 캐스팅
    println!("casted var05: {}", var05 as i32);
    println!("casted var04: {}", var04 as i32);
    println!("casted var03: {}", var03 as f64);
}

fn main() {
    let a = 100;
    let b = Box::new(a);
    let c = b.as_ref();
    let d = &b;

    println!("a + *b = {}", a + *b);
    println!("변수 b의 주소: {:p}", &b); // 스택 메모리 주소 출력
    println!("변수 c의 주소: {:p}", &c); // 스택 메모리 주소 출력
    println!("변수 b가 참조하는 주소: {:p}", b); // 힙 메모리 주소 출력
    println!("변수 c가 참조하는 주소: {:p}", c); // 힙 메모리 주소 출력
    println!("변수 d의 값: {}", d);
    println!("변수 d의 주소: {:p}", &d); // 스택 메모리 주소 출력
    println!("변수 d가 참조하는 주소: {:p}", d); // 스택 메모리 주소 출력
}

fn main() {
    let mut x = 10;
    let y = &mut x;

    *y += 10;

    println!("y: {}", *y); // 이제 여기서 mutable ref(mutable borrow) 생명주기가 끝난것으로 컴파일러가 판단

    let z= &x; // 새로운 immutable ref 할당.
    println!("z: {}", *z);

    // ---------------------------------------------------------------------------------------------

    // mutable ref & immutable ref 순서 문제
    // 아래 코드는 컴파일 자체가 안됨
    // 왜?
    // 컴파일러가 immutable, mutable ref를 감지하는데
    // immutable이 먼저 할당(1)되고 읽기 전용으로 사용(4)되는 사이에
    // mutable을 할당할 수 없는 문제
    // 왜냐하면, immutable 할당 된 변수가 참조하는 값이 수정될 수 있는 unsafe 한 코드로 판단.
    // 마치 DB tx lost update 문제와 같음
    // let mut x = 20;
    // let y = &x; // 1) immutable ref 할당
    //
    // let z = &mut x; // 2) mutable ref 할당
    //
    // *z += 1; // 3) mutable ref 사용
    //
    // println!("y: {}", *y); // 4) immutable ref 사용

    // ---------------------------------------------------------------------------------------------

    // Shadowing
    let var = "value";
    let var_ref = &var;
    let var = 10;
    println!("{}, {}", *var_ref, var);
}

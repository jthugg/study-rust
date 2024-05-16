fn main() {
    let var00: i32; // 선언만 하고 컴파일하면 컴파일은 된다. 경고만 뱉어냄.
    // println!("{}", var00); // 하지만 이렇게 초기화 하지 않은 채로 사용하려들면 컴파일 조차 안된다.

    // ---------------------------------------------------------------------------------------------

    // 하지만 다음과 같이 사용은 할 수 있다.
    let var01; // 변수 선언

    { // 코드 블록 오픈
        var01 = 10; // 변수 값 할당.
    } // 여기서 코드블록이 끝났으니 var01은 더이상 사용 할 수 없나?

    println!("var01: {}", var01); // 사용할 수 있다. 왜냐하면 선언 위치가 코드블록 바깥이기 때문.

    // ---------------------------------------------------------------------------------------------

    // 이렇게도 사용 가능
    let var02 = { // 익명함수 처럼 사용
        10
    };

    println!("var02: {}", var02);

    // ---------------------------------------------------------------------------------------------

    // 이렇게도 사용 가능
    let var03;

    {
        var03 = loop_then_return();
    }

    println!("var03: {}", var03);
}

fn loop_then_return() -> i32 {
    let mut count = 0;
    loop {
        count += 1;
        println!("now: {}", count);
        if count == 10 {
            break;
        }
    }
    count
}

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

    // ---------------------------------------------------------------------------------------------

    // 함수 간 참조
    // move semantics
    let value = "something".to_string();
    print_arg(value);
    // print_arg(value); // 이렇게 한번 더 사용하면 컴파일 에러 발생.
    // 왜?
    // move semantics
    // 첫번째 print_arg(String) 함수가 value 변수의 소유권을 가져가고 함수가 끝날때 할당 해제 함.
    // 더이상 main()에서 변수 value에 대한 소유권을 잃고 사라진다.
    // 이 문제 해결을 위해 value 변수를 다시 할당하고 print_arg_ref(String)함수를 보자.
    let value = "something2".to_string();
    print_arg_ref(&value);
    print_arg_ref(&value);

    // 함수도 &mut를 사용할 수 있다.
    let mut value = "hi".to_string();
    append_abc(&mut value);
    println!("print value in main: {}", value);

    let value = "hello".to_string();
    print_string_appended_abc(value);
    // println!("print value in main(): {}", value); // 여기서 컴파일 에러
    // main()이 소유권을 잃기 때문
    // 다시 소유권을 얻고자 한다면 변수를 다시 할당하고 함수 append_string_appended_abc_2(String)을 보자

    let mut value = "ola".to_string();
    value = print_string_appended_abc_2(value);
    println!("print value in main(): {}", value);

    // !참고!
    // .clone()을 너무 많이 쓰지는 말자
    // 힙에 과도하게 많은 객체가 할당될 수 있고 복사하는데 드는 비용도 만만찮다.
    // 가능하면 ref를 적절하게 사용해보자

    // ---------------------------------------------------------------------------------------------

    // copy types
    // 변수에 할당 된 값을 복사해서 함수에 인자로 넣는다.
    let my_string = "abc".to_string();

    append123(my_string);
    // println!("print value in main(String): {}", my_string); // 컴파일 에러
    // 왜?
    // 위에서도 설명했듯 append123(String) 함수가 소유권을 가져갔고 함수가 끝남에 따라 변수가 사라짐
    // 변수를 새로 할당하고 append123(String)에 인자를 어떻게 넣는지 보자
    let my_string = "def".to_string();
    append123(my_string.clone());
    append123(my_string); // 이렇게 해도 되고 뒤에서 더 사용하고싶다면 다시 한번 .clone()하면 된다.
    // 새로운 값을 만들어서 함수에 인자로 전달하는 것.

    // 지난번 챕터 007에서 나왔듯
    // 기본타입 중 스칼라 타입과 스칼라 타입으로 구성된 튜플들은
    // 별도의 .clone() 없이 바로 복사되므로 그냥 사용해도 무방.
    let number = 1;
    add10(number);
    add10(number);
    add10(number);
}

fn print_arg(value: String) {
    println!("{}", value);
}

fn print_arg_ref(value: &String) {
    println!("{}", value);
}

fn append_abc(value: &mut String) { // 함수의 매개변수로 &mut String을 요구
    value.push_str(" abc!");
    println!("print value in append_abc(String): {}", value);
}

fn print_string_appended_abc(mut value: String) { // 함수의 매개변수로 String을 받고 mut value: String으로 새로운 변수 할당
    value.push_str(" abc!");
    println!("print value in append_string_appended_abc(String): {}", value);
}

fn print_string_appended_abc_2(mut value: String) -> String { // 반환 타입이 String 이다.
    value.push_str(" abc!");
    println!("print value in append_string_appended_abc_2(String): {}", value);
    value
}

fn append123(mut value: String) {
    value.push_str("123");
    println!("print value in append123(String): {}", value);
}

fn add10(mut number: i32) {
    number += 10;
}

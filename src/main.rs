fn main() {
    let country = String::from("KOREA");
    println!("print on main: {}", country);
    println!("value's location: {:p}", country.as_ptr());
    print_value_via_reference(&country);
    print_value_directly(country.clone());
    // print_value(country);
    // println!("print on main: {}", country); // compile error

    println!("print on main: {}", country);
    println!("value's location: {:p}", country.as_ptr());

    let mut hello = String::from("hello");

    append(&mut hello);

    println!("print on main: {}", hello);
    println!("value's location: {:p}", hello.as_ptr());

    let hello = append_new(&hello);

    println!("print on main: {}", hello);
    println!("value's location: {:p}", hello.as_ptr());

    let x = 0; // 이렇게 스택에 할당된 값은 소유권 개념이 없는 것 처럼 동작한다.
    /* from: https://rinthel.github.io/rust-lang-book-ko/ch04-01-what-is-ownership.html#%EC%8A%A4%ED%83%9D%EC%97%90%EB%A7%8C-%EC%9E%88%EB%8A%94-%EB%8D%B0%EC%9D%B4%ED%84%B0-%EB%B3%B5%EC%82%AC
    그래서 어떤 타입이 Copy가 될까요? 여러분은 주어진 타입에 대해 확신을 하기 위해 문서를 확인할 수도 있겠지만,
    일반적인 규칙으로서 단순한 스칼라 값들의 묶음은 Copy가 가능하고,
    할당이 필요하거나 어떤 자원의 형태인 경우 Copy를 사용할 수 없습니다.
    Copy가 가능한 몇가지 타입을 나열해 보겠습니다:
    u32와 같은 모든 정수형 타입들
    true와 false값을 갖는 부울린 타입 bool
    f64와 같은 모든 부동 소수점 타입들
    Copy가 가능한 타입만으로 구성된 튜플들. (i32, i32)는 Copy가 되지만, (i32, String)은 안됩니다.*/

    println!("x: {:p}", &x);

    add5(x);

    println!("x: {:p}", &x);
    println!("x: {}", x);
}

fn print_value_directly(value: String) {
    println!("print on function: {}", value);
    println!("value's location: {:p}", value.as_ptr());
}

fn print_value_via_reference(value: &String) { // & -> immutable ref(shared ref): 읽기 전용, 재할당 불가
    println!("print on function: {}", value);
    println!("value's location: {:p}", value.as_ptr());
}

fn append(value: &mut String) { // &mut -> mutable ref(unique ref): 재할당 가능한 참조, 한 단계만 사용 가능
    value.push_str("!!!");
    println!("print on function: {}", value);
    println!("value's location: {:p}", value.as_ptr());
}

fn append_new(value: &String) -> String {
    let mut str = value.clone();
    str.push_str("!!!");
    println!("print on function: {}", str);
    println!("value's location: {:p}", str.as_ptr());
    str
}

fn add5(mut value: i32) { // 인자로 받은 x를 저장할 value만 mutable 하게 만든다.
    // 인자로 넣어준 x와 value는 서로 다른 스택 주소를 가진다.
    // 함수에 인자를 넣을 때 함수는 받은 인자를 매개변수에 복사한다.
    // 매개변수는 인자와 다른 변수다. 새로운 스택 공간 할당.
    value += 5;
    println!("x: {:p}", &value);
    println!("x: {}", value);
}

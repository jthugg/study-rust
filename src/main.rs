fn main() {
    let str01 = "Hello, "; // "Hello, " type of &str witch is allocated to static

    println!("str01 변수의 스택 주소: {:p}", &str01);
    println!("str01에 할당된 문자열의 스태틱 주소: {:p}", str01.as_ptr());

    let str02 = String::from("world!"); // "world!" type of String witch is allocated to heap

    println!("str02 변수의 스택 주소: {:p}", &str02);
    println!("str02에 할당된 문자열의 힙 주소: {:p}", str02.as_ptr());

    let str03 = str01.to_owned() + str02.as_str(); // "Hello, world!" type of String witch is allocated to heap

    println!("str03 변수의 스택 주소: {:p}", &str03);
    println!("str03에 할당된 문자열의 힙 주소: {:p}", str03.as_ptr());

    let str04 = str03.as_str();
    println!("str04 변수의 스택 주소: {:p}", &str04);
    println!("str04에 할당된 문자열의 힙 주소: {:p}", str04.as_ptr());
    // str04 는 왜 힙주소를 참조하나?
    // str04에 할당된 값은 이미 힙에 할당된 객체

    let str05 = "Hello, ";

    println!("str05 변수의 스택 주소: {:p}", &str05);
    println!("str05에 할당된 문자열의 스태틱 주소: {:p}", str05.as_ptr());
    // str05는 왜 str01과 같은 메모리를 참조하나?
    // 리터럴 최적화 작동 방식, 마치 자바의 스프링 풀(String Pool)과 같이 동작

    let str06 = "Hi ";
    let str07 = "I'm ";
    let str08 = "Neo.";

    let str09 = str06.to_owned() + str07 + &str08;

    println!("str09: {}", str09);
    // str09는 왜 String 타입일까?
    // &str 타입은 불변 타입, 가변타입인 String 타입으로 형변환 필요

    let str10 = String::from("asdf");
    let str11 = String::from("qwer");
    let str12 = str10 + str11.as_str();

    println!("str12: {}", str12);

    let mut str13 = String::from("asdfasdf");
    str13.push('!');
    str13.push_str("zxcvzxcv");
    let str14 = String::from("qwerqwer");
    str13.push_str(str14.as_str());

    println!("{}", str13);
    // push와 push_str은 변수에 값을 재할당 하는 방식.
    // 따라서 mut 키워드가 있어야 한다.

    let mut str15 = String::new();
    println!("str15 is {} length of {} capacity of {}", str15, str15.len(), str15.capacity());
    str15.push_str("this");
    println!("str15 is {} length of {} capacity of {}", str15, str15.len(), str15.capacity());
    str15.push_str(" is");
    println!("str15 is {} length of {} capacity of {}", str15, str15.len(), str15.capacity());
    str15.push_str(" mutable");
    println!("str15 is {} length of {} capacity of {}", str15, str15.len(), str15.capacity());
    str15.push_str(" string");
    println!("str15 is {} length of {} capacity of {}", str15, str15.len(), str15.capacity());

    let mut str16 = String::with_capacity(10);
    // 크기를 인자로 전달해서 문자열 용량을 설정할 수 있다.
    // 만약 push_str()과 같은 메서드로 크기가 커진다면 (초기 용량 * 2^n)으로 커짐
    println!("str16 is {} length of {} capacity of {}", str16, str16.len(), str16.capacity());
    str16.push_str("this");
    println!("str16 is {} length of {} capacity of {}", str16, str16.len(), str16.capacity());
    str16.push_str(" is");
    println!("str16 is {} length of {} capacity of {}", str16, str16.len(), str16.capacity());
    str16.push_str(" mutable");
    println!("str16 is {} length of {} capacity of {}", str16, str16.len(), str16.capacity());
    str16.push_str(" string");
    println!("str16 is {} length of {} capacity of {}", str16, str16.len(), str16.capacity());

    let char00 = str16.pop().unwrap();
    println!("str16 is {} length of {} capacity of {}", str16, str16.len(), str16.capacity());
    println!("char00 is {}", char00);
    // String.pop() 해도 용량이 줄어들진 않는다.
}
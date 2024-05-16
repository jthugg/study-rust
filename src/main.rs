use crate::print_my_constant_2::{print_my_constant_2, print_your_constant};

mod print_my_constant_2;

const MY_CONSTANT: i32 = 128; // const는 반드시 타입을 지정해줘야한다. 또 상수 네이밍은 대문자 스네이크 케이스
// 소문자를 사용하면 경고를 뱉는다.
// 소문자 사용을 허용하고싶다면 #attribute 전처리 선언
#[allow(non_upper_case_globals)]
const test: i32 = 256;

static mut YOUR_CONSTANT: i32 = 512; // static은 mut 키워드로 가변형 선언 가능. 하지만 거의 안쓰인다.

fn main() {
    println!("MY_CONSTANT: {}, test: {}", MY_CONSTANT, test);
    print_my_constant();
    print_my_constant_2();

    unsafe { // 반드시 unsafe 키워드 추가 해야 컴파일 가능
        YOUR_CONSTANT = 1024; // 이렇게 재할당 가능하지만 안전하지 않은 코드.
        println!("unsafe block print YOUR_CONSTANT. value: {}", YOUR_CONSTANT);
        print_your_constant();
    }

}

fn print_my_constant() {
    println!("println in print_my_constant. value: {}", MY_CONSTANT)
}

use crate::{MY_CONSTANT, YOUR_CONSTANT}; // 다른 파일에 있는 요소 불러오기

pub fn print_my_constant_2() {
    // 다른 파일에서도 사용할 수 있다.
    println!("println in print_my_constant_2. value: {}", MY_CONSTANT);
}

pub unsafe fn print_your_constant() {
    println!("{}", YOUR_CONSTANT);
}
fn main() {

    loop {
        // ... 명령문 ...
        break; // 탈출
    }

    let array = ["Hi", " I'm", " Neo."];

    for elem in array {
        print!("{}", elem);
    }
    print!("\n");

    let vec = vec!["elem1", "elem3", "elem3"];

    for elem in vec {
        println!("elem: {}", elem);
    }

    for elem in (0..=50).rev() { // (n..m): Range Type
        println!("Count Down: {}!", elem);
    }

    let mut number = 0;
    while number < 10 {
        if number % 2 == 1 {
            println!("홀수번째입니다! number: {}", number);
        } else {
            println!("짝수번째입니다! number: {}", number);
        }
        number += 1; // 러스트는 증감 연산자가 없다!
    }

    let number = 10;
    if number == 11 {
        println!("it's 11!");
    } else if number == 12 {
        println!("it's 12!");
    } else {
        println!("it's {}", number);
    }

    let var = true;

    // 러스트에는 3항 연산자가 없다.
    let variable = if var { 1 } else { 0 }; // 3항 연산자 처럼 사용 가능

}

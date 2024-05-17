fn main() {

    let number = 10;

    match number {
        10 => println!("it's a ten!"),
        11 => println!("it's a eleven!"),
        _ => println!("there's no match case!") // 자바 switch-case의 default 역할, 러스트에서는 반드시 있어야함
    };

    let decimal_str = match number {
        10 => "ten",
        11 => "eleven",
        _ => "not matched"
    };

    println!("decimal_str: {}", decimal_str);

    let [a, b] = ["A", "B"];

    match (a, b) {
        ("A", "B") => println!("upper case both!"),
        ("a", "b") => println!("lower case both!"),
        ("a", "B") | ("A", "b") => println!("different cases"),
        _ => println!("not alphabets!")
    }

    let [a, b] = ["a", "B"];

    match (a, b) {
        ("A", "B") => println!("upper case both!"),
        ("a", "b") => println!("lower case both!"),
        (_, "B") => println!("b is upper case but not sure a."),
        ("A", _) => println!("a is upper case but not sure b."),
        _ => println!("not alphabets!")
    }

    let (child, married) = (2, false);

    match (child, married) {
        (c, m) if !m => println!("i'm not married but i have {} child", c),
        (c, m) if m => println!("i'm married and i have {} child", c),
        _ => println!("no matched case!")
    }

    let my_number = 10;
    let some_variable = match my_number { // match의 반환 타입을 각 경우가 모두 같은 타입으로 맞춰야한다.
        10 => "ten",
        11 => "eleven",
        // 20 => String::from("20"), // <- 이런 경우 안됨! 주의!
        _ => "NaN"
    };

    println!("my nubmer: {}", some_variable);

    let mut decimal = 100;

    decimal = match_fn_1(decimal);
    match_fn_2(decimal);

}

fn match_fn_1(number: i32) -> i32 {
    match number {
        input => input / 10 // match statement 안에서 변수 선언 가능
    }
}

fn match_fn_2(number: i32) {
    match number {
        input @ 0..=10 => println!("number is between 0 and 10. number: {}", input),
        _ => println!("number is greater than 10")
    }
}

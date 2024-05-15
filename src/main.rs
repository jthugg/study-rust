fn main() {
    println!("Hello, world!");

    let city = "Seoul";
    let area_square = 605.2;
    let population = 977;

    println!("{}의 면적은 {}제곱킬로미터, 인구는 {}만 입니다.", city, area_square, population);
    println!("{0}의 면적은 {1}제곱킬로미터, 인구는 {2}만 입니다. {0}은 대한민국의 수도입니다.", city, area_square, population);
    println!("{city}의 면적은 {area_square}제곱킬로미터, 인구는 {population}만 입니다.");

    print!("AAA");
    print!("BBB");

    println!();

    println!(r#"c:\aaa\bbb\ccc"#); // raw text r#"raw text"#
    println!("this
is
new
String");

    let x = ();
    println!("{:?}", x); // debug print

    println!("{:p}", &x); // x의 메모리 주소
    println!("{:p}", &9); // 9가 할당 된 메모리 주소

    println!("{:X}", 9984); // hex decimal
    println!("{:b}", 9984); // binary

    let title = "TODAY'S NEWS";
    let bar = "|";
    let a = "SEOUL";
    let b = "TOKYO";

    println!("{:-^30}", title);
    println!("{: <15}{: >15}", bar, bar);
    println!("{:-<15}{:->15}", a, b);

    // learn more? visit https://dhghomon.github.io/easy_rust/Chapter_13.html
}

fn main() {
    println!("Hello, world!");

    let city = "Seoul";
    let area_square = 605.2;
    let population = 977;

    println!("{}의 면적은 {}제곱킬로미터, 인구는 {}만 입니다.", city, area_square, population);
    println!("{0}의 면적은 {1}제곱킬로미터, 인구는 {2}만 입니다. {0}은 대한민국의 수도입니다.", city, area_square, population);
    println!("{city}의 면적은 {area_square}제곱킬로미터, 인구는 {population}만 입니다.");
}

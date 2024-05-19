fn main() {
    println!("discount percentage: {:?}%", get_discount_percentage(&Membership::Normal));
    println!("discount percentage: {:?}%", get_discount_percentage(&Membership::Royal));
    println!("discount percentage: {:?}%", get_discount_percentage(&Membership::Premium));

    println!("{}", Membership::Normal as u32); // ordinal

    println!("coin value: {}", Coin::Dime as u32);

    let vec = vec![get_number(123), get_number(0), get_number(-123)];

    println!("vec: {:?}", vec);
}

fn get_discount_percentage(spend_amount: &Membership) -> u32 {
    match spend_amount {
        Membership::Premium => 30,
        Membership::Royal => 15,
        Membership::Normal => 0
    }
}

// fn get_discount_percentage(spend_amount: &Membership) -> u32 { // 이렇게도 쓸 수 있음
//     use Membership::*; // import
//     match spend_amount {
//         Premium => 30,
//         Royal => 15,
//         Normal => 0
//     }
// }

fn get_number(value: i32) -> Number {
    match value.is_positive() {
        true => Number::U32(value as u32),
        false => Number::I32(value)
    }
}

#[derive(Debug)]
enum Membership {
    Normal,
    Royal,
    Premium
}

#[derive(Debug)]
enum Coin {
    Half = 50,
    Quarter = 25,
    Dime = 10,
    Nickel = 5,
    Penny = 1
}

#[derive(Debug)]
enum Number {
    U32(u32),
    I32(i32)
}

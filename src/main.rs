fn main() {
    let man = Person {
        name: "neo".to_string(),
        another_name: "Neo".to_string(),
        height: 200,
        happiness: false,
        reason_of_happiness: "he is still jobless".to_string()
    };

    println!(
        "his name is {}({}), he is {} cm. he is {} because {}",
        man.name,
        man.another_name,
        man.height,
        man.is_happy(),
        man.reason_of_happiness
    );

    let Person {
        name,
        another_name,
        height,
        happiness,
        reason_of_happiness
    } = man; // destructuring

    println!(
        "his name is {}({}), he is {} cm. is he happy? {}. because {}",
        name,
        another_name,
        height,
        happiness,
        reason_of_happiness
    );

    let man = Person {
        name: "neo".to_string(),
        another_name: "Neo".to_string(),
        height: 200,
        happiness: false,
        reason_of_happiness: "he is still jobless".to_string()
    };

    let Person {
        name: a,
        another_name: b,
        height: c,
        happiness: d,
        reason_of_happiness: e
    } = man; // destructuring

    println!(
        "his name is {}({}), he is {} cm. is he happy? {}. because {}",
        a,
        b,
        c,
        d,
        e
    );

    let man = Person {
        name: "neo".to_string(),
        another_name: "Neo".to_string(),
        height: 200,
        happiness: false,
        reason_of_happiness: "he is still jobless".to_string()
    };

    let stranger = Alien::from_person(man);

    println!("this is unknown creature. {:?}", stranger);

}

struct Person {
    name: String,
    another_name: String,
    height: u8,
    happiness: bool,
    reason_of_happiness: String
}

impl Person {
    fn is_happy(&self) -> String {
        match self.happiness {
            true => "happy".to_string(),
            _ => "unhappy".to_string()
        }
    }
}

#[derive(Debug)]
struct Alien {
    name: String,
    height: u8
}

impl Alien {
    fn from_person(person: Person) -> Self {
        let Person { name, height, .. } = person;
        Self { name, height }
    }
}
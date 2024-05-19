fn main() {

    let animal = Animal::new_dog();
    println!("animal: {:?}", animal);
    animal.print(); // Animal::print(&animal);과 같다. dot operator => syntactic sugar
    Animal::print(&animal);

    let mut animal = Animal::new_cat();
    animal.print();
    animal.transform(AnimalType::Tiger);
    animal.print();

    let animal = Animal::new_wolf();
    animal.print();

    animal.check_type();
    animal.animal_type.print_type();

    let phone = SmartPhone::Android("Galaxy24".to_string());
    phone.print();

}

#[derive(Debug)]
struct Animal {
    age: u32,
    animal_type: AnimalType
}

// struct impl block
impl Animal {

    // fn new()를 만들어서 생성자 처럼 사용할 수 있다.
    // associate function: 객체를 만드는 함수
    // 함수 오버로딩 불가능
    fn new() -> Self {
        Self {
            age: 10,
            animal_type: AnimalType::Tiger
        }
    }

    fn new_dog() -> Self { // new는 예약어가 아님.
        Self {
            age: 10,
            animal_type: AnimalType::Dog
        }
    }

    fn new_cat() -> Self {
        Self {
            age: 10,
            animal_type: AnimalType::Cat
        }
    }

    fn print(&self) { // method
        println!("I am an {:?}", self);
    }

    pub fn transform(&mut self, animal_type: AnimalType) {
        self.animal_type = animal_type;
    }

}

// struct impl block
impl Animal { // impl block은 여러개 허용된다.

    fn new_wolf() -> Self {
        Self {
            age: 10,
            animal_type: AnimalType::Wolf
        }
    }

    fn check_type(&self) {
        match self.animal_type {
            AnimalType::Tiger => println!("tiger"),
            AnimalType::Wolf => println!("wolf"),
            AnimalType::Lion => println!("lion"),
            AnimalType::Dog => println!("dog"),
            AnimalType::Cat => println!("cat")
        }
    }

}

#[derive(Debug)]
enum AnimalType {
    Tiger,
    Lion,
    Wolf,
    Dog,
    Cat
}

impl AnimalType { // enum 도 구현부를 가질 수 있다.

    fn print_type(&self) {
        match self {
            AnimalType::Tiger => println!("tiger"),
            AnimalType::Lion => println!("lion"),
            AnimalType::Wolf => println!("wolf"),
            AnimalType::Dog => println!("dog"),
            AnimalType::Cat => println!("cat")
        };
    }

}

enum SmartPhone {
    Android(String),
    Ios(String)
}

impl SmartPhone {
    fn print(&self) {
        match self {
            SmartPhone::Android(model) => println!("model: {}", model),
            SmartPhone::Ios(model) => println!("model: {}", model),
        }
    }
}

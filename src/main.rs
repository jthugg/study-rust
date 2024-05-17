fn main() {
    // Array
    let arr1 = ["a", "b"];
    let arr2 = ["a", "b", "c"];

    // println!("is arr1 same as arr2? {}", arr1 == arr2); // 컴파일 에러
    // 왜?
    // arr1 -> [&str; 2]
    // arr2 -> [&str; 3]
    // 위와같이 같은 array타입이라도 길이가 다르므로 다른 타입으로 간주한다.
    // 하지만 다음 코드를 보자

    let arr2 = ["a", "c"];
    println!("is arr1 same as arr2? {}", arr1 == arr2); // 이렇게 길이가 같다면 비교를 할 수 있는데 요소가 모두 같은지를 비교한다. false
    let arr3 = ["a", "b"];
    println!("is arr1 same as arr3? {}", arr1 == arr3); // 길이와 요소가 모두 같다면 true를 출력한다.

    let arr4 = [String::from("a"), String::from("b")];
    let arr5 = [String::from("a"), String::from("b")];
    println!("is arr4 same as arr5? {}", arr4 == arr5); // 길이와 요소가 모두 같다면 true를 출력한다.
    println!("arr4[0] addr: {:p}", arr4[0].as_ptr());
    println!("arr5[0] addr: {:p}", arr5[0].as_ptr());
    println!("arr4[0] addr: {:p}", arr4[1].as_ptr());
    println!("arr5[1] addr: {:p}", arr5[1].as_ptr());
    // 자바와 다르게 힙에 참조하는 메모리 주소가 달라도 같다고 본다.

    // arr5.asdfsadfa(); // tip! 어떤 타입인지 컴파일리가 잡아주길 원한다면 이렇게 없는 메서드를 호출해봐도 된다. 컴파일 에러와 함께 타입도 알려준다.

    let arr6 = ["abc"; 10]; // 배열을 선언하고 초기값과 길이를 세팅해서 생성할 수 있다.
    println!("arr6: {:?}", arr6);

    let mut arr6 = ["abc"; 10]; // 배열의 요소를 변경하고싶다면 mut 키워드 필요.
    arr6[0] = "def";
    println!("arr6: {:?}", arr6);
    arr6.fill("가나다");
    println!("arr6: {:?}", arr6);

    // .get(index);
    // println!("arr6[16]: {}", arr6[16]) // 컴파일 에러. 배열의 길이를 벗어나면 컴파일시에 에러를 뱉는다.
    println!("arr6.get(16): {:?}", arr6.get(16)); // 하지만, .get(index)를 사용하면 None을 뱉는다.
    // Option 타입인데 자바의 Optional이랑 비슷하다고 보면 된다.
    // 러스트는 null을 차용하지 않았는데 열거형 타입 Option으로 해당 메모리 공간에 할당된 객체(또는 값)가 있는지 알 수 있다.

    // 배열 자르기 Array slices
    // let arr7 = arr6[0..2]; // 컴파일 에러
    // println!("{:?}", arr6[0..3]); // 컴파일 에러
    // 왜?
    // 배열의 크기를 컴파일 시점에 알 수 없음 따라서 포인터를 전달해줘야한다.
    let arr7 = &arr6[0..2];
    println!("{:?}", arr7);

    let arr8 = &arr6[..];
    println!("arr8: {:?}", arr8);

    let arr9 = &arr6[0..=2];
    println!("arr9: {:?}", arr9);

    // 그런데 희안하게 arr6, 7, 8이 참조하고있는 배열의 주소는 같다.
    // 러스트는 [n..m]을 통한 배열 슬라이스 시 새 객체를 할당하지 않고 기존 객체를 참조하며 그 참조하는 객체 + 길이의 메타데이터를 가진다.
    println!("arr6's variable address: {:p}, arr6's address: {:p}", &arr6, arr6.as_ptr());
    println!("arr7's variable address: {:p}, arr7's address: {:p}", &arr7, arr7.as_ptr());
    println!("arr8's variable address: {:p}, arr8's address: {:p}", &arr8, arr8.as_ptr());
    println!("arr9's variable address: {:p}, arr9's address: {:p}", &arr9, arr9.as_ptr());

    // 배열의 Destructuring
    let [a, b, c] = ["aaa", "bbb", "ccc"];
    println!("a: {}, b: {}, c: {}", a, b, c);

    let[a, _, _] = ["aaaa", "bbbb", "cccc"]; // _는 변수 선언과 데이터 할당을 하지 않겠다는 뜻
    println!("a: {}", a);

    // ---------------------------------------------------------------------------------------------

    // Vector(Vec)
    // 따지고 보면 String 타입도 Vec로 구현돼있다. 결국 문자 타입의 컬렉션인 셈.
    // 꺽쇠 괄호 안에 타입을 넣어주는 형태로 자바의 제네릭과 매우 유사하다.
    // 선언을 할 때 타입을 지정해야한다.
    // let vec00 = Vec::new(); // 선언만 하고 아무것도 안넣으면 컴파일 에러, 뒤에 뭔가 넣는 작업이 있다면 에러 x
    let mut vec00:Vec<String> = Vec::new();
    for value in 1..100 {
        let mut prefix = "value".to_string();
        prefix.push_str(&value.to_string());
        vec00.push(prefix);
    }
    println!("vec00: {:?}", vec00);

    let vec01 = vec![1, 2, 3]; // 이렇게 선언할 수 있다.
    println!("vec01: {:?}", vec01);
    let vec01 = vec!["a", "b", "c"]; // 이렇게 선언할 수 있다.
    println!("vec01: {:?}", vec01);
}

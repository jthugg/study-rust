use std::mem::size_of_val;

fn main() {

    let user =  User {
        id: 182756384359874,
        username: "testUser".to_string(),
        followers: 0,
        following: 0
    };

    println!("this user: {:?}", user);
    println!("this user: {:#?}", user);
    println!("id: {}\nusername: {}\nfollowers: {}\nfollowing: {}", user.id, user.username, user.followers, user.following);

    let foo = Foo;
    println!("foo: {:?}", foo);

    let my_tuple = MyTuple(10, "value".to_string(), false);
    println!("my tuple: {:?}", my_tuple);
    println!("0: {}", my_tuple.0);
    println!("1: {}", my_tuple.1);
    println!("2: {}", my_tuple.2);

    // ---------------------------------------------------------------------------------------------

    let user_id = 9382374;
    let username = "testName".to_string();
    let followers = 0;
    let following = 0;

    let user = User {
        id: user_id, // 변수 이름과 필드 이름이 다를 때
        username, // 변수와 타겟 필드 이름이 같을 때
        followers,
        following
    };

    println!("user: {:?}", user);
    println!("user takes {} bytes", size_of_val(&user));

}

// named struct
#[derive(Debug)] // debug print 가능하게 해주는 어노테이션
struct User {
    id: u64,
    username: String,
    followers: u64,
    following: u64,
}

// unit struct
#[derive(Debug)]
struct Foo;

// tuple struct
#[derive(Debug)]
struct MyTuple(u32, String, bool);

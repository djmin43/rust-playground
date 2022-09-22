fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.coma"),
        sign_in_count: user1.sign_in_count,
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let black_point = &black.0;
    let origin_point = origin.2;

    println!("{}", black_point);
    println!("{}", origin.2);

    let x = "hello";
    let c = String::from("bye");
    let y = x;
    println!("{}", x)

}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
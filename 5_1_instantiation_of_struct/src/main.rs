// 構造体
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// タプル構造体
struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        username: String::from("someone@example.com"),
        email: String::from("someusername123"),
        sign_in_count: 1,
        active: true,
    };

    // 構造体の値の更新
    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        username: String::from("anotherusername567"),
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
        active: user1.active,
    };

    let user2 = User {
        username: String::from("anotherusername567"),
        email: String::from("another@example.com"),
        ..user1 // 省略記法
    };

    let user2 = build_user(String::from("a"), String::from("b"));

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// 構造体のインスタンスは全データを所有する必要があるため
// &strではなくString型を引数にとる
// また、返り値に構造体Userを指定できる
fn build_user(email: String, username: String) -> User {
    User {
        email,               // 省略記法
        username,
        sign_in_count: 1,
        active: true,
    }
}


pub fn use_struct() {
    let user = User {
        username: String::from("ishibashi.futos"),
        email: String::from("ishibashi.futos@github.com"),
        active: true,
        sign_in_count: 0,
    };

    let mut user = user;
    user.email = String::from("ishibashi.futos@gitlab.com");

    let user = new_user(user.email, user.username);
    print_user(&user);

    // 構造体更新記法を使用すると、新しいインスタンスに対して値をセットできる
    let user = User {
        sign_in_count: 1,
        ..user
    };
    print_user(&user);

    let black = Color(0, 0, 0, 1.0);
    print_color(black);
    let white = BorderColor(255, 255, 255, 1.0);
    print_border_color(white);
}

struct User {
    username: String, // &strではなくStringを指定しているが、ライフタイム指定が必要なため
    email: String,    // &strではなくStringを指定しているが、ライフタイム指定が必要なため
    sign_in_count: u64,
    active: bool,
}

fn new_user(email: String, username: String) -> User {
    User {
        // 構造体のプロパティ名と同じ変数名を指定すると、そのプロパティに値を代入することができる
        email,
        username,
        active: true,
        sign_in_count: 0,
    }
}

fn print_user(user: &User) {
    println!(
        "name: {}, email: {}, isActive: {}, sign_in_count: {}",
        user.username, user.email, user.active, user.sign_in_count
    );
}

// 同じフィールドを持つtuple構造体だが、異なる型として扱われる
struct Color(u8, u8, u8, f32);
struct BorderColor(u8, u8, u8, f32);

fn print_color(color: Color) {
    println!(
        "r: {}, g: {}, b: {}, a: {}",
        color.0, color.1, color.2, color.3
    );
}

fn print_border_color(color: BorderColor) {
    println!(
        "r: {}, g: {}, b: {}, a: {}",
        color.0, color.1, color.2, color.3
    );
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn main() {
    // Fields can't be modified
    let u_vms = User {
        username: String::from("vms20591"),
        email: String::from("vms20591@gmail.com"),
        sign_in_count: 0,
        active: true
    };

    println!("Email for `{}` is `{}`", u_vms.username, u_vms.email);

    // use "mut" to modify fields of a struct
    let mut u_vms = User {
        username: String::from("vms20591"),
        email: String::from("vms20591@gmail.com"),
        sign_in_count: 0,
        active: true
    };

    u_vms.email = String::from("vms20591@protonmail.com");

    println!("Email for `{}` is `{}`", u_vms.username, u_vms.email);

    let new_user = build_user(u_vms.username.clone(), u_vms.email.clone());
}

fn build_user(username: String, email: String) -> User {
    User {
        username: username,
        email: email,
        active: true,
        sign_in_count: 0
    }
}
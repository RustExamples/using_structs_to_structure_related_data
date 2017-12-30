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

    // Create instance from another with "Struct update syntax"
    let u_xyz = User {
        username: String::from("xyz"),
        email: String::from("xyz@gmail.com"),
        ..u_vms // Fill in other fields from u_vms instance
    };
}
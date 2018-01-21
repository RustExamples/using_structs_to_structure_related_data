// Refer "Debug" trait to print object
// when using "{:?}"
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    let rect1 = Rectangle {
        width: width1,
        height: height1
    };
    
    // *c++* uses `.` to access methods directly on objects - `obj.show()`
    // *c++* uses `->` to access methods from pointer to object - `ptr_to_obj->show()` which is `*(ptr_to_obj).show()`
    // `->` basically *dereferences* an object and calls `.` on it
    // Rust has *automatic reference and dereference* & *calling methods* is one of few places rust uses it
    // Based on method signature Rust does this - `obj.show()` => `p1.show(&p2)` or `(&p1).show(&p2)`
    println!("Area of rectangle is {} sq. pixels", rect1.area());

    // {} -> Rust searches "Display" trait to print 
    // "struct" doesn't define it because its ambiguous
    
    // {:?} -> Rust searches "Debug" trait to print
    // "struct" defines it, but has to be referred

    // {:#?} -> Pretty print debug information
    println!("rect1 is {:#?}", rect1);

    let rect2 = Rectangle {
        width: 30,
        height: 40
    };

    let rect3 = Rectangle {
        width: 200,
        height: 50
    };

    println!("can rect1 hold rect2: {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3: {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(30);

    println!("square is {:#?}", square);
}

// Organize and  group methods of Struct
impl Rectangle {
    // First param is always "self"

    // Type not required like "self: Rectangle", because 
    // Rust knows the context of function
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        rect.width <= self.width && rect.height <= self.height
    }

    // Associated functions belong to struct
    // Doesn't have "self" as first param
    // Often used as constructor like "String::from()"
    // Called using "::" like "Rectange::square(10)"
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {   
        self.area() >= other.width * other.height // Weil Rückgabewert angegeben und das die letzte Zeile der expression ist, evaluirt dies automatisch zu true oder false
    }
}


fn main(){
    
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Umfasst rect1 rect2? {}", rect1.can_hold(&rect2));
    println!("Umfasst rect1 rect3? {}", rect1.can_hold(&rect3));
}









// fn main() {
    
//    let rect1 = Rectangle {
//        width: 30,
//        height: 50,
//    };

//    println!(
//        "Die Fläche des Rechtecks ist {} Quadratpixel.",
//        area(&rect1)
//    );

//    println!("rect1 ist {:#?}", rect1);

//}

//fn area(rectangle: &Rectangle) -> u32 {
//    rectangle.width * rectangle.height
//}

// https://rust-lang-de.github.io/rustbook-de/ch05-02-example-structs.html
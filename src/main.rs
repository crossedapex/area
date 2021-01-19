
#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
    name: String,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other:&Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

}

fn main(){
    let rect1 = Rectangle{
        width: 50,
        height: 42,
        name: String::from("Rectangle 1"),
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
        name: String::from("Rectangle 2"),
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
        name: String::from("Rectangle 3"),
    };

    let rectangles = vec![rect1,rect2,rect3];

    // SEE IF ANY RECTANGLES CAN FIT ANOTHER
    println!("These rectangles are large enough to hold others:");
    for rectangle in rectangles.iter() {
        for other in rectangles.iter() {
            if rectangle.can_hold(other) {
                println!("{} can hold {}",rectangle.name,other.name)
            } 

            }
        }
    }
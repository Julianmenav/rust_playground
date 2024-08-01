
struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        other_rect.width <= self.width && other_rect.height <= self.height
    }

    fn square(size: i32) -> Self {
        Self {
            width: size,
            height: size
        }
    }    
}


fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 20
    };

    let rect2 = Rectangle::square(10);

    println!("El area del rectangulo 1 es: {}, siendo su ancho de {} y su altura de {}", rect1.area(), rect1.width, rect1.height);
    println!("El area del rectangulo 2 es: {}, siendo su ancho de {} y su altura de {}", rect2.area(), rect2.width, rect2.height);
    println!("El rectángulo 2 cabe dentro del 1? : {} ", if rect1.can_hold(&rect2) {"SI"} else {"NO"});

}

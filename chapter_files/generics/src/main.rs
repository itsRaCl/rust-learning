#[derive(Copy, Clone)]
struct Point<X1: Copy, Y1: Copy>{
    x: X1,
    y: Y1,
}




impl<X1: Copy, Y1: Copy> Point<X1, Y1> {
    fn mixup<X2: Copy, Y2: Copy>(self, other: &Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(&p2);

    println!("trying to print p2 {},{}",p2.x, p2.y);
    println!("trying to print p1 {},{}",p1.x, p1.y);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

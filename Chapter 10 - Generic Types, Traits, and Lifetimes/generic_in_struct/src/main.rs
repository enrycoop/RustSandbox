struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct MixedPoint<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> MixedPoint<X1, Y1> {
    fn mixup<X2, Y2>(self, other: MixedPoint<X2, Y2>) -> MixedPoint<X1, Y2> {
        MixedPoint { x: self.x, y: other.y }
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    let p2 = Point { x: 5.0, y: 10.0};
    
    println!("p.x = {}", p.x());
    println!("p2.x = {}", p2.x());
    println!("distance= {}", p2.distance_from_origin());

    let p1 = MixedPoint { x: 5, y: 10.4 };
    let p2 = MixedPoint { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

}
struct Point<T, Z>{
    x: T,
    y: Z,
}

impl<T,Z> Point<T,Z>{
    fn mixed<K, H>(self, other: Point<K, H>) -> Point<T, H>{
        Point { x: self.x, y: other.y }
    }
}

fn main() {
    let p1 = Point{x: 5, y: 10.5};
    let p2 = Point{x:"hello", y: 'x'};
    // let p2 = Point{x:10.5, y: 20.5};

    let p3 = p1.mixed(p2);
    println!("{}, {}", p3.x, p3.y);
}

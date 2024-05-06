struct Point<T> {
    x: T,
    y: T,
}

// this function works for any generic types 
impl<U> Point<U> {
    fn x(&self) -> &U {
        &self.x
    }
}

// this fn only works for Point which x and y are f64
impl Point<f64> {
    fn y(&self) -> &f64 {
        &self.y
    }
}

struct bPoint<U, T> {
    x: U,
    y: T,
}

impl<U, T> bPoint<U, T> {
    fn mixedUp<V, W>(self, other: bPoint<V, W>) -> bPoint<U, W> {
        bPoint {
            x: self.x,
            y: other.y
        }
    }
}

fn main() {
    let p1 = Point{x: 5, y: 10};
    p1.x(); //only x method is available
    let p2 = Point{x: 5.0, y: 10.0};
    p2.x();
    p2.y(); //both methods are avalible
    let p3 = bPoint{x:"hello", y:53};
    let p4 = bPoint{x: 'a', y:5.0};
    let p5 = p3.mixedUp(p4);
}


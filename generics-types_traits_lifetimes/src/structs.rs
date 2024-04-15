//generics
pub struct Point<T> {
    pub x: T,
    pub y: T
}

impl<T> Point<T> {
    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn y(&self) -> &T {
        &self.y
    }
}

//to make a function restrict to specific struct
// distance_from_origin is only relevant to this struct with f32 type
impl Point<f32> {
    pub fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
//mixup can also be done, to increase the flexiblity.
//this is to show that generics can also be declared with function definations.
// this is to introduce function "mixup" with its own set of generic types independent of those used in the struct Point2.
pub struct Point2<X1, Y1> {
    pub x: X1,
    pub y: Y1
}

impl<X1,Y1> Point2<X1, Y1>{
    pub fn mixup<X2,Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}
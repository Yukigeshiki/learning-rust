struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

impl<T, U> Point<T, U> {
    fn mix_up<X1, Y1>(self, other: Point<X1, Y1>) -> Point<T, Y1> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<f64, f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let both_integer = Point { x: 5, y: 10 };
    println!("x: {}, y: {}", both_integer.x, both_integer.y);
    println!("x: {}, y: {}", both_integer.x(), both_integer.y());
    let both_float = Point { x: 1.0, y: 4.0 };
    println!("Distance from origin: {}", both_float.distance_from_origin());
    let _integer_and_float = Point { x: 5, y: 4.0 };

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mix_up(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
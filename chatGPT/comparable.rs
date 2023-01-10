// In Rust, the Comparable trait is used to define the behavior of types that can be compared with the cmp method. This method compares two values and returns an Ordering enum, which indicates whether the first value is less than, equal to, or greater than the second value.

// To use the Comparable trait, you first need to make sure that your type implements the trait and defines the cmp method. For example, you might define a Point struct that represents a point in two-dimensional space, and implement the Comparable trait for it.

use std::cmp::Ordering;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x.cmp(&other.x).then(self.y.cmp(&other.y))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Here, the Point struct implements the PartialEq, Eq, Ord, and PartialOrd traits, which are used by the Comparable trait. The cmp method compares the x and y coordinates of the two points to determine their ordering.

// Once your type implements the Comparable trait, you can use the cmp method to compare values of that type. For example, you could use it to sort a vector of points in ascending order by their x and y coordinates.

let mut points = vec![Point { x: 2, y: 3 }, Point { x: 1, y: 5 }, Point { x: 2, y: 2 }];
points.sort();

for point in points {
    println!("{:?}", point);
}

// // This would print the points in the following order:

// Point { x: 1, y: 5 }
// Point { x: 2, y: 2 }
// Point { x: 2, y: 3 }

// You can also use the cmp method in other contexts where you need to compare values, such as in a match expression or with the comparison operators (<, <=, >, >=).

let point1 = Point { x: 2, y: 3 };
let point2 = Point { x: 1, y: 5 };

match point1.cmp(&point2) {
    Ordering::Less => println!("{:?} is less than {:?}", point1, point2),
    Ordering::Equal => println!("{:?} is equal to {:?}", point1, point2),
    Ordering::Greater => println!("{:?} is greater than {:?}", point1, point2),
}

assert!(point1 >= point2);
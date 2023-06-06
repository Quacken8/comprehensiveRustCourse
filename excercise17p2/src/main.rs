use std::f64::consts::PI;
#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn magnitude(&self) -> f64 {
        f64::sqrt((self.x * self.x + self.y * self.y) as f64)
    }

    fn dist(self, other: Point) -> f64 {
        let diff: Point = self - other;
        return diff.magnitude();
    }
}
impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::cmp::PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    fn new() -> Polygon {
        Polygon { points: Vec::new() }
    }

    fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }

    fn left_most_point(&self) -> Option<Point> {
        if self.points.len() == 0 {
            return None;
        }
        let mut leftmost = &self.points[0];
        for point in &self.points {
            if point.x < leftmost.x {
                leftmost = point;
            }
        }

        return Some(leftmost.clone());
    }

    fn iter(&self) -> std::slice::Iter<Point> {
        self.points.iter()
    }
}

pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    fn new(center: Point, radius: i32) -> Circle {
        Circle {
            center: center,
            radius: radius,
        }
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl Shape {
       

    fn perimeter(&self) -> f64 {
        match self {
            Shape::Circle(circle) => return 2.0 * PI * (circle.radius as f64),
            Shape::Polygon(polygon) => {
                let mut perimeter: f64 = 0.0;
                for i in 1..polygon.points.len() {
                    let firstPoint = polygon.points[i - 1];
                    let secondPoint = polygon.points[i];
                    perimeter += firstPoint.dist(secondPoint);
                }
                let firstPoint = polygon.points[polygon.points.len() - 1];
                let secondPoint = polygon.points[0];
                perimeter += firstPoint.dist(secondPoint);
                return perimeter;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_perimeters() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let perimeters = shapes
            .iter()
            .map(Shape::perimeter)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(perimeters, vec![15.48, 31.42]);
    }
}

#[allow(dead_code)]
fn main() {}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Triangle {
    v1: Point,
    v2: Point,
    v3: Point,
}

fn given_point(x: i32, y: i32) -> Point {
    Point {
        x,
        y
    }
}

fn is_triangle_valid(triangle: Triangle) -> bool {
    let (v1, v2, v3) = (triangle.v1, triangle.v2, triangle.v3);
    let e_v1_v2 = distance(&v1, &v2);
    let e_v1_v3 = distance(&v1, &v3);
    let e_v2_v3 = distance(&v2, &v3);

    e_v1_v2 + e_v1_v3 > e_v2_v3 &&
    e_v1_v3 + e_v2_v3 > e_v1_v2  &&
    e_v2_v3 + e_v1_v2 > e_v1_v3 
}

fn distance(p1: &Point, p2: &Point) -> f64 {
    let (x1, y1, x2, y2) = (f64::from(p1.x), f64::from(p1.y), f64::from(p2.x), f64::from(p2.y));
    ((x2 - x1).powf(2.0) + (y2 - y1).powf(2.0)).sqrt()
}

fn main() {
    let valid_triangle = Triangle{
        v1: given_point(1, 5),
        v2: given_point(2, 4),
        v3: given_point(4, 6),
    };

    let invalid_triangle = Triangle{
        v1: given_point(1, 1),
        v2: given_point(1, 4),
        v3: given_point(1, 5),
    };

    println!("valid triangle should be true: {}", is_triangle_valid(valid_triangle));
    println!("invalid triangle should be false: {}", is_triangle_valid(invalid_triangle));
}

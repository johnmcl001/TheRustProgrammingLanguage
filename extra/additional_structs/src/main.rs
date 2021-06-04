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

impl Triangle {
    fn is_triangle_valid(&self) -> bool {
        let e_v1_v2 = self.edge(&self.v1, &self.v2);
        let e_v1_v3 = self.edge(&self.v1, &self.v3);
        let e_v2_v3 = self.edge(&self.v2, &self.v3);

        e_v1_v2 + e_v1_v3 > e_v2_v3 &&
        e_v1_v3 + e_v2_v3 > e_v1_v2  &&
        e_v2_v3 + e_v1_v2 > e_v1_v3 
    }

    fn edge(&self, p1: &Point, p2: &Point) -> f64 {
        let (x1, y1, x2, y2) = (f64::from(p1.x), f64::from(p1.y), f64::from(p2.x), f64::from(p2.y));
        ((x2 - x1).powf(2.0) + (y2 - y1).powf(2.0)).sqrt()
    }
}

fn given_point(x: i32, y: i32) -> Point {
    Point {
        x,
        y
    }
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

    println!("valid triangle should be true: {}", valid_triangle.is_triangle_valid());
    println!("invalid triangle should be false: {}", invalid_triangle.is_triangle_valid());
}

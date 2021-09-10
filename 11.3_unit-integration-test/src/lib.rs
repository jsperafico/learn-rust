pub struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Point {
            x, y, z
        }
    }

    pub fn draw(&self) -> String {
        format!("({}, {}, {})", self.x, self.y, self.z)
    }

    fn is_overlaping(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

// Declaration of Unit Test
// Will have access to private methods as well..
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn must_create_new_point() {
        Point::new(1,1,1);
    }

    #[test]
    fn must_draw_a_point() {
        let p = Point::new(1,1,1);
        assert_eq!(p.draw(), "(1, 1, 1)");
    }

    #[test]
    fn must_not_be_overlaping() {
        let p1 = Point::new(1,1,1);
        let p2 = Point::new(1,1,2);
        assert!(!p1.is_overlaping(&p2));

        let p1 = Point::new(1,1,1);
        let p2 = Point::new(1,2,1);
        assert!(!p1.is_overlaping(&p2));

        let p1 = Point::new(1,1,1);
        let p2 = Point::new(2,1,1);
        assert!(!p1.is_overlaping(&p2));
    }

    #[test]
    fn must_be_overlaping() {
        let p1 = Point::new(1,1,2);
        let p2 = Point::new(1,1,2);
        assert!(p1.is_overlaping(&p2));

        let p1 = Point::new(1,2,1);
        let p2 = Point::new(1,2,1);
        assert!(p1.is_overlaping(&p2));

        let p1 = Point::new(2,1,1);
        let p2 = Point::new(2,1,1);
        assert!(p1.is_overlaping(&p2));

        let p1 = Point::new(1,1,1);
        let p2 = Point::new(1,1,1);
        assert!(p1.is_overlaping(&p2));
    }
}

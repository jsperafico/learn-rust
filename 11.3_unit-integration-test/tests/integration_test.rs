use unit_integration_test::Point;
mod common;

#[test]
fn must_create_new_point() {
    common::setup();
    
    Point::new(1,1,1);
}

#[test]
fn must_draw_a_point() {
    common::setup();

    let p = Point::new(1,1,1);
    assert_eq!(p.draw(), "(1, 1, 1)");
}

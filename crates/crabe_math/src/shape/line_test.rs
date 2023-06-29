#[test]
fn test_line(){
    let l1 = Line::new(Point2::new(0, 0), Point2::new(1, 1));
    let l2 = Line::new(Point2::new(0, 1), Point2::new(1, 0));
    l1.intersect(l2);
}
mod exactpred;

fn main() {
    let p1_x = 0.0f64;
    let p1_y = 0.0f64;
    let p2_x = 100.0f64;
    let p2_y = 100.0f64;
    let p3_x = 50.0f64;
    let p3_y = 50.0f64;

    assert_eq!(
        run_robust2d(p1_x, p1_y, p2_x, p2_y, p3_x, p3_y),
        run_exactpred(p1_x, p1_y, p2_x, p2_y, p3_x, p3_y),
        "({}, {}) ({}, {}) ({}, {})", p1_x, p1_y, p2_x, p2_y, p3_x, p3_y
    );
}

fn run_robust2d(p1_x: f64, p1_y: f64, p2_x: f64, p2_y: f64, p3_x: f64, p3_y: f64) -> f64 {
    let p1 = robust2d::primitives::Point::new(p1_x, p1_y);
    let p2 = robust2d::primitives::Point::new(p2_x, p2_y);
    let p3 = robust2d::primitives::Point::new(p3_x, p3_y);
    unsafe { robust2d::ffi::orient2d(&p1.x, &p2.x, &p3.x) }
}

fn run_exactpred(p1_x: f64, p1_y: f64, p2_x: f64, p2_y: f64, p3_x: f64, p3_y: f64) -> f64 {
    let p1 = [p1_x, p1_y];
    let p2 = [p2_x, p2_y];
    let p3 = [p3_x, p3_y];
    exactpred::orient2d(p1, p2, p3)
}

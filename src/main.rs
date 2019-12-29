use std::{
    fs,
    io::{self, Read, Write},
};

mod exactpred;

fn main() {
    loop {
        run_once()
    }
}

fn run_once() {
    let (p1_x, p1_y) = (rand_f64(), rand_f64());
    let (p2_x, p2_y) = (rand_f64(), rand_f64());
    let (p3_x, p3_y) = (rand_f64(), rand_f64());

    let left = run_robust2d(p1_x, p1_y, p2_x, p2_y, p3_x, p3_y);
    let right = run_exactpred(p1_x, p1_y, p2_x, p2_y, p3_x, p3_y);

    if left.is_nan() && right.is_nan() {
        return;
    }

    assert_eq!(
        left, right,
        "({}, {}) ({}, {}) ({}, {})",
        p1_x, p1_y, p2_x, p2_y, p3_x, p3_y
    );

    print_dot();
}

fn print_dot() {
    print!(".");
    io::stdout().flush().expect("could not flush stdout");
}

fn rand_f64() -> f64 {
    let mut urandom_file = fs::File::open("/dev/urandom").expect("could not open /dev/urandom");
    let mut bytes = [0u8; 8];
    urandom_file
        .read_exact(&mut bytes)
        .expect("could not read bytes from /dev/urandom");
    f64::from_be_bytes(bytes)
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

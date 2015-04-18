
use types::{SPoint, Point, Vector, Index, rotate_ccw};
use std::path::Path;
use tau::TAU;

/// Converts a ascii Path to a &str
pub fn ascii_path_to_string(path: &Path) -> Option<&str> {
    use std::ascii::AsciiExt;
    let s: &str = match path.to_str() {
        Some(x) => x,
        None => return None,
    };

    if s[..].is_ascii() {
        Some(s)
    } else {
        None
    }
}
#[test]
fn test_ascii_path_to_string() {
    let p = &Path::new("/test/file.foo");
    let strified = ascii_path_to_string(p);
    assert_eq!(strified, Some("/test/file.foo"));
}


pub fn smooth_line_normal(&a: &Point, a_r: f64, a_inblob: bool,
                          &b: &Point, b_r: f64, b_inblob: bool) -> Vector {
    use na::*;
    //                     ___
    //        \n     ___---\ b_r
    //  a      \__---       b
    //r_a\__---blob
    //
    //                   __c
    //            _____-- x\   x is 90 degrees.
    //  z ____----         y\a_r+b_r == delta
    // a------------d--------b
    //
    //          d   ___---b
    //       ___----     y| a_r+b_r
    //  __--z            x|
    // a------------------c
    // Form a triangle with ab as the hypotenuse,
    // r1±r2 as the height, and the third side unknown
    // delta / distance = sin(z) = cos(y)
    let mut d:Vector = b - a;
    let distance:f64 = d.normalize_mut();
    // d is now normalized
    let delta = if a_inblob == b_inblob {
        (a_r - b_r)/distance
    } else {
        (a_r + b_r)/distance
    };
    assert!(delta <= 1., "The radii not must overlap! a_r:{}, b_r:{} distance:{}",
           a_r, b_r, distance);

    let mut y = delta.acos();
    if !a_inblob && b_inblob {
        y = (TAU/2.)-y
    }
    rotate_ccw(y, d)
}


#[test]
fn test_smooth_line_normal_up() {
    use na;
    let a = Point::new(0.0, 0.0);
    let b = Point::new(1.0, 0.0);

    let up = smooth_line_normal(&a, 0.2, true, &b, 0.2, true);
    println!("Up is {:?}", up);
    assert!(na::approx_eq(&up, &Vector::new(0.0, 1.0)));
}

#[test]
fn test_smooth_line_normal_up2() {
    use na;
    let a = Point::new(0.0, 0.0);
    let b = Point::new(1.0, 0.0);

    let up2 = smooth_line_normal(&a, 0.2, false, &b, 0.2, false);
    println!("up2 is {:?}", up2);
    assert!(na::approx_eq(&up2, &Vector::new(0.0,  1.0)));
}

#[test]
fn test_smooth_line_normal_upish_leftish() {
    let a = Point::new(0.0, 0.0);
    let b = Point::new(1.0, 0.0);

    let uppish_leftish = smooth_line_normal(&a, 0.2, true, &b, 0.8, true);
    println!("Uppish leftish is {:?}", uppish_leftish);
    assert!(uppish_leftish.x < 0.);
    assert!(uppish_leftish.y > 0.);
}

#[test]
fn test_smooth_line_normal_up_left() {
    let a = Point::new(0.0, 0.0);
    let b = Point::new(1.0, 0.0);
    //           \         ____
    //  o         \____---- o
    //    ____---- blob
    //
    let up_left = smooth_line_normal(&a, 0.2, false, &b, 0.2, true);
    println!("up_left is {:?}", up_left);
    assert!(up_left.x < 0.);
    assert!(up_left.y > 0.);
}

#[test]
fn test_smooth_line_normal_up_right() {
    let a = Point::new(0.0, 0.0);
    let b = Point::new(1.0, 0.0);

    let up_right = smooth_line_normal(&a, 0.2, true, &b, 0.2, false);
    println!("up_right is {:?}", up_right);
    assert!(up_right.x > 0.);
    assert!(up_right.y > 0.);
}
#[test]
fn test_smooth_line_normal_torus_bad() {
    let a = Point::new(0.707,-0.707);
    let b = Point::new(0.0, 0.0);

    let n = smooth_line_normal(&a, 0.1, true, &b, 0.1, false);
    println!("n: {:?}", n);
    assert!(n.x < 0.);
    assert!(n.y < 0.);
    assert!(n.x < n.y); // Pointing more x-ly
}


/// Normalizes an angle to be within [0,2*PI)
/// # Examples
/// ```
/// ```
fn normalize_angle(mut a: f64) -> f64 {
    while a < 0. { a = a + TAU; }
    while a >= TAU { a = a - TAU; }
    a
}
#[test]
fn test_normalize_angle() {
    use std::f64::consts::PI;

    assert_eq!(normalize_angle(-PI), PI);
    assert!(normalize_angle(-0.785398) > 0.);
    assert_eq!(normalize_angle((-135.0 as f64).to_radians()), (225.0 as f64).to_radians());
}


pub fn smooth_line_angle(a: &Point, a_r: f64, a_inblob: bool,
                         b: &Point, b_r: f64, b_inblob: bool) -> (f64,f64) {
    let n = smooth_line_normal(a, a_r, a_inblob, b, b_r, b_inblob);
    let theta = n.y.atan2(n.x); // Yes, I know it's strange.

    let theta = normalize_angle(theta);

    if a_inblob == b_inblob {
        (theta, theta)
    } else {
        (theta, normalize_angle(theta + TAU/2.))
    }
}

#[test]
fn test_smooth_line_angle() {
    let a = Point::new(0.,0.);
    let points = vec![
        Point::new(0., -1.),  // 0
        Point::new(1., -1.),  // 1
        Point::new(1.,  0.),  // 2
        Point::new(1.,  1.),  // 3
        Point::new(0.,  1.),  // 4
        Point::new(-1., 1.),  // 5
        Point::new(-1., 0.),  // 6
        Point::new(-1., -1.),  // 7
        ];

    for (i, &b) in points.iter().enumerate() {
        let angle = (i * 45) as f64;
        let (theta, _theta2) = smooth_line_angle(&a, 0.1, true, &b, 0.1, true);
        assert!((theta.to_degrees() - angle).abs() < 0.01,
            "true true angles incorrect in {}, expected {}, got {}",
            i, angle, theta.to_degrees());
        let (theta, _theta2) = smooth_line_angle(&a, 0.1, false, &b, 0.1, false);
        assert!((theta.to_degrees() - angle).abs() < 0.01, "false false angles incorrect in {}", i);
    }
}


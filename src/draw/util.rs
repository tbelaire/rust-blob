
use types::{Point, Vector, Index, Color};
use std::f64;
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


pub fn smooth_line_normal(&a: &Point, a_r: &f64, a_inblob: bool,
                          &b: &Point, b_r: &f64, b_inblob: bool) -> Vector {
    use na::*;
    use std::ops::Sub;
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

    let mut y = delta.acos();
    if !a_inblob && b_inblob {
        y = (TAU/2.)-y
    }
    println!("Angle is {}", y.to_degrees());
    Rot2::<f64>::new(Vec1::new(y)) * d
}


#[test]
fn test_smooth_line_normal_up() {
    use na;
    let a = Point::new(0.0, 0.0);
    let b = Point::new(1.0, 0.0);

    let up = smooth_line_normal(&a, &0.2, true, &b, &0.2, true);
    println!("Up is {:?}", up);
    assert!(na::approx_eq(&up, &Vector::new(0.0, 1.0)));
}

#[test]
fn test_smooth_line_normal_up2() {
    use na;
    let a = Point::new(0.0, 0.0);
    let b = Point::new(1.0, 0.0);

    let up2 = smooth_line_normal(&a, &0.2, false, &b, &0.2, false);
    println!("up2 is {:?}", up2);
    assert!(na::approx_eq(&up2, &Vector::new(0.0,  1.0)));
}

#[test]
fn test_smooth_line_normal_upish_leftish() {
    use na;
    let a = Point::new(0.0, 0.0);
    let b = Point::new(1.0, 0.0);

    let uppish_leftish = smooth_line_normal(&a, &0.2, true, &b, &0.8, true);
    println!("Uppish leftish is {:?}", uppish_leftish);
    assert!(uppish_leftish.x < 0.);
    assert!(uppish_leftish.y > 0.);
}

#[test]
fn test_smooth_line_normal_up_left() {
    use na;
    let a = Point::new(0.0, 0.0);
    let b = Point::new(1.0, 0.0);
    //           \         ____
    //  o         \____---- o
    //    ____---- blob
    //
    let up_left = smooth_line_normal(&a, &0.2, false, &b, &0.2, true);
    println!("up_left is {:?}", up_left);
    assert!(up_left.x < 0.);
    assert!(up_left.y > 0.);
}

#[test]
fn test_smooth_line_normal_up_right() {
    use na;
    let a = Point::new(0.0, 0.0);
    let b = Point::new(1.0, 0.0);

    let up_right = smooth_line_normal(&a, &0.2, true, &b, &0.2, false);
    println!("up_right is {:?}", up_right);
    assert!(up_right.x > 0.);
    assert!(up_right.y > 0.);
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
}


fn smmoth_line_angle(a: &Point, a_r: &f64, a_inblob: bool,
                     b: &Point, b_r: &f64, b_inblob: bool) -> (f64,f64) {
    let n = smooth_line_normal(a, a_r, a_inblob, b, b_r, b_inblob);
    let theta = n.y.atan2(n.x); // Yes, I know it's strange.

    if a_inblob == b_inblob {
        (theta, theta)
    } else {
        (theta, normalize_angle((theta + TAU/2.)))
    }
}



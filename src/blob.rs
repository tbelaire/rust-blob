use types::{Point, Index, Radius};
use config::Config;
use EPSILON;

use std::f64;



use self::Orientation::*;
#[derive(PartialEq, Clone, Debug)]
enum Orientation {
    Colinear,
    Clockwise,
    CounterClockwise
}
fn orientation(a: Point, b: Point, c: Point) -> Orientation {
    // println!("a: {:?}, b:{:?}, c:{:?}", a, b, c);
    let ab = b - a;
    let ac = c - a;
    // println!("ab: {:?}, ac: {:?}", ab, ac);
    let cross = ab.y * ac.x - ab.x * ac.y;
    // println!("Cross: {}", cross);
    if cross.abs() < EPSILON {
        return Colinear;
    } else if cross > 0.0 {
        return Clockwise;
    } else {
        return CounterClockwise;
    }
}
#[test]
fn orientation_test() {
    let a = Point::new(0.0, 0.0);
    let b = Point::new(0.0, 1.0);
    let c = Point::new(1.0, 1.0);
    let d = Point::new(1.0, 0.0);

    assert_eq!(orientation(a,b,d), Clockwise);
    assert_eq!(orientation(a,d,b), CounterClockwise);
    assert_eq!(orientation(a,c,b), CounterClockwise);
}
#[test]
fn orientation_test_torus() {
    let points = vec![
        Point::new(0.0, 0.0), // 0
        Point::new(-0.707, 0.707), // 1
        Point::new(0.0, 1.0), // 2
        Point::new( 0.707, 0.707), // 3
        Point::new(1.0, 0.0), // 4
        Point::new( 0.707,-0.707), // 5
        Point::new(0.0, -1.), // 6
        Point::new(-0.707,-0.707), // 7
        Point::new(-1., 0.0), // 8
    ];

    assert_eq!(orientation(points[8],points[4],points[6]), Clockwise);
    assert_eq!(orientation(points[8],points[6],points[7]), Clockwise);
}
#[test]
fn orientation_test_colinear() {
    let a = Point::new(0.0, 0.0);
    let b = Point::new(0.0, 1.0);
    let c = Point::new(0.0, 2.0);

    assert_eq!(orientation(a,b,c), Colinear);
}

/// Giftwrap algorithm for finding the convex hull of a set of points.
/// Runs in O(n * m), where n = included.len() and m = result.len();
pub fn giftwrap(points: &Vec<Point>,
            included: &Vec<Index>)
        -> Vec<Index> { // Hull

    use std::collections::HashSet;

    if included.len() <= 2 {
        return included.clone();
    }

    let mut leftmost = Point::new(f64::INFINITY, f64::INFINITY);
    let mut leftmost_ix :isize = -1;

    for &i in included {
        let p = points[i];
        if p.x <= leftmost.x {
            if p.x < leftmost.x || p.y < leftmost.y {
                leftmost = p;
                leftmost_ix = i as isize;
            }
        }
    }

    assert!(leftmost_ix >= 0);
    let leftmost_ix : usize = leftmost_ix as usize;
    // println!("Leftmost is {:?} at {}", leftmost, leftmost_ix);

    let mut ix_left_to_insert:HashSet<Index> = HashSet::with_capacity(included.len());
    for &i in included {
        ix_left_to_insert.insert(i);
    }
    ix_left_to_insert.remove(&leftmost_ix);

    let mut hull = vec![]; // we're going to insert leftmost last

    let start_ix = leftmost_ix;
    let mut base_ix = leftmost_ix;
    let mut need_to_reinsert_leftmost = true;
    loop {
        // println!("With base {:?} at ({})", points[base_ix], base_ix);
        let mut end_ix:Index = match ix_left_to_insert.iter().next() {
            Some(x) => *x,
            None    => break, // Ran out of points, we're done
        };
        for &other_ix in ix_left_to_insert.iter().skip(1) {
            match orientation(points[base_ix], points[end_ix], points[other_ix]) {
                Colinear => continue, // TODO check distance
                Clockwise => continue,
                CounterClockwise => {
                    /*
                    println!(
                        "With {base:?}, Swapping {end:?}({end_ix:?})
                         for {other:?}({other_ix:?}) \n
                        as {base_ix:?} {end_ix:?} {other_ix:?}
                         form a CounterClockwise triange",
                        base=points[base_ix], end=points[end_ix], other=points[other_ix],
                        base_ix=base_ix, end_ix=end_ix, other_ix=other_ix);
                    */
                    end_ix = other_ix},
            };
        }
        ix_left_to_insert.remove(&end_ix);
        hull.push(end_ix);
        base_ix = end_ix;
        if base_ix == start_ix {
            break;
        }
        if need_to_reinsert_leftmost {
            need_to_reinsert_leftmost = false;
            ix_left_to_insert.insert(leftmost_ix);
        }
    }
    hull
}

#[test]
fn test_giftwrap() {
    let points = vec![
        Point::new(0.0, 0.0), // 0
        Point::new(1.0, 0.0), // 1
        Point::new(1.0, 1.0), // 2
        Point::new(0.0, 1.0), // 3
        Point::new(0.5, 0.5), // 4
    ];
    let inpoints = vec![0,1,2,3,4];
    let inpoints = vec![4,1,0,2,3];

    let hull = giftwrap(&points, &inpoints);
    assert_eq!(hull, vec![ 3, 2, 1, 0 ]);
}
#[test]
fn test_giftwrap_torus() {
    let points = vec![
        Point::new(0.0, 0.0), // 0
        Point::new(-0.707, 0.707), // 1
        Point::new(0.0, 1.0), // 2
        Point::new( 0.707, 0.707), // 3
        Point::new(1.0, 0.0), // 4
        Point::new( 0.707,-0.707), // 5
        Point::new(0.0, -1.), // 6
        Point::new(-0.707,-0.707), // 7
        Point::new(-1., 0.0), // 8
    ];
    let inpoints = vec![0,2,4,6,8];
    let hull = giftwrap(&points, &inpoints);
    assert_eq!(hull, vec![ 2, 4, 6, 8, ]);

    let inpoints = vec![0,1,2,3,4,5,6,7,8];
    let hull = giftwrap(&points, &inpoints);
    assert_eq!(hull, vec![ 1, 2, 3, 4, 5, 6, 7, 8, ]);
}


/// The main workhorse function.
/// Finds the perfect hull, and radii for the blob.
pub fn find_hull(
            config: &Config,
            points: &Vec<Point>,
            inblob: &Vec<bool>,
            inpoints: &Vec<Index>,
            expoints: &Vec<Index>,
            ) -> (Vec<Index>, Vec<Radius>) {

    let mut hull = giftwrap(&points, &inpoints);

    debug!("After giftwrap");
    if config.run.fix_hull {
        // todo fix hull
        hull = fix_hull(&points, &inblob, hull, &inpoints, &expoints);
    }
    debug!("After fix_hull");
    if config.run.refine_poly {
        // todo refine poly
    }
    debug!("After refine_poly");
    if config.run.rm_crossing {
        // todo remove crossing
    }
    debug!("After rm_crossings");
    let dist = compute_nearest_distances(&points);
    trace!("Distances {:?}", dist);
    let radii = dist.into_iter().map(|x| x / config.b2.mindist_radius_factor).collect();
    trace!("Radii {:?}", radii);
    debug!("After compute radii");

    (hull, radii)
}

pub fn make_inblob(size: usize, included: &Vec<Index>) -> Vec<bool> {
    let mut inblob: Vec<bool> = Vec::with_capacity(size);
    inblob.resize(size, false);
    for &i in included {
        inblob[i] = true;
    }
    inblob
}

fn partial_min(a:f64, b:f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}
pub fn compute_nearest_distances(
        points: &Vec<Point>) -> Vec<Radius> {

    use na::Norm;
    // To avoid a lot of sqrts, I compute all the radii squared
    // and sqrt it all at the end.
    let mut radii2 = vec![f64::INFINITY; points.len()];
    for i in 0..points.len() {
        for j in i+1..points.len() {
            let sqnorm = (points[i] - points[j]).sqnorm();
            // Nans or other incomparables will return nothing
            // if partial_min is passed them
            // We know sqnorm is not nan though.
            radii2[i] = partial_min(radii2[i], sqnorm);
            radii2[j] = partial_min(radii2[j], sqnorm);
        }
    }
    // TODO factor out the 0.3 into config.b2.mindist_radius_factor
    let radii = radii2.into_iter().map(|r2:f64| -> f64 {r2.sqrt()}).collect();
    radii
}

pub fn fix_hull(
        points: &Vec<Point>,
        inblob: &Vec<bool>,
        mut hull: Vec<Index>,
        inpoints: &Vec<Index>,
        expoints: &Vec<Index>) -> Vec<Index> {

    for &eix in expoints {
        // TODO improve runtime by banishing in_hull.
        if point_inside(&points, &points[eix], &hull) && !in_hull(eix, &hull) {

            warn!("There's a excluded point inside: {1:?}({0})", eix, points[eix]);

        }
    }
    hull
}

fn point_inside(points: &Vec<Point>, p: &Point, hull: &Vec<Index>) -> bool {
    let mut inside = false;
    let mut e0 = points[hull[hull.len() - 1]];
    let mut y0 = (e0.y > p.y);
    for &ix in hull.iter() {
        let e1 = points[ix];
        let y1 = (e1.y > p.y);
        if y0 != y1 {
            // Mybe re-read http://www.ecse.rpi.edu/~wrf/Research/Short_Notes/pnpoly.html
            // t == y1 is more efficient maybe, or we can just use t2 instead
            // as it it more obviously correct.  Should benchmark it.
            // bool t = ((e1.y - p.y) * (e0.x - e1.x) >= (e1.x - p.x) * (e0.y - e1.y));
            let t2 = (p.x < ((e1.x-e0.x) * (p.y-e0.y) / (e1.y-e0.y)) + e0.x);
            if t2 {
                inside = !inside;
            }
        }
        e0 = e1;
        y0 = y1;
    }
    return inside;
}
fn in_hull(a : Index, p: &Vec<Index>) -> bool {
    match p.iter().position(|x| *x == a) {
        Some(_) => true,
        None    => false,
    }
}

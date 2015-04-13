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
    let ab = b - a;
    let ac = c - a;
    let cross = ab.y * ac.x - ab.x * ac.y;
    if cross.abs() < EPSILON {
        return Colinear;
    } else if cross < 0.0 {
        return Clockwise;
    } else {
        return CounterClockwise;
    }
}
#[test]
fn orientation_test() {
    let a = Point::new(0.0, 0.0);
    let b = Point::new(0.0, 1.0);
    let c = Point::new(1.0, 0.0);

    assert_eq!(orientation(a,b,c), CounterClockwise);
    assert_eq!(orientation(a,c,b), Clockwise);
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
    let mut leftmost_ix = -1;

    for &i in included {
        let p = points[i];
        if p.x <= leftmost.x {
            if p.x < leftmost.x || p.y < leftmost.y {
                leftmost = p;
                leftmost_ix = i;
            }
        }
    }
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
                    "With {base:?}, Swapping {end:?}({end_ix:?}) for {other:?}({other_ix:?}) \nas {base_ix:?} {end_ix:?} {other_ix:?} form a counterclockwise triange",
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
        Point::new(0.0, 1.0), // 1
        Point::new(1.0, 1.0), // 2
        Point::new(1.0, 0.0), // 3
        Point::new(0.5, 0.5), // 4
    ];
    let inpoints = vec![0,1,2,3,4];

    let hull = giftwrap(&points, &inpoints);
    assert_eq!(hull, vec![ 3, 2, 1, 0 ]);
}


/// The main workhorse function.
/// Finds the perfect hull, and radii for the blob.
pub fn find_hull(
            config: &Config,
            points: &Vec<Point>,
            inpoints: &Vec<Index>,
            expoints: &Vec<Index>,
            ) -> (Vec<Index>, Vec<Radius>) {

    let hull = giftwrap(&points, &inpoints);

    if config.run.fix_hull {
        // todo fix hull
    }
    if config.run.refine_poly {
        // todo refine poly
    }
    if config.run.rm_crossing {
        // todo remove crossing
    }
    let mut radii = vec![];
    radii.resize(points.len(), 1.);

    (hull, radii)
}

extern crate cairo;
use config::Config;
use types::{Point, Vector, Index, Color};

use self::cairo::surface::Surface;
use self::cairo::Cairo;

use std::f64;
use std::path::Path;

use tau::TAU;

pub fn draw(config: &Config,
            points: &Vec<Point>,
            hull: &Vec<Index>,
            hull_color: &Color,
            inpoints: &Vec<Index>,
            inpoints_color: &Color,
            expoints: &Vec<Index>,
            expoints_color: &Color,
            path: &Path,
            ) {
    use self::cairo::surface::format::Format;
    let mut surface = Surface::create_image(Format::ARGB32,
                                            config.img.width,
                                            config.img.height);

    let mut cr = Cairo::create(&mut surface);


    let scale = scale_world(&mut cr, config.img.edge_buffer, config.img.width,
                            config.img.height, &points);


    if config.draw.background {
        cr.set_source_rgba(1.,1.,1.,1.);
        cr.paint();
    }
    // Axis
    if config.draw.axis {
        cr.set_line_width(config.draw.axis_thickness / scale);
        cr.set_source_rgba(0.,0.,0.,0.5);
        cr.line_to(0.0,-100.0);
        cr.line_to(0.0,100.0);
        cr.stroke();
        cr.line_to(100.0,0.0);
        cr.line_to(-100.0,0.0);
        cr.stroke();
    }

    cr.set_line_width(config.draw.polygon_thickness / scale); // Replace 1 with line width.
    if config.draw.polygon {
        cr.set_source_rgba(hull_color.r, hull_color.g, hull_color.b, 0.8);
        draw_hull(&mut cr, &points, &hull);
    }

    if config.draw.points {
        cr.set_source_rgba(inpoints_color.r, inpoints_color.g, inpoints_color.b, 0.9);
        draw_points(&mut cr, points, inpoints, config.draw.point_radius / scale);

        cr.set_source_rgba(expoints_color.r, expoints_color.g, expoints_color.b, 0.9);
        draw_points(&mut cr, points, expoints, config.draw.point_radius / scale);
    }


    let filename = ascii_path_to_string(path).expect("Filename not ascii?!");
    surface.write_to_png(filename);

}

fn ascii_path_to_string(path: &Path) -> Option<&str> {
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




fn scale_world(cr: &mut Cairo,
        boundary: f64, img_width: i32, img_height: i32,
        points: &Vec<Point>) -> f64 {

    let mut maxx = f64::NEG_INFINITY;
    let mut maxy = f64::NEG_INFINITY;
    let mut minx = f64::INFINITY;
    let mut miny = f64::INFINITY;

    for p in points {
        maxx = maxx.max(p.x);
        maxy = maxy.max(p.y);
        minx = minx.min(p.x);
        miny = miny.min(p.y);
    }
    let user_width  :f64 = boundary * (maxx - minx);
    let user_height :f64 = boundary * (maxy - miny);

    let scalex :f64 = img_width  as f64/ user_width;
    let scaley :f64 = img_height as f64/ user_height;

    println!("Scaling by {},{}", scalex, scaley);

    cr.scale(scalex, -scaley);
    let offsetx = (user_width as f64 - user_width as f64/boundary) / 2. - minx;
    let offsety = -maxy * ((boundary - 1.) / 2. + 1.);
    cr.translate(offsetx, offsety);

    scalex
}

fn draw_hull(cr : &mut Cairo,
              points: &Vec<Point>,
              hull: &Vec<Index>) {
    cr.new_path();
    for &i in hull {
        cr.line_to(points[i].x, points[i].y);
    }
    cr.close_path();
    cr.stroke();
}

fn draw_points(cr: &mut Cairo,
               points: &Vec<Point>,
               indices: &Vec<Index>,
               radius: f64) {
    for &i in indices {
        cr.new_path();
        cr.arc(points[i].x, points[i].y, radius, 0., TAU);
        cr.fill();
    }
}

fn trace_blob(cr: &mut Cairo,
             points: &Vec<Point>,
             hull: &Vec<Index>,
             radii: &Vec<f64>) {
    cr.new_path();
    let mut previous_angle:f64;


}

fn smooth_line_normal(&a: &Point, a_r: &f64, a_inblob: bool,
                      &b: &Point, b_r: &f64, b_inblob: bool) -> Vector {
    use na::*;
    use std::ops::Sub;
    let mut u:Vector = b - a;
    let norm:f64 = u.normalize_mut();
    // u is now normalized
    let delta = if a_inblob == b_inblob {
        (a_r - b_r)/norm
    } else {
        (a_r + b_r)/norm
    };

    if a_inblob {
        Rot2::<f64>::new(Vec1::new(delta.acos())) * u
    } else {
        Rot2::<f64>::new(Vec1::new(TAU - delta.acos())) * u
    }
}


#[test]
fn test_smooth_line_normal() {
    use na;
    let a = Point::new(0.0, 0.0);
    let b = Point::new(1.0, 0.0);

    let up = smooth_line_normal(&a, &0.2, true, &b, &0.2, true);
    println!("Up is {:?}", up);
    assert!(na::approx_eq(&up, &Vector::new(0.0, 1.0)));

    // TODO  is this correct???
    let down = smooth_line_normal(&a, &0.2, false, &b, &0.2, false);
    println!("Down is {:?}", down);
    assert!(na::approx_eq(&down, &Vector::new(0.0, -1.0)));

    let uppish_leftish = smooth_line_normal(&a, &0.2, true, &b, &0.8, true);
    println!("Uppish leftish is {:?}", uppish_leftish);
    assert!(uppish_leftish.x < 0.);
    assert!(uppish_leftish.y > 0.);

    let uppish_rightish = smooth_line_normal(&a, &0.2, true, &b, &0.2, false);
    println!("uppish rightish is {:?}", uppish_rightish);
    assert!(uppish_rightish.x > 0.);
    assert!(uppish_rightish.y > 0.);
}





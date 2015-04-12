
extern crate cairo;
use types::{Point, Index};

use cairo::surface::Surface;
use cairo::Cairo;

use std::f64;

const TAU: f64 = 6.28318530718;

pub fn draw(img_width: i32, img_height: i32,
            points: &Vec<Point>,
            hull: &Vec<Index>,
            inpoints: &Vec<Index>,
            expoints: &Vec<Index>
            ) {
    use cairo::surface::format::Format;
    let mut surface = Surface::create_image(Format::ARGB32, img_width, img_height);

    let mut cr = Cairo::create(&mut surface);


    let scale = scale_world(&mut cr, 1.2, img_width, img_height, &points);


    cr.set_line_width(1. / scale); // Replace 1 with line width.
    // Axis
    cr.set_source_rgba(0.,0.,0.,0.5);
    cr.line_to(0.0,-10.0);
    cr.line_to(0.0,10.0);
    cr.stroke();
    cr.line_to(10.0,0.0);
    cr.line_to(-10.0,0.0);
    cr.stroke();

    cr.set_source_rgba(1.,0.3,0.0,0.8);
    draw_hull(&mut cr, &points, &hull);

    cr.set_source_rgba(1.0, 0.2, 0.2, 0.9);
    draw_points(&mut cr, points, inpoints, 0.01);


    surface.write_to_png("Test.png");

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

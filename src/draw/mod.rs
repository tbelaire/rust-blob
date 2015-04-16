// This is the mount point for cairo.
extern crate cairo;
use draw::cairo::surface::Surface;
use draw::cairo::Cairo;

// Our helper functions live a in util.
mod util;
use self::util::*;

// For all the parameters, we look to Config.
use config::Config;

// We need our types.
use types::{Point, Vector, Index, Color, Radius};

use std::f64;
use std::path::Path;

use tau::TAU;


/// This file deals with all the direct calls to cairo things.
/// Helper utility functions will be in draw::util.

pub fn draw(config: &Config,
            points: &Vec<Point>,
            hull: &Vec<Index>,
            hull_color: &Color,
            inpoints: &Vec<Index>,
            inpoints_color: &Color,
            expoints: &Vec<Index>,
            expoints_color: &Color,
            inblob: &Vec<bool>,
            radii: &Vec<Radius>,
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

    if config.draw.polygon {
        cr.set_line_width(config.draw.polygon_thickness / scale);
        cr.set_source_rgba(hull_color.r, hull_color.g, hull_color.b, 0.8);
        draw_hull(&mut cr, &points, &hull);
    }

    if config.draw.labels {
        cr.set_source_rgba(0.,0.,0.,1.);
        cr.select_font_face("Sans", cairo::font::slant::Slant::Normal,
                            cairo::font::weight::Weight::Bold);
        cr.set_font_size(config.draw.fontsize/scale);
        label_points(&mut cr, &points, &hull);
    }

    if config.draw.blob {
        cr.set_line_width(config.draw.polygon_thickness / scale);
        cr.set_source_rgba(hull_color.r, hull_color.g, hull_color.b, 0.4);
        trace_blob(&mut cr, &points, &hull, &inblob, &radii);
        cr.fill_preserve();
        cr.set_source_rgba(hull_color.r, hull_color.g, hull_color.b, 0.8);
        cr.stroke();
    }


    if config.draw.points {
        cr.set_source_rgba(inpoints_color.r, inpoints_color.g, inpoints_color.b, 0.9);
        draw_points(&mut cr, points, inpoints, config.draw.point_radius / scale);

        cr.set_source_rgba(expoints_color.r, expoints_color.g, expoints_color.b, 0.9);
        draw_points(&mut cr, points, expoints, config.draw.point_radius / scale);
    }


    let filename = util::ascii_path_to_string(path).expect("Filename not ascii?!");
    surface.write_to_png(filename);

}


/// This adjusts cairo's user transform such that drawing the points
/// in their co-ordinate system puts them in the middle of the image.
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

    debug!("Scaling by {},{}", scalex, scaley);

    cr.scale(scalex, scaley);
    let offsetx = (user_width as f64 - user_width as f64/boundary) / 2. - minx;
    let offsety = (user_height as f64 - user_height as f64/boundary) / 2. - miny;
    // let offsety = -1.*(-maxy * ((boundary - 1.) / 2. + 1.));
    cr.translate(offsetx, offsety);

    scalex
}

/// This traces a polygon with straight lines to cairo
/// It does not set any cairo options, like fill color or line width.
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

fn label_points(cr: &mut Cairo,
                points: &Vec<Point>,
                indices: &Vec<Index>) {
    for (i,&ix) in indices.iter().enumerate() {
        cr.move_to(points[ix].x, points[ix].y);
        cr.show_text(&format!(" {}", i));
    }
}

fn trace_blob(cr: &mut Cairo,
             points: &Vec<Point>,
             hull: &Vec<Index>,
             inblob: &Vec<bool>,
             radii: &Vec<f64>) {
    cr.new_path();
    let last_ix = hull[hull.len() - 1];
    let first_ix = hull[0];
    let (_, mut previous_angle) = smooth_line_angle(
        &points[last_ix], radii[last_ix], inblob[last_ix],
        &points[first_ix], radii[first_ix], inblob[first_ix]);

    for hull_ix in 0..hull.len() {
        let i:      Index = hull[hull_ix];
        let next_i: Index = hull[(hull_ix + 1) % hull.len()];
        let a = points[i];
        let b = points[next_i];
        trace!("i: {}, {}, a:{:?}, b:{:?}", i, next_i, a, b);

        let a_r = radii[i];
        let b_r = radii[next_i];

        let a_inblob = inblob[i];
        let b_inblob = inblob[next_i];

        trace!("a_inblob: {}, b_inblob: {}", a_inblob, b_inblob);
        let (a_ang, b_ang) = smooth_line_angle(&a, a_r, a_inblob,
                                               &b, b_r, b_inblob);
        trace!("a_ang: {}, b_ang: {}", a_ang.to_degrees(), b_ang.to_degrees());
        if a_inblob {
            debug!("cr.arc_negative({}, {}, {}, {}, {})", a.x, a.y, a_r, previous_angle.to_degrees(), a_ang.to_degrees());
            cr.arc_negative(a.x, a.y, a_r, previous_angle, a_ang);
        } else {
            debug!("cr.arc({}, {}, {}, {}, {})", a.x, a.y, a_r, previous_angle.to_degrees(), a_ang.to_degrees());
            cr.arc(a.x, a.y, a_r, previous_angle, a_ang);
        }
        previous_angle = b_ang;
    }
    cr.close_path();
}


use na::{Vec2,Vec1,Pnt2,Rot2};

pub type Point = Pnt2<f64>;

pub struct SPoint {
    p: Point,
    radius: Radius,
    in_blob: bool,
}

pub type Vector = Vec2<f64>;

pub type Rotation = Rot2<f64>;
pub fn rotate_ccw(theta: f64, v: Vector) -> Vector {
    Rot2::<f64>::new(Vec1::new(theta)) * v
}

pub type Index = usize;
pub type Radius = f64;

pub type Comb = Vec<Vec<Index>>;

#[derive(Clone, Debug, PartialEq)]
pub struct Color {
    pub r:f64,
    pub g:f64,
    pub b:f64
}

impl Color {
    pub fn new(r:f64, g:f64, b:f64) -> Color {
        Color{r:r, g:g, b:b}
    }
    pub fn from_hex(hex: &str) -> Color {
        use rustc_serialize::hex::FromHex;
        if hex.len() != 6 {
            panic!("Wrong length hex code");
        }
        let digits = hex.from_hex().unwrap();
        let r:f64 = digits[0] as f64;
        let g:f64 = digits[1] as f64;
        let b:f64 = digits[2] as f64;
        Color::new(r/255., g/255., b/255.)
    }
}

#[test]
fn test_hex_code() {
    let c = Color::from_hex("ff0055");
    assert_eq!(c, Color::new(255./255., 0., 0x55 as f64/255.));
}


impl SPoint {
    fn new(p: Point, r: Radius, inb: bool) -> SPoint {
        SPoint{ p:p, radius:r, in_blob:inb }
    }
}

use std::fmt::{Display, Formatter};
use std::fmt;
impl Display for SPoint {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result{
        if self.in_blob {
            Display::fmt(&format!("({},{};{})", self.p.x, self.p.y, self.radius), fmt)
        } else {
            Display::fmt(&format!("<{},{};{}>", self.p.x, self.p.y, self.radius), fmt)
        }
    }
}

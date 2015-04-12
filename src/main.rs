extern crate cairo;
extern crate toml;

mod draw;
mod types;
use types::Point;

extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

// Write the Docopt usage string.
static USAGE: &'static str = "
Usage: ./draw [-d] <points-file> <sets-file> <dest-prefix>

Options:
    -d, --output-directories  Output a forlder for each comb.
";

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_points_file: String,
    arg_sets_file: String,
    arg_dest_prefix: String,
    flag_output_directories: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);


    let points =vec![
                Point{ x: 0.0, y: 0.0 },
                Point{ x: 0.0, y: 1.0 },
                Point{ x: 1.0, y: 1.0 },
                Point{ x: 1.0, y: 0.0 }];
    let hull = vec![0,1,2,3];
    let inpoints = vec![0,2];
    let expoints = vec![1,3];
    draw::draw( 400, 400, &points, &hull, &inpoints, &expoints );
}

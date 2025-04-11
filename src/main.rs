mod mandelbrot;
use mandelbrot::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: mandelbrot FILE PIXELS UPPERLEFT LOWERRIGHT");
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
            args[0]
        );
        std::process::exit(1);
    }

    let bounds = cli::parse_pair(&args[2], 'X').expect("error parsing image dimensions");
    let upper_left = cli::parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = cli::parse_complex(&args[4]).expect("error parsing lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    generator::render(&mut pixels, bounds, upper_left, lower_right);

    output::write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}

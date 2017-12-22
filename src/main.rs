extern crate clap;

use clap::App;

mod polydiv;

fn main() {
    // Setup command-line interface (CLI)
    let cli_args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("Perform polynomial division, by providing the coefficients
            of two polynomials.")
        .author("John B. <johnboydiv@gmail.com>")
        .get_matches();

    polydiv::polynomial_division();
}

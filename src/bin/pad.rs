use std::io::{self, Read};
use std::fs::File;

#[macro_use]
extern crate clap;

extern crate failure;
use failure::Error;

extern crate xpad;

// our arguments are optional: we really don't care if they're not present,
// but we want to abort if there are other errors
macro_rules! optional {
    ($val:expr) => {
        match $val {
            Ok(d) => Some(d),
            Err(clap::Error{kind: clap::ErrorKind::ArgumentNotFound, ..}) => None,
            Err(e) => e.exit(),
        }
    }
}

fn do_pad() -> Result<(), Error> {
    let matches = clap_app!(pad =>
        (version: crate_version!())
        (author: crate_authors!("\n"))
        (about: crate_description!())
        (@arg file: -f --file +takes_value "Read from the named file instead of stdin.")
        (@arg delim: -d --delimiter +takes_value "Specify the delimiter with which to distinguish input fields.")
        (@arg sep: -s --separator +takes_value "Specify the separator with which to separate output fields.")
        (@arg align: -a --align ... +takes_value "Specify a column's alignment. May be repeated. Use 'l', 'r', 'c' for left, right, center alignment.")
    ).get_matches();


    let filename = optional!(value_t!(matches, "file", String));
    let delim = optional!(value_t!(matches, "delim", char)).unwrap_or(' ');
    let sep = optional!(value_t!(matches, "sep", char)).unwrap_or(' ');
    let aligns = optional!(values_t!(matches, "align", xpad::Alignment)).unwrap_or(Vec::new());

    // read the input into a string
    // we need to keep track of the entire thing until EOF, so just bite the bullet
    // and keep it all
    let mut input = String::new();
    match filename {
        Some(filename) => {
            File::open(filename)?.read_to_string(&mut input)?;
        }
        None => {
            io::stdin().read_to_string(&mut input)?;
        }
    }

    let document = xpad::parse_document(&input, delim);
    let oc = xpad::configure_output(&document, &aligns, sep);
    xpad::print(&document, &oc)?;
    Ok(())
}

fn main() {
    ::std::process::exit(match do_pad() {
       Ok(_) => 0,
       Err(err) => {
           eprintln!("error: {}", err);
           1
       }
    });
}
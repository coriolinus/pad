#[macro_use]
extern crate clap;

extern crate pad;

fn main() {
    let matches = clap_app!(pad =>
        (version: crate_version!())
        (author: crate_authors!("\n"))
        (about: crate_description!())
        (@arg file: -f --file +takes_value "Read from the named file instead of stdin")
        (@arg delim: -d --delimiter +takes_value "Specify the delimiter with which to distinguish input fields")
        (@arg sep: -s --separator +takes_value "Specify the separator with which to separate output fields")
        (@arg align: -a --align ... +takes_value "Specify a column's alignment. May be repeated.")
    ).get_matches();

    println!("{:?}", matches);
}
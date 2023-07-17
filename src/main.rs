use clap::builder::{Arg, Command};
use clap::{crate_authors, crate_description, crate_name, crate_version};

fn main() {
    let matches = Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about(crate_description!())
        .help_template(
            "\
{before-help}{name} {version}
    {author-with-newline}{about-with-newline}
{usage-heading} {usage}
{all-args} {after-help}
",
        )
        .arg(
            Arg::new("test")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .num_args(0),
        )
        .get_matches();

    println!("{:#?}", matches);
}

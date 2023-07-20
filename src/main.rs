use clap::builder::{Arg, Command};
use clap::{crate_authors, crate_description, crate_name, crate_version, ArgAction, ArgMatches};

fn main() {
    let matches: ArgMatches = Command::new(crate_name!())
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
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .num_args(0)
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let text: Vec<&str> = matches
        .get_many::<String>("text")
        .expect("Must contains text argument")
        .map(|s| s.as_str())
        .collect();

    // If the -n flag is passed, we do not print new line
    print!(
        "{}{}",
        text.join(" "),
        if matches.get_flag("omit_newline") {
            ""
        } else {
            "\n"
        }
    );
}

use clap::{value_parser, Arg, ArgMatches, Command};

pub fn parse() -> ArgMatches {
    return Command::new("gi")
        .arg(
            Arg::new("issue")
                .value_parser(value_parser!(u64))
                .required(false),
        )
        .get_matches();
}

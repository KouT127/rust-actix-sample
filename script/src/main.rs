use clap::{App, Arg, ArgMatches, SubCommand};

fn main() {
    let app = App::new("practice")
        .version("0.1.0")
        .author("")
        .about("actix sample commands")
        .subcommand(
            SubCommand::with_name("echo")
                .about("echo argument")
                .arg(
                    Arg::with_name("dryrun") // フラグを定義
                        .help("dry run")
                        .long("dryrun")
                        .short("d"),
                )
                .arg(
                    Arg::with_name("path")
                        .short("p")
                        .long("path")
                        .takes_value(true)
                        .required(true),
                ),
        );

    let matches = app.get_matches();
    if let Some(sub_matches) = matches.subcommand_matches("echo") {
        handle_echo_command(sub_matches);
    }
}

fn handle_echo_command(matches: &ArgMatches) {
    let dry_run = matches.is_present("dryrun");
    if dry_run {
        println!("Dry run execute");
    }

    if matches.value_of("path").is_none() {
        println!("Path option is none");
    };
    let path = matches.value_of("path").unwrap();
    println!("target path: {} dryrun: {}", path, dry_run)
}

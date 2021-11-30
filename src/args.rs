use clap::{App, Arg, SubCommand};

pub fn get_app() -> clap::App<'static, 'static> {
    App::new("catbox")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            SubCommand::with_name("upload")
                .about("Upload to Catbox. Max size 200MB.")
                .arg(
                    Arg::with_name("user hash")
                        .long("user")
                        .short("u")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("files")
                        .required(true)
                        .takes_value(true)
                        .multiple(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("Delete files")
                .arg(
                    Arg::with_name("user hash")
                        .long("user")
                        .short("u")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("files")
                        .required(true)
                        .takes_value(true)
                        .multiple(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("album")
                .about("Album commands")
                .subcommand(
                    SubCommand::with_name("create")
                        .about("Create a new album")
                        .arg(
                            Arg::with_name("title")
                                .long("title")
                                .short("t")
                                .takes_value(true),
                        )
                        .arg(
                            Arg::with_name("description")
                                .long("desc")
                                .short("d")
                                .takes_value(true),
                        )
                        .arg(
                            Arg::with_name("user hash")
                                .long("user")
                                .short("u")
                                .takes_value(true),
                        )
                        .arg(
                            Arg::with_name("files")
                                .required(true)
                                .takes_value(true)
                                .multiple(true),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("edit")
                        .about("Edit an album")
                        .arg(
                            Arg::with_name("short")
                                .long("short")
                                .short("s")
                                .required(true)
                                .takes_value(true),
                        )
                        .arg(
                            Arg::with_name("title")
                                .long("title")
                                .short("t")
                                .takes_value(true),
                        )
                        .arg(
                            Arg::with_name("description")
                                .long("desc")
                                .short("d")
                                .takes_value(true),
                        )
                        .arg(
                            Arg::with_name("user hash")
                                .long("user")
                                .short("u")
                                .takes_value(true),
                        )
                        .arg(
                            Arg::with_name("files")
                                .required(true)
                                .takes_value(true)
                                .multiple(true),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("add")
                        .about("Add files to an album")
                        .arg(
                            Arg::with_name("short")
                                .long("short")
                                .short("s")
                                .required(true)
                                .takes_value(true),
                        )
                        .arg(
                            Arg::with_name("user hash")
                                .long("user")
                                .short("u")
                                .takes_value(true),
                        )
                        .arg(
                            Arg::with_name("files")
                                .required(true)
                                .takes_value(true)
                                .multiple(true),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("remove")
                        .about("Remove files from an album")
                        .arg(
                            Arg::with_name("short")
                                .long("short")
                                .short("s")
                                .required(true)
                                .takes_value(true),
                        )
                        .arg(
                            Arg::with_name("user hash")
                                .long("user")
                                .short("u")
                                .takes_value(true),
                        )
                        .arg(
                            Arg::with_name("files")
                                .required(true)
                                .takes_value(true)
                                .multiple(true),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("delete")
                        .about("Delete an album")
                        .arg(Arg::with_name("short").required(true).takes_value(true))
                        .arg(
                            Arg::with_name("user hash")
                                .long("user")
                                .short("u")
                                .takes_value(true),
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("litter")
                .about("Upload a temporary file to Litterbox. Max size 1GB.")
                .arg(
                    Arg::with_name("files")
                        .required(true)
                        .takes_value(true)
                        .multiple(true),
                )
                .arg(
                    Arg::with_name("time")
                        .long("time")
                        .short("t")
                        .required(true)
                        .possible_values(&["1h", "12h", "24h", "72h"])
                        .default_value("1h"),
                ),
        )
}

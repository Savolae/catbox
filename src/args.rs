use clap::{Arg, Command};

pub fn get_app() -> Command<'static> {
    Command::new("catbox")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            Command::new("upload")
                .about("Upload to Catbox. Max size 200MB.")
                .arg(
                    Arg::new("user hash")
                        .long("user")
                        .short('u')
                        .takes_value(true),
                )
                .arg(
                    Arg::new("files")
                        .required(true)
                        .takes_value(true)
                        .multiple_values(true),
                ),
        )
        .subcommand(
            Command::new("delete")
                .about("Delete files")
                .arg(
                    Arg::new("user hash")
                        .long("user")
                        .short('u')
                        .takes_value(true),
                )
                .arg(
                    Arg::new("files")
                        .required(true)
                        .takes_value(true)
                        .multiple_values(true),
                ),
        )
        .subcommand(
            Command::new("album")
                .about("Album commands")
                .subcommand(
                    Command::new("create")
                        .about("Create a new album")
                        .arg(Arg::new("title").long("title").short('t').takes_value(true))
                        .arg(
                            Arg::new("description")
                                .long("desc")
                                .short('d')
                                .takes_value(true),
                        )
                        .arg(
                            Arg::new("user hash")
                                .long("user")
                                .short('u')
                                .takes_value(true),
                        )
                        .arg(
                            Arg::new("files")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true),
                        ),
                )
                .subcommand(
                    Command::new("edit")
                        .about("Edit an album")
                        .arg(
                            Arg::new("short")
                                .long("short")
                                .short('s')
                                .required(true)
                                .takes_value(true),
                        )
                        .arg(Arg::new("title").long("title").short('t').takes_value(true))
                        .arg(
                            Arg::new("description")
                                .long("desc")
                                .short('d')
                                .takes_value(true),
                        )
                        .arg(
                            Arg::new("user hash")
                                .long("user")
                                .short('u')
                                .takes_value(true),
                        )
                        .arg(
                            Arg::new("files")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true),
                        ),
                )
                .subcommand(
                    Command::new("add")
                        .about("Add files to an album")
                        .arg(
                            Arg::new("short")
                                .long("short")
                                .short('s')
                                .required(true)
                                .takes_value(true),
                        )
                        .arg(
                            Arg::new("user hash")
                                .long("user")
                                .short('u')
                                .takes_value(true),
                        )
                        .arg(
                            Arg::new("files")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true),
                        ),
                )
                .subcommand(
                    Command::new("remove")
                        .about("Remove files from an album")
                        .arg(
                            Arg::new("short")
                                .long("short")
                                .short('s')
                                .required(true)
                                .takes_value(true),
                        )
                        .arg(
                            Arg::new("user hash")
                                .long("user")
                                .short('u')
                                .takes_value(true),
                        )
                        .arg(
                            Arg::new("files")
                                .required(true)
                                .takes_value(true)
                                .multiple_values(true),
                        ),
                )
                .subcommand(
                    Command::new("delete")
                        .about("Delete an album")
                        .arg(Arg::new("short").required(true).takes_value(true))
                        .arg(
                            Arg::new("user hash")
                                .long("user")
                                .short('u')
                                .takes_value(true),
                        ),
                ),
        )
        .subcommand(
            Command::new("litter")
                .about("Upload a temporary file to Litterbox. Max size 1GB.")
                .arg(
                    Arg::new("files")
                        .required(true)
                        .takes_value(true)
                        .multiple_values(true),
                )
                .arg(
                    Arg::new("time")
                        .long("time")
                        .short('t')
                        .required(true)
                        .possible_values(&["1h", "12h", "24h", "72h"]),
                ),
        )
}

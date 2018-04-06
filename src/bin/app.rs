use clap::{App, Arg};

pub fn build_app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("headers")
                .short("H")
                .long("header")
                .help("Adds headers to any HTTP request made")
                .takes_value(true)
                .value_name("header:value")
                .number_of_values(1)
                .multiple(true),
        )
        .arg(
            Arg::with_name("head")
                .short("I")
                .long("head")
                .help("Prints HTTP headers"),
        )
        .arg(
            Arg::with_name("show_control_frames")
                .short("C")
                .long("show-control-frames")
                .help("Enables echoing of control frames"),
        )
        .arg(
            Arg::with_name("echo")
                .short("e")
                .long("echo")
                .help("Enables echoing of outgoing frames"),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .multiple(true)
                .help("Increments verbosity by one"),
        )
        .arg(
            Arg::with_name("silent")
                .short("s")
                .long("silent")
                .help("Supresses all output except incoming frames"),
        )
        .arg(
            Arg::with_name("url")
                .required(true)
                .index(1)
                .value_name("URL")
                .help("The URL of the server to connect to"),
        )
}

/// Return the "long" format of wurl's version string.
///
/// If a revision hash is given, then it is used. If one isn't given, then
/// the WURL_BUILD_GIT_HASH env var is inspect for it. If that isn't set,
/// then a revision hash is not included in the version string returned.
#[allow(dead_code)]
pub fn long_version(revision_hash: Option<&str>) -> String {
    let mut features = vec![];
    if cfg!(feature = "ssl") {
        features.push("+SSL");
    } else {
        features.push("-SSL");
    }
    // Do we have a git hash?
    // (Yes, if url was built on a machine with `git` installed.)
    let hash = match revision_hash.or(option_env!("WURL_BUILD_GIT_HASH")) {
        None => String::new(),
        Some(githash) => format!(" (rev {})", githash),
    };
    // Put everything together.
    format!("{}{}\n{}", crate_version!(), hash, features.join(" "))
}

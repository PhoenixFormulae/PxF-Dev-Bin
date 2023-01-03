// Relative Modules
pub(crate) mod interpreters;
pub(crate) mod renderers;

// Standard Uses

// Crate Uses
use crate::interpreters::python;

// External Uses
use clap::Parser;
use log::{debug, error, log_enabled, info, Level};
use env_logger::{Builder, Env};



const APP_NAME: &str = "PhoenixFormulae Development Binary";
const APP_DESC: &str = "Binary with interpreter and UI rendering capabilities for development";


fn main() {
    init_logger();

    debug!("{}", APP_NAME);
    debug!("{}", APP_DESC);

    let args = Args::parse();

    if args.python {
        info!("Running as a python interpreter");
        // interpreters::init();
        python::create_interpreter().expect("Could not create python interpreter");
    }
    else {
        renderers::create();
    }

    info!("Initialized all foundations")
}


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = APP_DESC)]
struct Args {
    /// Runs the application as a python interpreter
    #[arg(short, long, default_value_t = false)]
    python: bool,
}


fn init_logger() {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    Builder::from_env(env)
        .format_level(true)
        .init();
}


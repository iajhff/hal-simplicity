use std::{panic, process};

pub use elements::bitcoin;
use hal_simplicity::cmd;
pub use hal_simplicity::{GetInfo, Network};

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use shell_words;
use log;
use fern;
use clap::{App, AppSettings, ArgMatches};

/// Setup logging with the given log level.
fn setup_logger(lvl: log::LevelFilter) {
    fern::Dispatch::new()
        .format(|out, message, _record| out.finish(format_args!("{}", message)))
        .level(lvl)
        .chain(std::io::stderr())
        .apply()
        .expect("error setting up logger");
}

/// Create the main app object (Clap 2.32-Syntax).
fn init_app<'a, 'b>() -> App<'a, 'b> {
    App::new("hal-simplicity")
        .bin_name("hal-simplicity")
        .version(clap::crate_version!())
        .about("hal-simplicity -- a Simplicity-enabled fork of hal")
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::AllArgsOverrideSelf)
        .subcommands(cmd::subcommands())
        .arg(
            cmd::opt("verbose", "print verbose logging output to stderr")
                .short("v")
                .takes_value(false)
                .global(true),
        )
}

/// Try to execute built-in command.  
/// Returns `false` if no command found.
fn execute_builtin<'a>(matches: &ArgMatches<'a>) -> bool {
    match matches.subcommand() {
        ("address", Some(m)) => cmd::address::execute(m),
        ("block", Some(m)) => cmd::block::execute(m),
        ("keypair", Some(m)) => cmd::keypair::execute(m),
        ("simplicity", Some(m)) => cmd::simplicity::execute(m),
        ("tx", Some(m)) => cmd::tx::execute(m),
        _ => return false,
    };
    true
}

/// Führt ein CLI-Kommando aus (übergeben als String).
#[pyfunction]
fn run_cli_command(cmdline: &str) -> PyResult<()> {
    // Panic-Hook für schönere Fehlermeldungen
    panic::set_hook(Box::new(|info| {
        let message = if let Some(m) = info.payload().downcast_ref::<String>() {
            m.as_str()
        } else if let Some(m) = info.payload().downcast_ref::<&str>() {
            m
        } else {
            "No error message provided"
        };
        eprintln!("Execution failed: {}", message);
        process::exit(1);
    }));

    // Kommandozeilen-String in Argumente aufsplitten
    let args = shell_words::split(cmdline).map_err(|e| {
        pyo3::exceptions::PyValueError::new_err(format!("failed to parse command line: {}", e))
    })?;

    let app = init_app();
    let matches = app.get_matches_from(args);

    // Logging konfigurieren
    if matches.is_present("verbose") {
        setup_logger(log::LevelFilter::Trace);
    } else {
        setup_logger(log::LevelFilter::Warn);
    }

    // Ausführen
    if execute_builtin(&matches) {
        Ok(())
    } else {
        Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
            "Subcommand not found: {}",
            matches.subcommand().0
        )))
    }
}

/// Hauptmodul-Definition (PyO3 0.25-Syntax).
#[pymodule]
fn hal_simplicity_py(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_cli_command, m)?)?;
    Ok(())
}

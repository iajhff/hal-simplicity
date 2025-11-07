use std::panic;

pub use elements::bitcoin;
use hal_simplicity::cmd;
pub use hal_simplicity::{GetInfo, Network};

use clap::{App, AppSettings, ArgMatches};
use fern;
use log;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use shell_words;
use std::sync::{Arc, Mutex, OnceLock};
use std::fmt::Write as FmtWrite;


static LOG_BUFFER: OnceLock<Arc<Mutex<String>>> = OnceLock::new();

#[pyfunction]
pub fn setup_logger(level: &str) -> PyResult<()> {
	let lvl = match level.to_lowercase().as_str() {
		"off" => log::LevelFilter::Off,
		"error" => log::LevelFilter::Error,
		"warn" | "warning" => log::LevelFilter::Warn,
		"info" => log::LevelFilter::Info,
		"debug" => log::LevelFilter::Debug,
		"trace" => log::LevelFilter::Trace,
		_ => {
			return Err(pyo3::exceptions::PyValueError::new_err(format!(
				"invalid log level: {}",
				level
			)));
		}
	};

	let buf = Arc::new(Mutex::new(String::new()));
	let _ = LOG_BUFFER.set(buf.clone());

	fern::Dispatch::new()
		.format(move |out, message, record| {
			let mut b = buf.lock().unwrap();
			writeln!(b, "[{}] {}", record.level(), message).unwrap();
			out.finish(format_args!(""))
		})
		.level(lvl)
		.chain(fern::Output::call(|_| {}))
		.apply()
		.map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
}

#[pyfunction]
pub fn get_logs() -> PyResult<String> {
	if let Some(buf) = LOG_BUFFER.get() {
		Ok(buf.lock().unwrap().clone())
	} else {
		Ok("Logger not initialized".to_string())
	}
}

// this is a simple testing function
#[pyfunction]
pub fn do_something() -> PyResult<()> {
	log::info!("Start do_something()");
	log::debug!("something happens…");

	if 1 + 1 != 2 {
		log::error!("math is broken!");
	}

	Ok(())
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

#[pyfunction]
fn run_cli_command(cmdline: &str) -> PyResult<String> {
	let result = panic::catch_unwind(|| {
		// argv[0] must exist
		let mut args = vec!["".to_string()];
		args.extend(
			shell_words::split(cmdline)
				.map_err(|e| format!("failed to parse command line: {}", e))?,
		);

		// log::info!("Parsed args: {:?}", args);
		if args.iter().any(|a| a == "-V" || a == "--version") {
		    return Ok(clap::crate_version!().to_string());  // return actual version
		}

		let app = init_app();

		let matches = app
			.get_matches_from_safe(args)
			.map_err(|e| format!("Argument parsing failed: {}", e))?;

		if execute_builtin(&matches) {
			Ok("Command executed successfully".to_string())
		} else {
			Err(format!("Subcommand not found: {}", matches.subcommand_name().unwrap_or("")))
		}
	});

	match result {
		Ok(inner) => match inner {
			Ok(output) => Ok(output),
			Err(err_msg) => Ok(format!("Execution failed: {}", err_msg)),
		},
		Err(panic_info) => {
			let msg = if let Some(s) = panic_info.downcast_ref::<String>() {
				s.clone()
			} else if let Some(s) = panic_info.downcast_ref::<&str>() {
				s.to_string()
			} else {
				"Unknown panic".to_string()
			};
			Ok(format!("Execution panicked: {}", msg))
		}
	}
}

/// Hauptmodul-Definition (PyO3 0.25-Syntax).
#[pymodule]
fn hal_simplicity_py(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
	m.add_function(wrap_pyfunction!(run_cli_command, m)?)?;
	m.add_function(wrap_pyfunction!(do_something, m)?)?;
	m.add_function(wrap_pyfunction!(setup_logger, m)?)?;
	m.add_function(wrap_pyfunction!(get_logs, m)?)?;
	Ok(())
}

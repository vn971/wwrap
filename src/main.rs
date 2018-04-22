#!/usr/bin/env run-cargo-script

use std::process::Command;
use std::env;
use std::collections::HashSet;
use std::ops::Deref;
use std::ffi::{OsString};
use std::os::unix::process::CommandExt;

fn restore_env(cmd: &mut Command, env_var: &str) {
	if let Some(old) = env::var_os(OsString::from(env_var)) {
		cmd.env(env_var, old);
	}
}

fn main() {
	let exclude_args: HashSet<_> = [
		"--ok-net",
		"--ok-ipc",
		"--ok-user",
		"--ok-pid",
		"--ok-uts",
		"--ok-cgroup",
		"--ok-parent",
		"--ok-session",
	].iter().cloned().collect();
	let cleaned_args: Vec<_> = env::args().skip(1).filter(
		|x| !exclude_args.contains(x.deref())
	).collect();
	eprintln!("Running command {:?} {:?}", env::args().nth(1).unwrap(), cleaned_args);

//	let mut command = Command::new(env::args().nth(1).unwrap());
	let mut command = Command::new("bwrap");
	let command: &mut Command = command.args(cleaned_args);

	let arg_set: HashSet<_> = env::args().collect();
	if !arg_set.contains("--ok-net") {
		command.arg("--unshare-net");
	}
	if !arg_set.contains("--ok-ipc") {
		command.arg("--unshare-ipc");
	}
	if !arg_set.contains("--ok-user") {
		command.arg("--unshare-user");
	}
	if !arg_set.contains("--ok-pid") {
		command.arg("--unshare-pid");
	}
	if !arg_set.contains("--ok-uts") {
		command.arg("--unshare-uts");
	}
	if !arg_set.contains("--ok-cgroup") {
		command.arg("--unshare-cgroup");
	}
	if !arg_set.contains("--ok-parent") {
		command.arg("--die-with-parent");
	}
	if !arg_set.contains("--ok-session") {
		command.arg("--new-session");
	}

	command.env_clear();
	restore_env(command, "PATH");

	command.exec();
	eprintln!("ERROR: Command not found");
	std::process::exit(1);
}

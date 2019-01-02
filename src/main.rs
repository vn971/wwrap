#!/usr/bin/env run-cargo-script

use std::borrow::BorrowMut;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::ops::Deref;
use std::os::unix::io::IntoRawFd;
use std::os::unix::process::CommandExt;
use std::process::Command;

extern crate libc;
use libc::{ fcntl, F_GETFD, FD_CLOEXEC, F_SETFD };


fn set_no_cloexec(file_descriptor: i32) {
	let flags = unsafe { fcntl(file_descriptor, F_GETFD) };
	if flags == -1 {
		panic!("cannot get seccomp fd flags");
	}
	let flags = flags & !FD_CLOEXEC;
	if unsafe { fcntl(file_descriptor, F_SETFD, flags) } == -1 {
		panic!("cannot set seccomp fd flags");
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
		"--ok-seccomp",
		"--ok-all-env", // compat with old versions of wwrap
	].iter().cloned().collect();
	let cleaned_args: Vec<_> = env::args().skip(1).filter(
		|x| !exclude_args.contains(x.deref())
	).collect();

	let mut command = Command::new(env::var("bwrap_path").unwrap_or("bwrap".to_string()));
	let command: &mut Command = command.borrow_mut();

	let arg_set: HashSet<_> = env::args().collect();
	if !arg_set.contains("--ok-net") {
		command.arg("--unshare-net");
	}
	if !arg_set.contains("--ok-ipc") {
		command.arg("--unshare-ipc");
	}
	if !arg_set.contains("--ok-user") && !arg_set.contains("--unshare-user-try") {
		command.arg("--unshare-user");
	}
	if !arg_set.contains("--ok-pid") {
		command.arg("--unshare-pid");
	}
	if !arg_set.contains("--ok-uts") {
		command.arg("--unshare-uts");
	}
	if !arg_set.contains("--ok-cgroup") && !arg_set.contains("--unshare-cgroup-try") {
		command.arg("--unshare-cgroup");
	}
	if !arg_set.contains("--ok-parent") {
		command.arg("--die-with-parent");
	}
	if !arg_set.contains("--ok-session") && arg_set.contains("--ok-seccomp") {
		command.arg("--new-session");
	}
	if !arg_set.contains("--seccomp") && !arg_set.contains("--ok-seccomp") {
		let file = File::open("/home/vasya/.jails/seccomp.bpf").unwrap();
		let file_descriptor = file.into_raw_fd();
		set_no_cloexec(file_descriptor);
		command.arg("--seccomp");
		command.arg(file_descriptor.to_string());
	}
	command.args(cleaned_args);

	let fork_error: std::io::Error = command.exec();
	eprintln!("ERROR running bwrap: {}", fork_error.to_string());
	std::process::exit(1);
}

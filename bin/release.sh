#!/bin/bash -euET
{

rustup update

cargo upgrade
cargo update

cargo fmt --all --
cargo clippy --all-targets --all-features -- -D warnings

if ! test -z "$(git status --porcelain)"; then
  >&2 printf '%s\n' "error: uncommitted changes"
  exit 1
fi

cargo build --release --target=x86_64-unknown-linux-musl
cp -a target/x86_64-unknown-linux-musl/release/wwrap ~/bin/wwrap

exit
}

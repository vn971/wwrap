#!/bin/bash -euET
{
cargo build --release
cp -a target/release/wwrap ~/bin/wwrap

exit
}
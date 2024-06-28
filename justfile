#!/usr/bin/env -S just --justfile

set dotenv-load := true
set dotenv-path := ".env"

alias i := install-tools
alias v := versions
alias f := fmt
alias fc := fmt-check
alias l := lint
alias t := test
alias w := watch
alias d := dev
alias r := run

@default:
  just --list

versions:
  rustc --version
  cargo --version

install-tools:
  cargo install cargo-binstall
  cargo binstall taplo-cli cargo-insta cargo-nextest cargo-outdated cargo-edit cargo-deny bacon
  rustup component add rustfmt
  rustup component add clippy

fmt:
  taplo format
  cargo fmt --all

fmt-check:
  taplo format --check
  cargo fmt --all -- --check

lint:
  cargo clippy 
  cargo deny check

test args="":
  cargo nextest run {{args}}

watch:
  bacon

dev:
  RUST_BACKTRACE=1 RUST_LOG=$RUST_LOG cargo run 

run:
  cargo run --release

ci: lint fmt-check test

deny:
  cargo deny check

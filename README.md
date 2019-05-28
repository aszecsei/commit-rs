#  commit-rs

> A rust script to enable automated git commits according to conventional commits

## Purpose

The merits of having a standardized commit message type have been discussed elsewhere. Rather than use a NodeJS solution, such as the commitizen CLI, I wanted to use a simple binary. Rust is cross-platform and produces binaries free of dependencies.

## Logic

The bulk of the program deals with basic prompts provided by the `dialoguer` package. Once the user has entered all information, a Rust command is then issued to perform a `git commit` with the automatically-formatted commit message along with any additional flags passed in by the user. `commit-rs` is designed to be interchangeable with a traditional `git commit`.

## Installation

Add the `cargo-commit` binary to any location in your path. From then on, you can simply run `cargo commit` instead of `git commit`.
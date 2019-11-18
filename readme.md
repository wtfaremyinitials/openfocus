openfocus
=========

> Open source command line client for [OmniFocus](https://www.omnigroup.com/omnifocus) written in Rust

[![Build Status](https://travis-ci.com/wtfaremyinitials/openfocus.svg?token=i1GUdTYu6pqpNYhyT4Hu&branch=master)](https://travis-ci.com/wtfaremyinitials/openfocus)

`openfocus` is a command line tool to interact with OmniFocus database files on
platforms where OmniFocus is not available, primarily Linux.
In addition to (hopefully) being useful, it serves as my project for CSCI3010 at
CU Boulder.

## Example Usage

**View inbox tasks**
`$ cargo run --bin cli -- example.ofocus/ inbox`

**View flagged tasks**
`$ cargo run --bin cli -- example.ofocus/ flagged`

**View tasks assigned to projects**
`$ cargo run --bin cli -- example.ofocus/ projects`

**Add a task to the inbox**
`$ cargo run --bin cli -- example.ofocus/ new "Take out the trash"`

## Roadmap

See [plan.md](./plan.md).

## Dependencies

See [Cargo.toml](./Cargo.toml).

## System Requirements

Any OS that Rust version 1.38.0 supports.

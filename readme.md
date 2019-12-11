openfocus
=========

> Open source command line client for [OmniFocus](https://www.omnigroup.com/omnifocus) written in Rust

[![Build Status](https://travis-ci.com/wtfaremyinitials/openfocus.svg?token=i1GUdTYu6pqpNYhyT4Hu&branch=master)](https://travis-ci.com/wtfaremyinitials/openfocus)

`openfocus` is a command line tool to interact with OmniFocus database files on
platforms where OmniFocus is not available, primarily Linux.
In addition to (hopefully) being useful, it serves as my project for CSCI3010 at
CU Boulder.

## Usage

```
of <ofocus file> <inbox | flagged | forecast | projects | completed>
of <ofocus file> new "<title>"
of <ofocus file> update [-title "<title>"]
                        [-project <parent id>]
                        [-complete]
                        [-incomplete]
                        [-flag]
                        [-due <date>]
                        [-defer <date>]
                        [-duration <minutes>]
```

## Example Usage

`$ alias of='cargo run --bin cli --'` *or install to your $PATH*

**View inbox tasks**

`$ of example.ofocus/ inbox`

**View flagged tasks**

`$ of example.ofocus/ flagged`

**View tasks assigned to projects**

`$ of example.ofocus/ projects`

**Add a task to the inbox**

`$ of example.ofocus/ new "Take out the trash"`

**Flag a task**

`$ of example.ofocus/ update TGltYxe7SNY -flag`

**Change a task's title**

`$ of example.ofocus/ update TGltYxe7SNY -title "New title"`

## Roadmap

See [plan.md](./plan.md).

## Dependencies

See [Cargo.toml](./Cargo.toml).

## System Requirements

Any OS that Rust version 1.38.0 supports.

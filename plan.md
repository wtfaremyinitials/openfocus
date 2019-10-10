William Franzen Project Proposal
================================

## Project Overview

My project is a command line interface for my task management software of choice
OmniFocus. OmniFocus is a commercial software application for task organization
that has many complex features such as folders, sequential and parallel projects,
tags, location awareness, custom user defined filters with boolean logic, nested
projects, and more. I would like to build a free and open source command line
version for native use on Linux. The original Mac and iOS apps for OmniFocus are
great but I prefer to use command line tools on Linux as they integrate so well.
The tool will be compatible with the original app's database format for
interoperability. The tool will allow todo items to be created, updated, and marked
completed. Additionally the database will be able to be queried using the same
"perspectives" system as in the original software.

## Technologies Involved

### Programming Language

I'll be writing the tool in the programming language Rust. Rust is similar to C++
in that it is a strongly typed, compiled, object-oriented language, with a focus
on performance. Rust differs from C++ in a number of ways including: a well
thought out standard library, type inference, and a rich type system that
ensures memory safety and thread safety at compile time. Also of note: while Rust
is object oriented, it uses composition and traits for code sharing rather than
inheritance. For a more detailed discussion of Rust's OOP-ness refer to
[the OOP chapter of *The Rust Programming Language* book](https://doc.rust-lang.org/1.24.0/book/second-edition/ch17-01-what-is-oo.html).
I am quite familiar with Rust but this will be my largest project in Rust to date.

### Network Library

The original OmniFocus software syncs its proprietary-format database via the
open WebDAV protocol. I am considering this out of scope and will use an open
source library or tool to handle this, most likely [vdirsyncer](https://github.com/pimutils/vdirsyncer).
While I have never written code that interfaces with WebDAV, I have spent
countless hours working with a subset of it: RESTful HTTP.

### Data Parsing

The OmniFocus data format is based on XML 1.0. I'll consider the actual parsing
of XML out of scope for the sake of this project and use a generic Rust XML
parser. This will be either [xml-rs](https://crates.io/crates/xml-rs) or
[quick-xml](https://crates.io/crates/quick-xml). I am familiar with neither of
these libraries.

### Date Parsing

The OmniFocus data format uses ISO8601 date stamps. To handle these I will use
the [chrono](https://github.com/chronotope/chrono) library. I have used this
library in the past with much success in implementing a productivity
time-tracking command line tool.

### Data Caching

For performance purposes data may need to be cached outside of the OmniFocus
file format. For this purpose I will use [sqlite](https://sqlite.org/index.html).
I am quite familiar with SQLite having used it for a number of projects in the
past.

### User Interface

The primary user interface for this tool will be the command line, similar to
the official git client. The purpose of this project is to create an OmniFocus
implementation for Linux, and on Linux I prefer to use command line tools
whenever possible. If this project moves ahead of schedule I may consider
extending it to include a
[curses](https://en.wikipedia.org/wiki/Curses_%28programming_library%29)-like
TUI (terminal user interface).

## Outside resources

Little is needed in terms of outside resources except for the official OmniFocus
software to reverse engineer, which I already own. As discussed above the data
format does not appear to be too esoteric, relying primarily on ZIPed XML files.
To get data back into the standard OmniFocus client, database changes will be
synced back to the original server via WebDAV.

## Architecture

The architecture will consist of a front end and a back end both written in Rust.
The front end will be a command line tool that wraps a back end library that
handles the actual functionality, much like the architecture of the cURL command
line tool and library.

The database will be the same ZIPed XML as the official OmniFocus software.

The data model will consist of a few objects that follow the serialized format
closely. Firstly, there will be a `Document` object which encompasses all of the
data for a given OmniFocus database, a `Project` object which in OmniFocus
parlance is an optionally ordered list of `Task`s that must be completed in
order to reach a tangible end goal. A `Task` object represents a singular
physical action that must be taken to advance a `Project` toward completion.
There will also be `Tag` objects that represent tags that can be applied to
both `Project`s and `Task`s. When a tag is applied to a `Project` that tag is
applied to each `Task` within it. Additionally a `Perspective` object will be
used to handle standard and user-defined filters that can be applied to the
database as a whole in order to surface useful and relevant next actions.

Code sharing between objects will be done with idiomatic Rust polymorphic traits.
The best example of this will be code sharing between `Project` and `Task`
objects as they share a significant amount of functionality including but not
limited to: having a title, having notes, having tags, having subtasks which can
be parallel or sequential, and having "defer until" dates, due dates, completion
dates, and the ability to be dropped.

The two design patterns from class that I will be demonstrating are Singleton
and Iterator. The `Document` will be a singleton and the filtering engine will
be an iterator.

## Plan

### Week 1 (2019-10-7)

This week the project proposal will be approved and the development environment
will be set up. This will include a public repository on GitHub, dependencies
installed, a continuous integration system set up, an example database, and a
README file. The user will be able to interact with and admire the repository at
this point.

### Week 2 (2019-10-14)

The second week the project will be spend on the ability to parse the OmniFocus
database format into objects in memory. In order to showcase this functionality
there will be a command line tool to dump the entire database in a pretty
printed format to standard output.

### Week 3 (2019-10-21)

This week will start the development of the core functionality of the project:
the filtering engine. This will take in the constraints of a given perspective
and find the matching tasks in the database. By the end of this week simple
perspectives such as the default Inbox and Projects.

### Week 4 (2019-10-28)

This week will mark the end of implementing the filtering engine. By this point
the code will be able to handle all possible perspectives, default and user
created.

### Week 5 (2019-11-04)

This week will be spent implementing the basics of the front end including only
the ability to query the database.

### Week 6 (2019-11-11)

This week will be spent implementing the ability to create tasks and update
their basic attributes.

### Week 7 (2019-11-18)

This week will be spent on bug fixes and polish to the project. Aspects of the
project that I expect to require polishing include: documentation, comments,
tests, error messages, and colored command output.

### Week 8 (2019-11-25)

No progress is planned for the week of Thanksgiving Break. If I find myself with
free time I'll implement JSON output.

### Week 9 (2019-12-02)

This week the final project presentation will be created. This includes a slide
deck and speaker notes for what will be discussed at each slide. Additionally
I'll test the software outside of my development environment with a fresh data
set.

### Week 10 (2019-12-09)

The presentation will be rehearsed and given this week. This will be the week of
the 1.0 release.

Homework 3
==========

**Planned Work**

The second week the project will be spend on the ability to parse the OmniFocus
database format into objects in memory. In order to showcase this functionality
there will be a command line tool to dump the entire database in a pretty
printed format to standard output.

**Completed Work**

This was complete entirely. There is a command line tool to dump all the tasks
in the OmniFocus database file. It can be invoked with
`cargo run --bin dump -- example.ofocus/00000000000000\=h4xQU5Ux7GB+chN6WaF70I_.zip`

**Plan for Next Deadline**

The first week will start the development of the core functionality of the project:
the filtering engine. This will take in the constraints of a given perspective
and find the matching tasks in the database. By the end of this week simple
perspectives such as the default Inbox and Projects.


The second week will mark the end of implementing the filtering engine. By this point
the code will be able to handle all possible perspectives, default and user
created.


The third week will be spent implementing the basics of the front end including only
the ability to query the database.

**"Screenshots"**

```
$ cargo run --bin dump -- example.ofocus/00000000000000\=h4xQU5Ux7GB+chN6WaF70I_.zip
Task { id: "iWjcZSsiWtL", parent: None, rank: -1, inbox: true, added: 2018-08-30T22:25:41.190Z, modified: 2019-10-10T20:50:10.741Z, name: "Throw a party", note: Some(""), context: Some("lWCMT0TFh2x"), flagged: false, due: Some(2019-10-14T23:00:00Z), start: None, completed: None, estimated_duration: None, complete_by_children: false, order: Sequential }
Task { id: "kuNxDxmJryU", parent: Some("iWjcZSsiWtL"), rank: 1207959552, inbox: true, added: 2018-08-30T23:10:07.486Z, modified: 2018-09-10T20:09:10.160Z, name: "Sweep the house", note: Some(""), context: Some("j0pS5ingsqA"), flagged: false, due: Some(2019-10-12T23:00:00Z), start: None, completed: None, estimated_duration: None, complete_by_children: false, order: Parallel }
Task { id: "pbH-1Zw476y", parent: Some("iWjcZSsiWtL"), rank: 1174405120, inbox: true, added: 2018-08-30T23:11:20.404Z, modified: 2018-09-10T20:08:58.067Z, name: "Order a cake", note: Some(""), context: Some("lNNNk7g-ipb"), flagged: false, due: Some(2019-10-10T23:00:00Z), start: None, completed: None, estimated_duration: None, complete_by_children: false, order: Parallel }
Task { id: "oRql4OdW0lz", parent: Some("iWjcZSsiWtL"), rank: 1476395007, inbox: true, added: 2018-09-05T18:19:10.870Z, modified: 2018-09-10T20:09:18.772Z, name: "Clean out the fridge", note: Some(""), context: Some("j0pS5ingsqA"), flagged: false, due: Some(2019-10-12T23:00:00Z), start: None, completed: None, estimated_duration: None, complete_by_children: false, order: Parallel }
Task { id: "oUEK2p7ebP2", parent: None, rank: -1431655766, inbox: true, added: 2018-09-06T00:57:56.021Z, modified: 2018-09-12T16:44:10.436Z, name: "Welcome to OmniFocus", note: Some(""), context: Some("lWCMT0TFh2x"), flagged: false, due: None, start: None, completed: None, estimated_duration: None, complete_by_children: false, order: Parallel }
Task { id: "cE3WP1ng4Oi", parent: Some("iWjcZSsiWtL"), rank: 1811939328, inbox: true, added: 2018-09-06T20:30:45.662Z, modified: 2018-09-12T16:44:10.436Z, name: "Get ice cream", note: Some(""), context: Some("lNNNk7g-ipb"), flagged: false, due: None, start: None, completed: None, estimated_duration: None, complete_by_children: false, order: Parallel }
Task { id: "a0fJr_XCdwj", parent: None, rank: 1431655764, inbox: true, added: 2018-09-06T21:10:58.989Z, modified: 2019-10-10T20:49:26.263Z, name: "Spring garden cleanup", note: Some(""), context: Some("lWCMT0TFh2x"), flagged: false, due: None, start: None, completed: None, estimated_duration: None, complete_by_children: false, order: Parallel }
Task { id: "djqFHOCRrH3", parent: Some("a0fJr_XCdwj"), rank: 0, inbox: true, added: 2018-09-06T21:11:36.631Z, modified: 2018-09-07T22:51:33.240Z, name: "Repair gate", note: Some(""), context: Some("lWCMT0TFh2x"), flagged: false, due: None, start: None, completed: None, estimated_duration: None, complete_by_children: false, order: Parallel }
Task { id: "igF6JfA9aaP", parent: Some("a0fJr_XCdwj"), rank: 1879048192, inbox: true, added: 2018-09-06T21:11:51.651Z, modified: 2018-09-12T16:44:10.436Z, name: "Build a garden shed", note: Some(""), context: Some("lWCMT0TFh2x"), flagged: false, due: None, start: None, completed: None, estimated_duration: None, complete_by_children: false, order: Parallel }
Task { id: "nAe-bFGfbIA", parent: Some("a0fJr_XCdwj"), rank: 1610612736, inbox: true, added: 2018-09-06T21:12:08.056Z, modified: 2018-09-12T16:44:10.436Z, name: "Test the sprinkler system", note: Some(""), context: Some("lWCMT0TFh2x"), flagged: true, due: None, start: None, completed: None, estimated_duration: None, complete_by_children: false, order: Parallel }
Task { id: "muZXMz_e48J", parent: None, rank: 0, inbox: true, added: 2018-09-07T00:03:31.883Z, modified: 2018-09-12T16:47:07.250Z, name: "Check out the new restaurant", note: Some(""), context: Some("lWCMT0TFh2x"), flagged: false, due: None, start: None, completed: None, estimated_duration: None, complete_by_children: false, order: Parallel }
Task { id: "n0QKZl8CDEY", parent: None, rank: 1789569706, inbox: true, added: 2019-10-10T20:49:31.223Z, modified: 2019-10-10T20:49:54.033Z, name: "Maintain Servers", note: Some(""), context: None, flagged: false, due: None, start: None, completed: None, estimated_duration: None, complete_by_children: false, order: Parallel }
Task { id: "eNJKbQtViaH", parent: Some("n0QKZl8CDEY"), rank: 0, inbox: true, added: 2019-10-10T20:49:37.066Z, modified: 2019-10-10T20:49:41.279Z, name: "Try out NixOS", note: Some(""), context: None, flagged: false, due: None, start: None, completed: None, estimated_duration: None, complete_by_children: false, order: Parallel }
Task { id: "f23PngiKv2G", parent: Some("n0QKZl8CDEY"), rank: 1073741824, inbox: true, added: 2019-10-10T20:49:41.279Z, modified: 2019-10-10T20:49:54.033Z, name: "Update webserver", note: Some(""), context: None, flagged: false, due: None, start: None, completed: None, estimated_duration: None, complete_by_children: false, order: Parallel }
```

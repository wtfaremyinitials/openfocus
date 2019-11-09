Homework 4
==========

**Planned Work**

The first week will start the development of the core functionality of the project:
the filtering engine. This will take in the constraints of a given perspective
and find the matching tasks in the database. By the end of this week simple
perspectives such as the default Inbox and Projects.

The second week will mark the end of implementing the filtering engine. By this point
the code will be able to handle all possible perspectives, default and user
created.

The third week will be spent implementing the basics of the front end including only
the ability to query the database.

**Completed Work**

Everything but user defined perspectives was completed. This was due in large
part to additional unexpected complexity in the perspective data format. The
format used is nothing short of horrifying: perspective rules are stored as
html-entities-escaped JSON contained in XML. Getting useful information out of
that will not be fun. Time was spent mostly fighting with perspective parsing.
The other significant sources of time spent were pretty printing tasks and
making the basic CLI.

**Plan for Next Deadline**

The first week will be spent implementing the ability to create tasks and update
their basic attributes.

The second week will be spent on bug fixes and polish to the project. Aspects of the
project that I expect to require polishing include: documentation, comments,
tests, error messages, and colored command output.

**"Screenshots"**

```
$ cargo run --bin cli -- example.ofocus/ inbox
 [ ] Throw a party			(2019-10-14 23:00:00 UTC)
 [ ] Sweep the house			(2019-10-12 23:00:00 UTC)
 [ ] Order a cake			(2019-10-10 23:00:00 UTC)
 [ ] Clean out the fridge		(2019-10-12 23:00:00 UTC)
 [ ] Welcome to OmniFocus		
 [ ] Get ice cream			
 [ ] Spring garden cleanup		
 [ ] Repair gate			
 [ ] Build a garden shed		
![ ] Test the sprinkler system	
 [ ] Check out the new restaurant	
 [ ] Maintain Servers			
 [ ] Try out NixOS			
 [ ] Update webserver			
```

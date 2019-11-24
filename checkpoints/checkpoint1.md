Checkpoint 1
============

**Planned Work**

The first week will be spent implementing the ability to create tasks and update
their basic attributes.

The second week will be spent on bug fixes and polish to the project. Aspects of the
project that I expect to require polishing include: documentation, comments,
tests, error messages, and colored command output.

**Completed Work**

Nearly everything planned was completed over the last two weeks. The most
significant changes were to do with supporting the database update mechanism
which involves many delta files that act on the root zip file. This took a
significant amount of time to get right. After that I worked on documentation
and got the entire code base commented. The only pieces missing from the plan
are flushing the database changes to disk and colored output. Ideally database
flushing could have been pushed to Week 7 but my course load ramped up that week
ahead of fall break leaving me no extra time.

**Plan for Next Deadline**

Before the next deadline I'll do user testing, automated testing, and clear
the project backlog in addition to creating the final presentation.

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
 [ ] This is a new item
```

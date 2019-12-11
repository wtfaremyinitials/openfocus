Homework 5
==========

**Planned Work**

The first week will be spent implementing the ability to create tasks and update
their basic attributes.

The second week will be spent on bug fixes and polish to the project. Aspects of the
project that I expect to require polishing include: documentation, comments,
tests, error messages, and colored command output.

**Completed Work**

Since the previous final project checkpoint, I have accomplished everything I
had set out to do in addition to clearing the entire project backlog. This brings
the project to completion.

These accomplishments include: a working ID generator, colored command output,
better error handling (file name and error number), command line for updading
tasks, database delta writing support, complete custom perspective parsing, and
documented everything.

**"Screenshots"**

```
$ of example.ofocus/ inbox
(iWjcZSsiWtL)	 [ ] Throw a party				(2019-10-14 23:00:00 UTC)
(kuNxDxmJryU)	 [ ] Sweep the house			(2019-10-12 23:00:00 UTC)
(pbH-1Zw476y)	 [ ] Order a cake				(2019-10-10 23:00:00 UTC)
(oRql4OdW0lz)	 [ ] Clean out the fridge		(2019-10-12 23:00:00 UTC)
(oUEK2p7ebP2)	 [ ] Welcome to OmniFocus		
(cE3WP1ng4Oi)	 [ ] Get ice cream				
(a0fJr_XCdwj)	 [ ] Spring garden cleanup		
(djqFHOCRrH3)	 [ ] Repair gate				
(igF6JfA9aaP)	 [ ] Build a garden shed		
(nAe-bFGfbIA)	![ ] Test the sprinkler system	
(muZXMz_e48J)	 [ ] Check out the new restaurant	
(n0QKZl8CDEY)	 [ ] Maintain Servers			
(eNJKbQtViaH)	 [ ] Try out NixOS			
(f23PngiKv2G)	 [ ] Update webserver			
(m0SsIGQYq83)	 [ ] This is a new item		
$ of example.ofocus/ update djqFHOCRrH3 -t "Fix gate" -f
![ ] Fix gate
$ of example.ofocus/ inbox

(iWjcZSsiWtL)	 [ ] Throw a party				(2019-10-14 23:00:00 UTC)
(kuNxDxmJryU)	 [ ] Sweep the house			(2019-10-12 23:00:00 UTC)
(pbH-1Zw476y)	 [ ] Order a cake				(2019-10-10 23:00:00 UTC)
(oRql4OdW0lz)	 [ ] Clean out the fridge		(2019-10-12 23:00:00 UTC)
(oUEK2p7ebP2)	 [ ] Welcome to OmniFocus		
(cE3WP1ng4Oi)	 [ ] Get ice cream			
(a0fJr_XCdwj)	 [ ] Spring garden cleanup		
(djqFHOCRrH3)	![ ] Fix gate				
(igF6JfA9aaP)	 [ ] Build a garden shed		
(nAe-bFGfbIA)	![ ] Test the sprinkler system	
(muZXMz_e48J)	 [ ] Check out the new restaurant	
(n0QKZl8CDEY)	 [ ] Maintain Servers			
(eNJKbQtViaH)	 [ ] Try out NixOS			
(f23PngiKv2G)	 [ ] Update webserver			
(m0SsIGQYq83)	 [ ] This is a new item		
```

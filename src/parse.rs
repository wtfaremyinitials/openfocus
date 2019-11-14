use std::fs::File;
use zip::read::ZipArchive;
use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;
use chrono::prelude::*;

use crate::error::*;
use crate::task::{Task, SubtaskOrder, ID};
use crate::perspective::{Perspective};
use crate::plist;

#[derive(Debug)]
pub struct Content {
    pub tasks: Vec<Task>
}

impl Content {
    pub fn update(&mut self, delta: Content) {
        for task in delta.tasks {
        }
    }
}

// entry point of parser. takes a File of zip data and extracts Tasks
pub fn parse(f: File) -> Result<Content, Error> {
    // get the contents.xml from the zip file
    let mut zip = ZipArchive::new(f)?;
    let contents = zip.by_name("contents.xml")?;
    assert!(contents.is_file());

    // set up an XML parser
    let mut parser = EventReader::new(contents).into_iter();

    // create vector to store parsed tasks
    let mut tasks: Vec<Task> = Vec::new();

    // iterate over the XML events
    while let Some(evt) = parser.next() {
        match evt {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                match name_to_str(&name) {
                    // <task> found
                    "task" => {
                        let task = parse_task(&mut parser, attributes)?;
                        tasks.push(task);
                    }
                    // <perspective>
                    "perspective" => {
                        // TODO parse_perspective(&mut parser, attributes)?;
                    }
                    "omnifocus" => continue,
                    _ => skip(&mut parser)?
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                // </omnifocus> denotes the end of the file
                if name_to_str(&name) == "omnifocus" {
                    break
                }
            }
            Err(e) => {
                return Err(Box::new(e))
            }
            _ => {}
        }
    }

    // return parsed tasks
    Ok(Content { tasks })
}

// skips over an arbitrary XML structure by keeping track of depth
fn skip<'a>(
    parser: &mut xml::reader::Events<zip::read::ZipFile<'a>>
) -> Result<(), Error> {
    let mut depth = 1;

    while let Some(evt) = parser.next() {
        match evt {
            Ok(XmlEvent::StartElement { .. }) => {
                depth += 1;
            }
            Ok(XmlEvent::EndElement { .. }) => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            Err(e) => { return Err(Box::new(e)) }
            _ => {}
        }
    }

    Ok(())
}

// parses a single Task from a <task>
fn parse_task<'a>(
    parser: &mut xml::reader::Events<zip::read::ZipFile<'a>>,
    root_attrs: Vec<OwnedAttribute>,
) -> Result<Task, Error> {
    // === data to be parsed ===

    // metadata
    let id: ID = attrs_get_val(root_attrs, "id")
        .expect("tasks must have IDs");
    let mut parent:   Option<ID> = None;
    let mut rank:     Option<i64> = None;
    let mut inbox:    bool = false;
    let mut added:    Option<DateTime<Utc>> = None;
    let mut modified: Option<DateTime<Utc>> = None;
    // attributes
    let mut title:    Option<String> = None;
    let mut note:     Option<String> = None;
    let mut context:  Option<ID> = None;
    let mut order:    Option<SubtaskOrder> = None;
    let mut flagged:  bool = false;
    let mut estimated_duration: Option<u64> = None;
    let mut complete_by_children: bool = false;
    let mut start: Option<DateTime<Utc>> = None;
    let mut completed: Option<DateTime<Utc>> = None;
    let mut due: Option<DateTime<Utc>> = None;

    let mut depth = 1;
    while let Some(evt) = parser.next() {
        match evt {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                // when a tag opens increase depth by 1
                depth += 1;
                match name_to_str(&name) {
                    // a <task> inside a <task> is the pointer to the task's parent
                    "task" => {
                        parent = attrs_get_val(attributes, "idref");
                    }
                    // parse the "rank" of the task. used for sorting in some contexts
                    "rank" => {
                        let text = get_text_content(parser.next())?;
                        rank = Some(text.parse()?);
                    }
                    // parse the inbox status of the task
                    "inbox" => {
                        inbox = true;
                    }
                    // parse added date of a task
                    "added" => {
                        let text = get_text_content(parser.next())?;
                        added = Some(text.parse()?);
                    }
                    // parses the modified date of a task
                    "modified" => {
                        let text = get_text_content(parser.next())?;
                        modified = Some(text.parse()?);
                    }
                    // parses the starting "defer" date of a task
                    "start" => {
                        if let Ok(text) = get_text_content(parser.next()) {
                            start = Some(text.parse()?);
                        } else {
                            depth -= 1;
                        }
                    }
                    // parses the completed date of a task
                    "completed" => {
                        if let Ok(text) = get_text_content(parser.next()) {
                            completed = Some(text.parse()?);
                        } else {
                            depth -= 1;
                        }
                    }
                    // parses the due date of a task
                    "due" => {
                        if let Ok(text) = get_text_content(parser.next()) {
                            due = Some(text.parse()?);
                        } else {
                            depth -= 1
                        }
                    }
                    // parses the name of the task
                    "name" => {
                        if let Ok(text) = get_text_content(parser.next()) {
                            title = Some(text);
                        } else {
                            title = Some(String::new());
                            depth -= 1;
                        }
                    }
                    // parses the additional notes attached to a task
                    "note" => {
                        // TODO
                        skip(parser)?;
                        depth -= 1;
                        note = Some(String::new());
                    },
                    // parses a context/tag associated with a task
                    "context" => {
                        context = attrs_get_val(attributes, "idref");
                    }
                    // parses the order subtasks can be completed in
                    // either Parallel or Sequential
                    "order" => {
                        let text = get_text_content(parser.next())?;
                        order = Some(text.parse()?);
                    }
                    // parses the flagged status of a task
                    "flagged" => {
                        let text = get_text_content(parser.next())?;
                        flagged = text.parse()?;
                    }
                    // parses the estimated minutes of a task
                    "estimated-minutes" => {
                        if let Ok(text)= get_text_content(parser.next()) {
                            estimated_duration = Some(text.parse()?);
                        } else {
                            depth -= 1;
                        }
                    }
                    // parses whether this task is auto complete when all of its
                    // children are complete
                    "completed-by-children" => {
                        let text = get_text_content(parser.next())?;
                        complete_by_children = text.parse()?;
                    }
                    // parses the project the task belongs to
                    "project" => {
                        // TODO
                        skip(parser)?;
                        depth -= 1;
                    }
                    _ => {/*println!("child {:?} {:?}", name, attributes)*/}
                }
            }
            // at each closing tag decrease depth
            Ok(XmlEvent::EndElement { .. }) => {
                depth -= 1;
                // when depth is zero we're done parsing a <task>
                if depth == 0 {
                    break;
                }
            }
            Err(e) => { return Err(Box::new(e)) }
            _ => {}
        }
    }

    // return parsed task
    Ok(Task {
        id,
        parent,
        rank,
        inbox,
        added: added.expect("Tasks must have an added datetime"),
        modified,
        name: title.expect("Tasks must have a name"),
        note,
        completed,
        context,
        flagged,
        due,
        start,
        estimated_duration,
        complete_by_children,
        order,
    })
}

// parses a <perspective>. WORK IN PROGRESS
fn parse_perspective<'a>(
    mut parser: &mut xml::reader::Events<zip::read::ZipFile<'a>>,
    root_attrs: Vec<OwnedAttribute>,
) -> Result<Perspective, Error> {
    // TODO: depth purely for error handling? 

    let mut added: Option<DateTime<Utc>> = None;

    while let Some(evt) = parser.next() {
        match evt {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                match name_to_str(&name) {
                    "added" => {
                        let text = get_text_content(parser.next())?;
                        added = Some(text.parse()?);
                    }
                    "plist" => {
                        let plist = plist::parse_plist(&mut parser);
                        dbg!(plist);
                    }
                    _ => {}
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                if name_to_str(&name) == "perspective" {
                    break
                }
            }
            _ => {}
        }
    }

    Ok(Perspective {
        id: "foo".into(), // TODO grab id
        added: added.expect("added is required"),
    })
}

// turns an OwnedName struct into a &str to make it actually useful
pub fn name_to_str<'a>(name: &'a xml::name::OwnedName) -> &'a str {
    name.local_name.as_str()
}

// get the value of an OwnedAttribute
fn attrs_get_val(attrs: Vec<OwnedAttribute>, name: &str) -> Option<String> {
    attrs
        .iter()
        .find(|attr| name_to_str(&attr.name) == name)
        .map(|a| a.value.clone())
}

// gets the text content from a given tag
// <example>THIS TEXT IS EXTRACTED</example>
pub fn get_text_content(
    next: Option<Result<XmlEvent, xml::reader::Error>>
) -> Result<String, Error> {
    if let Some(Ok(XmlEvent::Characters(text))) = next {
        Ok(text)
    } else {
        Err(Box::new(OpenFocusError::Parse))
    }
}

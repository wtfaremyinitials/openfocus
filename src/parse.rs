use std::fs::File;
use zip::read::ZipArchive;
use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;
use chrono::prelude::*;

use crate::error::*;
use crate::task::{Task, SubtaskOrder, ID};

pub fn parse(f: File) -> Result<(), Error> {
    let mut zip = ZipArchive::new(f)?;
    let contents = zip.by_name("contents.xml")?;
    assert!(contents.is_file());

    let mut parser = EventReader::new(contents).into_iter();

    while let Some(evt) = parser.next() {
        match evt {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                match name_to_str(&name) {
                    "task" => {
                        let task = parse_task(&mut parser, attributes)?;
                        println!("task {:?}", task);
                    }
                    "omnifocus" => continue,
                    // TODO: other object parsers
                    _ => skip(&mut parser)?
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                if name_to_str(&name) == "omnifocus" {
                    unimplemented!("done parsing yay!!")
                }
            }
            Err(e) => {
                return Err(Box::new(e))
            }
            _ => {}
        }
    }

    panic!("got to end of loop prematurely")
}

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
                depth += 1;
                match name_to_str(&name) {
                    "task" => {
                        parent = attrs_get_val(attributes, "idref");
                    }
                    "rank" => {
                        let text = get_text_content(parser.next())?;
                        rank = Some(text.parse()?);
                    }
                    "inbox" => {
                        inbox = true;
                    }
                    "added" => {
                        let text = get_text_content(parser.next())?;
                        added = Some(text.parse()?);
                    }
                    "modified" => {
                        let text = get_text_content(parser.next())?;
                        modified = Some(text.parse()?);
                    }
                    "start" => {
                        if let Ok(text) = get_text_content(parser.next()) {
                            start = Some(text.parse()?);
                        } else {
                            depth -= 1;
                        }
                    }
                    "completed" => {
                        if let Ok(text) = get_text_content(parser.next()) {
                            completed = Some(text.parse()?);
                        } else {
                            depth -= 1;
                        }
                    }
                    "due" => {
                        if let Ok(text) = get_text_content(parser.next()) {
                            due = Some(text.parse()?);
                        } else {
                            depth -= 1
                        }
                    }
                    "name" => {
                        let text = get_text_content(parser.next())?;
                        title = Some(text);
                    }
                    "note" => {
                        // TODO
                        skip(parser)?;
                        depth -= 1;
                        note = Some(String::new());
                    },
                    "context" => {
                        context = attrs_get_val(attributes, "idref");
                    }
                    "order" => {
                        let text = get_text_content(parser.next())?;
                        order = Some(text.parse()?);
                    }
                    "flagged" => {
                        let text = get_text_content(parser.next())?;
                        flagged = text.parse()?;
                    }
                    "estimated-minutes" => {
                        if let Ok(text)= get_text_content(parser.next()) {
                            estimated_duration = Some(text.parse()?);
                        } else {
                            depth -= 1;
                        }
                    }
                    "completed-by-children" => {
                        let text = get_text_content(parser.next())?;
                        complete_by_children = text.parse()?;
                    }
                    "project" => {
                        // TODO
                        skip(parser)?;
                        depth -= 1;
                    }
                    _ => {/*println!("child {:?} {:?}", name, attributes)*/}
                }
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

    Ok(Task {
        id,
        parent,
        rank: rank.expect("Tasks must have a rank"),
        inbox,
        added: added.expect("Tasks must have an added datetime"),
        modified: modified.expect("Tasks must have a modified datetime"),
        name: title.expect("Tasks must have a name"),
        note,
        completed,
        context,
        flagged,
        due,
        start,
        estimated_duration,
        complete_by_children,
        order: order.expect("Tasks must have a subtask order"),
    })
}

fn name_to_str<'a>(name: &'a xml::name::OwnedName) -> &'a str {
    name.local_name.as_str()
}

fn attrs_get_val(attrs: Vec<OwnedAttribute>, name: &str) -> Option<String> {
    attrs
        .iter()
        .find(|attr| name_to_str(&attr.name) == name)
        .map(|a| a.value.clone())
}

fn get_text_content(
    next: Option<Result<XmlEvent, xml::reader::Error>>
) -> Result<String, Error> {
    if let Some(Ok(XmlEvent::Characters(text))) = next {
        Ok(text)
    } else {
        Err(Box::new(OpenFocusError::Parse))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works_too() {
        assert_eq!(2 + 2, 4);
    }
}

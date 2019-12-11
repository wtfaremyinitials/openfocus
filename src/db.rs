use regex::Regex;
use std::fs::{read_dir, File};
use std::path::PathBuf;
use std::io::prelude::*;
use chrono::Utc;
use crate::parse::parse;
use crate::error::*;
use crate::util::generate_id;
use xml::writer::{EventWriter, XmlEvent};
use zip::write::ZipWriter;
pub use crate::parse::Content;

// represents the whole of a `.ofocus` file (actually a directory)
pub struct Database {
    file_path: PathBuf,
    head_id: Option<String>, // the ID of the most recent archive file
    archives: Vec<Archive>,  // all of the archive files in the database
    content: Content,        // all of the loaded data
}

impl Database {
    // creates a database struct and gets a list of all archive files inside
    pub fn new(path: PathBuf) -> Result<Database, Error> {
        // find all files in the dir ending in .zip and create a corresponding
        // Archive struct
        let archives = read_dir(&path)?
            .filter(|p| {
                p.is_ok() &&
                p.as_ref().unwrap().path().to_str().unwrap().ends_with(".zip")
            })
            .map(|p| Archive::new(p.unwrap().path()))
            .collect::<Result<Vec<Archive>, Error>>()?;

        // create a database
        let mut db = Database {
            file_path: path,
            head_id: None,
            archives,
            content: Content { tasks: Vec::new(), perspectives: Vec::new() }
        };

        // load all the archives
        db.load_all()?;

        // return the new database
        Ok(db)
    }

    // load all the archives in the database in order. last part of an archive
    // filename is a pointer to the next filename
    fn load_all(&mut self) -> Result<(), Error> {
        // get the root archive which has a timestamp of "00000000000000"
        let mut curr: Option<&Archive> = Some(self.archives.iter()
                        .filter(|a| a.date == "00000000000000")
                        .next()
                        .expect("database has no root!"));

        // while there is another archive to process
        while let Some(archive) = curr {
            self.head_id = Some(archive.id.clone());

            // read the archive data into the database
            self.content.update(archive.read()?);

            // find the next archive to read
            curr = self.archives.iter()
                    .filter(|a| a.parent_id == archive.id)
                    .next();
        }

        Ok(())
    }

    // write a new Content struct out to the filesystem
    pub fn write(&mut self, delta: Content) -> Result<(), Error> {
        // if this was a long running process Database.content would need to be
        // updated somehow
        let cur_head = self.head_id.as_ref()
            .expect("attempted to write to db without loading it first");
        let archive = Archive::save(cur_head, &self.file_path, delta)?;
        self.head_id = Some((&archive.id).clone());
        self.archives.push(archive);
        Ok(())
    }

    // returns a readonly ref to the content (mutations are done by creating
    // Content structs as deltas
    pub fn content(&self) -> &Content {
        &self.content
    }
}

// represents a single zip file in the database which is either the root or a
// delta on the root
struct Archive {
    file_path: PathBuf,
    date: String,
    id: String,
    parent_id: String,
}

impl Archive {
    // create an archive struct from the path
    fn new(path: PathBuf) -> Result<Archive, Error> {
        // parse the relevant information out of the filename
        let path_parser =
            Regex::new(r"(\d{14})=([A-Za-z0-9_-]{11})\+([A-Za-z0-9_-]{11}).zip$")
            .unwrap();

        let path_string: String = path.to_str().unwrap().into();
        let caps = path_parser.captures(&path_string);

        if let Some(caps) = caps {
            Ok(Archive {
                file_path: path,
                date:      caps.get(1).unwrap().as_str().into(),
                parent_id: caps.get(2).unwrap().as_str().into(),
                id:        caps.get(3).unwrap().as_str().into(),
            })
        } else {
            Err(crate::err!(Parse))
        }
    }

    // write out a Content struct as a delta
    fn save(parent_id: &str, db_path: &PathBuf, delta: Content) -> Result<Archive, Error> {
        let id = generate_id();
        let gmt = Utc::now().format("%Y%m%d%6f").to_string();
        let file_name = format!("{}={}+{}.zip", gmt, parent_id, id);
        let file_path = {
            let mut tmp = db_path.clone();
            tmp.push(file_name);
            tmp
        };


        let archive = Archive {
            id,
            parent_id: parent_id.to_string(),
            file_path,
            date: gmt,
        };

        let file = File::create(archive.file_path.as_path())?;
        let mut zip = ZipWriter::new(file);
        zip.start_file("contents.xml", zip::write::FileOptions::default())?;
        let mut xml = EventWriter::new(zip);

        // i honestly just don't feel like making the generics work right now
        type ConcreteEventWriter = EventWriter<ZipWriter<File>>;

        fn end(xml: &mut ConcreteEventWriter) -> Result<(), Error> {
            let tmp: XmlEvent = XmlEvent::end_element().into();
            xml.write(tmp)?;
            Ok(())
        }

        fn text(
            xml: &mut ConcreteEventWriter,
            name: &str,
            text: &str
        ) -> Result<(), Error> {
            let tmp: XmlEvent = XmlEvent::start_element(name).into();
            xml.write(tmp)?;
            let tmp: XmlEvent = XmlEvent::characters(text).into();
            xml.write(tmp)?;
            end(xml)?;
            Ok(())
        }

        fn attrs_open(
            xml: &mut ConcreteEventWriter,
            name: &str,
            attrs: Vec<(&str, &str)>
        ) -> Result<(), Error> {
            let mut tmp = XmlEvent::start_element(name);
            for (k, v) in attrs {
                tmp = tmp.attr(k, v);
            }
            let tmp: XmlEvent = tmp.into();
            xml.write(tmp)?;
            Ok(())
        }

        fn attrs(
            xml: &mut ConcreteEventWriter,
            name: &str,
            attrs: Vec<(&str, &str)>
        ) -> Result<(), Error> {
            attrs_open(xml, name, attrs)?;
            end(xml)?;
            Ok(())
        }

        xml.write(XmlEvent::StartDocument {
            encoding: Some("UTF-8"),
            standalone: None,
            version: xml::common::XmlVersion::Version10,
        })?;
        xml.inner_mut().write(b"\n")?;

        attrs_open(&mut xml, "omnifocus", vec![
            ("xmlns", "http://www.omnigroup.com/namespace/OmniFocus/v2"),
            ("app-id", "wtf.will.openfocus"),
            ("app-version", "0.0.0"),
            ("os-name", "unknown"),
            ("os-version", "unknown"),
            ("machine-model", "unknown")
        ])?;

        for task in delta.tasks {
            // write <task id="{id}">
            attrs_open(&mut xml, "task", vec![("id", &task.id)])?;

            // write <project/>
            attrs(&mut xml, "project", vec![])?;

            // write <inbox>{true/false}</inbox>
            text(&mut xml, "inbox", &task.inbox.to_string())?;

            // write <task />
            if let Some(parent_id) = task.parent {
                attrs(&mut xml, "task", vec![("id", &parent_id)])?;
            } else {
                attrs(&mut xml, "task", vec![])?;
            }

            // write <added>{date}</added>
            text(&mut xml, "added", &task.added.to_rfc3339_opts(
                chrono::SecondsFormat::Millis,
                true
            ))?;

            // write <name>{title}</name>
            text(&mut xml, "name", &task.title)?;

            // write <rank>{rank}</rank>
            if let Some(rank) = task.rank {
                text(&mut xml, "rank", &rank.to_string())?;
            }

            // write <context />
            if let Some(context_id) = task.context {
                attrs(&mut xml, "context", vec![("id", &context_id)])?;
            } else {
                attrs(&mut xml, "context", vec![])?;
            }

            // write <start>{date}</start>
            if let Some(start) = task.start {
                text(&mut xml, "start", &start.to_rfc3339_opts(
                    chrono::SecondsFormat::Millis,
                    true
                ))?;
            } else {
                attrs(&mut xml, "start", vec![])?;
            }

            // write <due>{date}</due>
            if let Some(due) = task.due {
                text(&mut xml, "due", &due.to_rfc3339_opts(
                    chrono::SecondsFormat::Millis,
                    true
                ))?;
            } else {
                attrs(&mut xml, "due", vec![])?;
            }

            // write <completed>{date}</completed>
            if let Some(completed) = task.completed {
                text(&mut xml, "completed", &completed.to_rfc3339_opts(
                    chrono::SecondsFormat::Millis,
                    true
                ))?;
            } else {
                attrs(&mut xml, "completed", vec![])?;
            }

            // write <modified>{date}</modified>
            if let Some(modified) = task.modified {
                text(&mut xml, "modified", &modified.to_rfc3339_opts(
                    chrono::SecondsFormat::Millis,
                    true
                ))?;
            } else {
                attrs(&mut xml, "modified", vec![])?;
            }

            // write <estimated-minutes>
            if let Some(est) = task.estimated_duration {
                text(&mut xml, "estimated-minutes", &est.to_string())?;
            } else {
                attrs(&mut xml, "estimated-minutes", vec![])?;
            }

            // write <flagged>{true/false}</flagged>
            text(&mut xml, "flagged", &task.flagged.to_string())?;

            // write <complete-by-children>{true/false}</complete-by-children>
            text(
                &mut xml,
                "complete-by-children",
                &task.complete_by_children.to_string()
            )?;

            // write <order>{parallel/sequential}</order>
            if let Some(order) = task.order {
                text(&mut xml, "order", match order {
                    crate::task::SubtaskOrder::Parallel => "parallel",
                    crate::task::SubtaskOrder::Sequential => "sequential",
                })?;
            } else {
                attrs(&mut xml, "order", vec![])?;
            }

            // </task>
            end(&mut xml)?;
        }

        end(&mut xml)?;
        xml.inner_mut().write(b"\n")?;

        Ok(archive)
    }

    // read the contents of this Archive from the underlying file
    fn read(&self) -> Result<Content, Error> {
        parse(File::open(&self.file_path)?)
    }
}

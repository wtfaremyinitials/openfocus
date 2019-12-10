use regex::Regex;
use std::fs::{read_dir, File};
use std::path::PathBuf;
use crate::parse::parse;
use crate::error::*;

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
            content: Content { tasks: Vec::new() }
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
        let archive = Archive::save(cur_head, delta)?;
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
    fn save(parent_id: &str, delta: Content) -> Result<Archive, Error> {
        println!("writing delta containing:\n{:?}", delta);
        // TODO: XML serialize, then create a ZIP archive, then write to disk
        unimplemented!()
    }

    // read the contents of this Archive from the underlying file
    fn read(&self) -> Result<Content, Error> {
        parse(File::open(&self.file_path)?)
    }
}

use regex::Regex;
use std::fs::read_dir;
///use std::fs::{read_dir, File};
use std::path::PathBuf;
use crate::task::Task;
use crate::parse::parse;
use crate::error::*;

pub struct Content {
    pub tasks: Vec<Task>
}

impl Content {
    fn update(&mut self, delta: Content) {
        for task in delta.tasks {
        }
    }
}

pub struct Database {
    file_path: PathBuf,
    head_id: Option<String>,
    archives: Vec<Archive>,
    content: Content,
}

impl Database {
    pub fn new(path: PathBuf) -> Result<Database, Error> {
        let archives = read_dir(&path)?
            .filter(|p| {
                p.is_ok() &&
                p.as_ref().unwrap().path().to_str().unwrap().ends_with(".zip")
            })
            .map(|p| Archive::new(p.unwrap().path()))
            .collect::<Result<Vec<Archive>, Error>>()?;


        let mut db = Database {
            file_path: path,
            head_id: None,
            archives,
            content: Content {
                tasks: Vec::new()
            }
        };

        db.load_all()?;

        Ok(db)
    }

    fn load_all(&mut self) -> Result<(), Error> {
        let mut curr: Option<&Archive> = Some(self.archives.iter()
                        .filter(|a| a.date == "00000000000000")
                        .next()
                        .expect("database has no root!"));

        while let Some(archive) = curr {
            self.content.update(archive.read()?);

            curr = self.archives.iter()
                    .filter(|a| a.parent_id == archive.id)
                    .next();
        }

        Ok(())
    }

    pub fn write(&mut self, delta: Content) -> Result<(), Error> {
        let cur_head = self.head_id.as_ref()
            .expect("attempted to write to db without loading it first");
        let archive = Archive::save(cur_head, delta)?;
        self.head_id = Some((&archive.id).clone());
        self.archives.push(archive);
        Ok(())
    }

    pub fn content(&self) -> &Content {
        &self.content
    }
}

struct Archive {
    file_path: PathBuf,
    date: String,
    id: String,
    parent_id: String,
}

impl Archive {
    fn new(path: PathBuf) -> Result<Archive, Error> {
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
            Err(Box::new(OpenFocusError::Parse))
        }
    }

    fn save(parent_id: &str, delta: Content) -> Result<Archive, Error> {
        unimplemented!()
    }

    fn read(&self) -> Result<Content, Error> {
        // TODO: invoke parse.rs
        println!("read() {}", &self.id);
        Ok(Content {
            tasks: Vec::new()
        })
    }
}

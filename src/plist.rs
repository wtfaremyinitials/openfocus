use std::collections::HashMap;
use xml::reader::XmlEvent;

use crate::error::*;
use crate::parse::{name_to_str, get_text_content};

// helpers to handle OmniFocus's use of Apple's Property List (plist) format

#[derive(Debug)]
pub enum PlistItem {
    String(String),
    Dict(HashMap<String, PlistItem>),
}

impl PlistItem {
    pub fn unwrap_string<'a>(&'a self) -> &'a String {
        match self {
            PlistItem::String(s) => s,
            _ => panic!()
        }
    }

    pub fn unwrap_dict<'a>(&'a self) -> &'a HashMap<String, PlistItem> {
        match self {
            PlistItem::Dict(d) => d,
            _ => panic!()
        }
    }
}

// TODO: use dynamic something or other do decouple this from ZipFile
pub fn parse_plist<'a>(
    mut parser: &mut xml::reader::Events<zip::read::ZipFile<'a>>,
) -> Result<PlistItem, Error> {
    if let Some(Ok(XmlEvent::StartElement { name, .. })) = parser.next() {
        match name_to_str(&name) {
            "string" => {
                Ok(PlistItem::String(get_text_content(parser.next())?))
            }
            "dict" => {
                Ok(PlistItem::Dict(parse_plist_dict(&mut parser)?))
            }
            _ => Err(crate::err!(Parse))
        }
    } else {
        Err(crate::err!(Parse))
    }
}

pub fn parse_plist_dict<'a>(
    parser: &mut xml::reader::Events<zip::read::ZipFile<'a>>,
) -> Result<HashMap<String, PlistItem>, Error> {
    let mut map = HashMap::new();
    while let Some(evt) = parser.next() {
        match evt {
            Ok(XmlEvent::StartElement { name, .. }) => {
                assert!(name_to_str(&name) == "key");
                let key = get_text_content(parser.next())?;
                parser.next();
                let value = parse_plist(parser)?;
                map.insert(key, value);
            },
            Ok(XmlEvent::EndElement { name, .. }) => {
                if name_to_str(&name) == "dict" {
                    break
                }
            },
            _ => return Err(crate::err!(Parse)),
        }
    }
    Ok(map)
}

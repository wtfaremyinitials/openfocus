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
    mut parser: &mut xml::reader::Events<zip::read::ZipFile<'a>>,
) -> Result<HashMap<String, PlistItem>, Error> {
    unimplemented!()
}

use crate::parse::{group, track};
use roxmltree::Document;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AlsData {
    pub name: String,
    pub groups: Vec<group::Group>,
    pub tracks: Vec<track::Track>,
}

#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub als_data: Option<Vec<AlsData>>,
}

impl AlsData {
    /// Uses all `parse` modules to parse *als* files
    pub fn parse(name: String, xml_contents: String) -> AlsData {
        let doc = Document::parse(&xml_contents).unwrap();
        let root = doc.root_element();

        let tracks: Vec<track::Track> = root
            .descendants()
            .filter(|n| {
                n.has_tag_name("MidiTrack")
                    || n.has_tag_name("AudioTrack")
                    || n.has_tag_name("ReturnTrack")
            })
            .map(|n| track::Track::parse(n))
            .collect();

        let groups: Vec<group::Group> = root
            .descendants()
            .filter_map(|n| group::Group::parse(n))
            .collect();

        return AlsData {
            name,
            groups,
            tracks,
        };
    }
}

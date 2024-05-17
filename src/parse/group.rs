use roxmltree::Node;
use serde::Serialize;

use crate::get_attribute_value;

#[derive(Debug, Serialize)]
pub struct Group {
    pub id: u32,
    pub name: String,
    pub color: Option<u32>,
}

impl Group {
    pub fn parse(node: Node) -> Option<Group> {
        if !node.has_tag_name("GroupTrack") {
            return None;
        }

        let id = node.attribute("Id").unwrap().parse().unwrap();
        let name = get_attribute_value!(node, "Name", "EffectiveName").to_string();

        let color = None;

        Some(Group { id, name, color })
    }
}

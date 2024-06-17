use crate::parse::clip;
use roxmltree::Node;
use serde::{Deserialize, Serialize};

use crate::get_attribute_value;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Track {
    pub group_id: i32,
    pub name: String,
    pub color: Option<u32>,
    pub clips: Vec<clip::Clip>,
}

impl Track {
    pub fn parse(node: Node) -> Track {
        let group_id: i32 = get_attribute_value!(node, "TrackGroupId")
            .parse()
            .unwrap_or(-1);
        let name = get_attribute_value!(node, "Name", "EffectiveName").to_string();
        let color = None;

        let clips = node
            .descendants()
            .filter(|n| n.has_tag_name("MidiClip") || n.has_tag_name("AudioClip"))
            .map(|n| clip::Clip::parse(n))
            .collect();

        Track {
            group_id,
            name,
            color,
            clips,
        }
    }
}

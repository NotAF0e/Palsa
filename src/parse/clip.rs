use roxmltree::Node;
use serde::{Deserialize, Serialize};

use crate::get_attribute_value;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Clip {
    pub name: String,
    pub time: f64,
    pub length: f64,
    pub loop_data: Option<Loop>,
}

impl Clip {
    pub fn parse(node: Node) -> Clip {
        let name: String = get_attribute_value!(node, "Name").to_string();

        let time = node.attribute("Time").unwrap().parse().unwrap();
        let loop_node = node.descendants().find(|n| n.has_tag_name("Loop")).unwrap();
        let loop_data = Some(Loop {
            start: get_attribute_value!(loop_node, "LoopStart")
                .parse()
                .unwrap(),
            end: get_attribute_value!(loop_node, "LoopEnd").parse().unwrap(),
        });
        let length = loop_data.as_ref().unwrap().end - loop_data.as_ref().unwrap().start;
        Clip {
            name,
            time,
            length,
            loop_data,
        }
    }
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Loop {
    pub start: f64,
    pub end: f64,
}

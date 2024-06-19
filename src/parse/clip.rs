use roxmltree::Node;
use serde::{Deserialize, Serialize};

use crate::get_attribute_value;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Clip {
    pub name: String,
    pub start: f32,
    pub end: f32,
    pub loop_data: Option<Loop>,
}

impl Clip {
    pub fn parse(node: Node) -> Clip {
        let name: String = get_attribute_value!(node, "Name").to_string();

        let start: f32 = get_attribute_value!(node, "CurrentStart")
            .parse()
            .unwrap_or(-1.0);
        let end: f32 = get_attribute_value!(node, "CurrentEnd")
            .parse()
            .unwrap_or(-1.0);

        // eprintln!("{:?}", &(start, end));

        let loop_node = node.descendants().find(|n| n.has_tag_name("Loop")).unwrap();
        let loop_data = Some(Loop {
            start: get_attribute_value!(loop_node, "LoopStart")
                .parse()
                .unwrap(),
            end: get_attribute_value!(loop_node, "LoopEnd").parse().unwrap(),
        });

        Clip {
            name,
            start,
            end,
            loop_data,
        }
    }
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Loop {
    pub start: f64,
    pub end: f64,
}

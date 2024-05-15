use roxmltree::{Document, Node};

#[derive(Debug, serde::Serialize)]
pub struct Clip {
    time: f64,
    length: f64,
    loop_data: Option<Loop>,
}

#[derive(Debug, serde::Serialize)]
struct Loop {
    start: f64,
    end: f64,
}

#[derive(Debug, serde::Serialize)]
pub struct Track {
    pub name: String,
    pub color: u32,
    pub clips: Vec<Clip>,
}

pub fn parse_als(xml_contents: String) -> Vec<Track> {
    let doc = Document::parse(&xml_contents).unwrap();
    let root = doc.root_element();

    let tracks: Vec<Track> = root
        .descendants()
        .filter(|n| {
            n.has_tag_name("MidiTrack")
                || n.has_tag_name("AudioTrack")
                || n.has_tag_name("ReturnTrack")
        })
        .map(|n| parse_track(n))
        .collect();

    return tracks;
}

fn parse_clip(node: Node) -> Clip {
    let time = node.attribute("Time").unwrap().parse().unwrap();
    let loop_node = node.descendants().find(|n| n.has_tag_name("Loop")).unwrap();
    let loop_data = Some(Loop {
        start: loop_node
            .descendants()
            .find(|n| n.has_tag_name("LoopStart"))
            .unwrap()
            .attribute("Value")
            .unwrap()
            .parse()
            .unwrap(),
        end: loop_node
            .descendants()
            .find(|n| n.has_tag_name("LoopEnd"))
            .unwrap()
            .attribute("Value")
            .unwrap()
            .parse()
            .unwrap(),
    });

    let length = loop_data.as_ref().unwrap().end - loop_data.as_ref().unwrap().start;

    Clip {
        time,
        length,
        loop_data,
    }
}

fn parse_track(node: Node) -> Track {
    let name = node
        .descendants()
        .find(|n| n.has_tag_name("Name"))
        .unwrap()
        .descendants()
        .find(|n| n.has_tag_name("EffectiveName"))
        .unwrap()
        .attribute("Value")
        .unwrap()
        .to_string();

    // let color = node.attribute("Color").unwrap().parse().unwrap();
    let color = 0; // default color

    let clips = node
        .descendants()
        .filter(|n| n.has_tag_name("MidiClip") || n.has_tag_name("AudioClip"))
        .map(|n| parse_clip(n))
        .collect();

    Track { name, color, clips }
}

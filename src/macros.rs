#[macro_export]
macro_rules! get_attribute_value {
    // Some cool marcro stuff
    // First expression is the xml node
    // The second alows for a recursive list of tags to be entered
    ($node:expr, $($tag_name:expr),*) => {{
        let mut current_node = $node;
        $(
            current_node = current_node
                .descendants()
                .find(|n| n.has_tag_name($tag_name))
                .unwrap();
        )*
        current_node
            .attribute("Value")
            .unwrap()
            .to_string()
    }};
}

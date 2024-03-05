use practice_tool_core::{
    key::Key,
    widgets::{group::Group, Widget},
};

pub(crate) fn group(
    label: &str,
    commands: Vec<Box<dyn Widget>>,
    key_close: Key,
) -> Box<dyn Widget> {
    Box::new(Group::new(label, key_close, commands))
}

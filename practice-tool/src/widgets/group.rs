use practice_tool_core::{
    key::Key,
    widgets::{group::Group, Widget},
};

pub(crate) fn group(
    label: &str,
    key_close: Key,
    commands: Vec<Box<dyn Widget>>,
) -> Box<dyn Widget> {
    Box::new(Group::new(label, key_close, commands))
}

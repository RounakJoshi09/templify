pub(crate) struct PlaceholderDefinition {
    pub name: String,
    pub description: String,
    pub get_value: fn() -> String,
}

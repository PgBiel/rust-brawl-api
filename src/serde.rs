// TODO: String visitor

pub fn serde_string_to_usize<'de, D>(deserializer: D) -> Result<T, D::Error>
    where D: Deserializer<'de> {
    deserializer.deserialize_string()
}
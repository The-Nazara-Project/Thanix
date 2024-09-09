//! Various supporting functionality.

/// Names of NetBox API objects which can cause the client to crash at response data serialization.
///
/// This workaround is necessary as a lot of the time, some database entries can have fieds set to
/// `null` even though the API schema states that they are not nullable.
/// This leads to problems when creating an API client crate just by using the YAML schema as the
/// client then expects all response data to be correct, otherwise `serde` cannot build the
/// required structs.
///
/// To work around this, the `--workaround` flag was added to Thanix, which will check at struct
/// generation, whether the struct is part of this **manually maintained list of troublemakers**
///
/// > [!Note]
/// > This list is maintained manually by the Nazara Team, as there is currently no real way to
/// automate this.
/// > If you have problems and need something to be added to it, please open a bug in our [issues
/// section](https://github.com/The-Nazara-Project/Thanix/issues/).
static UNSANITARY_OBJECTS: &[&str] = &["interface"];

/// Check if a given struct's name contains any entry from the `UNSANITARY_OBJECTS` list.
pub fn is_unsanitary(name: &str) -> bool {
    UNSANITARY_OBJECTS
        .iter()
        .any(|&word| name.to_lowercase().contains(word))
}

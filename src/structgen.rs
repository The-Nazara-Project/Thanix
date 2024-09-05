//! Generate structs from API objects.

use crate::{bindgen, util::is_unsanitary};
use check_keyword::CheckKeyword;
use openapiv3::{ReferenceOr, Schema, SchemaKind, Type};

/// Generate the structs to be used as API request payloads.
///
/// If `workaround_mode` is enabled, will check if the current struct matches with the names listed
/// in `unsanitary_data` and make all fields of these structs optional.
/// This can help when normally generated API clients crash with serialization issues due to
/// NetBox's response data having some fileds set to `null`, despite the YAML stating that they are
/// not nullable.
///
/// > [!Note]
/// > The workaround mentioned above is *only* valid and useful when creating an API client with
/// NetBox.
/// > Using the `--workaround` flag with any other use case is **not advised** because it weakens
/// data validation.
///
/// # Parameters
///
/// * `name: &str` - The name of the struct to generate.
/// * `schema: &Schema` - The schema this struct follows.
/// * `workaround_mode: bool` - Whether `--workaround` flag has been set or not.
///
/// # Returns
///
/// * `Option<String>` - The string represnetation of the given struct.
pub fn gen(name: &str, schema: &Schema, workaround_mode: bool) -> Option<String> {
    let typ = match &schema.schema_kind {
        SchemaKind::Type(x) => x,
        _ => return None,
    };
    // If not an ObjectType, return None.
    let obj = match &typ {
        Type::Object(x) => x,
        _ => return None,
    };

    // Assemble struct string.
    let mut result =
        "#[derive(Serialize, Deserialize, Debug, Default, Clone)]\npub struct ".to_owned();
    result += name;
    result += " {\n";

    // For every component property.
    for (prop_name, prop) in &obj.properties {
        let p = prop.clone().unbox();
        // Assemble a field declaration in the struct.
        let type_name = bindgen::type_to_string(&p);

        // If the property has a description, prepend a doc string.
        if let ReferenceOr::Item(item) = &p {
            if let Some(desc) = &item.schema_data.description {
                result += bindgen::make_comment(Some(desc.clone()), 1).as_str();
            }
        }
        result += "\t";
        result += &format!("pub {}", &prop_name.clone().into_safe());
        result += ": ";

        // HACK
        // Turn all fields in a Response struct (except te id) into an Option to prevent unsanitary
        // response data from crashing serialization.
        if workaround_mode {
            if is_unsanitary(name)
                && !type_name.contains("Option<")
                && !result.ends_with("\tpub id:")
            {
                result += &format!("Option<{}>", type_name);
            }
        } else {
            result += &type_name;
        }
        result += ",\n";
    }
    result += "}\n";

    Some(result)
}

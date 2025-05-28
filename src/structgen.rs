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
pub fn generate(name: &str, schema: &Schema, workaround_mode: bool) -> Option<String> {
    let typ = match &schema.schema_kind {
        SchemaKind::Type(x) => x,
        _ => return None,
    };

    // Assemble struct string.
    let mut result =
        "#[derive(Serialize, Deserialize, Debug, Default, Clone)]\npub struct ".to_owned();
    result += name;

    // If not an ObjectType or an Array of objects, return None.
    match &typ {
        Type::Object(obj) => {
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

                // The NetBox schema may be incorrect and we can't rely on what we get as a response.
                // Therefore, we must make every response field nullable, even if it's technically not correct.
                if workaround_mode && !name.ends_with("Request") {
                    if prop_name == "id" {
                        result += &type_name;
                    } else if !type_name.contains("Option<") && !result.ends_with("\tpub id:") {
                        result += &format!("Option<{}>", type_name);
                    } else {
                        result += &type_name;
                    }
                } else {
                    result += &type_name;
                }
                result += ",\n";
            }

            result += "}\n";
        }
        Type::Array(obj) => {
            let p = obj.items.clone().unwrap().clone().unbox();
            // Assemble a field declaration in the struct.
            let type_name = bindgen::type_to_string(&p);

            // If the property has a description, prepend a doc string.
            if let ReferenceOr::Item(item) = &p {
                if let Some(desc) = &item.schema_data.description {
                    result += bindgen::make_comment(Some(desc.clone()), 1).as_str();
                }
            }
            result += "(pub ";
            result += &type_name;
            result += ");\n";
        }
        _ => {
            return None;
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use openapiv3::{Schema, SchemaKind, StringType, Type};

    #[test]
    fn test_generate_with_non_object_schema() {
        let schema = Schema {
            schema_data: Default::default(),
            schema_kind: SchemaKind::Type(Type::String(StringType {
                ..Default::default()
            })), // Not an object
        };
        let result = generate("InvalidStruct", &schema, false);
        assert_eq!(result, None);
    }

    // TODO: Expand these tests.
}

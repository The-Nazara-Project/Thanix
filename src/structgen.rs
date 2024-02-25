use crate::bindgen;
use check_keyword::CheckKeyword;
use openapiv3::{ReferenceOr, Schema, SchemaKind, Type};

pub fn gen(name: &str, schema: &Schema) -> Option<String> {
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
    let mut result = "#[derive(Serialize, Deserialize, Debug)]\npub struct ".to_owned();
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
        result += &type_name;
        result += ",\n";
    }
    result += "}\n";

    Some(result)
}

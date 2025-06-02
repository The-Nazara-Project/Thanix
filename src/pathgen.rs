//! Generate API request functions.

use crate::bindgen::{self, make_comment};
use check_keyword::CheckKeyword;
use convert_case::{Case, Casing};
use openapiv3::{Operation, Parameter, ParameterSchemaOrContent, PathItem, ReferenceOr};

pub fn generate(name: &str, path_item: &PathItem, debug: bool) -> Option<String> {
    let mut result = String::new();

    if let Some(op) = &path_item.get {
        result += gen_fn(name, "get", op, debug).as_str();
    }
    if let Some(op) = &path_item.put {
        result += gen_fn(name, "put", op, debug).as_str();
    }
    if let Some(op) = &path_item.post {
        result += gen_fn(name, "post", op, debug).as_str();
    }
    if let Some(op) = &path_item.delete {
        result += gen_fn(name, "delete", op, debug).as_str();
    }
    if let Some(op) = &path_item.options {
        result += gen_fn(name, "options", op, debug).as_str();
    }
    if let Some(op) = &path_item.head {
        result += gen_fn(name, "head", op, debug).as_str();
    }
    if let Some(op) = &path_item.patch {
        result += gen_fn(name, "patch", op, debug).as_str();
    }
    if let Some(op) = &path_item.trace {
        result += gen_fn(name, "trace", op, debug).as_str();
    }

    Some(result)
}

fn gen_fn(name: &str, op_type: &str, op: &Operation, debug: bool) -> String {
    let mut result = String::new();

    // Build function name.
    let fn_name = op
        .operation_id
        .clone()
        .unwrap_or(make_fn_name_from_path(name) + "_" + op_type);

    let mut fn_query_params = Vec::new();
    let mut fn_header_params = Vec::new();
    let mut fn_path_params = Vec::new();

    // Assign all parameters to their respective slots.
    for param in op.parameters.clone() {
        // Filter out only parameter items.
        let p = match param {
            ReferenceOr::Item(x) => x,
            _ => continue,
        };
        match p {
            // If we have a query, append as a field to the query struct.
            Parameter::Query { parameter_data, .. } => {
                // We only respect Schemas.
                let query_param_type = match &parameter_data.format {
                    ParameterSchemaOrContent::Schema(schema) => schema,
                    _ => continue,
                };
                // Format as a struct field.
                fn_query_params.push(format!(
                    "{}\tpub {}: Option<{}>,\n",
                    make_comment(parameter_data.description, 1),
                    parameter_data.name.into_safe(),
                    bindgen::type_to_string(query_param_type)
                ))
            }
            // If we have a header, append to the header params.
            Parameter::Header { parameter_data, .. } => {
                // We only respect Schemas.
                let header_param_type = match &parameter_data.format {
                    ParameterSchemaOrContent::Schema(schema) => schema,
                    _ => continue,
                };

                fn_header_params.push((
                    parameter_data.name.clone(),
                    bindgen::type_to_string(header_param_type),
                ));
            }
            // If we have a path, append to the path params.
            Parameter::Path { parameter_data, .. } => {
                // We only respect Schemas.
                let path_param_type = match &parameter_data.format {
                    ParameterSchemaOrContent::Schema(schema) => schema,
                    _ => continue,
                };

                fn_path_params.push(format!(
                    ", {}: {}",
                    parameter_data.name.into_safe(),
                    bindgen::type_to_string(path_param_type)
                ));
            }
            // TODO
            Parameter::Cookie { .. } => {
                todo!()
            }
        }
    }

    // Build the request body.
    let fn_request_type = match &op.request_body {
        Some(req) => match req {
            ReferenceOr::Item(x) => match x.content.get("application/json") {
                Some(media) => Some(bindgen::type_to_string(&media.schema.clone().unwrap())),
                None => None,
            },
            _ => None,
        },
        None => None,
    };

    // Build the query struct for this function if we have at least one parameter.
    let need_query = fn_query_params.len() > 0;
    let fn_query_name = fn_name.to_case(Case::Pascal) + "Query";
    let fn_query_struct = format!(
        "#[derive(Serialize, Deserialize, Debug, Default, Clone)]\npub struct {} {{\n{}\n}}\n",
        fn_query_name,
        fn_query_params.clone().into_iter().collect::<String>()
    );

    if need_query {
        result += &fn_query_struct;
    }

    // Build the response enum.
    let fn_response_name = fn_name.to_case(Case::Pascal) + "Response";
    result += "#[derive(Debug)]\npub enum ";
    result += &fn_response_name;
    result += " {\n";

    for (status, response) in &op.responses.responses {
        result += "\t";
        match response {
            ReferenceOr::Item(x) => {
                result += &format!("Http{}", status);
                if let Some(y) = &x.content.get("application/json") {
                    result += "(";
                    result += &bindgen::type_to_string(&y.schema.as_ref().unwrap());
                    result += ")";
                }
                result += ",\n";
            }
            _ => (),
        }
    }

    result += "\tOther(Response)\n";
    result += "}\n";

    // Build function description.
    result += &bindgen::make_comment(op.description.clone(), 0);

    // Build function declaration.
    result += "pub fn ";
    result += &fn_name;
    result += "(state: &ThanixClient";

    // Build the query arg.
    if need_query {
        result += ", query: ";
        result += &fn_query_name;
    }

    // Build the JSON arg.
    if let Some(x) = &fn_request_type {
        result += ", body: ";
        result += x;
    }

    // Build the path args.
    result += &fn_path_params.into_iter().collect::<String>();

    // Build the header args.
    result += &fn_header_params
        .iter()
        .map(|(name, ty)| format!(", header_{}: {}", name, ty))
        .collect::<String>();

    result += ") -> ";

    // Build the response type.
    result += "Result<";
    result += &fn_response_name;
    result += ", Error> {\n";

    // Build the function body.
    if need_query {
        result += "\tlet qstring = serde_qs::to_string(&query).unwrap();\n";
        result += "\tlet qstring_clean = remove_square_braces(&qstring);\n";
    }

    result += "\n\tlet mut r#request = state.client.";
    result += op_type;
    result += "(format!(\"{}";
    result += name;

    if need_query {
        result += "?{}";
    }
    result += "\", state.base_url";
    if need_query {
        result += ", qstring_clean";
    }
    result += "))\n";

    // Auth header.
    result +=
        "\t\t.header(\"Authorization\", format!(\"Token {}\", state.authentication_token));\n";

    // JSON body.
    if let Some(_) = &fn_request_type {
        result += "\tr#request = r#request.json(&body);\n";
    }
    fn_header_params
        .iter()
        .for_each(|(name, _)| result += &format!("\n.header(\"{}\", header_{})", &name, &name));

    if debug {
        result += "\teprint!(\"{:?} = \", &r#request);\n";
    }

    result += "\tlet r#response = r#request.send()?;\n";

    if debug {
        result += "\teprintln!(\"= {:?}\", &r#response);\n";
    }

    result += "\tmatch r#response.status().as_u16() {\n";

    // Match response code.
    for (status, response) in &op.responses.responses {
        match response {
            ReferenceOr::Item(x) => {
                if let Some(y) = &x.content.get("application/json") {
                    result += &format!(
                        "\t\t{} => {{ Ok({}::Http{}(r#response.json::<{}>()?)) }},\n",
                        status,
                        &fn_response_name,
                        status,
                        &bindgen::type_to_string(&y.schema.as_ref().unwrap())
                    );
                }
            }
            _ => (),
        }
    }

    // Unknown response code.
    result += "\t\tr#other_status => { Ok(";
    result += &fn_response_name;
    result += "::Other(r#response)) }\n\t}\n}\n";

    return result;
}

fn make_fn_name_from_path(input: &str) -> String {
    input.replace("/api/", "").replace('/', "_")
}

#[cfg(test)]
mod tests {
    use openapiv3::{Operation, PathItem};

    use super::*;

    #[test]
    fn test_generate_no_op() {
        let path_item = PathItem::default();
        let result = generate("/test", &path_item, false);
        assert_eq!(result, Some(String::new()));
    }

    #[test]
    fn test_generate_multi_op() {
        let mut path_item = PathItem::default();
        path_item.get = Some(Operation::default());
        path_item.post = Some(Operation::default());

        let result = generate("/test", &path_item, false);
        assert!(result.is_some());
        let output = result.unwrap();
        assert!(output.contains("get"));
        assert!(output.contains("post"));
    }

    #[test]
    fn test_generate_all_op() {
        let mut path_item = PathItem::default();
        path_item.get = Some(Operation::default());
        path_item.put = Some(Operation::default());
        path_item.post = Some(Operation::default());
        path_item.delete = Some(Operation::default());
        path_item.options = Some(Operation::default());
        path_item.head = Some(Operation::default());
        path_item.patch = Some(Operation::default());
        path_item.trace = Some(Operation::default());

        let result = generate("/test", &path_item, false);
        assert!(result.is_some());
        let output = result.unwrap();
        assert!(output.contains("get"));
        assert!(output.contains("put"));
        assert!(output.contains("post"));
        assert!(output.contains("delete"));
        assert!(output.contains("options"));
        assert!(output.contains("head"));
        assert!(output.contains("patch"));
        assert!(output.contains("trace"));
    }

    #[test]
    fn test_gen_fn_basic() {
        let operation = Operation::default();
        let result = gen_fn("/test", "get", &operation, false);
        assert!(result.contains("pub fn"));
        assert!(result.contains("get"));
    }

    #[test]
    fn test_make_fn_name_from_path() {
        let result = make_fn_name_from_path("/api/user/profile");
        assert_eq!(result, "user_profile");
    }
}

use crate::bindgen::{self, make_comment};
use check_keyword::CheckKeyword;
use convert_case::{Case, Casing};
use openapiv3::{Operation, Parameter, ParameterSchemaOrContent, PathItem, ReferenceOr};

pub fn gen(name: &str, path_item: &PathItem) -> Option<String> {
    let mut result = String::new();

    if let Some(op) = &path_item.get {
        result += gen_fn(name, "get", op).as_str();
    }
    if let Some(op) = &path_item.put {
        result += gen_fn(name, "put", op).as_str();
    }
    if let Some(op) = &path_item.post {
        result += gen_fn(name, "post", op).as_str();
    }
    if let Some(op) = &path_item.delete {
        result += gen_fn(name, "delete", op).as_str();
    }
    if let Some(op) = &path_item.options {
        result += gen_fn(name, "options", op).as_str();
    }
    if let Some(op) = &path_item.head {
        result += gen_fn(name, "head", op).as_str();
    }
    if let Some(op) = &path_item.patch {
        result += gen_fn(name, "patch", op).as_str();
    }
    if let Some(op) = &path_item.trace {
        result += gen_fn(name, "trace", op).as_str();
    }

    Some(result)
}

fn gen_fn(name: &str, op_type: &str, op: &Operation) -> String {
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
                    "{}\t{}: Option<{}>,\n",
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
        "#[derive(Serialize, Deserialize, Debug, Default)]\npub struct {} {{\n{}\n}}\n",
        fn_query_name,
        fn_query_params.clone().into_iter().collect::<String>()
    );

    if need_query {
        result += &fn_query_struct;
    }

    // Build the response enum.
    let fn_response_name = fn_name.to_case(Case::Pascal) + "Response";
    result += "#[derive(Serialize, Deserialize, Debug, Default)]\n#[serde(untagged)]\npub enum ";
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

    result += "\t#[default]\n";
    result += "\tNone\n";
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
    result += ", Error>";

    // Build the function body.
    result += " {\n\tlet r#response = state.client.";
    result += op_type;
    result += "(format!(\"{}";
    result += name;
    if need_query {
        result += "?{}";
    }
    result += "\", state.base_url";
    if need_query {
        result += ", serde_qs::to_string(&query).unwrap()";
    }
    result += "))\n";

    // Auth header.
    result += "\t\t.header(\"Authorization\", format!(\"Token {}\", state.authentication_token))\n";

    // JSON body.
    if let Some(_) = &fn_request_type {
        result += "\t\t.json(&body)\n";
    }
    fn_header_params
        .iter()
        .for_each(|(name, _)| result += &format!("\n.header(\"{}\", header_{})", &name, &name));
    result += "\t\t.send()?;\n";
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
    result += "\t\t_ => { Ok(";
    result += &fn_response_name;
    result += "::None) }\n\t}\n}\n";

    return result;
}

fn make_fn_name_from_path(input: &str) -> String {
    input.replace("/api/", "").replace('/', "_")
}

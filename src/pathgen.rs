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
    // Build description.
    let mut description = op.description.clone().unwrap_or(String::new());
    description = bindgen::make_comment(
        Some(format!("`{}`\n{}", op_type.to_uppercase(), description)),
        0,
    );

    // Build function name.
    let fn_name = op
        .operation_id
        .clone()
        .unwrap_or(make_fn_name_from_path(name) + "_" + op_type);

    let mut fn_struct_params = Vec::new();
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
                fn_struct_params.push(format!(
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

    // Build the query struct for this function.
    let fn_struct_name = fn_name.to_case(Case::Pascal) + "Query";
    let fn_struct = format!(
        "#[derive(Serialize, Deserialize, Debug)]\npub struct {} {{\n{}\n}}\n",
        fn_struct_name,
        fn_struct_params.into_iter().collect::<String>()
    );

    // Build the header args.
    let fn_header_args = &fn_header_params
        .iter()
        .map(|(name, ty)| format!(", header_{}: {}", name, ty))
        .collect::<String>();

    // Build the header calls.
    let fn_header = &fn_header_params
        .iter()
        .map(|(name, _)| format!("\n.header(\"{}\", header_{})", &name, &name))
        .collect::<String>();

    // Build the function body.
    let fn_body = format!(
        include_str!("templates/path.template"),
        op_type, name, fn_header
    );

    // Build the function args.
    let fn_path_args = fn_path_params.into_iter().collect::<String>();

    // TODO
    format!(
        "{}{}pub fn {}(state: &ThanixClient, query: {}{}{}) -> Result<Response, Error> {{\n{}\n}}\n",
        fn_struct, description, fn_name, fn_struct_name, fn_header_args, fn_path_args, fn_body
    )
}

fn make_fn_name_from_path(input: &str) -> String {
    input.replace("/api/", "").replace('/', "_")
}

use std::fs::File;
use std::io::BufReader;
use serde_json::{Value, Map};
use codegen::{Field, Scope, Variant};
use std::io::Write;
use convert_case::{Case, Casing};

fn main() {
    let file = File::open("protocol.json")
        .expect("Can't find protocol document");

    let buf = BufReader::new(file);

    let protocol: Value = serde_json::from_reader(buf)
        .expect("Can't parse protocol document");

    {
        let types = protocol["types"].as_object().unwrap();

        let mut types_decl = Scope::new();
        types_decl.import("serde", "Serialize");
        types_decl.import("serde", "Deserialize");
        types_decl.import("std::collections", "HashMap");
        types_decl.import("crate::errors", "SignaldError");

        let mut variants: Vec<Variant> = Vec::new();
        for version in vec!["v1", "v0"].iter() {
            let types = types[*version].as_object().unwrap();
            add_types(&mut types_decl, types, version, &mut variants);
        }
        let types_enum = types_decl.new_enum("SignaldTypes")
            .vis("pub")
            .derive("Serialize")
            .derive("Deserialize");
        types_enum.push_variant(Variant::new("SignaldError(SignaldError)"));
        types_enum.push_variant(Variant::new("NoResponse"));
        types_enum.push_variant(Variant::new("String(String)"));

        for variant in variants.drain(0..) {
            types_enum.push_variant(variant);
        }

        let mut source_file = File::create("src/types.rs")
            .expect("Can't create source file");

        source_file.write_all(types_decl.to_string().as_bytes())
            .expect("Failed to write to source file");
    }

    {
        let actions = protocol["actions"].as_object().unwrap();
        let actions = actions["v1"].as_object().unwrap();

        let mut actions_decl = Scope::new();
        actions_decl.import("crate::socket", "AsyncSocket");
        actions_decl.import("crate", "SocketError");
        actions_decl.import("crate::types", "*");
        actions_decl.import("uuid", "Uuid");
        actions_decl.import("crate::socket", "MessageCommon");
        actions_decl.import("crate::errors", "SignaldError");

        add_actions(&mut actions_decl, actions, "v1");

        actions_decl.new_struct("SocketWrapper")
            .vis("pub")
            .generic("T")
            .field("pub socket", "T");


        let mut source_file = File::create("src/actions.rs")
            .expect("Can't create source file");

        source_file.write_all(actions_decl.to_string().as_bytes())
            .expect("Failed to write to source file");
    }
}

fn add_actions(scope: &mut Scope, actions: &Map<String, Value>, version: &str) {
    let version = version.to_uppercase();
    let api_impl = scope.new_impl("SocketWrapper")
        .generic("T")
        .target_generic("T")
        .bound("T", "AsyncSocket");

    let mut lines = Vec::new();

    for (key, value) in actions.iter() {
        let request_type = value["request"].as_str().unwrap().to_owned() + &version;
        let response_type = match value["response"].as_str() {
            Some(response) => {
                match response {
                    "String" => Some(response.to_owned()),
                    _ => Some(response.to_owned() + &version)
                }
            },
            None => None,
        };

        lines.push(format!("    \"{}\" => {{", key));
        lines.push(format!("        if let SignaldTypes::{}(msg) = msg {{", request_type));
        lines.push(format!("            self.{}(msg, Some(id)).await", key));

        if let Some(response) = &response_type {
            lines.push(format!("                .map(|response| SignaldTypes::{}(response))", response));
        } else {
            lines.push("                .map(|_| SignaldTypes::NoResponse)".to_owned())
        }

        lines.push("        } else {".to_owned());
        lines.push("            Err(SocketError::General(\"Incorrect message type\"))".to_owned());
        lines.push("        }".to_owned());
        lines.push("    },".to_owned());

        let new_fn = api_impl
            .new_fn(key.as_str());

        if let Some(doc) = value.get("doc") {
            new_fn.doc(doc.as_str().unwrap());
        }

        new_fn
            .vis("pub")
            .set_async(true)
            .ret(format!(
                    "Result<{}, SocketError>",
                    match &response_type {
                        Some(response_type) => response_type.clone(),
                        None => String::from("()")
                    }
            ))
            .arg_mut_self()
            .arg("msg", &request_type)
            .arg("id", "Option<Uuid>")
            .line("let id = match id {")
            .line("    Some(id) => id,")
            .line("    None => Uuid::new_v4()")
            .line("};")
            .line("let msg = MessageCommon::new(")
            .line("    id.to_simple().to_string(),")
            .line(format!("    String::from(\"{}\"),", key))
            .line("    \"v1\".to_owned(),")
            .line("    msg")
            .line(");")
            .line("")
            .line("let mut msg = serde_json::to_vec(&msg).unwrap();")
            .line("msg.push(b'\\n');")
            .line("")
            .line("self.socket.write(&msg, &id).await?;")
            .line("let response = self.socket.get_response(id).await?;")
            .line("")
            .line("match response.get(\"error\") {")
            .line(
                match response_type {
                    Some(response_type) => {
                        format!("    None => Ok(serde_json::from_value::<{}>(response).unwrap()),", response_type)
                    },
                    None => String::from("    None => Ok(()),")
                }
            )
            .line("    Some(_) => Err(SocketError::Signald(serde_json::from_value::<SignaldError>(response).unwrap()))")
            .line("}");
    }

    let call_fn = api_impl.new_fn("remote_call")
        .set_async(true)
        .vis("pub")
        .arg_mut_self()
        .arg("api_fn", "&str")
        .arg("id", "Uuid")
        .arg("msg", "SignaldTypes")
        .ret("Result<SignaldTypes, SocketError>")
        .doc("Call api function indirectly from string key")
        .line("match api_fn {");

    for line in lines {
        call_fn.line(line);
    }
    call_fn.line("    _ => Err(SocketError::General(\"Unknown api function\"))");
    call_fn.line("}");
}

fn add_types(scope: &mut Scope, types: &Map<String, Value>, version: &str, variants: &mut Vec<Variant>) {
    for (key, value) in types.iter() {
        let type_name = key.to_owned() + &version.to_uppercase();

        // Get all types as enum variants
        variants.push(
            Variant::new(format!("{}({})", type_name, type_name).as_str())
        );

        let new_struct = scope
            .new_struct((key.to_owned() + version.to_uppercase().as_str()).as_str())
            .vis("pub")
            .derive("Serialize")
            .derive("Deserialize")
            .derive("Clone")
            .derive("Default");

        let value = value.as_object().unwrap();

        if let Some(doc) = value.get("doc") {
            new_struct.doc(doc.as_str().unwrap());
        }

        let fields = value
            .get("fields")
            .unwrap()
            .as_object()
            .unwrap();

        for (field, info) in fields.iter() {
            new_struct.push_field(
                get_field(
                    field,
                    info
                )
            );
        }
    }

}

fn get_field(field: &String, info: &Value) -> Field {
    let mut doc = Vec::new();
    let mut example = String::from("Example: ");

    if let Some(doc_text) = info["doc"].as_str() {
        doc.push(doc_text);
    }

    if let Some(example_text) = info["example"].as_str() {
        example.push_str(example_text);
        doc.push(example.as_str());
    }

    let new_field = match info["version"].as_str() {
        Some(version) => {
            let version = version.to_uppercase();
            info["type"].as_str().unwrap().to_owned() + version.as_str()
        },
        None => {
            get_type(info["type"].as_str().unwrap())
        }
    };

    let new_field = match info["list"].as_bool() {
        Some(list) => {
            if list {
                format!("Vec<{}>", new_field)
            } else {
                new_field
            }
        },
        None => new_field
    };

    let mut new_field = get_clean_field(
        field.as_str().clone(),
        &new_field
    );

    new_field.doc(doc);

    new_field
}

fn get_clean_field(name: &str, ty: &str) -> Field {
    let mut annotations = vec![r#"#[serde(skip_serializing_if = "Option::is_none")]"#.to_owned()];
    let converted = &name.to_case(Case::Snake);
    if converted.as_str() != name {
        annotations.push(format!(r#"#[serde(rename = "{}")]"#, name));
    }
    
    let name = String::from("pub ") + converted;
    let mut field = match name.as_str() {
        "pub async" => {
            let field = Field::new(
                "pub async_",
                format!("Option<{}>", ty)
            );
            annotations.push(r#"#[serde(rename = "async")]"#.to_owned());
            field
        },
        "pub type" => {
            let field = Field::new(
                "pub type_",
                format!("Option<{}>", ty)
            );
            annotations.push(r#"#[serde(rename = "type")]"#.to_owned());
            field
        },
        _ => {
            Field::new(
                name.as_str(),
                format!("Option<{}>", ty)
            )
        },
    };

    field.annotation(
        annotations.iter().map(|string| string.as_str()).collect()
    );
    field
}

fn get_type(type_name: &str) -> String {
    String::from(match type_name {
        "String" => "String",
        "int" => "i32",
        "Integer" => "i32",
        "long" => "i64",
        "Long" => "i64",
        "Map" => "HashMap<String, String>",
        "Object" => "HashMap<String, String>",
        "boolean" => "bool",
        "Boolean" => "bool",
        "UUID" => "String",
        _ => panic!("Failed to parse protocol doc: invalid type")
    })
}

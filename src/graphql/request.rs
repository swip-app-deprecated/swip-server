use std::io::Read;
use juniper::{Variables, InputValue};
use serde_json::{from_str, Value, Number, Map};
use rocket::{Request as RocketRequest, Data, Outcome};
use rocket::data::{self, FromData};
use rocket::http::{Status, ContentType};
use rocket::Outcome::*;

#[derive(Debug)]
pub struct Request {
    pub query: String,
    pub variables: Variables,
    pub operation_name: Option<String>
}

impl FromData for Request {
    type Error = String;

    fn from_data(
        req: &RocketRequest, data: Data
    ) -> data::Outcome<Self, String> {
        // Ensure the content type is correct before opening the data.
        let content_type = ContentType::new("application", "json");
        if req.content_type() != Some(content_type) {
            return Outcome::Forward(data);
        }

        // Read the data into a String.
        let mut string = String::new();
        if let Err(e) = data.open().read_to_string(&mut string) {
            return Failure((Status::InternalServerError, format!("{:?}", e)));
        }

        // Parse JSON body
        let json: Value = match from_str(&string[..]) {
            Ok(value) => value,
            Err(_) => return Failure(
                (Status::BadRequest, "Body isn't a valid JSON".to_string())
            )
        };

        // Extract the query string
        let query: String = match json["query"].as_str() {
            Some(value) => value.to_string(),
            _ => return Failure(
                (
                    Status::BadRequest,
                    "Body is missing value for `query`".to_string()
                )
            )
        };

        // Extract operation name
        let operation_name = match json["operationName"].as_str() {
            Some(value) => Some(value.to_string()),
            _ => None
        };

        // Extract variables
        let variables = convert_serde_value_to_variables(json);

        // Return successfully.
        Success(Request {
            query: query,
            variables: variables,
            operation_name: operation_name
        })
    }
}


fn convert_serde_value_to_variables(json: Value) -> Variables {
    match json["variables"].clone() {
        Value::Object(map) => {
            map
            .into_iter()
            .map(|(k, v)| (k, convert_serde_value_to_input_value(v)))
            .collect()
        },
        _ => Variables::new()
    }
}

fn convert_serde_value_to_input_value(value: Value) -> InputValue {
    match value {
        Value::Null => InputValue::null(),
        Value::Bool(val) => InputValue::boolean(val),
        Value::Number(val) => convert_serde_number_to_input_value_number(val),
        Value::String(val) => InputValue::string(val),
        Value::Array(val) => convert_serde_array_to_input_value_list(val),
        Value::Object(map) => convert_serde_object_to_input_value_object(map)
    }
}

fn convert_serde_number_to_input_value_number(number: Number)
    -> InputValue
{
    if number.is_i64() {
        match number.as_i64() {
            Some(int) => return InputValue::Int(int),
            _ => return InputValue::Null
        }
    }

    match number.as_f64() {
        Some(float) => InputValue::Float(float),
        _ => InputValue::Null
    }
}

fn convert_serde_array_to_input_value_list(array: Vec<Value>) -> InputValue {
    InputValue::list(
        array
        .into_iter()
        .map(convert_serde_value_to_input_value)
        .collect::<Vec<InputValue>>()
    )
}

fn convert_serde_object_to_input_value_object(map: Map<String, Value>)
    -> InputValue
{
    InputValue::object(
        map
        .into_iter()
        .map(|(k, v)| (k, convert_serde_value_to_input_value(v)))
        .collect()
    )
}

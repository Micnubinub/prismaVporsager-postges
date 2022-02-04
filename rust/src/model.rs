use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Serialize, RustcDecodable, RustcEncodable)]
pub struct Query {
    pub req_type: String,
    pub token: String,
    pub email: Option<String>,
    pub firebase_id: Option<String>,
    pub req: String,
    pub from: String,
}

#[derive(Debug, Deserialize, RustcDecodable, RustcEncodable)]
pub struct ShiftQuery {
    pub from: String,
    pub to: String,
    pub id: String,
}

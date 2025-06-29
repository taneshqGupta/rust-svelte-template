use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo{
    pub id: i32,
    pub descript: String,
    pub done: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewTodo{
    pub descript: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteResponse {
    pub success: bool,
    pub id: i32,
    pub message: String,
}

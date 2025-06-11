use super::api::ApiResponse;
use super::metric::Entry;
use super::task::Task;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response {
    pub current_task: Option<Task>,
}

impl From<Response> for ApiResponse<Response> {
    fn from(response: Response) -> Self {
        ApiResponse::new(true, response)
    }
}

impl From<Response> for HttpResponse {
    fn from(response: Response) -> Self {
        ApiResponse::new(true, response).into()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct Request {
    pub address: String,
    pub task_id: Option<String>,
    pub task_state: Option<String>,
    pub metrics: Option<Vec<Entry>>,
    #[serde(default)]
    pub version: Option<String>,
    pub timestamp: Option<u64>,
    #[serde(default)]
    pub p2p_id: String,
}

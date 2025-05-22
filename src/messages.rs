use crate::NRFCloudClient;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Default, Debug)]
pub struct ListMessagesParams {
    pub appId: Option<String>,
    pub deviceId: Option<String>,
    pub topic: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub pageLimit: Option<u32>,
    pub pageNextToken: Option<String>,
    pub pageSort: Option<String>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ListMessagesResponse {
    pub items: Option<Vec<serde_json::Value>>,
    pub pageNextToken: Option<String>,
    // Add more fields as needed based on actual API response
}

impl NRFCloudClient {
    pub async fn list_messages(
        &self,
        params: &ListMessagesParams,
    ) -> Result<ListMessagesResponse, reqwest::Error> {
        self.get_json_with_params("/messages", params).await
    }
}

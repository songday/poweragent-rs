use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
pub(crate) struct Workflow {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) canvas: String,
    // #[serde(skip_serializing)]
    // pub(crate) nodes: String,
    // pub(crate) valid: bool,
}

#[derive(Deserialize)]
pub(crate) struct CanvasCells {
    pub(crate) cells: Vec<CanvasCell>,
}

#[derive(Deserialize)]
pub(crate) struct CanvasCell {
    pub(crate) shape: String,
    pub(crate) data: Option<Node>,
    #[serde(flatten)]
    pub(in crate::workflow) extra: HashMap<String, Value>,
}

#[derive(Deserialize)]
#[serde(tag = "nodeType")]
pub(crate) enum Node {
    CronJobNode(CronJobNode),
    HttpReqNode(HttpReqNode),
    LlmNode(LlmNode),
}

#[derive(Deserialize, PartialEq, Eq)]
pub(crate) enum BranchType {
    GotoAnotherNode,
    Condition,
    InfoCollectedSuccessfully,
    EmailSentSuccessfully,
}

#[derive(Deserialize)]
pub(crate) struct Branch {
    #[serde(rename = "branchId")]
    pub(crate) branch_id: String,
    #[serde(rename = "branchName")]
    pub(crate) branch_name: String,
    #[serde(rename = "branchType")]
    pub(crate) branch_type: BranchType,
    #[serde(rename = "targetNodeId")]
    pub(crate) target_node_id: String,
    // #[serde(rename = "conditionGroup")]
    // pub(crate) condition_group: Vec<Vec<BranchCondition>>,
}

#[derive(Deserialize)]
pub(crate) struct CronJobNode {
    pub(crate) sec: String,
    pub(crate) min: String,
    pub(crate) hour: String,
    #[serde(rename = "dayOfMonth")]
    pub(crate) day_of_month: String,
    pub(crate) month: String,
    #[serde(rename = "dayOfWeek")]
    pub(crate) day_of_week: String,
    #[serde(rename = "timezoneOffsetMin")]
    pub(crate) timezone_offset_min: String,
    #[serde(rename = "triggerTimestampVarName")]
    pub(crate) trigger_timestamp_var_name: String,
}

#[derive(Deserialize)]
pub(crate) struct HttpReqNode {
    #[serde(rename = "reqInfo")]
    pub(crate) req_info: crate::util::http::HttpReqInfo,
    #[serde(rename = "statusCodeVarName")]
    pub(crate) status_code_var_name: String,
    #[serde(rename = "responseBodyVarName")]
    pub(crate) response_body_var_name: String,
}

#[derive(Deserialize)]
pub(crate) struct LlmNode {
    #[serde(rename = "nodeName")]
    pub(crate) node_name: String,
    #[serde(rename = "modelCategory")]
    pub(crate) model_category: String,
    #[serde(rename = "llmApiProvider")]
    pub(crate) llm_api_provider: String,
    #[serde(rename = "modelId")]
    pub(crate) model_id: String,
    #[serde(rename = "ollamaApiUrl")]
    pub(crate) ollama_api_url: String,
    #[serde(rename = "apiKey")]
    pub(crate) api_key: String,
    #[serde(rename = "systemPrompt")]
    pub(crate) system_prompt: String,
    #[serde(rename = "userPrompt")]
    pub(crate) user_prompt: String,
    #[serde(rename = "contextLength")]
    pub(crate) context_length: String,
    #[serde(rename = "timeoutMillis")]
    pub(crate) timeout_millis: String,
    #[serde(rename = "asyncReq")]
    pub(crate) async_req: String,
    #[serde(rename = "responseVarName")]
    pub(crate) response_var_name: String,
    #[serde(rename = "executionTimeMillisVarName")]
    pub(crate) execution_time_millis_var_name: String,
}

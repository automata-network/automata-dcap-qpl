use indexmap::IndexMap;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct EnclaveIdentity {
    #[serde(rename = "enclaveIdentity")]
    pub enclave_identity: IndexMap<String, Value>,
    pub signature: String,
}

#[derive(Debug, Deserialize)]
pub struct TcbInfo {
    #[serde(rename = "tcbInfo")]
    pub tcb_info: IndexMap<String, Value>,
    pub signature: String,
}

#[derive(Debug, Deserialize)]
pub struct TcbEvalDataNumber {
    #[serde(rename = "tcbEvaluationDataNumbers")]
    pub tcb_evaluation_data_numbers: IndexMap<String, Value>,
    pub signature: String,
}

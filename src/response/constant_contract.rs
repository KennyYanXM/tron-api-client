use serde::{Deserialize, Serialize};
use crate::response::transaction::Transaction;
use crate::response::transaction_info::Log;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConstantContractResponse {
    pub result: ResultInfo,
    pub energy_used: i64,
    pub constant_result: Vec<String>,
    pub logs: Vec<Log>,
    pub transaction: Transaction,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultInfo {
    pub result: bool,
}

#[derive(Debug, Deserialize)]
pub struct EstimateEnergyResponse {
    pub result: EstimateEnergyResult,
    pub energy_required: i64,
}

#[derive(Debug, Deserialize)]
pub struct EstimateEnergyResult {
    pub result: bool,
    pub code: String,
    pub message: String,
}

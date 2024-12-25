use serde_derive::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct AccountResource {
    #[serde(rename = "freeNetUsed")]
    pub free_net_used: i64,
    #[serde(rename = "freeNetLimit")]
    pub free_net_limit: i64,
    #[serde(rename = "NetUsed")]
    pub net_used: i64,
    #[serde(rename = "NetLimit")]
    pub net_limit: i64,
    #[serde(rename = "TotalNetLimit")]
    pub total_net_limit: i64,
    #[serde(rename = "TotalNetWeight")]
    pub total_net_weight: i64,
    #[serde(rename = "totalTronPowerWeight")]
    pub total_tron_power_weight: i64,
    #[serde(rename = "tronPowerLimit")]
    pub tron_power_limit: i64,
    #[serde(rename = "tronPowerUsed")]
    pub tron_power_used: i64,
    #[serde(rename = "EnergyUsed")]
    pub energy_used: i64,
    #[serde(rename = "EnergyLimit")]
    pub energy_limit: i64,
    #[serde(rename = "TotalEnergyLimit")]
    pub total_energy_limit: i64,
    #[serde(rename = "TotalEnergyWeight")]
    pub total_energy_weight: i64,
    #[serde(rename = "assetNetUsed")]
    pub asset_net_used: HashMap<String, i64>,
    #[serde(rename = "assetNetLimit")]
    pub asset_net_limit: HashMap<String, i64>,
} 
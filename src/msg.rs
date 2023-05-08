use cosmwasm_schema::cw_serde;

use crate::circom_proof::CircomProof;

#[cw_serde]
pub struct InstantiateMsg {
    pub min_age: String,
}

#[cw_serde]
pub enum ExecuteMsg {}

#[cw_serde]
pub enum QueryMsg {
    ProofAge {
        proof: CircomProof,
    }
}

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary};
use cw2::set_contract_version;
use chrono::{DateTime, Utc, NaiveDateTime};

use crate::circom_proof::CircomProof;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::verifier::{PublicSignals, Verifier};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-zk-base";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ProofAge{ proof } => to_binary(&proof_age(env, proof)?),
    }
}

fn proof_age(env: Env, proof: CircomProof) -> StdResult<bool> {
    let naive_datetime = NaiveDateTime::from_timestamp_opt(env.block.time.seconds() as i64, 0);

    match naive_datetime {
        Some(naive_datetime) => {
            let datetime: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc);
            let curr_date = datetime.format("%Y%m%d").to_string();
            let public_signals = PublicSignals::from_values(curr_date);
            let proof = proof.to_proof();
            let inputs = public_signals.get();
            
            return Ok(Verifier::new()
                .verify_proof(proof, &inputs));
        },
        None => {
            return Ok(false);
        }
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::{mock_dependencies, mock_info, mock_env}, from_binary, BlockInfo, Timestamp};
    use super::*;
        
    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        let env = Env {
            block: BlockInfo {
                height: 1,
                chain_id: String::from("2"),
                time: Timestamp::from_seconds(1683556139),
            },
            ..mock_env()
        };
        let msg = InstantiateMsg { 
            min_age: String::from("18")
        };
        let info = mock_info("creator", &[]);

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let naive_datetime = NaiveDateTime::from_timestamp_opt(env.block.time.seconds() as i64, 0).unwrap();
        let datetime: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc);
        let curr_date = datetime.format("%Y%m%d").to_string();
        println!("{}", curr_date);
        // it worked, let's query the state
        let proof = CircomProof::from(r#"{"pi_a":["17405346007370852090812173940571472089240177646443646965111091817914350431322","494947448847157459285956367074845651027552708911354114036087729617743357430","1"],"pi_b":[["3367167488907853508477413841074123861365597488800120674442924090857576745071","6400180945105117052748211084772301376035226442899178670042549162984424328320"],["16087784337286198044108728173085412230568111249402356775805869430770256749607","10664614094619432585161433072537883719597980678494161184318451947719026486663"],["1","0"]],"pi_c":["6032135842962151284544500496458533183797169339441000842179910745143273064697","7948864440914050499557664222210680959523697784032467161007360158813510949336","1"],"protocol":"groth16","curve":"bn128"}"#.to_string());
        let res = query(deps.as_ref(), env, QueryMsg::ProofAge { proof }).unwrap();
        let value: bool = from_binary(&res).unwrap();
        assert_eq!(value, true);
    }
}

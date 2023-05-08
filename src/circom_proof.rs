use std::str::FromStr;
use ark_bn254::{Bn254, G1Affine, G2Affine};
use ark_ff::{Fp256, QuadExtField};
use ark_groth16::Proof;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct CircomProof {
    #[serde(rename = "pi_a")]
    pub pi_a: Vec<String>,
    #[serde(rename = "pi_b")]
    pub pi_b: Vec<Vec<String>>,
    #[serde(rename = "pi_c")]
    pub pi_c: Vec<String>,
    pub protocol: String,
    pub curve: String,
}

impl CircomProof {
    pub fn from(json_str: String) -> Self {
        serde_json::from_str(&json_str).unwrap()
    }

    pub fn to_proof(self) -> Proof<Bn254> {
        let a = G1Affine::new(
            Fp256::from_str(&self.pi_a[0]).unwrap(),
            Fp256::from_str(&self.pi_a[1]).unwrap(),
            false,
        );
        let b = G2Affine::new(
            QuadExtField::new(
                Fp256::from_str(&self.pi_b[0][0]).unwrap(),
                Fp256::from_str(&self.pi_b[0][1]).unwrap(),
            ),
            QuadExtField::new(
                Fp256::from_str(&self.pi_b[1][0]).unwrap(),
                Fp256::from_str(&self.pi_b[1][1]).unwrap(),
            ),
            false,
        );

        let c = G1Affine::new(
            Fp256::from_str(&self.pi_c[0]).unwrap(),
            Fp256::from_str(&self.pi_c[1]).unwrap(),
            false,
        );
        Proof { a, b, c }
    }
}
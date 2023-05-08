use ark_groth16::{prepare_verifying_key, verify_proof, Proof, VerifyingKey};
use serde::{Deserialize, Serialize};

use ark_bn254::{Bn254, Fq, Fq2, Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use schemars::JsonSchema;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PublicSignals(pub Vec<String>);

// Public signals from circom
impl PublicSignals {
    pub fn from(public_signals: Vec<String>) -> Self {
        PublicSignals(public_signals)
    }
    pub fn from_values(current_date: String) -> Self {
        let signals: Vec<String> = vec![
            String::from("17117841954853285987668489547323623797569071287263030350231384311492698179645"),
            current_date,
        ];
        PublicSignals(signals)
    }
    pub fn from_json(public_signals_json: String) -> Self {
        let v: Vec<String> = serde_json::from_str(&public_signals_json).unwrap();
        PublicSignals(v)
    }

    pub fn get(self) -> Vec<Fr> {
        let mut inputs: Vec<Fr> = Vec::new();
        for input in self.0 {
            inputs.push(Fr::from_str(&input).unwrap());
        }
        inputs
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Verifier {
    vk_json: String,
}

#[allow(clippy::new_without_default)]
impl Verifier {
    pub fn new() -> Self {
        let vk_json = include_str!("../circuit/verification_key.json");

        Self {
            vk_json: vk_json.to_string(),
        }
    }

    pub fn verify_proof(self, proof: Proof<Bn254>, inputs: &[Fr]) -> bool {
        let vk_json: VerifyingKeyJson = serde_json::from_str(&self.vk_json).unwrap();

        let vk = vk_json.to_verifying_key();
        let pvk = prepare_verifying_key(&vk);

        verify_proof(&pvk, &proof, inputs).unwrap()
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct VerifyingKeyJson {
    #[serde(rename = "IC")]
    pub ic: Vec<Vec<String>>,
    pub vk_alpha_1: Vec<String>,
    pub vk_beta_2: Vec<Vec<String>>,
    pub vk_gamma_2: Vec<Vec<String>>,
    pub vk_delta_2: Vec<Vec<String>>,
    pub vk_alphabeta_12: Vec<Vec<Vec<String>>>,
}

impl VerifyingKeyJson {
    pub fn to_verifying_key(self) -> VerifyingKey<Bn254> {
        let alpha_g1 = G1Affine::from(G1Projective::new(
            str_to_fq(&self.vk_alpha_1[0]),
            str_to_fq(&self.vk_alpha_1[1]),
            str_to_fq(&self.vk_alpha_1[2]),
        ));
        let beta_g2 = G2Affine::from(G2Projective::new(
            // x
            Fq2::new(
                str_to_fq(&self.vk_beta_2[0][0]),
                str_to_fq(&self.vk_beta_2[0][1]),
            ),
            // y
            Fq2::new(
                str_to_fq(&self.vk_beta_2[1][0]),
                str_to_fq(&self.vk_beta_2[1][1]),
            ),
            // z,
            Fq2::new(
                str_to_fq(&self.vk_beta_2[2][0]),
                str_to_fq(&self.vk_beta_2[2][1]),
            ),
        ));

        let gamma_g2 = G2Affine::from(G2Projective::new(
            // x
            Fq2::new(
                str_to_fq(&self.vk_gamma_2[0][0]),
                str_to_fq(&self.vk_gamma_2[0][1]),
            ),
            // y
            Fq2::new(
                str_to_fq(&self.vk_gamma_2[1][0]),
                str_to_fq(&self.vk_gamma_2[1][1]),
            ),
            // z,
            Fq2::new(
                str_to_fq(&self.vk_gamma_2[2][0]),
                str_to_fq(&self.vk_gamma_2[2][1]),
            ),
        ));

        let delta_g2 = G2Affine::from(G2Projective::new(
            // x
            Fq2::new(
                str_to_fq(&self.vk_delta_2[0][0]),
                str_to_fq(&self.vk_delta_2[0][1]),
            ),
            // y
            Fq2::new(
                str_to_fq(&self.vk_delta_2[1][0]),
                str_to_fq(&self.vk_delta_2[1][1]),
            ),
            // z,
            Fq2::new(
                str_to_fq(&self.vk_delta_2[2][0]),
                str_to_fq(&self.vk_delta_2[2][1]),
            ),
        ));

        let gamma_abc_g1: Vec<G1Affine> = self
            .ic
            .iter()
            .map(|coords| {
                G1Affine::from(G1Projective::new(
                    str_to_fq(&coords[0]),
                    str_to_fq(&coords[1]),
                    str_to_fq(&coords[2]),
                ))
            })
            .collect();

        VerifyingKey::<Bn254> {
            alpha_g1,
            beta_g2,
            gamma_g2,
            delta_g2,
            gamma_abc_g1,
        }
    }
}

pub fn str_to_fq(s: &str) -> Fq {
    Fq::from_str(s).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::circom_proof::CircomProof;

    #[test]
    fn test_verifier() {
        let v = Verifier::new();

        let proof = CircomProof::from(r#"{"pi_a":["16787120696716752435605955071903451364503536426600026106560137368712802468368","3136285615097585417623821110268581305398806259380922853792981741944761326493","1"],"pi_b":[["4475918907801943822757113887350117905852992033995115252320487339833520457860","8998000599227712566109026913032347552774700811046072537557808781244704382069"],["18319865981758543939313944468626719214462188242931104853080446850528811402262","6970954525007895378105325311784501411486453077509433345936316823813673412919"],["1","0"]],"pi_c":["8152508171278180192142638115902762144994178788479042671048722200701454668766","16494639112777074728035684281527064025944434237740203672755403439137947500700","1"],"protocol":"groth16","curve":"bn128"}"#.to_string())
            .to_proof();
        let public_signals = PublicSignals::from_json(r#"["17117841954853285987668489547323623797569071287263030350231384311492698179645","20230508"]"#.to_string());

        let res = v.verify_proof(proof, &public_signals.get());

        assert!(res);
    }
}

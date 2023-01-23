use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen};

const DEFAULT_SIGNATURE: &str = "<IPFS Hash to uploaded signature>";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    signature_id: String
}

impl Default for Contract{
    fn default() -> Self{
        Self { signature_id: DEFAULT_SIGNATURE.to_string()}
    }
}

#[near_bindgen]
impl Contract {
    pub fn get_signature(&self) -> String {
        return self.signature_id.clone();
    }

    pub fn set_signature(&mut self, signature_id: String) {
        log!("Saving signature {}", signature_id);
        self.signature_id = signature_id;
    }
}

#[cfg(test)]
mod tests {}

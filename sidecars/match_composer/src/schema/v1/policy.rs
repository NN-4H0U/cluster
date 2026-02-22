use serde::{Deserialize, Serialize};
use crate::schema::Schema;

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PolicyV1 {
    Bot {
        image: String
    },
    Agent,
}

impl PolicyV1 {
    pub fn bot(image: String) -> PolicyV1 {
        PolicyV1::Bot {
            image,
        }
    }

    pub fn agent() -> PolicyV1 {
        PolicyV1::Agent
    }
}

impl Schema for PolicyV1 {
    fn verify(&self) -> Result<(), &'static str> {
        let policy = match self {
            PolicyV1::Bot { image } => image,
            PolicyV1::Agent => return Ok(()),
        };
        
        let mut res = policy.split('/');
        if let Some(provider) = res.next() {
            if let Some(_policy_name) = res.next() {
                return Ok(())
            }

            if provider == "*" {
                return Ok(())
            }

            return Err(r"Invalid policy name, should be in pattern /^\w+/(\w+|\*):?w*?$/")
        }
        
        Ok(())
    }
}

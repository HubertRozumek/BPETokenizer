use std::collections::HashMap;
use serde::{Serialize,Deserialize};


pub struct MergeRules {
    pub rules: HashMap<(String, String), usize>,
}

impl MergeRules {
    pub fn should_merge(&self, left: &str, right:&str) -> bool {
        self.rules.contains_key(&(left.to_string(), right.to_string()))
    }

    pub fn apply_merges(&self, mut tokens: Vec<String>) -> Vec<String> {
        loop {
            let mut merged = false;
            let mut i = 0;

            while i < tokens.len() - 1 {
                if self.should_merge(&tokens[i], &tokens[i + 1]) {
                    let merged_token = format!("{}{}", tokens[i],tokens[i+1]);
                    tokens.splice(i..=i+1, [merged_token]);
                    merged = true;
                } else {
                    i += 1;
                }
            }

            if !merged {
                break;
            }
        }
        tokens
    }
}


#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn test_apply_merges(){
        let mut rules = HashMap::new();
        rules.insert(("l".to_string(), "o".to_string()), 0);
        rules.insert(("lo".to_string(), "w".to_string()),1);

        let merge_rules = MergeRules {rules};

        let tokens = vec![
            "l".to_string(),
            "o".to_string(),
            "w".to_string(),
            "e".to_string(),
            "r".to_string(),
        ];

        let merged = merge_rules.apply_merges(tokens);

        assert_eq!(merged, vec!["low","e","r"])
    } 
}
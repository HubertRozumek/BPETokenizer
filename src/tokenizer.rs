use crate::vocab::Vocab;
use crate::merges::MergeRules;
use lru::LruCache;
use std::num::NonZeroUsize;


pub struct BPETokenizer {
    pub vocab: Vocab,
    pub merges: MergeRules,
    pub cache: LruCache<String, Vec<String>>,
}

impl BPETokenizer{

    pub fn new(vocab: Vocab, merges: MergeRules) -> Self{
        let cache = LruCache::new(NonZeroUsize::new(10_000).unwrap());
        Self {
             vocab: (vocab),
             merges: (merges),
             cache: (cache) 
            }
    }

    pub fn tokenize(&mut self, word: &str) -> Vec<String> {
        if let Some(cached) = self.cache.get(word) { return cached.clone(); }

        let chars: Vec<String> = word.chars().map(|c| c.to_string()).collect();
        let merged = self.merges.apply_merges(chars);
        self.cache.put(word.to_string(), merged.clone());

        merged
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_tokenizer_b() {
        let mut vocab = Vocab::new();
        vocab.add_token("l");
        vocab.add_token("o");
        vocab.add_token("w");
        vocab.add_token("e");
        vocab.add_token("r");

        let mut rules = HashMap::new();
        rules.insert(("l".to_string(), "o".to_string()),0);
        rules.insert(("lo".to_string(), "w".to_string()),1);

        let merge_rules = MergeRules { rules };

        let mut tokenizer = BPETokenizer::new(vocab,merge_rules);

        let tokens = tokenizer.tokenize("lower");
        assert_eq!(tokens, vec!["low","e","r"]);

    }
}
use std::collections::HashMap;

pub struct Utilities;

impl Utilities {
    pub fn invert_map<K, V>(input: &HashMap<K, V>) -> HashMap<V, Vec<K>>
    where
        K: Eq + std::hash::Hash + Clone,
        V: Eq + std::hash::Hash + Clone,
    {
        let mut output: HashMap<V, Vec<K>> = HashMap::new();

        for (k, v) in input {
            output.entry(v.clone()).or_default().push(k.clone());
        }

        output
    }
}



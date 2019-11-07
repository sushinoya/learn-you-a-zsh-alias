extern crate shells;

use shells::zsh;
use std::collections::HashMap;
use std::env::args;

struct AliasMapping {
    alias: String,
    full: String
}

impl AliasMapping {
    fn new(alias_line: &str) -> AliasMapping {
        let alias_mapping: Vec<&str> = alias_line.splitn(2, '=').collect();
        AliasMapping {
            alias: alias_mapping[0].to_string(),
            full: alias_mapping[1].trim_matches('\'').to_string()
        }
    }
}

fn get_aliases() -> Vec<AliasMapping> {
    let (_, stdout, _) = zsh!("source ~/.zshrc; alias");
    let alias_list: Vec<&str> = stdout.trim().split('\n').collect();
    let alias_list: Vec<AliasMapping> = alias_list.iter().map(|x| AliasMapping::new(&x)).collect();
    return alias_list
}

fn main() {
    let args: Vec<String> = args().collect();
    let aliases = get_aliases();
    let mut cmd_to_alias = HashMap::new();

    for mapping in aliases {
        cmd_to_alias.insert(mapping.full, mapping.alias);
    }

    if cmd_to_alias.contains_key(&args[1]) {
        println!("Next time, try {}", cmd_to_alias.get(&args[1]).unwrap().to_string())
    }
}
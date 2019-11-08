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

fn format_tip(s: &str, prefix: &str) -> String {
    let color_blue_normal = "\033[94m";
    let color_blue_bold = "\033[1;94m";
    let color_reset = "\033[0m";
    return format!("{}{}{}{}{}", color_blue_normal, prefix, color_blue_bold, s, color_reset);
}


fn expand_input(input: &str, alias_mappings: Vec<AliasMapping>) -> String {
    let command = String::from(input);
    let mut max_exp = 0;
    let mut max_expanded: Option<String> = None;

    for mapping in alias_mappings {
        if command.starts_with(&format!("{} ", mapping.alias)) && 
            mapping.full.len() > mapping.alias.len() &&
            mapping.full.len() > max_exp {
            max_expanded = Some(command.replacen(&mapping.alias, &mapping.full, 1));
            max_exp = mapping.full.len();
        }
    }
   
   return max_expanded.unwrap_or(command)
}

// def find_alias(aliases, input):
//     aliases.sort(key=lambda x: len(x[1]), reverse=True)

//     res_prev, res = None, input

//     while res_prev != res:
//         res_prev = res
//         for alias, expanded in aliases:
//             if res == expanded or res.startswith(expanded + ' '):
//                 idx = len(expanded)
//                 res = alias + res[idx:]

//     return res





fn main() {
    println!("{}", format_tip("suyash", "shekhar"));
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
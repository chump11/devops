use std::{fs, os::unix::fs::PermissionsExt};

use serde::Deserialize;
use glob::glob;

const JSON : &str = include_str!("../rules.json");

#[derive(Deserialize, Debug)]
struct ComplianceRule {
    path_regex: String,
    file_permissions:  u32,
    required_files: Vec<String>
}

impl ComplianceRule {
    fn new(path_regex: String, file_permissions:u32, required_files: Vec<String>) -> Self {
        Self {
            path_regex,
            file_permissions,
            required_files
        }
    }
    
}

fn load_rules() -> Vec<ComplianceRule> {
    let loaded_json: Vec<ComplianceRule> = serde_json::from_str(JSON).unwrap();
    let mut rules: Vec<ComplianceRule> = Vec::new();
    for rule in loaded_json {
        rules.push(ComplianceRule::new(rule.path_regex, rule.file_permissions, rule.required_files));
    }
    rules
}
fn parse_glob(rule: ComplianceRule)  {
    println!("{:?}", rule);
    let mut failed = false;
    let mut seen_files: Vec<String> = Vec::new();
   for entry in glob(&rule.path_regex).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                 seen_files.push(path.to_str().unwrap().to_string());
                let metadata = fs::metadata(&path).unwrap();
                if metadata.permissions().mode() != rule.file_permissions {
                    failed = true;
                    println!("File permissions do not match {:?}", path);
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
    for file in &rule.required_files {
       failed = true;
        if !seen_files.contains(&file) {
            println!("Required file does not exist");
        }
    }
    if failed {
       std::process::exit(1);
    }   
}

fn main() {
   let rules = load_rules();
   println!("{:?}", rules);
    for rule in rules {
         parse_glob(rule);
    }
     //
  // println!("{:?}", rules);
}

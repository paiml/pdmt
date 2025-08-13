#![no_main]
use libfuzzer_sys::fuzz_target;
use pdmt::models::todo::TodoList;

fuzz_target!(|data: &[u8]| {
    // Try to parse as YAML
    let yaml_str = String::from_utf8_lossy(data);
    
    // Try to deserialize as TodoList
    let _ = serde_yaml::from_str::<TodoList>(&yaml_str);
    
    // Try to deserialize as generic Value first
    if let Ok(value) = serde_yaml::from_str::<serde_yaml::Value>(&yaml_str) {
        // Try to convert to JSON
        let _ = serde_json::to_string(&value);
    }
    
    // Try parsing as template parameters
    let _ = serde_yaml::from_str::<std::collections::HashMap<String, serde_json::Value>>(&yaml_str);
});
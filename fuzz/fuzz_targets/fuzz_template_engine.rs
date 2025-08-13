#![no_main]
use libfuzzer_sys::fuzz_target;
use pdmt::template::{TemplateEngine, TemplateDefinition};
use serde_json::json;

fuzz_target!(|data: &[u8]| {
    // Create engine
    let mut engine = TemplateEngine::new();
    
    // Try to parse data as template definition
    if data.len() < 10 || data.len() > 100000 {
        return;
    }
    
    let template_str = String::from_utf8_lossy(data);
    
    // Create a simple template
    let template = TemplateDefinition::new(
        "fuzz_template",
        "1.0.0",
        &template_str,
    );
    
    // Try to register template
    let _ = engine.register_template(template);
    
    // Try to generate with random input
    let input = json!({
        "name": "fuzz",
        "value": template_str.chars().take(100).collect::<String>(),
    });
    
    // Try to generate (async in sync context)
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    
    let _ = runtime.block_on(async {
        engine.generate("fuzz_template", input).await
    });
});
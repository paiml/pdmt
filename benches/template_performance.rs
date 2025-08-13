//! Template performance benchmarks

use criterion::{criterion_group, criterion_main, Criterion};
use pdmt::{models::todo::TodoInput, TemplateEngine};
use tokio::runtime::Runtime;

fn bench_todo_generation(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("todo_generation", |b| {
        b.iter(|| {
            rt.block_on(async {
                let mut engine = TemplateEngine::new();
                engine.load_builtin_templates().await.unwrap();

                let input = TodoInput {
                    project_name: "Benchmark Project".to_string(),
                    requirements: vec![
                        "Implement authentication".to_string(),
                        "Create REST API".to_string(),
                        "Add database layer".to_string(),
                        "Write tests".to_string(),
                    ],
                    granularity: pdmt::models::todo::TodoGranularity::High,
                    project_context: None,
                    quality_config: None,
                    max_todos: Some(20),
                    include_estimates: true,
                    default_priority: None,
                };

                engine.generate("todo_list", input).await.unwrap()
            })
        });
    });
}

criterion_group!(benches, bench_todo_generation);
criterion_main!(benches);

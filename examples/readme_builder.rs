//! README Builder Example
//!
//! This example demonstrates using PDMT's deterministic templating system
//! to generate consistent, well-structured README.md files for projects.
//! The deterministic nature ensures standardized documentation across projects.
//!
//! Run with: cargo run --example readme_builder --features="full"

use clap::Parser;
use pdmt::template::{definition::TemplateDefinition, engine::TemplateEngine};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(name = "readme-builder")]
#[command(about = "Generate deterministic README.md files from structured templates")]
struct Args {
    /// Project name
    #[arg(short = 'n', long)]
    name: Option<String>,

    /// Project description
    #[arg(short = 'd', long)]
    description: Option<String>,

    /// Primary programming language
    #[arg(short = 'l', long)]
    language: Option<String>,

    /// License type (MIT, Apache-2.0, GPL-3.0, etc.)
    #[arg(long, default_value = "MIT")]
    license: String,

    /// GitHub username or organization
    #[arg(short = 'u', long)]
    github_user: Option<String>,

    /// Repository name
    #[arg(short = 'r', long)]
    repo: Option<String>,

    /// Include badges (CI, coverage, version, etc.)
    #[arg(short = 'b', long)]
    badges: bool,

    /// Include installation section
    #[arg(long, default_value = "true")]
    installation: bool,

    /// Include contributing section
    #[arg(long, default_value = "true")]
    contributing: bool,

    /// Interactive mode
    #[arg(short = 'i', long)]
    interactive: bool,

    /// Output file path
    #[arg(short = 'o', long, default_value = "README.md")]
    output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ReadmeInput {
    project: ProjectInfo,
    badges: Vec<Badge>,
    sections: ReadmeSections,
    features: Vec<Feature>,
    installation: InstallationInfo,
    usage: UsageInfo,
    api: Option<ApiDocumentation>,
    testing: TestingInfo,
    contributing: ContributingInfo,
    license: LicenseInfo,
    acknowledgements: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProjectInfo {
    name: String,
    description: String,
    version: String,
    language: String,
    github_user: String,
    repo_name: String,
    documentation_url: Option<String>,
    homepage: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Badge {
    name: String,
    url: String,
    image_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ReadmeSections {
    include_toc: bool,
    include_features: bool,
    include_installation: bool,
    include_usage: bool,
    include_api: bool,
    include_testing: bool,
    include_contributing: bool,
    include_license: bool,
    include_acknowledgements: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Feature {
    emoji: String,
    title: String,
    description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InstallationInfo {
    package_manager: String,
    install_command: String,
    requirements: Vec<String>,
    optional_features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UsageInfo {
    quick_start: String,
    basic_example: CodeExample,
    advanced_examples: Vec<CodeExample>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CodeExample {
    title: String,
    language: String,
    code: String,
    description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ApiDocumentation {
    main_modules: Vec<String>,
    key_functions: Vec<String>,
    documentation_link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestingInfo {
    test_command: String,
    coverage_command: Option<String>,
    lint_command: Option<String>,
    benchmark_command: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ContributingInfo {
    guidelines_url: Option<String>,
    code_of_conduct_url: Option<String>,
    issue_template: bool,
    pr_template: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LicenseInfo {
    license_type: String,
    copyright_holder: String,
    year: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("📚 PDMT README Builder");
    println!("Generate deterministic, well-structured README files\n");

    // Create template engine
    let mut engine = TemplateEngine::new();

    // Define README template
    let readme_template = create_readme_template();
    engine.register_template(readme_template)?;

    // Get README input
    let readme_input = if args.interactive {
        get_interactive_input()?
    } else {
        get_input_from_args(&args)
    };

    // Generate README using deterministic template
    println!("🔄 Generating README.md...");
    let start = std::time::Instant::now();

    let input_json = serde_json::to_value(&readme_input)?;
    let result = engine.generate("readme_template", input_json).await?;

    let duration = start.elapsed();
    println!("✅ README generated in {:?}\n", duration);

    // Write to file
    std::fs::write(&args.output, &result.content)?;
    println!("💾 README saved to: {}", args.output);

    // Show preview
    println!("\n📄 Preview (first 20 lines):\n");
    for line in result.content.lines().take(20) {
        println!("{}", line);
    }
    println!("...\n");

    Ok(())
}

fn create_readme_template() -> TemplateDefinition {
    let template_content = r#"# {{project.name}}

{{#if badges}}
{{#each badges}}[![{{name}}]({{image_url}})]({{url}}) {{/each}}
{{/if}}

{{project.description}}

{{#if sections.include_toc}}
## 📑 Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
{{#if sections.include_api}}- [API Documentation](#api-documentation){{/if}}
- [Testing](#testing)
- [Contributing](#contributing)
- [License](#license)
{{#if sections.include_acknowledgements}}- [Acknowledgements](#acknowledgements){{/if}}
{{/if}}

{{#if sections.include_features}}
## ✨ Features

{{#each features}}
- **{{emoji}} {{title}}**: {{description}}
{{/each}}
{{/if}}

{{#if sections.include_installation}}
## 📦 Installation

### Requirements

{{#each installation.requirements}}
- {{this}}
{{/each}}

### Using {{installation.package_manager}}

```bash
{{installation.install_command}}
```

{{#if installation.optional_features}}
### Optional Features

{{#each installation.optional_features}}
- `{{this}}`
{{/each}}
{{/if}}
{{/if}}

{{#if sections.include_usage}}
## 🚀 Usage

### Quick Start

{{usage.quick_start}}

### Basic Example

{{#if usage.basic_example.description}}{{usage.basic_example.description}}{{/if}}

```{{usage.basic_example.language}}
{{usage.basic_example.code}}
```

{{#if usage.advanced_examples}}
### Advanced Examples

{{#each usage.advanced_examples}}
#### {{title}}

{{#if description}}{{description}}{{/if}}

```{{language}}
{{code}}
```

{{/each}}
{{/if}}
{{/if}}

{{#if sections.include_api}}
{{#if api}}
## 📖 API Documentation

### Main Modules

{{#each api.main_modules}}
- `{{this}}`
{{/each}}

### Key Functions

{{#each api.key_functions}}
- `{{this}}`
{{/each}}

For complete API documentation, see [{{api.documentation_link}}]({{api.documentation_link}})
{{/if}}
{{/if}}

{{#if sections.include_testing}}
## 🧪 Testing

Run tests with:

```bash
{{testing.test_command}}
```

{{#if testing.coverage_command}}
### Coverage

```bash
{{testing.coverage_command}}
```
{{/if}}

{{#if testing.lint_command}}
### Linting

```bash
{{testing.lint_command}}
```
{{/if}}

{{#if testing.benchmark_command}}
### Benchmarks

```bash
{{testing.benchmark_command}}
```
{{/if}}
{{/if}}

{{#if sections.include_contributing}}
## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines]({{#if contributing.guidelines_url}}{{contributing.guidelines_url}}{{else}}CONTRIBUTING.md{{/if}}) for details.

{{#if contributing.code_of_conduct_url}}
Please note that this project is released with a [Code of Conduct]({{contributing.code_of_conduct_url}}). By participating in this project you agree to abide by its terms.
{{/if}}

### Development Setup

```bash
git clone https://github.com/{{project.github_user}}/{{project.repo_name}}
cd {{project.repo_name}}
{{#if installation.install_command}}{{installation.install_command}}{{/if}}
{{testing.test_command}}
```
{{/if}}

{{#if sections.include_license}}
## 📄 License

This project is licensed under the {{license.license_type}} License - see the [LICENSE](LICENSE) file for details.

Copyright © {{license.year}} {{license.copyright_holder}}
{{/if}}

{{#if sections.include_acknowledgements}}
{{#if acknowledgements}}
## 🙏 Acknowledgements

{{#each acknowledgements}}
- {{this}}
{{/each}}
{{/if}}
{{/if}}

---

<p align="center">
Built with ❤️ using <a href="https://github.com/paiml/pdmt">PDMT</a>
</p>"#;

    TemplateDefinition::new(
        "readme_template",
        "1.0.0",
        template_content,
    )
}

fn get_input_from_args(args: &Args) -> ReadmeInput {
    let project_name = args.name.clone().unwrap_or_else(|| "my-project".to_string());
    let github_user = args.github_user.clone().unwrap_or_else(|| "username".to_string());
    let repo_name = args.repo.clone().unwrap_or_else(|| project_name.clone());
    let language = args.language.clone().unwrap_or_else(|| "rust".to_string());
    
    let badges = if args.badges {
        vec![
            Badge {
                name: "CI".to_string(),
                url: format!("https://github.com/{}/{}/actions", github_user, repo_name),
                image_url: format!("https://github.com/{}/{}/workflows/CI/badge.svg", github_user, repo_name),
            },
            Badge {
                name: "Crates.io".to_string(),
                url: format!("https://crates.io/crates/{}", repo_name),
                image_url: format!("https://img.shields.io/crates/v/{}.svg", repo_name),
            },
            Badge {
                name: "Documentation".to_string(),
                url: format!("https://docs.rs/{}", repo_name),
                image_url: format!("https://docs.rs/{}/badge.svg", repo_name),
            },
            Badge {
                name: format!("License: {}", args.license),
                url: "https://opensource.org/licenses/MIT".to_string(),
                image_url: format!("https://img.shields.io/badge/License-{}-yellow.svg", args.license),
            },
        ]
    } else {
        vec![]
    };

    ReadmeInput {
        project: ProjectInfo {
            name: project_name.clone(),
            description: args.description.clone().unwrap_or_else(|| 
                "A powerful, efficient solution built with modern best practices.".to_string()
            ),
            version: "1.0.0".to_string(),
            language: language.clone(),
            github_user: github_user.clone(),
            repo_name: repo_name.clone(),
            documentation_url: Some(format!("https://docs.rs/{}", repo_name)),
            homepage: Some(format!("https://github.com/{}/{}", github_user, repo_name)),
        },
        badges,
        sections: ReadmeSections {
            include_toc: true,
            include_features: true,
            include_installation: args.installation,
            include_usage: true,
            include_api: language == "rust",
            include_testing: true,
            include_contributing: args.contributing,
            include_license: true,
            include_acknowledgements: false,
        },
        features: vec![
            Feature {
                emoji: "🚀".to_string(),
                title: "High Performance".to_string(),
                description: "Optimized for speed and efficiency".to_string(),
            },
            Feature {
                emoji: "🛡️".to_string(),
                title: "Type Safe".to_string(),
                description: "Comprehensive type checking and validation".to_string(),
            },
            Feature {
                emoji: "📚".to_string(),
                title: "Well Documented".to_string(),
                description: "Extensive documentation and examples".to_string(),
            },
            Feature {
                emoji: "🧪".to_string(),
                title: "Thoroughly Tested".to_string(),
                description: "Comprehensive test coverage and quality assurance".to_string(),
            },
        ],
        installation: InstallationInfo {
            package_manager: if language == "rust" { "Cargo".to_string() } else { "npm".to_string() },
            install_command: if language == "rust" {
                format!("cargo add {}", repo_name)
            } else {
                format!("npm install {}", repo_name)
            },
            requirements: vec![
                if language == "rust" { "Rust 1.70+".to_string() } else { "Node.js 18+".to_string() },
                "Git".to_string(),
            ],
            optional_features: if language == "rust" {
                vec!["full".to_string(), "async".to_string()]
            } else {
                vec![]
            },
        },
        usage: UsageInfo {
            quick_start: format!("Get started with {} in minutes!", project_name),
            basic_example: CodeExample {
                title: "Basic Usage".to_string(),
                language: language.clone(),
                code: if language == "rust" {
                    format!(r#"use {};

fn main() {{
    println!("Hello from {{}}!", "{}");
}}"#, repo_name, repo_name)
                } else {
                    format!(r#"const {} = require('{}');

{}.doSomething();"#, repo_name, repo_name, repo_name)
                },
                description: Some("Here's a simple example to get you started:".to_string()),
            },
            advanced_examples: vec![
                CodeExample {
                    title: "Advanced Configuration".to_string(),
                    language: language.clone(),
                    code: if language == "rust" {
                        r#"use my_project::{Config, Engine};

let config = Config::builder()
    .with_option("value")
    .build()?;

let engine = Engine::new(config);
engine.run().await?;"#.to_string()
                    } else {
                        r#"const { configure } = require('my-project');

const instance = configure({
  option: 'value',
  advanced: true
});

await instance.run();"#.to_string()
                    },
                    description: Some("Configure advanced features:".to_string()),
                },
            ],
        },
        api: if language == "rust" {
            Some(ApiDocumentation {
                main_modules: vec![
                    format!("{}::core", repo_name),
                    format!("{}::utils", repo_name),
                    format!("{}::config", repo_name),
                ],
                key_functions: vec![
                    "new() -> Self".to_string(),
                    "configure(config: Config) -> Result<Self>".to_string(),
                    "run() -> Result<()>".to_string(),
                ],
                documentation_link: format!("https://docs.rs/{}", repo_name),
            })
        } else {
            None
        },
        testing: TestingInfo {
            test_command: if language == "rust" { 
                "cargo test".to_string() 
            } else { 
                "npm test".to_string() 
            },
            coverage_command: Some(if language == "rust" {
                "cargo tarpaulin --out Html".to_string()
            } else {
                "npm run coverage".to_string()
            }),
            lint_command: Some(if language == "rust" {
                "cargo clippy".to_string()
            } else {
                "npm run lint".to_string()
            }),
            benchmark_command: if language == "rust" {
                Some("cargo bench".to_string())
            } else {
                None
            },
        },
        contributing: ContributingInfo {
            guidelines_url: Some("CONTRIBUTING.md".to_string()),
            code_of_conduct_url: Some("CODE_OF_CONDUCT.md".to_string()),
            issue_template: true,
            pr_template: true,
        },
        license: LicenseInfo {
            license_type: args.license.clone(),
            copyright_holder: github_user,
            year: "2025".to_string(),
        },
        acknowledgements: None,
    }
}

fn get_interactive_input() -> Result<ReadmeInput, Box<dyn std::error::Error>> {
    use dialoguer::{Input, Select, Confirm};
    
    println!("🎯 Let's build your README interactively!\n");
    
    let name: String = Input::new()
        .with_prompt("Project name")
        .interact()?;
    
    let description: String = Input::new()
        .with_prompt("Project description")
        .interact()?;
    
    let languages = &["rust", "javascript", "python", "go", "java", "other"];
    let language_idx = Select::new()
        .with_prompt("Primary language")
        .items(languages)
        .default(0)
        .interact()?;
    let language = languages[language_idx].to_string();
    
    let include_badges = Confirm::new()
        .with_prompt("Include badges (CI, version, etc.)?")
        .default(true)
        .interact()?;
    
    let github_user: String = Input::new()
        .with_prompt("GitHub username/organization")
        .interact()?;
    
    let repo_name: String = Input::new()
        .with_prompt("Repository name")
        .default(name.clone())
        .interact()?;
    
    let licenses = &["MIT", "Apache-2.0", "GPL-3.0", "BSD-3-Clause", "ISC"];
    let license_idx = Select::new()
        .with_prompt("License type")
        .items(licenses)
        .default(0)
        .interact()?;
    let license = licenses[license_idx].to_string();
    
    // Build complete input from interactive responses
    let mut args = Args::parse();
    args.name = Some(name);
    args.description = Some(description);
    args.language = Some(language);
    args.badges = include_badges;
    args.github_user = Some(github_user);
    args.repo = Some(repo_name);
    args.license = license;
    
    Ok(get_input_from_args(&args))
}
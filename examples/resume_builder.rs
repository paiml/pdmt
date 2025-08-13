//! Resume Builder Example
//!
//! This example demonstrates using PDMT's deterministic templating system
//! to generate professional resumes from structured YAML input. The deterministic
//! nature ensures consistent formatting and structure across multiple generations.
//!
//! Run with: cargo run --example resume_builder --features="full"

use clap::Parser;
use pdmt::{
    models::content::GeneratedContent,
    template::{definition::TemplateDefinition, engine::TemplateEngine},
};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(name = "resume-builder")]
#[command(about = "Generate deterministic professional resumes from YAML templates")]
struct Args {
    /// Full name
    #[arg(short = 'n', long)]
    name: Option<String>,

    /// Professional title
    #[arg(short = 't', long)]
    title: Option<String>,

    /// Email address
    #[arg(short = 'e', long)]
    email: Option<String>,

    /// Phone number
    #[arg(short = 'p', long)]
    phone: Option<String>,

    /// LinkedIn URL
    #[arg(short = 'l', long)]
    linkedin: Option<String>,

    /// GitHub URL
    #[arg(short = 'g', long)]
    github: Option<String>,

    /// Output format
    #[arg(short = 'f', long, default_value = "markdown")]
    format: String,

    /// Use interactive mode
    #[arg(short = 'i', long)]
    interactive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ResumeInput {
    personal: PersonalInfo,
    professional_summary: String,
    experience: Vec<Experience>,
    education: Vec<Education>,
    skills: Skills,
    projects: Option<Vec<Project>>,
    certifications: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PersonalInfo {
    name: String,
    title: String,
    email: String,
    phone: String,
    linkedin: Option<String>,
    github: Option<String>,
    location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Experience {
    company: String,
    position: String,
    duration: String,
    location: String,
    achievements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Education {
    institution: String,
    degree: String,
    field: String,
    graduation: String,
    gpa: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Skills {
    technical: Vec<String>,
    languages: Vec<String>,
    tools: Vec<String>,
    soft_skills: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Project {
    name: String,
    description: String,
    technologies: Vec<String>,
    url: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("📄 PDMT Resume Builder");
    println!("Generate deterministic professional resumes\n");

    // Create template engine
    let mut engine = TemplateEngine::new();

    // Define resume template
    let resume_template = create_resume_template();
    engine.register_template(resume_template)?;

    // Get resume input (interactive or from args)
    let resume_input = if args.interactive {
        get_interactive_input()?
    } else {
        get_default_input(&args)
    };

    // Generate resume using deterministic template
    println!("🔄 Generating professional resume...");
    let start = std::time::Instant::now();

    let input_json = serde_json::to_value(&resume_input)?;
    let result = engine.generate("professional_resume", input_json).await?;

    let duration = start.elapsed();
    println!("✅ Resume generated in {:?}\n", duration);

    // Format output based on requested format
    let formatted_output = match args.format.as_str() {
        "markdown" => format_as_markdown(&result, &resume_input),
        "text" => format_as_text(&result, &resume_input),
        "latex" => format_as_latex(&result, &resume_input),
        _ => result.content.clone(),
    };

    println!("📄 Generated Resume:\n");
    println!("{}", formatted_output);

    // Save to file
    let filename = format!(
        "{}_resume.{}",
        resume_input.personal.name.replace(' ', "_").to_lowercase(),
        match args.format.as_str() {
            "latex" => "tex",
            _ => "md",
        }
    );
    
    std::fs::write(&filename, &formatted_output)?;
    println!("\n💾 Resume saved to: {}", filename);

    Ok(())
}

fn create_resume_template() -> TemplateDefinition {
    let template_content = r#"
# {{personal.name}}
## {{personal.title}}

{{#if personal.email}}📧 {{personal.email}}{{/if}}{{#if personal.phone}} | 📱 {{personal.phone}}{{/if}}
{{#if personal.linkedin}}🔗 [LinkedIn]({{personal.linkedin}}){{/if}}{{#if personal.github}} | 💻 [GitHub]({{personal.github}}){{/if}}
{{#if personal.location}}📍 {{personal.location}}{{/if}}

---

## Professional Summary
{{professional_summary}}

## Experience
{{#each experience}}
### {{position}} at {{company}}
**{{duration}}** | {{location}}
{{#each achievements}}
- {{this}}
{{/each}}

{{/each}}

## Education
{{#each education}}
### {{degree}} in {{field}}
**{{institution}}** | {{graduation}}{{#if gpa}} | GPA: {{gpa}}{{/if}}

{{/each}}

## Skills
{{#if skills.technical}}
**Technical:** {{#each skills.technical}}{{this}}{{#unless @last}}, {{/unless}}{{/each}}
{{/if}}
{{#if skills.languages}}
**Languages:** {{#each skills.languages}}{{this}}{{#unless @last}}, {{/unless}}{{/each}}
{{/if}}
{{#if skills.tools}}
**Tools:** {{#each skills.tools}}{{this}}{{#unless @last}}, {{/unless}}{{/each}}
{{/if}}

{{#if projects}}
## Projects
{{#each projects}}
### {{name}}
{{description}}
**Technologies:** {{#each technologies}}{{this}}{{#unless @last}}, {{/unless}}{{/each}}
{{#if url}}[View Project]({{url}}){{/if}}

{{/each}}
{{/if}}

{{#if certifications}}
## Certifications
{{#each certifications}}
- {{this}}
{{/each}}
{{/if}}
"#;

    TemplateDefinition::new(
        "professional_resume",
        "1.0.0",
        template_content,
    )
}

fn get_default_input(args: &Args) -> ResumeInput {
    ResumeInput {
        personal: PersonalInfo {
            name: args.name.clone().unwrap_or_else(|| "John Doe".to_string()),
            title: args.title.clone().unwrap_or_else(|| "Software Engineer".to_string()),
            email: args.email.clone().unwrap_or_else(|| "john.doe@email.com".to_string()),
            phone: args.phone.clone().unwrap_or_else(|| "+1 (555) 123-4567".to_string()),
            linkedin: args.linkedin.clone().or_else(|| Some("https://linkedin.com/in/johndoe".to_string())),
            github: args.github.clone().or_else(|| Some("https://github.com/johndoe".to_string())),
            location: Some("San Francisco, CA".to_string()),
        },
        professional_summary: "Experienced software engineer with 5+ years developing scalable web applications and distributed systems. Passionate about clean code, test-driven development, and continuous learning. Strong track record of delivering high-quality solutions on time.".to_string(),
        experience: vec![
            Experience {
                company: "Tech Corp".to_string(),
                position: "Senior Software Engineer".to_string(),
                duration: "2021 - Present".to_string(),
                location: "San Francisco, CA".to_string(),
                achievements: vec![
                    "Led development of microservices architecture serving 1M+ daily users".to_string(),
                    "Improved API response times by 40% through optimization and caching strategies".to_string(),
                    "Mentored 3 junior developers and conducted technical interviews".to_string(),
                    "Implemented CI/CD pipeline reducing deployment time from hours to minutes".to_string(),
                ],
            },
            Experience {
                company: "StartupXYZ".to_string(),
                position: "Software Engineer".to_string(),
                duration: "2019 - 2021".to_string(),
                location: "Remote".to_string(),
                achievements: vec![
                    "Built RESTful APIs and GraphQL endpoints for mobile applications".to_string(),
                    "Developed automated testing suite achieving 85% code coverage".to_string(),
                    "Collaborated with product team to define technical requirements".to_string(),
                ],
            },
        ],
        education: vec![
            Education {
                institution: "University of Technology".to_string(),
                degree: "Bachelor of Science".to_string(),
                field: "Computer Science".to_string(),
                graduation: "2019".to_string(),
                gpa: Some("3.8/4.0".to_string()),
            },
        ],
        skills: Skills {
            technical: vec![
                "Rust".to_string(),
                "Python".to_string(),
                "TypeScript".to_string(),
                "Go".to_string(),
                "SQL".to_string(),
            ],
            languages: vec![
                "English (Native)".to_string(),
                "Spanish (Fluent)".to_string(),
            ],
            tools: vec![
                "Docker".to_string(),
                "Kubernetes".to_string(),
                "AWS".to_string(),
                "Git".to_string(),
                "PostgreSQL".to_string(),
            ],
            soft_skills: vec![
                "Team Leadership".to_string(),
                "Agile/Scrum".to_string(),
                "Technical Writing".to_string(),
            ],
        },
        projects: Some(vec![
            Project {
                name: "Open Source Contributor - Rust Web Framework".to_string(),
                description: "Contributing to popular web framework with 10k+ GitHub stars".to_string(),
                technologies: vec!["Rust".to_string(), "WebAssembly".to_string()],
                url: Some("https://github.com/example/framework".to_string()),
            },
        ]),
        certifications: Some(vec![
            "AWS Certified Solutions Architect".to_string(),
            "Certified Kubernetes Administrator (CKA)".to_string(),
        ]),
    }
}

fn get_interactive_input() -> Result<ResumeInput, Box<dyn std::error::Error>> {
    use dialoguer::Input;
    
    println!("🎯 Let's build your resume interactively!\n");
    
    let name: String = Input::new()
        .with_prompt("Your full name")
        .default("John Doe".to_string())
        .interact()?;
    
    let title: String = Input::new()
        .with_prompt("Professional title")
        .default("Software Engineer".to_string())
        .interact()?;
    
    // ... (collect other fields interactively)
    
    // For brevity, returning default with updated name and title
    let mut input = get_default_input(&Args::parse());
    input.personal.name = name;
    input.personal.title = title;
    
    Ok(input)
}

fn format_as_markdown(result: &GeneratedContent, _input: &ResumeInput) -> String {
    // The template already generates markdown
    result.content.clone()
}

fn format_as_text(result: &GeneratedContent, _input: &ResumeInput) -> String {
    // Convert markdown to plain text
    let mut text = result.content.clone();
    text = text.replace("###", "");
    text = text.replace("##", "");
    text = text.replace("#", "");
    text = text.replace("**", "");
    text = text.replace("*", "");
    text = text.replace("[", "");
    text = text.replace("]", "");
    text = text.replace("(", " - ");
    text = text.replace(")", "");
    text
}

fn format_as_latex(_result: &GeneratedContent, input: &ResumeInput) -> String {
    format!(r#"\documentclass{{article}}
\usepackage{{geometry}}
\geometry{{a4paper, margin=1in}}

\begin{{document}}

\begin{{center}}
\huge\textbf{{{name}}}\\
\large {title}\\
\normalsize
{email} | {phone}\\
{linkedin} | {github}
\end{{center}}

\section{{Professional Summary}}
{summary}

\section{{Experience}}
{experience}

\section{{Education}}
{education}

\section{{Skills}}
\textbf{{Technical:}} {technical}\\
\textbf{{Languages:}} {languages}\\
\textbf{{Tools:}} {tools}

\end{{document}}"#,
        name = input.personal.name,
        title = input.personal.title,
        email = input.personal.email,
        phone = input.personal.phone,
        linkedin = input.personal.linkedin.as_ref().unwrap_or(&String::new()),
        github = input.personal.github.as_ref().unwrap_or(&String::new()),
        summary = input.professional_summary,
        experience = format_latex_experience(&input.experience),
        education = format_latex_education(&input.education),
        technical = input.skills.technical.join(", "),
        languages = input.skills.languages.join(", "),
        tools = input.skills.tools.join(", ")
    )
}

fn format_latex_experience(experiences: &[Experience]) -> String {
    experiences.iter()
        .map(|exp| format!(
            r#"\subsection{{{} at {}}}
\textit{{{} | {}}}
\begin{{itemize}}
{}
\end{{itemize}}"#,
            exp.position,
            exp.company,
            exp.duration,
            exp.location,
            exp.achievements.iter()
                .map(|a| format!("\\item {}", a))
                .collect::<Vec<_>>()
                .join("\n")
        ))
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn format_latex_education(education: &[Education]) -> String {
    education.iter()
        .map(|edu| format!(
            r#"\subsection{{{} in {}}}
\textit{{{} | {}}}"#,
            edu.degree,
            edu.field,
            edu.institution,
            edu.graduation
        ))
        .collect::<Vec<_>>()
        .join("\n\n")
}
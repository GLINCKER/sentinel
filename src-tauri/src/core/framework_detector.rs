//! Framework auto-detection.
//!
//! This module detects development frameworks from project directories.

use std::collections::HashMap;
use std::path::Path;
use tokio::fs;

use crate::core::process_config::{FrameworkDetection, FrameworkType};
use crate::error::Result as SentinelResult;

/// Detect framework from a working directory
pub async fn detect_framework(working_dir: &str) -> SentinelResult<FrameworkDetection> {
    let path = Path::new(working_dir);

    // Check for various framework indicators
    let mut detections = Vec::new();

    // Next.js detection
    if let Some(detection) = detect_nextjs(path).await {
        detections.push(detection);
    }

    // Vite detection
    if let Some(detection) = detect_vite(path).await {
        detections.push(detection);
    }

    // FastAPI detection
    if let Some(detection) = detect_fastapi(path).await {
        detections.push(detection);
    }

    // Spring Boot detection
    if let Some(detection) = detect_spring_boot(path).await {
        detections.push(detection);
    }

    // Django detection
    if let Some(detection) = detect_django(path).await {
        detections.push(detection);
    }

    // Express detection
    if let Some(detection) = detect_express(path).await {
        detections.push(detection);
    }

    // Flask detection
    if let Some(detection) = detect_flask(path).await {
        detections.push(detection);
    }

    // Return the detection with highest confidence, or Unknown
    if let Some(best) = detections.into_iter().max_by(|a, b| {
        a.confidence
            .partial_cmp(&b.confidence)
            .unwrap_or(std::cmp::Ordering::Equal)
    }) {
        Ok(best)
    } else {
        Ok(FrameworkDetection {
            framework_type: FrameworkType::Unknown,
            confidence: 0.0,
            detected_files: vec![],
            suggested_command: String::new(),
            suggested_args: vec![],
            suggested_port: None,
        })
    }
}

async fn detect_nextjs(path: &Path) -> Option<FrameworkDetection> {
    let mut detected_files = Vec::new();
    let mut confidence = 0.0;

    // Check for package.json with next dependency
    if let Ok(contents) = fs::read_to_string(path.join("package.json")).await {
        if contents.contains("\"next\"") {
            detected_files.push("package.json".to_string());
            confidence += 0.6;
        }
    }

    // Check for next.config.js/ts
    if path.join("next.config.js").exists() || path.join("next.config.ts").exists() {
        detected_files.push("next.config.js".to_string());
        confidence += 0.35;
    }

    if confidence > 0.0 {
        Some(FrameworkDetection {
            framework_type: FrameworkType::NextJs,
            confidence,
            detected_files,
            suggested_command: "npm".to_string(),
            suggested_args: vec!["run".to_string(), "dev".to_string()],
            suggested_port: Some(3000),
        })
    } else {
        None
    }
}

async fn detect_vite(path: &Path) -> Option<FrameworkDetection> {
    let mut detected_files = Vec::new();
    let mut confidence = 0.0;

    // Check for vite.config.js/ts
    if path.join("vite.config.js").exists() || path.join("vite.config.ts").exists() {
        detected_files.push("vite.config.js".to_string());
        confidence += 0.7;
    }

    // Check for package.json with vite
    if let Ok(contents) = fs::read_to_string(path.join("package.json")).await {
        if contents.contains("\"vite\"") {
            detected_files.push("package.json".to_string());
            confidence += 0.25;
        }
    }

    if confidence > 0.0 {
        Some(FrameworkDetection {
            framework_type: FrameworkType::Vite,
            confidence,
            detected_files,
            suggested_command: "npm".to_string(),
            suggested_args: vec!["run".to_string(), "dev".to_string()],
            suggested_port: Some(5173),
        })
    } else {
        None
    }
}

async fn detect_fastapi(path: &Path) -> Option<FrameworkDetection> {
    let mut detected_files = Vec::new();
    let mut confidence = 0.0;

    // Check for requirements.txt with fastapi
    if let Ok(contents) = fs::read_to_string(path.join("requirements.txt")).await {
        if contents.contains("fastapi") {
            detected_files.push("requirements.txt".to_string());
            confidence += 0.5;
        }
    }

    // Check for main.py with FastAPI import
    if let Ok(contents) = fs::read_to_string(path.join("main.py")).await {
        if contents.contains("from fastapi") || contents.contains("import fastapi") {
            detected_files.push("main.py".to_string());
            confidence += 0.45;
        }
    }

    if confidence > 0.0 {
        Some(FrameworkDetection {
            framework_type: FrameworkType::FastAPI,
            confidence,
            detected_files,
            suggested_command: "uvicorn".to_string(),
            suggested_args: vec!["main:app".to_string(), "--reload".to_string()],
            suggested_port: Some(8000),
        })
    } else {
        None
    }
}

async fn detect_spring_boot(path: &Path) -> Option<FrameworkDetection> {
    let mut detected_files = Vec::new();
    let mut confidence = 0.0;

    // Check for pom.xml
    if path.join("pom.xml").exists() {
        if let Ok(contents) = fs::read_to_string(path.join("pom.xml")).await {
            if contents.contains("spring-boot") {
                detected_files.push("pom.xml".to_string());
                confidence += 0.8;
            }
        }
    }

    // Check for build.gradle
    if path.join("build.gradle").exists() {
        if let Ok(contents) = fs::read_to_string(path.join("build.gradle")).await {
            if contents.contains("spring-boot") {
                detected_files.push("build.gradle".to_string());
                confidence += 0.8;
            }
        }
    }

    if confidence > 0.0 {
        Some(FrameworkDetection {
            framework_type: FrameworkType::SpringBoot,
            confidence,
            detected_files,
            suggested_command: "./mvnw".to_string(),
            suggested_args: vec!["spring-boot:run".to_string()],
            suggested_port: Some(8080),
        })
    } else {
        None
    }
}

async fn detect_django(path: &Path) -> Option<FrameworkDetection> {
    let mut detected_files = Vec::new();
    let mut confidence = 0.0;

    // Check for manage.py
    if path.join("manage.py").exists() {
        detected_files.push("manage.py".to_string());
        confidence += 0.9;
    }

    // Check for requirements.txt with django
    if let Ok(contents) = fs::read_to_string(path.join("requirements.txt")).await {
        if contents.contains("Django") || contents.contains("django") {
            detected_files.push("requirements.txt".to_string());
            confidence += 0.05;
        }
    }

    if confidence > 0.0 {
        Some(FrameworkDetection {
            framework_type: FrameworkType::Django,
            confidence,
            detected_files,
            suggested_command: "python".to_string(),
            suggested_args: vec!["manage.py".to_string(), "runserver".to_string()],
            suggested_port: Some(8000),
        })
    } else {
        None
    }
}

async fn detect_express(path: &Path) -> Option<FrameworkDetection> {
    let mut detected_files = Vec::new();
    let mut confidence = 0.0;

    // Check for package.json with express
    if let Ok(contents) = fs::read_to_string(path.join("package.json")).await {
        if contents.contains("\"express\"") {
            detected_files.push("package.json".to_string());
            confidence += 0.7;
        }
    }

    // Check for common Express entry files
    for entry in &["server.js", "app.js", "index.js"] {
        if path.join(entry).exists() {
            if let Ok(contents) = fs::read_to_string(path.join(entry)).await {
                if contents.contains("express()") {
                    detected_files.push(entry.to_string());
                    confidence += 0.25;
                    break;
                }
            }
        }
    }

    if confidence > 0.0 {
        Some(FrameworkDetection {
            framework_type: FrameworkType::Express,
            confidence,
            detected_files,
            suggested_command: "node".to_string(),
            suggested_args: vec!["server.js".to_string()],
            suggested_port: Some(3000),
        })
    } else {
        None
    }
}

async fn detect_flask(path: &Path) -> Option<FrameworkDetection> {
    let mut detected_files = Vec::new();
    let mut confidence = 0.0;

    // Check for requirements.txt with flask
    if let Ok(contents) = fs::read_to_string(path.join("requirements.txt")).await {
        if contents.contains("Flask") || contents.contains("flask") {
            detected_files.push("requirements.txt".to_string());
            confidence += 0.5;
        }
    }

    // Check for app.py with Flask import
    if let Ok(contents) = fs::read_to_string(path.join("app.py")).await {
        if contents.contains("from flask") || contents.contains("import flask") {
            detected_files.push("app.py".to_string());
            confidence += 0.45;
        }
    }

    if confidence > 0.0 {
        Some(FrameworkDetection {
            framework_type: FrameworkType::Flask,
            confidence,
            detected_files,
            suggested_command: "flask".to_string(),
            suggested_args: vec!["run".to_string()],
            suggested_port: Some(5000),
        })
    } else {
        None
    }
}

/// Scan a directory for projects (supports monorepos)
pub async fn scan_directory_for_projects(
    dir_path: &str,
) -> SentinelResult<Vec<crate::core::process_config::DetectedProject>> {
    use crate::core::process_config::DetectedProject;

    let path = Path::new(dir_path);
    let mut projects = Vec::new();

    // First, check the root directory itself
    if let Ok(detection) = detect_framework(dir_path).await {
        if detection.confidence > 0.0 {
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("project")
                .to_string();

            // Parse .env file for environment variables
            let env_vars = parse_env_file(path).await;

            projects.push(DetectedProject {
                path: dir_path.to_string(),
                name,
                framework_type: detection.framework_type,
                confidence: detection.confidence,
                suggested_command: detection.suggested_command,
                suggested_args: detection.suggested_args,
                suggested_port: detection.suggested_port,
                package_manager: detect_package_manager(path).await,
                detected_files: detection.detected_files,
                env_vars,
            });
        }
    }

    // Then, scan subdirectories (for monorepos)
    if let Ok(mut entries) = fs::read_dir(path).await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            if let Ok(metadata) = entry.metadata().await {
                if metadata.is_dir() {
                    let subdir_path = entry.path();

                    // Skip common non-project directories
                    if let Some(dir_name) = subdir_path.file_name().and_then(|n| n.to_str()) {
                        if dir_name.starts_with('.')
                            || dir_name == "node_modules"
                            || dir_name == "dist"
                            || dir_name == "build"
                            || dir_name == "target"
                            || dir_name == "__pycache__"
                        {
                            continue;
                        }
                    }

                    // Try to detect framework in subdirectory
                    if let Some(subdir_str) = subdir_path.to_str() {
                        if let Ok(detection) = detect_framework(subdir_str).await {
                            if detection.confidence > 0.3 {
                                // Only include if confidence is decent
                                let name = subdir_path
                                    .file_name()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or("project")
                                    .to_string();

                                // Parse .env file for environment variables
                                let env_vars = parse_env_file(&subdir_path).await;

                                projects.push(DetectedProject {
                                    path: subdir_str.to_string(),
                                    name,
                                    framework_type: detection.framework_type,
                                    confidence: detection.confidence,
                                    suggested_command: detection.suggested_command,
                                    suggested_args: detection.suggested_args,
                                    suggested_port: detection.suggested_port,
                                    package_manager: detect_package_manager(&subdir_path).await,
                                    detected_files: detection.detected_files,
                                    env_vars,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(projects)
}

/// Detect the package manager used in a project
async fn detect_package_manager(path: &Path) -> Option<String> {
    if path.join("pnpm-lock.yaml").exists() {
        return Some("pnpm".to_string());
    }
    if path.join("yarn.lock").exists() {
        return Some("yarn".to_string());
    }
    if path.join("package-lock.json").exists() {
        return Some("npm".to_string());
    }
    if path.join("requirements.txt").exists() {
        return Some("pip".to_string());
    }
    if path.join("pom.xml").exists() {
        return Some("maven".to_string());
    }
    if path.join("build.gradle").exists() || path.join("build.gradle.kts").exists() {
        return Some("gradle".to_string());
    }
    None
}

/// Parse .env file and return environment variables
async fn parse_env_file(path: &Path) -> HashMap<String, String> {
    let mut env_vars = HashMap::new();

    let env_path = path.join(".env");
    if let Ok(content) = fs::read_to_string(&env_path).await {
        for line in content.lines() {
            let line = line.trim();

            // Skip comments and empty lines
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Parse KEY=VALUE format
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim().to_string();
                let mut value = value.trim().to_string();

                // Remove quotes if present
                if (value.starts_with('"') && value.ends_with('"'))
                    || (value.starts_with('\'') && value.ends_with('\''))
                {
                    value = value[1..value.len() - 1].to_string();
                }

                env_vars.insert(key, value);
            }
        }
    }

    env_vars
}

/// Get built-in framework templates
pub fn get_framework_templates() -> Vec<crate::core::process_config::ProcessTemplate> {
    use crate::core::process_config::ProcessTemplate;

    vec![
        ProcessTemplate {
            name: "Next.js Development Server".to_string(),
            framework_type: FrameworkType::NextJs,
            description: "React framework with SSR and routing".to_string(),
            command: "npm".to_string(),
            args: vec!["run".to_string(), "dev".to_string()],
            default_port: Some(3000),
            default_env_vars: HashMap::from([("NODE_ENV".to_string(), "development".to_string())]),
            health_check_url: Some("http://localhost:3000".to_string()),
            icon: "▲".to_string(),
        },
        ProcessTemplate {
            name: "Vite Development Server".to_string(),
            framework_type: FrameworkType::Vite,
            description: "Fast build tool and dev server".to_string(),
            command: "npm".to_string(),
            args: vec!["run".to_string(), "dev".to_string()],
            default_port: Some(5173),
            default_env_vars: HashMap::new(),
            health_check_url: Some("http://localhost:5173".to_string()),
            icon: "⚡".to_string(),
        },
        ProcessTemplate {
            name: "FastAPI Server".to_string(),
            framework_type: FrameworkType::FastAPI,
            description: "Modern Python web framework".to_string(),
            command: "uvicorn".to_string(),
            args: vec![
                "main:app".to_string(),
                "--reload".to_string(),
                "--host".to_string(),
                "0.0.0.0".to_string(),
            ],
            default_port: Some(8000),
            default_env_vars: HashMap::new(),
            health_check_url: Some("http://localhost:8000/docs".to_string()),
            icon: "🐍".to_string(),
        },
        ProcessTemplate {
            name: "Spring Boot Application".to_string(),
            framework_type: FrameworkType::SpringBoot,
            description: "Java enterprise framework".to_string(),
            command: "./mvnw".to_string(),
            args: vec!["spring-boot:run".to_string()],
            default_port: Some(8080),
            default_env_vars: HashMap::from([(
                "SPRING_PROFILES_ACTIVE".to_string(),
                "dev".to_string(),
            )]),
            health_check_url: Some("http://localhost:8080/actuator/health".to_string()),
            icon: "☕".to_string(),
        },
        ProcessTemplate {
            name: "Django Development Server".to_string(),
            framework_type: FrameworkType::Django,
            description: "Python web framework".to_string(),
            command: "python".to_string(),
            args: vec!["manage.py".to_string(), "runserver".to_string()],
            default_port: Some(8000),
            default_env_vars: HashMap::from([(
                "DJANGO_SETTINGS_MODULE".to_string(),
                "settings".to_string(),
            )]),
            health_check_url: Some("http://localhost:8000".to_string()),
            icon: "🎸".to_string(),
        },
        ProcessTemplate {
            name: "Express Server".to_string(),
            framework_type: FrameworkType::Express,
            description: "Node.js web framework".to_string(),
            command: "node".to_string(),
            args: vec!["server.js".to_string()],
            default_port: Some(3000),
            default_env_vars: HashMap::from([("NODE_ENV".to_string(), "development".to_string())]),
            health_check_url: Some("http://localhost:3000".to_string()),
            icon: "🚂".to_string(),
        },
        ProcessTemplate {
            name: "Flask Application".to_string(),
            framework_type: FrameworkType::Flask,
            description: "Lightweight Python web framework".to_string(),
            command: "flask".to_string(),
            args: vec!["run".to_string(), "--debug".to_string()],
            default_port: Some(5000),
            default_env_vars: HashMap::from([
                ("FLASK_APP".to_string(), "app.py".to_string()),
                ("FLASK_ENV".to_string(), "development".to_string()),
            ]),
            health_check_url: Some("http://localhost:5000".to_string()),
            icon: "🌶️".to_string(),
        },
    ]
}

//! 代码生成模块

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use rayon::prelude::*;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

use crate::ast::AST;
use crate::error::Error;

/// 代码生成器
pub struct CodeGenerator {
    backends: Arc<HashMap<String, Arc<dyn CodeGenBackend + Send + Sync>>>,
    template_engine: Arc<Handlebars<'static>>,
}

impl CodeGenerator {
    pub fn new() -> Self {
        let mut backends: HashMap<String, Arc<dyn CodeGenBackend + Send + Sync>> = HashMap::new();
        
        // 注册内置后端
        backends.insert("rust".to_string(), Arc::new(RustBackend::new()));
        backends.insert("c".to_string(), Arc::new(CBackend::new()));
        backends.insert("javascript".to_string(), Arc::new(JavaScriptBackend::new()));
        backends.insert("python".to_string(), Arc::new(PythonBackend::new()));
        
        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(false);
        
        Self {
            backends: Arc::new(backends),
            template_engine: Arc::new(handlebars),
        }
    }
    
    /// 生成Rust代码
    pub fn generate_rust(&self, ast: &AST) -> Result<GeneratedRustCode, Error> {
        let backend = self.backends.get("rust")
            .ok_or_else(|| Error::UnsupportedTarget { target: "rust".to_string() })?;
        
        let code = backend.generate(ast)?;
        let rust_code = GeneratedRustCode {
            code,
            performance_metrics: PerformanceMetrics::new(),
            optimization_level: OptimizationLevel::Basic,
        };
        
        Ok(rust_code)
    }
    
    /// 生成指定目标语言的代码
    pub fn generate(&self, ast: &AST, target: &str) -> Result<GeneratedCode, Error> {
        let backend = self.backends.get(target)
            .ok_or_else(|| Error::UnsupportedTarget { target: target.to_string() })?;
        
        backend.generate(ast)
    }
    
    /// 并行生成多目标代码
    pub fn generate_parallel(&self, ast: &AST, targets: &[String]) -> Result<Vec<GeneratedCode>, Error> {
        let results: Result<Vec<_>, _> = targets.par_iter()
            .map(|target| self.generate(ast, target))
            .collect();
        
        results
    }
    
    /// 添加自定义后端
    pub fn add_backend(&mut self, name: String, backend: Arc<dyn CodeGenBackend + Send + Sync>) {
        let mut backends = HashMap::new();
        for (k, v) in self.backends.iter() {
            backends.insert(k.clone(), v.clone());
        }
        backends.insert(name, backend);
        self.backends = Arc::new(backends);
    }
    
    /// 获取支持的目标
    pub fn get_supported_targets(&self) -> Vec<String> {
        self.backends.keys().cloned().collect()
    }
}

/// 代码生成后端接口
pub trait CodeGenBackend {
    fn name(&self) -> &str;
    fn generate(&self, ast: &AST) -> Result<GeneratedCode, Error>;
    fn supports_incremental(&self) -> bool;
    fn get_optimization_levels(&self) -> Vec<OptimizationLevel>;
}

/// Rust后端
pub struct RustBackend {
    template_engine: Arc<Handlebars<'static>>,
}

impl RustBackend {
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(false);
        
        // 注册Rust模板
        let rust_template = include_str!("../templates/rust_parser.hbs");
        handlebars.register_template_string("rust_parser", rust_template)
            .expect("Failed to register Rust template");
        
        Self {
            template_engine: Arc::new(handlebars),
        }
    }
}

impl CodeGenBackend for RustBackend {
    fn name(&self) -> &str { "rust" }
    
    fn generate(&self, ast: &AST) -> Result<GeneratedCode, Error> {
        let context = self.build_context(ast)?;
        let code = self.template_engine.render("rust_parser", &context)
            .map_err(|e| Error::TemplateError { message: e.to_string() })?;
        
        let formatted_code = self.format_rust_code(&code)?;
        
        Ok(GeneratedCode {
            target: "rust".to_string(),
            files: vec![GeneratedFile {
                name: "parser.rs".to_string(),
                content: formatted_code,
                file_type: FileType::Rust,
                dependencies: vec!["serde".to_string(), "thiserror".to_string()],
            }],
            metadata: CodeMetadata::new(),
            dependencies: vec![],
        })
    }
    
    fn supports_incremental(&self) -> bool { true }
    
    fn get_optimization_levels(&self) -> Vec<OptimizationLevel> {
        vec![
            OptimizationLevel::None,
            OptimizationLevel::Basic,
            OptimizationLevel::Aggressive,
            OptimizationLevel::Maximum,
        ]
    }
}

impl RustBackend {
    fn build_context(&self, _ast: &AST) -> Result<serde_json::Value, Error> {
        let mut context = serde_json::Map::new();
        
        // 提取语法信息
        if let crate::ast::ASTNode::Grammar(grammar) = &_ast.root {
            context.insert("grammar_name".to_string(), serde_json::Value::String(grammar.name.clone()));
            
            let rules: Vec<serde_json::Value> = grammar.rules.iter().map(|rule| {
                let mut rule_obj = serde_json::Map::new();
                rule_obj.insert("name".to_string(), serde_json::Value::String(rule.name.clone()));
                rule_obj.insert("pattern".to_string(), serde_json::Value::String(format!("{:?}", rule.pattern)));
                serde_json::Value::Object(rule_obj)
            }).collect();
            
            context.insert("rules".to_string(), serde_json::Value::Array(rules));
        } else {
            context.insert("grammar_name".to_string(), serde_json::Value::String("Unknown".to_string()));
            context.insert("rules".to_string(), serde_json::Value::Array(vec![]));
        }
        
        Ok(serde_json::Value::Object(context))
    }
    
    fn format_rust_code(&self, code: &str) -> Result<String, Error> {
        // 简单的代码格式化
        let lines: Vec<&str> = code.lines().collect();
        let mut formatted = String::new();
        let mut indent: usize = 0;
        
        for line in lines {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                formatted.push('\n');
                continue;
            }
            
            if trimmed.starts_with('}') || trimmed.starts_with(']') || trimmed.starts_with(')') {
                indent = indent.saturating_sub(1);
            }
            
            for _ in 0..indent {
                formatted.push_str("    ");
            }
            formatted.push_str(trimmed);
            formatted.push('\n');
            
            if trimmed.ends_with('{') || trimmed.ends_with('[') || trimmed.ends_with('(') {
                indent += 1;
            }
        }
        
        Ok(formatted)
    }
}

/// C后端
pub struct CBackend {
    template_engine: Arc<Handlebars<'static>>,
}

impl CBackend {
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(false);
        
        // 注册C模板
        let c_template = include_str!("../templates/c_parser.hbs");
        handlebars.register_template_string("c_parser", c_template)
            .expect("Failed to register C template");
        
        Self {
            template_engine: Arc::new(handlebars),
        }
    }
}

impl CodeGenBackend for CBackend {
    fn name(&self) -> &str { "c" }
    
    fn generate(&self, ast: &AST) -> Result<GeneratedCode, Error> {
        let context = self.build_context(ast)?;
        let code = self.template_engine.render("c_parser", &context)
            .map_err(|e| Error::TemplateError { message: e.to_string() })?;
        
        Ok(GeneratedCode {
            target: "c".to_string(),
            files: vec![GeneratedFile {
                name: "parser.c".to_string(),
                content: code,
                file_type: FileType::C,
                dependencies: vec![],
            }],
            metadata: CodeMetadata::new(),
            dependencies: vec![],
        })
    }
    
    fn supports_incremental(&self) -> bool { false }
    
    fn get_optimization_levels(&self) -> Vec<OptimizationLevel> {
        vec![
            OptimizationLevel::None,
            OptimizationLevel::Basic,
            OptimizationLevel::Aggressive,
        ]
    }
}

impl CBackend {
    fn build_context(&self, _ast: &AST) -> Result<serde_json::Value, Error> {
        // 简化的上下文构建
        let mut context = serde_json::Map::new();
        context.insert("grammar_name".to_string(), serde_json::Value::String("Generated".to_string()));
        context.insert("rules".to_string(), serde_json::Value::Array(vec![]));
        Ok(serde_json::Value::Object(context))
    }
}

/// JavaScript后端
pub struct JavaScriptBackend {
    template_engine: Arc<Handlebars<'static>>,
}

impl JavaScriptBackend {
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(false);
        
        Self {
            template_engine: Arc::new(handlebars),
        }
    }
}

impl CodeGenBackend for JavaScriptBackend {
    fn name(&self) -> &str { "javascript" }
    
    fn generate(&self, ast: &AST) -> Result<GeneratedCode, Error> {
        let context = self.build_context(ast)?;
        let code = self.template_engine.render("javascript_parser", &context)
            .map_err(|e| Error::TemplateError { message: e.to_string() })?;
        
        Ok(GeneratedCode {
            target: "javascript".to_string(),
            files: vec![GeneratedFile {
                name: "parser.js".to_string(),
                content: code,
                file_type: FileType::JavaScript,
                dependencies: vec![],
            }],
            metadata: CodeMetadata::new(),
            dependencies: vec![],
        })
    }
    
    fn supports_incremental(&self) -> bool { true }
    
    fn get_optimization_levels(&self) -> Vec<OptimizationLevel> {
        vec![
            OptimizationLevel::None,
            OptimizationLevel::Basic,
            OptimizationLevel::Aggressive,
        ]
    }
}

impl JavaScriptBackend {
    fn build_context(&self, _ast: &AST) -> Result<serde_json::Value, Error> {
        let mut context = serde_json::Map::new();
        context.insert("grammar_name".to_string(), serde_json::Value::String("Generated".to_string()));
        context.insert("rules".to_string(), serde_json::Value::Array(vec![]));
        Ok(serde_json::Value::Object(context))
    }
}

/// Python后端
pub struct PythonBackend {
    template_engine: Arc<Handlebars<'static>>,
}

impl PythonBackend {
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(false);
        
        Self {
            template_engine: Arc::new(handlebars),
        }
    }
}

impl CodeGenBackend for PythonBackend {
    fn name(&self) -> &str { "python" }
    
    fn generate(&self, ast: &AST) -> Result<GeneratedCode, Error> {
        let context = self.build_context(ast)?;
        let code = self.template_engine.render("python_parser", &context)
            .map_err(|e| Error::TemplateError { message: e.to_string() })?;
        
        Ok(GeneratedCode {
            target: "python".to_string(),
            files: vec![GeneratedFile {
                name: "parser.py".to_string(),
                content: code,
                file_type: FileType::Python,
                dependencies: vec![],
            }],
            metadata: CodeMetadata::new(),
            dependencies: vec![],
        })
    }
    
    fn supports_incremental(&self) -> bool { true }
    
    fn get_optimization_levels(&self) -> Vec<OptimizationLevel> {
        vec![
            OptimizationLevel::None,
            OptimizationLevel::Basic,
            OptimizationLevel::Aggressive,
        ]
    }
}

impl PythonBackend {
    fn build_context(&self, _ast: &AST) -> Result<serde_json::Value, Error> {
        let mut context = serde_json::Map::new();
        context.insert("grammar_name".to_string(), serde_json::Value::String("Generated".to_string()));
        context.insert("rules".to_string(), serde_json::Value::Array(vec![]));
        Ok(serde_json::Value::Object(context))
    }
}

/// 生成的代码
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedCode {
    pub target: String,
    pub files: Vec<GeneratedFile>,
    pub metadata: CodeMetadata,
    pub dependencies: Vec<Dependency>,
}

/// 生成的Rust代码
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedRustCode {
    pub code: GeneratedCode,
    pub performance_metrics: PerformanceMetrics,
    pub optimization_level: OptimizationLevel,
}

/// 生成的文件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedFile {
    pub name: String,
    pub content: String,
    pub file_type: FileType,
    pub dependencies: Vec<String>,
}

/// 文件类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FileType {
    Rust,
    C,
    JavaScript,
    Python,
    TypeScript,
    Go,
    Java,
    CSharp,
}

/// 优化级别
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OptimizationLevel {
    None,
    Basic,
    Aggressive,
    Maximum,
}

/// 代码元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeMetadata {
    pub generation_time: std::time::SystemTime,
    pub optimization_level: OptimizationLevel,
    pub performance_metrics: PerformanceMetrics,
}

impl CodeMetadata {
    pub fn new() -> Self {
        Self {
            generation_time: std::time::SystemTime::now(),
            optimization_level: OptimizationLevel::Basic,
            performance_metrics: PerformanceMetrics::new(),
        }
    }
}

/// 性能指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub lines_of_code: usize,
    pub complexity_score: f64,
    pub estimated_performance: PerformanceEstimate,
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            lines_of_code: 0,
            complexity_score: 0.0,
            estimated_performance: PerformanceEstimate::Medium,
        }
    }
}

/// 性能估计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceEstimate {
    Low,
    Medium,
    High,
    Maximum,
}

/// 依赖
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub source: DependencySource,
}

/// 依赖源
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencySource {
    Crate,
    Npm,
    PyPI,
    Maven,
    NuGet,
    Custom(String),
}

impl GeneratedCode {
    pub fn new(target: String) -> Self {
        Self {
            target,
            files: Vec::new(),
            metadata: CodeMetadata::new(),
            dependencies: Vec::new(),
        }
    }
    
    pub fn add_file(&mut self, file: GeneratedFile) {
        self.files.push(file);
    }
    
    pub fn get_files(&self) -> &[GeneratedFile] {
        &self.files
    }
    
    pub fn write_to_directory(&self, path: &Path) -> Result<(), Error> {
        std::fs::create_dir_all(path)?;
        
        for file in &self.files {
            let file_path = path.join(&file.name);
            std::fs::write(file_path, &file.content)?;
        }
        
        Ok(())
    }
}

impl GeneratedRustCode {
    pub fn write_to_directory(&self, path: &Path) -> Result<(), Error> {
        self.code.write_to_directory(path)
    }
}

impl Default for CodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{AST, ASTNode, LiteralNode};
    use crate::error::Position;
    
    #[test]
    fn test_code_generator_creation() {
        let generator = CodeGenerator::new();
        let targets = generator.get_supported_targets();
        assert!(targets.contains(&"rust".to_string()));
        assert!(targets.contains(&"c".to_string()));
        assert!(targets.contains(&"javascript".to_string()));
        assert!(targets.contains(&"python".to_string()));
    }
    
    #[test]
    fn test_rust_code_generation() {
        let generator = CodeGenerator::new();
        let ast = AST {
            root: ASTNode::Literal(LiteralNode {
                value: "test".to_string(),
                position: Position::new(1, 1, 0),
            }),
            metadata: crate::ast::ASTMetadata::new(),
        };
        
        let result = generator.generate_rust(&ast);
        assert!(result.is_ok());
        
        let rust_code = result.unwrap();
        assert_eq!(rust_code.code.target, "rust");
        assert!(!rust_code.code.files.is_empty());
    }
    
    #[test]
    fn test_parallel_code_generation() {
        let generator = CodeGenerator::new();
        let ast = AST {
            root: ASTNode::Literal(LiteralNode {
                value: "test".to_string(),
                position: Position::new(1, 1, 0),
            }),
            metadata: crate::ast::ASTMetadata::new(),
        };
        
        let targets = vec!["rust".to_string(), "c".to_string()];
        let result = generator.generate_parallel(&ast, &targets);
        assert!(result.is_ok());
        
        let codes = result.unwrap();
        assert_eq!(codes.len(), 2);
    }
    
    #[test]
    fn test_generated_code_serialization() {
        let code = GeneratedCode::new("rust".to_string());
        let json = serde_json::to_string(&code).unwrap();
        assert!(json.contains("rust"));
    }
}

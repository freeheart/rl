//! # RL - Right wheeL
//! 
//! 高性能Rust解析器生成器
//! 
//! ## 特性
//! - 零拷贝解析
//! - 并行处理
//! - 智能缓存
//! - 内存优化
//! - 多目标代码生成
//! 
//! ## 使用示例
//! 
//! ```rust
//! use rl::RL;
//! use std::path::Path;
//! 
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let rl = RL::new();
//!     let grammar = r#"
//!         grammar Calculator {
//!             expr: term (('+' | '-') term)*;
//!             term: factor (('*' | '/') factor)*;
//!             factor: number | '(' expr ')';
//!             number: [0-9]+;
//!         }
//!     "#;
//! 
//!     let rust_code = rl.compile_grammar_to_rust(grammar)?;
//!     rust_code.write_to_directory(Path::new("generated_parser"))?;
//!     Ok(())
//! }
//! ```

pub mod parser;
pub mod ast;
pub mod codegen;
pub mod tools;
pub mod error;
pub mod performance;
pub mod ai_enhancement;
pub mod knowledge_graph;
pub mod ai_cli;
pub mod kg_cli;
pub mod grammar_parser;

use std::sync::Arc;
use rayon::prelude::*;

/// RL主入口
pub struct RL {
    parser_generator: Arc<parser::ParserGenerator>,
    code_generator: Arc<codegen::CodeGenerator>,
    tools: Arc<tools::Tools>,
}

impl RL {
    /// 创建新的RL实例
    pub fn new() -> Self {
        Self {
            parser_generator: Arc::new(parser::ParserGenerator::new()),
            code_generator: Arc::new(codegen::CodeGenerator::new()),
            tools: Arc::new(tools::Tools::new()),
        }
    }
    
    /// 从语法定义生成解析器
    pub fn generate_parser(&self, grammar: &parser::Grammar) -> Result<parser::GeneratedParser, error::Error> {
        self.parser_generator.generate(grammar)
    }
    
    /// 生成Rust代码
    pub fn generate_rust_code(&self, ast: &ast::AST) -> Result<codegen::GeneratedRustCode, error::Error> {
        self.code_generator.generate_rust(ast)
    }
    
    /// 一行代码：语法定义 -> Rust解析器
    pub fn compile_grammar_to_rust(&self, grammar: &str) -> Result<codegen::GeneratedRustCode, error::Error> {
        // 解析语法定义
        let grammar_def = self.parse_grammar_definition(grammar)?;
        
        // 生成解析器
        let parser = self.generate_parser(&grammar_def)?;
        
        // 解析语法生成AST
        let ast = self.parse_with_generated_parser(&parser, grammar)?;
        
        // 生成Rust代码
        self.generate_rust_code(&ast)
    }
    
    /// 并行编译多个语法
    pub fn compile_grammars_parallel(&self, grammars: &[String]) -> Result<Vec<codegen::GeneratedRustCode>, error::Error> {
        grammars.par_iter()
            .map(|grammar| self.compile_grammar_to_rust(grammar))
            .collect()
    }
    
    /// 解析语法定义
    fn parse_grammar_definition(&self, grammar: &str) -> Result<parser::Grammar, error::Error> {
        // 简单的语法解析器实现
        parser::GrammarParser::parse(grammar)
    }
    
    /// 使用生成的解析器解析
    fn parse_with_generated_parser(&self, parser: &parser::GeneratedParser, input: &str) -> Result<ast::AST, error::Error> {
        parser.parse(input)
    }
}

impl Default for RL {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rl_creation() {
        let rl = RL::new();
        // Arc always contains a value, so we just check that we can access it
        let _parser_gen = &rl.parser_generator;
        let _code_gen = &rl.code_generator;
        let _tools = &rl.tools;
    }
    
    #[test]
    fn test_simple_grammar_compilation() {
        let rl = RL::new();
        let grammar = r#"
            grammar Test {
                start: 'hello' 'world';
            }
        "#;
        
        let result = rl.compile_grammar_to_rust(grammar);
        assert!(result.is_ok());
    }
}

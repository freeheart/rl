//! 错误处理模块

use thiserror::Error;
use serde::{Serialize, Deserialize};

/// RL错误类型
#[derive(Error, Debug)]
pub enum Error {
    #[error("解析错误: {message} at {position}")]
    ParseError { message: String, position: Position },
    
    #[error("词法错误: {token} at {position}")]
    LexicalError { token: String, position: Position },
    
    #[error("语法错误: {message} at {position}")]
    SyntaxError { message: String, position: Position },
    
    #[error("语义错误: {message} at {position}")]
    SemanticError { message: String, position: Position },
    
    #[error("代码生成错误: {message}")]
    CodeGenError { message: String },
    
    #[error("模板错误: {message}")]
    TemplateError { message: String },
    
    #[error("IO错误: {source}")]
    IoError { source: std::io::Error },
    
    #[error("不支持的目标语言: {target}")]
    UnsupportedTarget { target: String },
    
    #[error("模板未找到: {template}")]
    TemplateNotFound { template: String },
    
    #[error("内存错误: {message}")]
    MemoryError { message: String },
    
    #[error("缓存错误: {message}")]
    CacheError { message: String },
    
    #[error("性能错误: {message}")]
    PerformanceError { message: String },
}

/// 位置信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl Position {
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self { line, column, offset }
    }
    
    pub fn advance(&self, consumed: usize) -> Self {
        Self {
            line: self.line,
            column: self.column + consumed,
            offset: self.offset + consumed,
        }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {}, column {}", self.line, self.column)
    }
}

/// 解析错误
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("语法错误: {message} at {position}")]
    SyntaxError { message: String, position: Position },
    
    #[error("词法错误: {token} at {position}")]
    LexicalError { token: String, position: Position },
    
    #[error("语义错误: {message} at {position}")]
    SemanticError { message: String, position: Position },
    
    #[error("歧义语法")]
    AmbiguousGrammar,
    
    #[error("左递归错误")]
    LeftRecursionError,
    
    #[error("未定义的符号: {symbol}")]
    UndefinedSymbol { symbol: String },
}

/// 词法错误
#[derive(Error, Debug)]
pub enum LexerError {
    #[error("意外的字符: {character} at {position}")]
    UnexpectedCharacter { character: char, position: usize },
    
    #[error("未终止的字符串")]
    UnterminatedString,
    
    #[error("未终止的注释")]
    UnterminatedComment,
    
    #[error("无效的数字字面量")]
    InvalidNumberLiteral,
    
    #[error("无效的字符类")]
    InvalidCharacterClass,
}

/// 代码生成错误
#[derive(Error, Debug)]
pub enum CodeGenError {
    #[error("模板渲染错误: {message}")]
    TemplateRenderError { message: String },
    
    #[error("代码格式化错误: {message}")]
    CodeFormatError { message: String },
    
    #[error("优化错误: {message}")]
    OptimizationError { message: String },
    
    #[error("依赖解析错误: {message}")]
    DependencyError { message: String },
}

/// 内存错误
#[derive(Error, Debug)]
pub enum MemoryError {
    #[error("内存分配失败: {size} bytes")]
    AllocationFailed { size: usize },
    
    #[error("内存池耗尽")]
    PoolExhausted,
    
    #[error("内存碎片化")]
    Fragmentation,
}

/// 缓存错误
#[derive(Error, Debug)]
pub enum CacheError {
    #[error("缓存未命中")]
    CacheMiss,
    
    #[error("缓存已满")]
    CacheFull,
    
    #[error("缓存损坏")]
    CacheCorrupted,
}

/// 性能错误
#[derive(Error, Debug)]
pub enum PerformanceError {
    #[error("性能阈值超出: {metric} = {value}")]
    ThresholdExceeded { metric: String, value: f64 },
    
    #[error("资源不足: {resource}")]
    ResourceExhausted { resource: String },
    
    #[error("超时: {operation}")]
    Timeout { operation: String },
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError { source: err }
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        match err {
            ParseError::SyntaxError { message, position } => Error::ParseError { message, position },
            ParseError::LexicalError { token, position } => Error::LexicalError { token, position },
            ParseError::SemanticError { message, position } => Error::SemanticError { message, position },
            _ => Error::ParseError { 
                message: err.to_string(), 
                position: Position::new(0, 0, 0) 
            },
        }
    }
}

impl From<LexerError> for Error {
    fn from(err: LexerError) -> Self {
        match err {
            LexerError::UnexpectedCharacter { character, position } => {
                Error::LexicalError { 
                    token: character.to_string(), 
                    position: Position::new(0, 0, position) 
                }
            }
            _ => Error::LexicalError { 
                token: err.to_string(), 
                position: Position::new(0, 0, 0) 
            },
        }
    }
}

impl From<CodeGenError> for Error {
    fn from(err: CodeGenError) -> Self {
        Error::CodeGenError { message: err.to_string() }
    }
}

impl From<MemoryError> for Error {
    fn from(err: MemoryError) -> Self {
        Error::MemoryError { message: err.to_string() }
    }
}

impl From<CacheError> for Error {
    fn from(err: CacheError) -> Self {
        Error::CacheError { message: err.to_string() }
    }
}

impl From<PerformanceError> for Error {
    fn from(err: PerformanceError) -> Self {
        Error::PerformanceError { message: err.to_string() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_position_creation() {
        let pos = Position::new(1, 5, 10);
        assert_eq!(pos.line, 1);
        assert_eq!(pos.column, 5);
        assert_eq!(pos.offset, 10);
    }
    
    #[test]
    fn test_position_advance() {
        let pos = Position::new(1, 5, 10);
        let new_pos = pos.advance(3);
        assert_eq!(new_pos.line, 1);
        assert_eq!(new_pos.column, 8);
        assert_eq!(new_pos.offset, 13);
    }
    
    #[test]
    fn test_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let rl_err: Error = io_err.into();
        
        match rl_err {
            Error::IoError { .. } => {}
            _ => panic!("Expected IoError"),
        }
    }
}

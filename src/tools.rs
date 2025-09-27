//! 工具模块

use std::path::{Path, PathBuf};
use std::sync::Arc;
use rayon::prelude::*;

use crate::error::{Error, Position};

/// 工具函数集合
pub struct Tools {
    file_utils: Arc<FileUtils>,
    string_utils: Arc<StringUtils>,
    position_utils: Arc<PositionUtils>,
    error_utils: Arc<ErrorUtils>,
}

impl Tools {
    pub fn new() -> Self {
        Self {
            file_utils: Arc::new(FileUtils::new()),
            string_utils: Arc::new(StringUtils::new()),
            position_utils: Arc::new(PositionUtils::new()),
            error_utils: Arc::new(ErrorUtils::new()),
        }
    }
    
    /// 读取文件
    pub fn read_file(&self, path: &Path) -> Result<String, Error> {
        self.file_utils.read_file(path)
    }
    
    /// 写入文件
    pub fn write_file(&self, path: &Path, content: &str) -> Result<(), Error> {
        self.file_utils.write_file(path, content)
    }
    
    /// 格式化字符串
    pub fn format_string(&self, s: &str) -> String {
        self.string_utils.format(s)
    }
    
    /// 计算位置
    pub fn calculate_position(&self, input: &str, offset: usize) -> Position {
        self.position_utils.calculate(input, offset)
    }
    
    /// 格式化错误
    pub fn format_error(&self, error: &Error) -> String {
        self.error_utils.format(error)
    }
    
    /// 并行读取多个文件
    pub fn read_files_parallel(&self, paths: &[PathBuf]) -> Result<Vec<String>, Error> {
        let results: Result<Vec<_>, _> = paths.par_iter()
            .map(|path| self.file_utils.read_file(path))
            .collect();
        
        results
    }
    
    /// 并行写入多个文件
    pub fn write_files_parallel(&self, files: &[(PathBuf, String)]) -> Result<(), Error> {
        files.par_iter()
            .map(|(path, content)| self.file_utils.write_file(path, content))
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(())
    }
}

/// 文件工具
pub struct FileUtils {
    cache: Arc<dashmap::DashMap<PathBuf, String>>,
}

impl FileUtils {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(dashmap::DashMap::new()),
        }
    }
    
    /// 读取文件（带缓存）
    pub fn read_file(&self, path: &Path) -> Result<String, Error> {
        let path_buf = path.to_path_buf();
        
        // 检查缓存
        if let Some(cached) = self.cache.get(&path_buf) {
            return Ok(cached.clone());
        }
        
        // 读取文件
        let content = std::fs::read_to_string(path)?;
        
        // 缓存结果
        self.cache.insert(path_buf, content.clone());
        
        Ok(content)
    }
    
    /// 写入文件
    pub fn write_file(&self, path: &Path, content: &str) -> Result<(), Error> {
        // 确保目录存在
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        std::fs::write(path, content)?;
        
        // 更新缓存
        let path_buf = path.to_path_buf();
        self.cache.insert(path_buf, content.to_string());
        
        Ok(())
    }
    
    /// 创建目录
    pub fn create_directory(&self, path: &Path) -> Result<(), Error> {
        std::fs::create_dir_all(path)?;
        Ok(())
    }
    
    /// 检查文件是否存在
    pub fn file_exists(&self, path: &Path) -> bool {
        path.exists()
    }
    
    /// 获取文件大小
    pub fn get_file_size(&self, path: &Path) -> Result<u64, Error> {
        let metadata = std::fs::metadata(path)?;
        Ok(metadata.len())
    }
    
    /// 列出目录中的文件
    pub fn list_files(&self, path: &Path) -> Result<Vec<PathBuf>, Error> {
        let mut files = Vec::new();
        
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                files.push(path);
            }
        }
        
        Ok(files)
    }
    
    /// 递归列出目录中的所有文件
    pub fn list_files_recursive(&self, path: &Path) -> Result<Vec<PathBuf>, Error> {
        let mut files = Vec::new();
        
        for entry in walkdir::WalkDir::new(path) {
            let entry = entry.map_err(|e| crate::error::Error::IoError { source: std::io::Error::new(std::io::ErrorKind::Other, e.to_string()) })?;
            let path = entry.path();
            
            if path.is_file() {
                files.push(path.to_path_buf());
            }
        }
        
        Ok(files)
    }
    
    /// 清除缓存
    pub fn clear_cache(&self) {
        self.cache.clear();
    }
    
    /// 获取缓存大小
    pub fn get_cache_size(&self) -> usize {
        self.cache.len()
    }
}

/// 字符串工具
pub struct StringUtils;

impl StringUtils {
    pub fn new() -> Self {
        Self
    }
    
    /// 格式化字符串
    pub fn format(&self, s: &str) -> String {
        s.trim().to_string()
    }
    
    /// 转义字符串
    pub fn escape(&self, s: &str) -> String {
        s.chars()
            .map(|c| match c {
                '"' => "\\\"".to_string(),
                '\\' => "\\\\".to_string(),
                '\n' => "\\n".to_string(),
                '\r' => "\\r".to_string(),
                '\t' => "\\t".to_string(),
                _ => c.to_string(),
            })
            .collect()
    }
    
    /// 反转义字符串
    pub fn unescape(&self, s: &str) -> String {
        let mut result = String::new();
        let mut chars = s.chars().peekable();
        
        while let Some(c) = chars.next() {
            if c == '\\' {
                if let Some(next) = chars.next() {
                    match next {
                        '"' => result.push('"'),
                        '\\' => result.push('\\'),
                        'n' => result.push('\n'),
                        'r' => result.push('\r'),
                        't' => result.push('\t'),
                        _ => {
                            result.push('\\');
                            result.push(next);
                        }
                    }
                } else {
                    result.push('\\');
                }
            } else {
                result.push(c);
            }
        }
        
        result
    }
    
    /// 转换为驼峰命名
    pub fn to_camel_case(&self, s: &str) -> String {
        let mut result = String::new();
        let mut capitalize_next = false;
        
        for c in s.chars() {
            if c == '_' || c == '-' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(c.to_uppercase().next().unwrap_or(c));
                capitalize_next = false;
            } else {
                result.push(c);
            }
        }
        
        result
    }
    
    /// 转换为蛇形命名
    pub fn to_snake_case(&self, s: &str) -> String {
        let mut result = String::new();
        
        for (i, c) in s.chars().enumerate() {
            if c.is_uppercase() && i > 0 {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap_or(c));
        }
        
        result
    }
    
    /// 转换为帕斯卡命名
    pub fn to_pascal_case(&self, s: &str) -> String {
        let camel_case = self.to_camel_case(s);
        if let Some(first) = camel_case.chars().next() {
            format!("{}{}", first.to_uppercase(), &camel_case[1..])
        } else {
            camel_case
        }
    }
    
    /// 检查字符串是否为有效标识符
    pub fn is_valid_identifier(&self, s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        
        let mut chars = s.chars();
        let first = chars.next().unwrap();
        
        if !first.is_alphabetic() && first != '_' {
            return false;
        }
        
        chars.all(|c| c.is_alphanumeric() || c == '_')
    }
    
    /// 生成唯一标识符
    pub fn generate_unique_identifier(&self, prefix: &str, existing: &[String]) -> String {
        let mut counter = 0;
        let mut identifier = format!("{}_{}", prefix, counter);
        
        while existing.contains(&identifier) {
            counter += 1;
            identifier = format!("{}_{}", prefix, counter);
        }
        
        identifier
    }
}

/// 位置工具
pub struct PositionUtils;

impl PositionUtils {
    pub fn new() -> Self {
        Self
    }
    
    /// 计算位置
    pub fn calculate(&self, input: &str, offset: usize) -> Position {
        let mut line = 1;
        let mut column = 1;
        let mut current_offset = 0;
        
        for (i, c) in input.char_indices() {
            if i >= offset {
                break;
            }
            
            if c == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
            
            current_offset = i;
        }
        
        Position::new(line, column, offset)
    }
    
    /// 计算行号
    pub fn calculate_line(&self, input: &str, offset: usize) -> usize {
        let mut line = 1;
        
        for (i, c) in input.char_indices() {
            if i >= offset {
                break;
            }
            
            if c == '\n' {
                line += 1;
            }
        }
        
        line
    }
    
    /// 计算列号
    pub     fn calculate_column(&self, input: &str, offset: usize) -> usize {
        let mut column = 1;
        
        for (i, c) in input.char_indices() {
            if i >= offset {
                break;
            }
            
            if c == '\n' {
                column = 1;
            } else {
                column += 1;
            }
        }
        
        column
    }
    
    /// 获取指定行的内容
    pub fn get_line(&self, input: &str, line_number: usize) -> Option<String> {
        let lines: Vec<&str> = input.lines().collect();
        lines.get(line_number - 1).map(|s| s.to_string())
    }
    
    /// 获取指定行的范围
    pub fn get_line_range(&self, input: &str, line_number: usize) -> Option<(usize, usize)> {
        let mut current_line = 1;
        let mut start = 0;
        
        for (i, c) in input.char_indices() {
            if current_line == line_number {
                start = i;
                break;
            }
            
            if c == '\n' {
                current_line += 1;
            }
        }
        
        if current_line != line_number {
            return None;
        }
        
        let mut end = start;
        for (i, c) in input[start..].char_indices() {
            if c == '\n' {
                end = start + i;
                break;
            }
        }
        
        Some((start, end))
    }
}

/// 错误工具
pub struct ErrorUtils;

impl ErrorUtils {
    pub fn new() -> Self {
        Self
    }
    
    /// 格式化错误
    pub fn format(&self, error: &Error) -> String {
        match error {
            Error::ParseError { message, position } => {
                format!("解析错误: {} at line {}, column {}", message, position.line, position.column)
            }
            Error::LexicalError { token, position } => {
                format!("词法错误: 意外的token '{}' at line {}, column {}", token, position.line, position.column)
            }
            Error::SyntaxError { message, position } => {
                format!("语法错误: {} at line {}, column {}", message, position.line, position.column)
            }
            Error::SemanticError { message, position } => {
                format!("语义错误: {} at line {}, column {}", message, position.line, position.column)
            }
            Error::CodeGenError { message } => {
                format!("代码生成错误: {}", message)
            }
            Error::TemplateError { message } => {
                format!("模板错误: {}", message)
            }
            Error::IoError { source } => {
                format!("IO错误: {}", source)
            }
            Error::UnsupportedTarget { target } => {
                format!("不支持的目标语言: {}", target)
            }
            Error::TemplateNotFound { template } => {
                format!("模板未找到: {}", template)
            }
            Error::MemoryError { message } => {
                format!("内存错误: {}", message)
            }
            Error::CacheError { message } => {
                format!("缓存错误: {}", message)
            }
            Error::PerformanceError { message } => {
                format!("性能错误: {}", message)
            }
        }
    }
    
    /// 格式化错误并包含上下文
    pub fn format_with_context(&self, error: &Error, source: &str) -> String {
        let base_message = self.format(error);
        
        match error {
            Error::ParseError { position, .. } |
            Error::LexicalError { position, .. } |
            Error::SyntaxError { position, .. } |
            Error::SemanticError { position, .. } => {
                let position_utils = PositionUtils::new();
                if let Some(line_content) = position_utils.get_line(source, position.line) {
                    let mut context = base_message;
                    context.push_str(&format!("\n  {} | {}", position.line, line_content));
                    context.push_str(&format!("\n     | {}^", " ".repeat(position.column - 1)));
                    context
                } else {
                    base_message
                }
            }
            _ => base_message,
        }
    }
    
    /// 生成错误报告
    pub fn generate_error_report(&self, errors: &[Error], source: &str) -> String {
        let mut report = String::new();
        report.push_str("错误报告:\n");
        report.push_str("==========\n\n");
        
        for (i, error) in errors.iter().enumerate() {
            report.push_str(&format!("错误 {}: {}\n", i + 1, self.format_with_context(error, source)));
            report.push_str("\n");
        }
        
        report
    }
}

impl Default for Tools {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_file_utils() {
        let file_utils = FileUtils::new();
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        
        // 测试文件写入
        let content = "Hello, World!";
        file_utils.write_file(&file_path, content).unwrap();
        
        // 测试文件读取
        let read_content = file_utils.read_file(&file_path).unwrap();
        assert_eq!(read_content, content);
        
        // 测试文件存在检查
        assert!(file_utils.file_exists(&file_path));
        
        // 测试缓存
        assert!(file_utils.get_cache_size() > 0);
    }
    
    #[test]
    fn test_string_utils() {
        let string_utils = StringUtils::new();
        
        // 测试转义
        let escaped = string_utils.escape("Hello \"World\"");
        assert_eq!(escaped, "Hello \\\"World\\\"");
        
        // 测试反转义
        let unescaped = string_utils.unescape("Hello \\\"World\\\"");
        assert_eq!(unescaped, "Hello \"World\"");
        
        // 测试命名转换
        assert_eq!(string_utils.to_camel_case("hello_world"), "helloWorld");
        assert_eq!(string_utils.to_snake_case("HelloWorld"), "hello_world");
        assert_eq!(string_utils.to_pascal_case("hello_world"), "HelloWorld");
        
        // 测试标识符验证
        assert!(string_utils.is_valid_identifier("hello"));
        assert!(string_utils.is_valid_identifier("_hello"));
        assert!(string_utils.is_valid_identifier("hello123"));
        assert!(!string_utils.is_valid_identifier("123hello"));
        assert!(!string_utils.is_valid_identifier("hello-world"));
    }
    
    #[test]
    fn test_position_utils() {
        let position_utils = PositionUtils::new();
        let input = "Hello\nWorld\nTest";
        
        // 测试位置计算
        let pos = position_utils.calculate(input, 7);
        assert_eq!(pos.line, 2);
        assert_eq!(pos.column, 2);
        
        // 测试行号计算
        assert_eq!(position_utils.calculate_line(input, 7), 2);
        
        // 测试列号计算
        assert_eq!(position_utils.calculate_column(input, 7), 2);
        
        // 测试获取行内容
        assert_eq!(position_utils.get_line(input, 1), Some("Hello".to_string()));
        assert_eq!(position_utils.get_line(input, 2), Some("World".to_string()));
        assert_eq!(position_utils.get_line(input, 3), Some("Test".to_string()));
    }
    
    #[test]
    fn test_error_utils() {
        let error_utils = ErrorUtils::new();
        let position = Position::new(1, 5, 4);
        
        let error = Error::ParseError {
            message: "Unexpected token".to_string(),
            position: position.clone(),
        };
        
        let formatted = error_utils.format(&error);
        assert!(formatted.contains("解析错误"));
        assert!(formatted.contains("line 1, column 5"));
    }
    
    #[test]
    fn test_tools_integration() {
        let tools = Tools::new();
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        
        // 测试文件操作
        tools.write_file(&file_path, "test content").unwrap();
        let content = tools.read_file(&file_path).unwrap();
        assert_eq!(content, "test content");
        
        // 测试字符串格式化
        let formatted = tools.format_string("  hello world  ");
        assert_eq!(formatted, "hello world");
        
        // 测试位置计算
        let pos = tools.calculate_position("hello\nworld", 7);
        assert_eq!(pos.line, 2);
        assert_eq!(pos.column, 2);
    }
}

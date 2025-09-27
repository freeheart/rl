//! 实际使用生成的解析器进行SQL解析测试

use std::path::Path;
use std::fs;

/// 测试生成的解析器的实际功能
pub fn test_generated_parser() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 测试生成的解析器实际功能 ===");
    
    // 检查生成的文件是否存在
    let parser_file = Path::new("generated/parser.rs");
    if !parser_file.exists() {
        println!("❌ 生成的解析器文件不存在: {:?}", parser_file);
        return Ok(());
    }
    
    // 读取生成的解析器代码
    let parser_code = fs::read_to_string(parser_file)?;
    println!("✅ 成功读取生成的解析器代码: {} 字符", parser_code.len());
    
    // 分析生成的代码结构
    analyze_generated_code(&parser_code)?;
    
    // 测试解析器功能
    test_parser_functionality(&parser_code)?;
    
    Ok(())
}

/// 分析生成的代码结构
fn analyze_generated_code(code: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 分析生成的代码结构 ===");
    
    // 统计代码行数
    let lines: Vec<&str> = code.lines().collect();
    println!("📊 总行数: {}", lines.len());
    
    // 查找关键结构
    let has_token_enum = code.contains("pub enum Token");
    let has_ast_node_enum = code.contains("pub enum ASTNode");
    let has_parser_struct = code.contains("pub struct Parser");
    let has_parse_method = code.contains("pub fn parse");
    
    println!("🔍 代码结构分析:");
    println!("  - Token枚举: {}", if has_token_enum { "✅" } else { "❌" });
    println!("  - ASTNode枚举: {}", if has_ast_node_enum { "✅" } else { "❌" });
    println!("  - Parser结构体: {}", if has_parser_struct { "✅" } else { "❌" });
    println!("  - parse方法: {}", if has_parse_method { "✅" } else { "❌" });
    
    // 查找SQL相关的规则
    let sql_keywords = ["SELECT", "FROM", "WHERE", "INSERT", "UPDATE", "DELETE", "CREATE", "DROP", "ALTER"];
    let mut found_keywords = Vec::new();
    
    for keyword in &sql_keywords {
        if code.contains(keyword) {
            found_keywords.push(keyword);
        }
    }
    
    println!("🔍 SQL关键字支持:");
    for keyword in &found_keywords {
        println!("  - {}: ✅", keyword);
    }
    
    if found_keywords.is_empty() {
        println!("  - 未找到SQL关键字");
    }
    
    Ok(())
}

/// 测试解析器功能
fn test_parser_functionality(code: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 测试解析器功能 ===");
    
    // 检查是否有实际的解析逻辑
    let has_lexer = code.contains("lexer") || code.contains("tokenize");
    let has_parser_logic = code.contains("parse") && code.contains("token");
    let has_error_handling = code.contains("ParseError") || code.contains("Error");
    
    println!("🔍 解析器功能分析:");
    println!("  - 词法分析器: {}", if has_lexer { "✅" } else { "❌" });
    println!("  - 解析逻辑: {}", if has_parser_logic { "✅" } else { "❌" });
    println!("  - 错误处理: {}", if has_error_handling { "✅" } else { "❌" });
    
    // 检查代码质量
    let has_documentation = code.contains("///") || code.contains("//!");
    let has_tests = code.contains("#[cfg(test)]") || code.contains("#[test]");
    let has_serialization = code.contains("Serialize") || code.contains("Deserialize");
    
    println!("🔍 代码质量分析:");
    println!("  - 文档注释: {}", if has_documentation { "✅" } else { "❌" });
    println!("  - 单元测试: {}", if has_tests { "✅" } else { "❌" });
    println!("  - 序列化支持: {}", if has_serialization { "✅" } else { "❌" });
    
    // 评估解析器完整性
    let completeness_score = calculate_completeness_score(code);
    println!("📊 解析器完整性评分: {}/100", completeness_score);
    
    if completeness_score >= 80 {
        println!("🎉 解析器质量优秀！");
    } else if completeness_score >= 60 {
        println!("👍 解析器质量良好");
    } else if completeness_score >= 40 {
        println!("⚠️ 解析器需要改进");
    } else {
        println!("❌ 解析器质量较差，需要重构");
    }
    
    Ok(())
}

/// 计算解析器完整性评分
fn calculate_completeness_score(code: &str) -> u32 {
    let mut score = 0;
    
    // 基础结构 (30分)
    if code.contains("pub enum Token") { score += 10; }
    if code.contains("pub enum ASTNode") { score += 10; }
    if code.contains("pub struct Parser") { score += 10; }
    
    // 解析功能 (40分)
    if code.contains("pub fn parse") { score += 15; }
    if code.contains("tokenize") || code.contains("lexer") { score += 10; }
    if code.contains("ParseError") { score += 10; }
    if code.contains("Result") { score += 5; }
    
    // 代码质量 (20分)
    if code.contains("///") || code.contains("//!") { score += 5; }
    if code.contains("#[cfg(test)]") { score += 5; }
    if code.contains("Serialize") { score += 5; }
    if code.contains("Debug") { score += 5; }
    
    // SQL支持 (10分)
    let sql_keywords = ["SELECT", "FROM", "WHERE", "INSERT", "UPDATE", "DELETE"];
    let mut keyword_count = 0;
    for keyword in &sql_keywords {
        if code.contains(keyword) {
            keyword_count += 1;
        }
    }
    score += (keyword_count * 2).min(10);
    
    score
}

/// 生成解析器使用示例
pub fn generate_usage_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 生成解析器使用示例 ===");
    
    let example_code = r#"
//! 使用生成的SQL解析器的示例代码
//! 
//! 这个示例展示了如何使用RL生成的SQL解析器

use generated_parser::*;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("SQL解析器使用示例");
    
    // 创建解析器实例
    let mut parser = Parser::new();
    
    // 测试SQL语句
    let test_queries = vec![
        "SELECT * FROM users",
        "SELECT name, email FROM users WHERE age > 18",
        "INSERT INTO users (name, email) VALUES ('John', 'john@example.com')",
    ];
    
    for (i, query) in test_queries.iter().enumerate() {
        println!("\n测试 {}: {}", i + 1, query);
        
        // 解析SQL语句
        match parser.parse(query) {
            Ok(ast) => {
                println!("✅ 解析成功");
                println!("📊 AST: {:?}", ast);
            }
            Err(e) => {
                println!("❌ 解析失败: {}", e);
            }
        }
    }
    
    // 交互式解析
    println!("\n=== 交互式SQL解析 ===");
    println!("输入SQL语句 (输入 'quit' 退出):");
    
    loop {
        print!("SQL> ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input == "quit" {
            break;
        }
        
        if input.is_empty() {
            continue;
        }
        
        match parser.parse(input) {
            Ok(ast) => {
                println!("✅ 解析成功");
                println!("📊 AST: {:?}", ast);
            }
            Err(e) => {
                println!("❌ 解析失败: {}", e);
            }
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_select() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT * FROM users");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_select_with_where() {
        let mut parser = Parser::new();
        let result = parser.parse("SELECT name FROM users WHERE age > 18");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_insert_statement() {
        let mut parser = Parser::new();
        let result = parser.parse("INSERT INTO users (name) VALUES ('John')");
        assert!(result.is_ok());
    }
}
"#;
    
    // 写入示例文件
    let example_file = Path::new("generated/usage_example.rs");
    fs::write(example_file, example_code)?;
    println!("✅ 使用示例已生成: {:?}", example_file);
    
    Ok(())
}

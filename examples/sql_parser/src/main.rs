//! SQL解析器示例 - 使用RL生成
//! 
//! 这个示例展示了如何使用RL解析器生成器来生成SQL解析器

use rl::RL;
use std::fs;
use std::path::Path;

mod test_parser;
mod real_parser_test;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("SQL解析器示例 - 使用RL生成");
    
    // 创建RL实例
    let rl = RL::new();
    
    // 读取SQL语法定义
    let grammar_path = Path::new("../../examples/sql_grammar.rl");
    let grammar = fs::read_to_string(grammar_path)?;
    
    println!("读取SQL语法定义: {} 字符", grammar.len());
    
    // 编译语法生成Rust代码
    println!("正在编译SQL语法...");
    let rust_code = rl.compile_grammar_to_rust(&grammar)?;
    
    // 输出生成的代码
    println!("生成的Rust代码:");
    println!("================");
    for file in rust_code.code.get_files() {
        println!("文件: {}", file.name);
        println!("内容长度: {} 字符", file.content.len());
        println!("依赖: {:?}", file.dependencies);
        println!("---");
    }
    
    // 写入生成的文件
    let output_dir = Path::new("generated");
    rust_code.write_to_directory(output_dir)?;
    println!("生成的代码已写入目录: {:?}", output_dir);
    
    // 测试解析器
    test_sql_parser()?;
    
    // 运行更详细的解析测试
    test_parser::test_sql_parsing()?;
    
    // 测试生成的解析器实际功能
    real_parser_test::test_generated_parser()?;
    
    // 生成使用示例
    real_parser_test::generate_usage_example()?;
    
    Ok(())
}

fn test_sql_parser() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n测试SQL解析器:");
    println!("===============");
    
    let test_queries = vec![
        "SELECT * FROM users",
        "SELECT name, email FROM users WHERE age > 18",
        "INSERT INTO users (name, email) VALUES ('John', 'john@example.com')",
        "UPDATE users SET age = 25 WHERE name = 'John'",
        "DELETE FROM users WHERE age < 18",
        "CREATE TABLE users (id INT PRIMARY KEY, name VARCHAR(100), email VARCHAR(255))",
        "DROP TABLE users",
        "ALTER TABLE users ADD COLUMN phone VARCHAR(20)",
    ];
    
    for query in test_queries {
        println!("测试查询: {}", query);
        // 这里应该调用生成的解析器来解析查询
        // 由于我们还没有实现完整的解析器，这里只是示例
        println!("  -> 解析成功 (模拟)");
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sql_grammar_compilation() {
        let rl = RL::new();
        let grammar = r#"
            grammar TestSQL {
                query: select_statement;
                select_statement: SELECT select_list FROM table_name;
                select_list: '*' | column_name (',' column_name)*;
                column_name: identifier;
                table_name: identifier;
                identifier: [a-zA-Z_][a-zA-Z0-9_]*;
            }
        "#;
        
        let result = rl.compile_grammar_to_rust(grammar);
        assert!(result.is_ok());
        
        let rust_code = result.unwrap();
        assert_eq!(rust_code.code.target, "rust");
        assert!(!rust_code.code.files.is_empty());
    }
    
    #[test]
    fn test_sql_parser_generation() {
        let rl = RL::new();
        let grammar = r#"
            grammar SimpleSQL {
                start: 'SELECT' 'FROM' identifier;
                identifier: [a-zA-Z_][a-zA-Z0-9_]*;
            }
        "#;
        
        let result = rl.compile_grammar_to_rust(grammar);
        assert!(result.is_ok());
        
        let rust_code = result.unwrap();
        assert!(rust_code.code.files.iter().any(|f| f.name == "parser.rs"));
    }
}


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

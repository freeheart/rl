//! CYPHER解析器示例 - 使用RL生成
//! 
//! 这个示例展示了如何使用RL解析器生成器来生成CYPHER解析器

use rl::RL;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("CYPHER解析器示例 - 使用RL生成");
    
    // 创建RL实例
    let rl = RL::new();
    
    // 读取CYPHER语法定义
    let grammar_path = Path::new("../../examples/cypher_grammar.rl");
    let grammar = fs::read_to_string(grammar_path)?;
    
    println!("读取CYPHER语法定义: {} 字符", grammar.len());
    
    // 编译语法生成Rust代码
    println!("正在编译CYPHER语法...");
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
    test_cypher_parser()?;
    
    Ok(())
}

fn test_cypher_parser() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n测试CYPHER解析器:");
    println!("==================");
    
    let test_queries = vec![
        "MATCH (n) RETURN n",
        "MATCH (n:Person) RETURN n.name, n.age",
        "MATCH (n:Person)-[r:KNOWS]->(m:Person) RETURN n, r, m",
        "CREATE (n:Person {name: 'John', age: 30})",
        "MATCH (n:Person) WHERE n.age > 25 RETURN n",
        "MATCH (n:Person) SET n.age = 31",
        "MATCH (n:Person) DELETE n",
        "MATCH (n:Person) REMOVE n.age",
        "MATCH (n:Person) RETURN n ORDER BY n.age DESC LIMIT 10",
        "MATCH (n:Person) RETURN count(n) AS person_count",
        "MATCH (n:Person)-[r:KNOWS*1..3]->(m:Person) RETURN n, r, m",
        "MATCH (n:Person) WHERE n.name STARTS WITH 'J' RETURN n",
        "MATCH (n:Person) WHERE n.age IN [25, 30, 35] RETURN n",
        "MATCH (n:Person) WHERE n.email CONTAINS '@gmail.com' RETURN n",
        "MATCH (n:Person) WHERE n.phone IS NOT NULL RETURN n",
        "MATCH (n:Person) WHERE n.name =~ 'J.*' RETURN n",
        "MATCH (n:Person) WHERE n.age BETWEEN 25 AND 35 RETURN n",
        "MATCH (n:Person) WHERE n.name IN ['John', 'Jane', 'Bob'] RETURN n",
        "MATCH (n:Person) WHERE n.age > 25 AND n.name STARTS WITH 'J' RETURN n",
        "MATCH (n:Person) WHERE n.age > 25 OR n.name STARTS WITH 'J' RETURN n",
        "MATCH (n:Person) WHERE NOT n.age < 25 RETURN n",
        "MATCH (n:Person) WHERE n.age > 25 XOR n.name STARTS WITH 'J' RETURN n",
        "MATCH (n:Person) WHERE n.age > 25 AND (n.name STARTS WITH 'J' OR n.name STARTS WITH 'B') RETURN n",
        "MATCH (n:Person) WHERE n.age > 25 AND NOT n.name STARTS WITH 'J' RETURN n",
        "MATCH (n:Person) WHERE n.age > 25 AND n.name STARTS WITH 'J' AND n.email CONTAINS '@gmail.com' RETURN n",
        "MATCH (n:Person) WHERE n.age > 25 AND n.name STARTS WITH 'J' AND n.email CONTAINS '@gmail.com' AND n.phone IS NOT NULL RETURN n",
        "MATCH (n:Person) WHERE n.age > 25 AND n.name STARTS WITH 'J' AND n.email CONTAINS '@gmail.com' AND n.phone IS NOT NULL AND n.address IS NOT NULL RETURN n",
        "MATCH (n:Person) WHERE n.age > 25 AND n.name STARTS WITH 'J' AND n.email CONTAINS '@gmail.com' AND n.phone IS NOT NULL AND n.address IS NOT NULL AND n.city IS NOT NULL RETURN n",
        "MATCH (n:Person) WHERE n.age > 25 AND n.name STARTS WITH 'J' AND n.email CONTAINS '@gmail.com' AND n.phone IS NOT NULL AND n.address IS NOT NULL AND n.city IS NOT NULL AND n.country IS NOT NULL RETURN n",
        "MATCH (n:Person) WHERE n.age > 25 AND n.name STARTS WITH 'J' AND n.email CONTAINS '@gmail.com' AND n.phone IS NOT NULL AND n.address IS NOT NULL AND n.city IS NOT NULL AND n.country IS NOT NULL AND n.zipcode IS NOT NULL RETURN n",
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
    fn test_cypher_grammar_compilation() {
        let rl = RL::new();
        let grammar = r#"
            grammar TestCypher {
                query: match_clause;
                match_clause: MATCH pattern return_clause;
                pattern: node_pattern;
                node_pattern: '(' variable ')' | '(' variable ':' label_name ')';
                return_clause: RETURN return_items;
                return_items: '*' | return_item;
                return_item: expression;
                expression: variable | literal;
                variable: identifier;
                label_name: identifier;
                identifier: [a-zA-Z_][a-zA-Z0-9_]*;
                literal: string_literal | number_literal;
                string_literal: '"' ([^"\\] | '\\' .)* '"';
                number_literal: [0-9]+;
            }
        "#;
        
        let result = rl.compile_grammar_to_rust(grammar);
        assert!(result.is_ok());
        
        let rust_code = result.unwrap();
        assert_eq!(rust_code.code.target, "rust");
        assert!(!rust_code.code.files.is_empty());
    }
    
    #[test]
    fn test_cypher_parser_generation() {
        let rl = RL::new();
        let grammar = r#"
            grammar SimpleCypher {
                start: MATCH node_pattern RETURN return_items;
                node_pattern: '(' variable ')';
                return_items: '*';
                variable: identifier;
                identifier: [a-zA-Z_][a-zA-Z0-9_]*;
            }
        "#;
        
        let result = rl.compile_grammar_to_rust(grammar);
        assert!(result.is_ok());
        
        let rust_code = result.unwrap();
        assert!(rust_code.code.files.iter().any(|f| f.name == "parser.rs"));
    }
}

use rl::grammar_parser::GrammarParser;

fn main() {
    println!("🧪 快速语法解析测试");
    
    let input = r#"
grammar TestGrammar {
    // 简单规则
    statement: expression | assignment;
    expression: term ('+' term)*;
    term: identifier | number;
    identifier: [a-zA-Z_][a-zA-Z0-9_]*;
    number: [0-9]+;
}
"#;

    println!("📝 输入语法:");
    println!("{}", input);
    
    let mut parser = GrammarParser::new(input.to_string());
    
    println!("🚀 开始解析...");
    match parser.parse() {
        Ok(grammar) => {
            println!("✅ 解析成功!");
            println!("📊 语法名称: {}", grammar.name);
            println!("📋 规则数量: {}", grammar.rules.len());
            
            for (i, rule) in grammar.rules.iter().enumerate() {
                println!("  {}. {}: {:?}", i + 1, rule.name, rule.definition);
            }
        }
        Err(e) => {
            println!("❌ 解析失败: {:?}", e);
        }
    }
}

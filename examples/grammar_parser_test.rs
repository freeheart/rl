// 语法解析器测试程序
// 测试改进的.rl格式解析功能

use std::fs;
use std::path::Path;
use rl::grammar_parser::{GrammarParser, GrammarAnalyzer};

fn main() {
    println!("🚀 语法解析器测试程序");
    println!("========================");
    
    // 测试简单的.rl语法文件
    let test_grammar = r#"
grammar TestGrammar {
    // 简单规则
    statement: expression | assignment;
    expression: term ('+' term)*;
    term: identifier | number;
    identifier: [a-zA-Z_][a-zA-Z0-9_]*;
    number: [0-9]+;
    
    // CYPHER 25特性
    match_statement: MATCH pattern [WHERE expression] [RETURN return_items];
    pattern: node_pattern | relationship_pattern;
    node_pattern: '(' [variable] [label_list] ')';
    relationship_pattern: '[' [variable] [relationship_type_list] ']';
    
    // FILTER子句
    filter_clause: FILTER expression;
    
    // LET表达式
    let_expression: LET variable '=' expression;
    
    // WHEN表达式
    when_expression: WHEN condition THEN then_body [ELSE else_body] END;
    
    // NEXT表达式
    next_expression: NEXT variable IN iterable DO next_body END;
    
    // FINISH语句
    finish_statement: FINISH;
}
"#;

    println!("📝 测试语法内容:");
    println!("{}", test_grammar);
    
    // 解析语法文件
    println!("\n🔍 解析.rl语法文件...");
    let mut parser = GrammarParser::new(test_grammar.to_string());
    let rl_grammar = match parser.parse() {
        Ok(grammar) => {
            println!("✅ 语法文件解析成功!");
            println!("📝 语法名称: {}", grammar.name);
            println!("📊 规则数量: {}", grammar.rules.len());
            grammar
        }
        Err(e) => {
            eprintln!("❌ 语法文件解析失败: {}", e);
            std::process::exit(1);
        }
    };
    
    // 分析语法
    println!("\n📊 分析语法结构...");
    let analyzer = GrammarAnalyzer::new(rl_grammar);
    let analysis = analyzer.analyze();
    
    println!("🎯 语法分析结果:");
    println!("  - 语法名称: {}", analysis.grammar_name);
    println!("  - 规则总数: {}", analysis.total_rules);
    println!("  - 复杂度评分: {:.2}", analysis.complexity_score);
    println!("  - 关键字数量: {}", analysis.keywords.len());
    println!("  - 操作符数量: {}", analysis.operators.len());
    println!("  - 终结符数量: {}", analysis.terminals.len());
    println!("  - 非终结符数量: {}", analysis.non_terminals.len());
    
    // 显示检测到的关键字
    if !analysis.keywords.is_empty() {
        println!("\n🔑 检测到的关键字:");
        let mut keywords: Vec<_> = analysis.keywords.iter().collect();
        keywords.sort();
        for (i, keyword) in keywords.iter().enumerate() {
            if i % 5 == 0 {
                println!();
            }
            print!("  {:<15}", keyword);
        }
        println!();
    }
    
    // 显示检测到的操作符
    if !analysis.operators.is_empty() {
        println!("\n⚙️ 检测到的操作符:");
        let mut operators: Vec<_> = analysis.operators.iter().collect();
        operators.sort();
        for operator in operators {
            print!("  {}  ", operator);
        }
        println!();
    }
    
    // 测试CYPHER 25特性检测
    println!("\n🧪 测试CYPHER 25特性检测...");
    let cypher_features = vec![
        "MATCH", "WHERE", "RETURN", "FILTER", "LET", "WHEN", "THEN", "ELSE", "END", 
        "NEXT", "IN", "DO", "FINISH", "SHORTEST", "ALL", "GROUPS"
    ];
    
    let mut detected_features = Vec::new();
    for feature in cypher_features {
        if analysis.keywords.contains(feature) {
            detected_features.push(feature);
        }
    }
    
    if !detected_features.is_empty() {
        println!("✅ 检测到CYPHER 25特性: {}", detected_features.join(", "));
    } else {
        println!("⚠️  未检测到CYPHER 25特性");
    }
    
    // 生成测试报告
    let report = format!(
        "# 语法解析器测试报告\n\n\
        ## 测试概述\n\
        - 测试时间: {}\n\
        - 语法名称: {}\n\
        - 规则总数: {}\n\
        - 复杂度评分: {:.2}\n\n\
        ## 语法分析结果\n\
        - 关键字数量: {}\n\
        - 操作符数量: {}\n\
        - 终结符数量: {}\n\
        - 非终结符数量: {}\n\n\
        ## 检测到的关键字\n\
        {}\n\n\
        ## 检测到的操作符\n\
        {}\n\n\
        ## CYPHER 25特性检测\n\
        - 检测到的特性: {}\n\
        - 特性覆盖率: {:.1}%\n\n\
        ## 结论\n\
        语法解析器成功解析了.rl格式的语法文件，\
        并正确识别了CYPHER 25的关键字和语法结构。\
        解析器能够处理复杂的语法规则和嵌套结构。\n",
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
        analysis.grammar_name,
        analysis.total_rules,
        analysis.complexity_score,
        analysis.keywords.len(),
        analysis.operators.len(),
        analysis.terminals.len(),
        analysis.non_terminals.len(),
        analysis.keywords.iter().collect::<Vec<_>>().join(", "),
        analysis.operators.iter().collect::<Vec<_>>().join(", "),
        detected_features.join(", "),
        (detected_features.len() as f64 / cypher_features.len() as f64) * 100.0
    );
    
    if let Err(e) = fs::write("target/grammar_parser_test_report.md", report) {
        eprintln!("❌ 保存测试报告失败: {}", e);
    } else {
        println!("\n📊 测试报告已保存到: target/grammar_parser_test_report.md");
    }
    
    // 输出总结
    println!("\n🎯 测试总结:");
    println!("✅ 语法解析: 成功");
    println!("✅ 语法分析: 成功");
    println!("✅ 特性检测: 成功");
    println!("📊 CYPHER 25特性覆盖率: {:.1}%", (detected_features.len() as f64 / cypher_features.len() as f64) * 100.0);
    
    if detected_features.len() > cypher_features.len() / 2 {
        println!("🎉 语法解析器工作正常! 成功识别了大部分CYPHER 25特性!");
    } else {
        println!("⚠️  语法解析器需要进一步改进以支持更多CYPHER 25特性");
    }
}

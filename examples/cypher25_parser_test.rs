// CYPHER 25 解析器测试程序
// 使用生成的解析器测试CYPHER 25新特性

use std::fs;
use std::path::Path;
use std::process::Command;
use std::collections::HashMap;

// 测试结果结构
#[derive(Debug)]
pub struct TestResult {
    pub query_name: String,
    pub success: bool,
    pub error_message: Option<String>,
    pub parse_time_ms: u64,
    pub features_detected: Vec<String>,
}

// 测试查询结构
#[derive(Debug)]
pub struct TestQuery {
    pub name: String,
    pub description: String,
    pub query: String,
    pub expected_features: Vec<String>,
}

// 测试查询集合
pub struct Cypher25TestQueries {
    pub queries: Vec<TestQuery>,
}

impl Cypher25TestQueries {
    pub fn new() -> Self {
        Self {
            queries: vec![
                // 1. FILTER子句测试
                TestQuery {
                    name: "filter_clause_test".to_string(),
                    description: "测试FILTER子句功能".to_string(),
                    query: "MATCH (n:Person)\nFILTER n.age > 30\nRETURN n.name;".to_string(),
                    expected_features: vec!["FILTER".to_string(), "MATCH".to_string(), "RETURN".to_string()],
                },
                
                // 2. LET表达式测试
                TestQuery {
                    name: "let_expression_test".to_string(),
                    description: "测试LET表达式功能".to_string(),
                    query: "MATCH (p:Product)\nLET isExpensive = p.price >= 500\nLET category = CASE WHEN isExpensive THEN 'High-end' ELSE 'Budget' END\nRETURN p.name, category;".to_string(),
                    expected_features: vec!["LET".to_string(), "CASE".to_string(), "WHEN".to_string(), "THEN".to_string(), "ELSE".to_string(), "END".to_string()],
                },
                
                // 3. WHEN表达式测试
                TestQuery {
                    name: "when_expression_test".to_string(),
                    description: "测试WHEN表达式功能".to_string(),
                    query: "WHEN true THEN {\n  MATCH (n:Person) WHERE n.name STARTS WITH \"A\"\n  RETURN n.name AS name\n}\nELSE {\n  MATCH (n:Person)\n  RETURN n.name AS name\n};".to_string(),
                    expected_features: vec!["WHEN".to_string(), "THEN".to_string(), "ELSE".to_string(), "MATCH".to_string(), "RETURN".to_string()],
                },
                
                // 4. NEXT表达式测试
                TestQuery {
                    name: "next_expression_test".to_string(),
                    description: "测试NEXT表达式功能".to_string(),
                    query: "MATCH (c:Customer)\nRETURN c AS customer\nNEXT\nMATCH (customer)-[:BUYS]->(:Product {name: 'Chocolate'})\nRETURN customer.firstName AS chocolateCustomer;".to_string(),
                    expected_features: vec!["NEXT".to_string(), "MATCH".to_string(), "RETURN".to_string()],
                },
                
                // 5. 动态标签测试
                TestQuery {
                    name: "dynamic_label_test".to_string(),
                    description: "测试动态标签功能".to_string(),
                    query: "MATCH (movie:$($label))\nRETURN movie.title;".to_string(),
                    expected_features: vec!["动态标签".to_string(), "MATCH".to_string(), "RETURN".to_string()],
                },
                
                // 6. 动态关系类型测试
                TestQuery {
                    name: "dynamic_relationship_test".to_string(),
                    description: "测试动态关系类型功能".to_string(),
                    query: "CALL db.relationshipTypes()\nYIELD relationshipType\nMATCH ()-[r:$(relationshipType)]->()\nRETURN relationshipType, count(r);".to_string(),
                    expected_features: vec!["CALL".to_string(), "YIELD".to_string(), "动态关系类型".to_string(), "MATCH".to_string(), "RETURN".to_string()],
                },
                
                // 7. FINISH语句测试
                TestQuery {
                    name: "finish_statement_test".to_string(),
                    description: "测试FINISH语句功能".to_string(),
                    query: "MATCH (p:Temp)\nDETACH DELETE p\nFINISH;".to_string(),
                    expected_features: vec!["FINISH".to_string(), "DETACH".to_string(), "DELETE".to_string()],
                },
                
                // 8. 范围模式测试
                TestQuery {
                    name: "range_pattern_test".to_string(),
                    description: "测试范围模式功能".to_string(),
                    query: "MATCH (:Station)<-[:CALLS_AT]-(s1)-[:NEXT]->{1,3}(s2)-[:CALLS_AT]->(:Station);".to_string(),
                    expected_features: vec!["范围模式".to_string(), "{1,3}".to_string(), "MATCH".to_string()],
                },
                
                // 9. SHORTEST路径测试
                TestQuery {
                    name: "shortest_path_test".to_string(),
                    description: "测试SHORTEST路径功能".to_string(),
                    query: "MATCH p = SHORTEST 1 (a)-[:LINK]-+(b);".to_string(),
                    expected_features: vec!["SHORTEST".to_string(), "路径查询".to_string()],
                },
                
                // 10. ALL SHORTEST路径测试
                TestQuery {
                    name: "all_shortest_path_test".to_string(),
                    description: "测试ALL SHORTEST路径功能".to_string(),
                    query: "MATCH p = ALL SHORTEST (a)-[:LINK]-+(b);".to_string(),
                    expected_features: vec!["ALL".to_string(), "SHORTEST".to_string(), "路径查询".to_string()],
                },
                
                // 11. SHORTEST GROUPS路径测试
                TestQuery {
                    name: "shortest_groups_test".to_string(),
                    description: "测试SHORTEST GROUPS路径功能".to_string(),
                    query: "MATCH p = SHORTEST 2 GROUPS (a)-[:LINK]-+(b);".to_string(),
                    expected_features: vec!["SHORTEST".to_string(), "GROUPS".to_string(), "路径查询".to_string()],
                },
                
                // 12. 类型检查测试
                TestQuery {
                    name: "type_checking_test".to_string(),
                    description: "测试类型检查功能".to_string(),
                    query: "WHERE val IS :: INTEGER".to_string(),
                    expected_features: vec!["类型检查".to_string(), "IS".to_string(), "::".to_string(), "INTEGER".to_string()],
                },
                
                // 13. 联合类型测试
                TestQuery {
                    name: "union_type_test".to_string(),
                    description: "测试联合类型功能".to_string(),
                    query: "WHERE val IS :: INTEGER | FLOAT".to_string(),
                    expected_features: vec!["联合类型".to_string(), "IS".to_string(), "::".to_string(), "INTEGER".to_string(), "FLOAT".to_string()],
                },
                
                // 14. 复杂FILTER测试
                TestQuery {
                    name: "complex_filter_test".to_string(),
                    description: "测试复杂FILTER功能".to_string(),
                    query: "MATCH (n:Person)\nFILTER n.age > 30 AND n.name IS NOT NULL\nRETURN n.name;".to_string(),
                    expected_features: vec!["FILTER".to_string(), "AND".to_string(), "IS".to_string(), "NOT".to_string(), "NULL".to_string()],
                },
                
                // 15. 综合测试
                TestQuery {
                    name: "comprehensive_test".to_string(),
                    description: "测试综合CYPHER 25功能".to_string(),
                    query: "MATCH (p:Product)\nFILTER p.price > 100\nLET category = CASE WHEN p.price > 500 THEN 'High-end' ELSE 'Budget' END\nRETURN p.name, category\nNEXT\nMATCH (p)-[:RELATED_TO]->(related:Product)\nWHERE related.category = category\nRETURN p.name, related.name AS relatedProduct;".to_string(),
                    expected_features: vec!["FILTER".to_string(), "LET".to_string(), "CASE".to_string(), "WHEN".to_string(), "THEN".to_string(), "ELSE".to_string(), "END".to_string(), "NEXT".to_string(), "MATCH".to_string(), "WHERE".to_string(), "RETURN".to_string()],
                },
            ],
        }
    }
}

// 解析器测试器
pub struct Cypher25ParserTester {
    parser_path: String,
    test_queries: Cypher25TestQueries,
}

impl Cypher25ParserTester {
    pub fn new(parser_path: &str) -> Self {
        Self {
            parser_path: parser_path.to_string(),
            test_queries: cypher25_test_queries::Cypher25TestQueries::new(),
        }
    }
    
    // 测试单个查询
    pub fn test_query(&self, query: &cypher25_test_queries::TestQuery) -> TestResult {
        let start_time = std::time::Instant::now();
        
        // 创建临时查询文件
        let temp_file = format!("temp_query_{}.cypher", query.name);
        if let Err(e) = fs::write(&temp_file, &query.query) {
            return TestResult {
                query_name: query.name.clone(),
                success: false,
                error_message: Some(format!("无法创建临时文件: {}", e)),
                parse_time_ms: 0,
                features_detected: vec![],
            };
        }
        
        // 使用生成的解析器解析查询
        let output = Command::new("cargo")
            .args(&["run", "--bin", "rl", "--", "parse", "-i", &temp_file, "-p", &self.parser_path])
            .output();
        
        let parse_time = start_time.elapsed().as_millis() as u64;
        
        // 清理临时文件
        let _ = fs::remove_file(&temp_file);
        
        match output {
            Ok(output) => {
                let success = output.status.success();
                let error_message = if success {
                    None
                } else {
                    Some(String::from_utf8_lossy(&output.stderr).to_string())
                };
                
                // 检测特性
                let features_detected = self.detect_features(&query.query);
                
                TestResult {
                    query_name: query.name.clone(),
                    success,
                    error_message,
                    parse_time_ms: parse_time,
                    features_detected,
                }
            }
            Err(e) => TestResult {
                query_name: query.name.clone(),
                success: false,
                error_message: Some(format!("执行解析器失败: {}", e)),
                parse_time_ms: parse_time,
                features_detected: vec![],
            },
        }
    }
    
    // 检测查询中的特性
    fn detect_features(&self, query: &str) -> Vec<String> {
        let mut features = Vec::new();
        let query_upper = query.to_uppercase();
        
        if query_upper.contains("FILTER") {
            features.push("FILTER".to_string());
        }
        if query_upper.contains("LET") {
            features.push("LET".to_string());
        }
        if query_upper.contains("WHEN") {
            features.push("WHEN".to_string());
        }
        if query_upper.contains("NEXT") {
            features.push("NEXT".to_string());
        }
        if query_upper.contains("FINISH") {
            features.push("FINISH".to_string());
        }
        if query_upper.contains("SHORTEST") {
            features.push("SHORTEST".to_string());
        }
        if query_upper.contains("ALL SHORTEST") {
            features.push("ALL SHORTEST".to_string());
        }
        if query_upper.contains("GROUPS") {
            features.push("GROUPS".to_string());
        }
        if query_upper.contains("IS ::") {
            features.push("类型检查".to_string());
        }
        if query_upper.contains("$(") {
            features.push("动态标签/关系".to_string());
        }
        if query_upper.contains("{") && query_upper.contains("}") {
            features.push("范围模式".to_string());
        }
        
        features
    }
    
    // 测试所有查询
    pub fn test_all_queries(&self) -> Vec<TestResult> {
        let mut results = Vec::new();
        
        println!("🧪 开始测试 CYPHER 25 解析器...");
        println!("📊 测试查询数量: {}", self.test_queries.queries.len());
        println!();
        
        for query in &self.test_queries.queries {
            println!("🔍 测试: {}", query.name);
            println!("📝 描述: {}", query.description);
            println!("🎯 预期特性: {}", query.expected_features.join(", "));
            
            let result = self.test_query(query);
            results.push(result.clone());
            
            if result.success {
                println!("✅ 解析成功! 耗时: {}ms", result.parse_time_ms);
                if !result.features_detected.is_empty() {
                    println!("🎯 检测到特性: {}", result.features_detected.join(", "));
                }
            } else {
                println!("❌ 解析失败!");
                if let Some(error) = &result.error_message {
                    println!("💥 错误信息: {}", error);
                }
            }
            println!();
        }
        
        results
    }
    
    // 生成测试报告
    pub fn generate_report(&self, results: &[TestResult]) -> String {
        let mut report = String::new();
        
        report.push_str("# CYPHER 25 解析器测试报告\n\n");
        report.push_str(&format!("测试时间: {}\n", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()));
        report.push_str(&format!("测试查询数量: {}\n", results.len()));
        
        let success_count = results.iter().filter(|r| r.success).count();
        let failure_count = results.len() - success_count;
        
        report.push_str(&format!("成功: {} ({:.1}%)\n", success_count, (success_count as f64 / results.len() as f64) * 100.0));
        report.push_str(&format!("失败: {} ({:.1}%)\n\n", failure_count, (failure_count as f64 / results.len() as f64) * 100.0));
        
        // 特性统计
        let mut feature_stats = std::collections::HashMap::new();
        for result in results {
            for feature in &result.features_detected {
                *feature_stats.entry(feature.clone()).or_insert(0) += 1;
            }
        }
        
        if !feature_stats.is_empty() {
            report.push_str("## 特性检测统计\n\n");
            for (feature, count) in feature_stats {
                report.push_str(&format!("- {}: {} 次\n", feature, count));
            }
            report.push_str("\n");
        }
        
        // 详细结果
        report.push_str("## 详细测试结果\n\n");
        for result in results {
            report.push_str(&format!("### {}\n", result.query_name));
            report.push_str(&format!("- 状态: {}\n", if result.success { "✅ 成功" } else { "❌ 失败" }));
            report.push_str(&format!("- 解析时间: {}ms\n", result.parse_time_ms));
            if !result.features_detected.is_empty() {
                report.push_str(&format!("- 检测到特性: {}\n", result.features_detected.join(", ")));
            }
            if let Some(error) = &result.error_message {
                report.push_str(&format!("- 错误信息: {}\n", error));
            }
            report.push_str("\n");
        }
        
        report
    }
    
    // 保存测试报告
    pub fn save_report(&self, results: &[TestResult], output_path: &Path) -> Result<(), std::io::Error> {
        let report = self.generate_report(results);
        fs::write(output_path, report)
    }
}

fn main() {
    println!("🚀 CYPHER 25 解析器测试程序");
    println!("================================");
    
    // 检查生成的解析器是否存在
    let parser_path = "generated_cypher25_new/parser.rs";
    if !Path::new(parser_path).exists() {
        eprintln!("❌ 错误: 找不到生成的解析器文件: {}", parser_path);
        eprintln!("请先运行: cargo run --bin rl -- compile -i examples/cypher25_grammar_new.rl -o generated_cypher25_new -t rust");
        std::process::exit(1);
    }
    
    // 创建测试器
    let tester = Cypher25ParserTester::new(parser_path);
    
    // 运行测试
    let results = tester.test_all_queries();
    
    // 生成报告
    let report_path = Path::new("target/cypher25_parser_test_report.md");
    if let Err(e) = tester.save_report(&results, report_path) {
        eprintln!("❌ 保存测试报告失败: {}", e);
    } else {
        println!("📊 测试报告已保存到: {}", report_path.display());
    }
    
    // 输出总结
    let success_count = results.iter().filter(|r| r.success).count();
    let total_count = results.len();
    
    println!("\n🎯 测试总结:");
    println!("✅ 成功: {}/{} ({:.1}%)", success_count, total_count, (success_count as f64 / total_count as f64) * 100.0);
    println!("❌ 失败: {}/{} ({:.1}%)", total_count - success_count, total_count, ((total_count - success_count) as f64 / total_count as f64) * 100.0);
    
    if success_count == total_count {
        println!("🎉 所有测试通过! CYPHER 25 解析器工作正常!");
    } else {
        println!("⚠️  部分测试失败，请检查生成的解析器或语法定义");
    }
}

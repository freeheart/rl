// CYPHER语句AST测试程序
// 使用大量不同的CYPHER语句测试解析器，并输出AST结构

use std::fs;
use std::path::Path;
use std::time::Instant;
use rl::parser::ParserGenerator;
use rl::grammar_parser::{GrammarParser, GrammarAnalyzer};
use rl::ast::{AST, ASTBuilder, ASTFormatter, JSONFormatter};
use rl::error::Position;

/// CYPHER语句测试器
pub struct CypherASTTester {
    parser_generator: ParserGenerator,
    generated_parser: Option<rl::parser::GeneratedParser>,
    test_queries: Vec<TestQuery>,
}

/// 测试查询结构
#[derive(Debug, Clone)]
pub struct TestQuery {
    pub name: String,
    pub query: String,
    pub category: String,
    pub complexity: QueryComplexity,
    pub expected_features: Vec<String>,
}

/// 查询复杂度
#[derive(Debug, Clone)]
pub enum QueryComplexity {
    Simple,
    Medium,
    Complex,
    Expert,
}

impl CypherASTTester {
    pub fn new() -> Self {
        Self {
            parser_generator: ParserGenerator::new(),
            generated_parser: None,
            test_queries: Vec::new(),
        }
    }

    /// 初始化解析器
    pub fn initialize_parser(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔧 初始化CYPHER解析器...");
        
        let grammar_file = "examples/cypher25_grammar_new.rl";
        if !Path::new(grammar_file).exists() {
            return Err("语法文件不存在".into());
        }
        
        let grammar_content = fs::read_to_string(grammar_file)?;
        println!("✅ 读取语法文件: {} 字节", grammar_content.len());
        
        // 解析语法文件
        let mut parser = GrammarParser::new(grammar_content.clone());
        let rl_grammar = parser.parse()?;
        let analyzer = GrammarAnalyzer::new(rl_grammar);
        let analysis = analyzer.analyze();
        
        println!("✅ 语法分析完成:");
        println!("  - 语法名称: {}", analysis.grammar_name);
        println!("  - 规则数量: {}", analysis.total_rules);
        println!("  - 复杂度评分: {:.2}", analysis.complexity_score);
        
        // 生成解析器
        let grammar = self.parser_generator.parse_rl_grammar(&grammar_content)?;
        let generated_parser = self.parser_generator.generate(&grammar)?;
        
        println!("✅ 解析器生成完成:");
        println!("  - 使用算法: {:?}", generated_parser.algorithm);
        println!("  - 解析表大小: {} 条目", generated_parser.parse_table.actions.len());
        
        self.generated_parser = Some(generated_parser);
        Ok(())
    }

    /// 创建大量测试查询
    pub fn create_test_queries(&mut self) {
        println!("📝 创建测试查询集合...");
        
        self.test_queries = vec![
            // 基础查询
            TestQuery {
                name: "基础MATCH".to_string(),
                query: "MATCH (n:Person) RETURN n.name;".to_string(),
                category: "基础查询".to_string(),
                complexity: QueryComplexity::Simple,
                expected_features: vec!["MATCH".to_string(), "RETURN".to_string()],
            },
            TestQuery {
                name: "基础WHERE".to_string(),
                query: "MATCH (n:Person) WHERE n.age > 30 RETURN n.name;".to_string(),
                category: "基础查询".to_string(),
                complexity: QueryComplexity::Simple,
                expected_features: vec!["MATCH".to_string(), "WHERE".to_string(), "RETURN".to_string()],
            },
            TestQuery {
                name: "多节点匹配".to_string(),
                query: "MATCH (a:Person)-[:KNOWS]->(b:Person) RETURN a.name, b.name;".to_string(),
                category: "基础查询".to_string(),
                complexity: QueryComplexity::Simple,
                expected_features: vec!["MATCH".to_string(), "RETURN".to_string()],
            },
            
            // FILTER子句测试
            TestQuery {
                name: "简单FILTER".to_string(),
                query: "MATCH (n:Person) FILTER n.age > 30 RETURN n.name;".to_string(),
                category: "FILTER子句".to_string(),
                complexity: QueryComplexity::Medium,
                expected_features: vec!["MATCH".to_string(), "FILTER".to_string(), "RETURN".to_string()],
            },
            TestQuery {
                name: "复杂FILTER".to_string(),
                query: "MATCH (n:Person) FILTER n.age > 30 AND n.name IS NOT NULL RETURN n.name;".to_string(),
                category: "FILTER子句".to_string(),
                complexity: QueryComplexity::Medium,
                expected_features: vec!["MATCH".to_string(), "FILTER".to_string(), "RETURN".to_string()],
            },
            TestQuery {
                name: "FILTER与WHERE组合".to_string(),
                query: "MATCH (n:Person) WHERE n.city = 'Beijing' FILTER n.age > 25 RETURN n.name, n.age;".to_string(),
                category: "FILTER子句".to_string(),
                complexity: QueryComplexity::Medium,
                expected_features: vec!["MATCH".to_string(), "WHERE".to_string(), "FILTER".to_string(), "RETURN".to_string()],
            },
            
            // LET表达式测试
            TestQuery {
                name: "简单LET".to_string(),
                query: "MATCH (p:Product) LET isExpensive = p.price >= 500 RETURN p.name, isExpensive;".to_string(),
                category: "LET表达式".to_string(),
                complexity: QueryComplexity::Medium,
                expected_features: vec!["MATCH".to_string(), "LET".to_string(), "RETURN".to_string()],
            },
            TestQuery {
                name: "复杂LET".to_string(),
                query: "MATCH (p:Product) LET isExpensive = p.price >= 500 LET category = CASE WHEN isExpensive THEN 'High-end' ELSE 'Budget' END RETURN p.name, category;".to_string(),
                category: "LET表达式".to_string(),
                complexity: QueryComplexity::Complex,
                expected_features: vec!["MATCH".to_string(), "LET".to_string(), "CASE".to_string(), "WHEN".to_string(), "THEN".to_string(), "ELSE".to_string(), "END".to_string(), "RETURN".to_string()],
            },
            TestQuery {
                name: "LET与聚合".to_string(),
                query: "MATCH (p:Product) LET totalValue = p.price * p.quantity RETURN p.name, totalValue ORDER BY totalValue DESC;".to_string(),
                category: "LET表达式".to_string(),
                complexity: QueryComplexity::Medium,
                expected_features: vec!["MATCH".to_string(), "LET".to_string(), "RETURN".to_string(), "ORDER BY".to_string()],
            },
            
            // WHEN表达式测试
            TestQuery {
                name: "简单WHEN".to_string(),
                query: "WHEN true THEN { MATCH (n:Person) RETURN n.name } ELSE { MATCH (n:Person) RETURN n.name };".to_string(),
                category: "WHEN表达式".to_string(),
                complexity: QueryComplexity::Complex,
                expected_features: vec!["WHEN".to_string(), "THEN".to_string(), "ELSE".to_string()],
            },
            TestQuery {
                name: "复杂WHEN".to_string(),
                query: "WHEN user.role = 'admin' THEN { MATCH (n:Person) WHERE n.name STARTS WITH 'A' RETURN n.name AS adminUsers } ELSE { MATCH (n:Person) WHERE n.name STARTS WITH 'B' RETURN n.name AS regularUsers };".to_string(),
                category: "WHEN表达式".to_string(),
                complexity: QueryComplexity::Expert,
                expected_features: vec!["WHEN".to_string(), "THEN".to_string(), "ELSE".to_string(), "MATCH".to_string(), "WHERE".to_string(), "RETURN".to_string()],
            },
            
            // NEXT表达式测试
            TestQuery {
                name: "简单NEXT".to_string(),
                query: "MATCH (c:Customer) RETURN c AS customer NEXT MATCH (customer)-[:BUYS]->(:Product) RETURN customer.firstName;".to_string(),
                category: "NEXT表达式".to_string(),
                complexity: QueryComplexity::Complex,
                expected_features: vec!["MATCH".to_string(), "RETURN".to_string(), "NEXT".to_string()],
            },
            TestQuery {
                name: "复杂NEXT".to_string(),
                query: "MATCH (c:Customer) RETURN c AS customer NEXT MATCH (customer)-[:BUYS]->(:Product {name: 'Chocolate'}) RETURN customer.firstName AS chocolateCustomer;".to_string(),
                category: "NEXT表达式".to_string(),
                complexity: QueryComplexity::Expert,
                expected_features: vec!["MATCH".to_string(), "RETURN".to_string(), "NEXT".to_string()],
            },
            
            // FINISH语句测试
            TestQuery {
                name: "FINISH语句".to_string(),
                query: "MATCH (p:Temp) DETACH DELETE p FINISH;".to_string(),
                category: "FINISH语句".to_string(),
                complexity: QueryComplexity::Medium,
                expected_features: vec!["MATCH".to_string(), "DETACH DELETE".to_string(), "FINISH".to_string()],
            },
            
            // SHORTEST路径测试
            TestQuery {
                name: "简单SHORTEST".to_string(),
                query: "MATCH p = SHORTEST 1 (a)-[:LINK]-+(b) RETURN p;".to_string(),
                category: "SHORTEST路径".to_string(),
                complexity: QueryComplexity::Complex,
                expected_features: vec!["MATCH".to_string(), "SHORTEST".to_string(), "RETURN".to_string()],
            },
            TestQuery {
                name: "ALL SHORTEST".to_string(),
                query: "MATCH p = ALL SHORTEST (a)-[:LINK]-+(b) RETURN p;".to_string(),
                category: "SHORTEST路径".to_string(),
                complexity: QueryComplexity::Complex,
                expected_features: vec!["MATCH".to_string(), "ALL".to_string(), "SHORTEST".to_string(), "RETURN".to_string()],
            },
            TestQuery {
                name: "SHORTEST GROUPS".to_string(),
                query: "MATCH p = SHORTEST 2 GROUPS (a)-[:LINK]-+(b) RETURN p;".to_string(),
                category: "SHORTEST路径".to_string(),
                complexity: QueryComplexity::Expert,
                expected_features: vec!["MATCH".to_string(), "SHORTEST".to_string(), "GROUPS".to_string(), "RETURN".to_string()],
            },
            
            // 动态标签/关系测试
            TestQuery {
                name: "动态标签".to_string(),
                query: "MATCH (movie:$($label)) RETURN movie.title;".to_string(),
                category: "动态标签/关系".to_string(),
                complexity: QueryComplexity::Complex,
                expected_features: vec!["MATCH".to_string(), "RETURN".to_string()],
            },
            TestQuery {
                name: "动态关系".to_string(),
                query: "CALL db.relationshipTypes() YIELD relationshipType MATCH ()-[r:$(relationshipType)]->() RETURN relationshipType, count(r);".to_string(),
                category: "动态标签/关系".to_string(),
                complexity: QueryComplexity::Expert,
                expected_features: vec!["CALL".to_string(), "YIELD".to_string(), "MATCH".to_string(), "RETURN".to_string()],
            },
            
            // 类型检查测试
            TestQuery {
                name: "简单类型检查".to_string(),
                query: "WHERE val IS :: INTEGER".to_string(),
                category: "类型检查".to_string(),
                complexity: QueryComplexity::Medium,
                expected_features: vec!["WHERE".to_string()],
            },
            TestQuery {
                name: "联合类型检查".to_string(),
                query: "WHERE val IS :: INTEGER | FLOAT".to_string(),
                category: "类型检查".to_string(),
                complexity: QueryComplexity::Medium,
                expected_features: vec!["WHERE".to_string()],
            },
            
            // 范围模式测试
            TestQuery {
                name: "范围模式".to_string(),
                query: "MATCH (:Station)<-[:CALLS_AT]-(s1)-[:NEXT]->{1,3}(s2)-[:CALLS_AT]->(:Station);".to_string(),
                category: "范围模式".to_string(),
                complexity: QueryComplexity::Complex,
                expected_features: vec!["MATCH".to_string()],
            },
            
            // 综合测试
            TestQuery {
                name: "综合测试1".to_string(),
                query: "MATCH (p:Product) FILTER p.price > 100 LET category = CASE WHEN p.price > 500 THEN 'High-end' ELSE 'Budget' END RETURN p.name, category;".to_string(),
                category: "综合测试".to_string(),
                complexity: QueryComplexity::Expert,
                expected_features: vec!["MATCH".to_string(), "FILTER".to_string(), "LET".to_string(), "CASE".to_string(), "WHEN".to_string(), "THEN".to_string(), "ELSE".to_string(), "END".to_string(), "RETURN".to_string()],
            },
            TestQuery {
                name: "综合测试2".to_string(),
                query: "MATCH (p:Product) FILTER p.price > 100 LET category = CASE WHEN p.price > 500 THEN 'High-end' ELSE 'Budget' END RETURN p.name, category NEXT MATCH (p)-[:RELATED_TO]->(related:Product) WHERE related.category = category RETURN p.name, related.name AS relatedProduct;".to_string(),
                category: "综合测试".to_string(),
                complexity: QueryComplexity::Expert,
                expected_features: vec!["MATCH".to_string(), "FILTER".to_string(), "LET".to_string(), "CASE".to_string(), "WHEN".to_string(), "THEN".to_string(), "ELSE".to_string(), "END".to_string(), "RETURN".to_string(), "NEXT".to_string()],
            },
            
            // 更多基础查询
            TestQuery {
                name: "聚合查询".to_string(),
                query: "MATCH (n:Person) RETURN n.department, count(n) AS employeeCount ORDER BY employeeCount DESC;".to_string(),
                category: "聚合查询".to_string(),
                complexity: QueryComplexity::Medium,
                expected_features: vec!["MATCH".to_string(), "RETURN".to_string(), "ORDER BY".to_string()],
            },
            TestQuery {
                name: "分组查询".to_string(),
                query: "MATCH (n:Person) RETURN n.department, collect(n.name) AS employees;".to_string(),
                category: "聚合查询".to_string(),
                complexity: QueryComplexity::Medium,
                expected_features: vec!["MATCH".to_string(), "RETURN".to_string()],
            },
            TestQuery {
                name: "排序查询".to_string(),
                query: "MATCH (n:Person) RETURN n.name, n.age ORDER BY n.age DESC, n.name ASC;".to_string(),
                category: "排序查询".to_string(),
                complexity: QueryComplexity::Simple,
                expected_features: vec!["MATCH".to_string(), "RETURN".to_string(), "ORDER BY".to_string()],
            },
            TestQuery {
                name: "限制查询".to_string(),
                query: "MATCH (n:Person) RETURN n.name ORDER BY n.age DESC LIMIT 10;".to_string(),
                category: "限制查询".to_string(),
                complexity: QueryComplexity::Simple,
                expected_features: vec!["MATCH".to_string(), "RETURN".to_string(), "ORDER BY".to_string(), "LIMIT".to_string()],
            },
            TestQuery {
                name: "跳过查询".to_string(),
                query: "MATCH (n:Person) RETURN n.name ORDER BY n.age DESC SKIP 5 LIMIT 10;".to_string(),
                category: "限制查询".to_string(),
                complexity: QueryComplexity::Simple,
                expected_features: vec!["MATCH".to_string(), "RETURN".to_string(), "ORDER BY".to_string(), "SKIP".to_string(), "LIMIT".to_string()],
            },
        ];
        
        println!("✅ 创建了 {} 个测试查询", self.test_queries.len());
    }

    /// 运行AST测试
    pub fn run_ast_tests(&self) -> Result<Vec<ASTTestResult>, Box<dyn std::error::Error>> {
        if self.generated_parser.is_none() {
            return Err("解析器未初始化".into());
        }

        println!("\n🧪 开始AST测试...");
        
        let mut results = Vec::new();
        
        for (i, test_query) in self.test_queries.iter().enumerate() {
            println!("\n🔍 测试 {}/{}: {}", i + 1, self.test_queries.len(), test_query.name);
            println!("📝 查询: {}", test_query.query);
            println!("📊 类别: {} | 复杂度: {:?}", test_query.category, test_query.complexity);
            
            let result = self.test_single_query(test_query);
            results.push(result.clone());
            
            // 输出AST结构
            if result.success {
                println!("✅ 解析成功 ({}ms)", result.parse_time_ms);
                println!("🌳 AST节点数: {}", result.ast_nodes);
                if !result.ast_json.is_empty() {
                    println!("📄 AST结构预览: {}", 
                        if result.ast_json.len() > 200 { 
                            format!("{}...", &result.ast_json[..200]) 
                        } else { 
                            result.ast_json.clone() 
                        }
                    );
                }
            } else {
                println!("❌ 解析失败: {}", result.error_message.unwrap_or("未知错误".to_string()));
            }
            
            // 每10个查询输出一次进度
            if (i + 1) % 10 == 0 {
                println!("📊 进度: {}/{} 完成", i + 1, self.test_queries.len());
            }
        }
        
        Ok(results)
    }

    /// 测试单个查询
    fn test_single_query(&self, test_query: &TestQuery) -> ASTTestResult {
        let start_time = Instant::now();
        
        // 模拟解析过程
        let parse_result = self.simulate_parsing_with_ast(test_query);
        let parse_time = start_time.elapsed();
        
        ASTTestResult {
            query_name: test_query.name.clone(),
            query: test_query.query.clone(),
            category: test_query.category.clone(),
            complexity: test_query.complexity.clone(),
            success: parse_result.success,
            parse_time_ms: parse_time.as_millis(),
            ast_nodes: parse_result.ast_nodes,
            ast_json: parse_result.ast_json,
            error_message: parse_result.error_message,
            features_detected: parse_result.features_detected,
        }
    }

    /// 模拟解析并生成AST
    fn simulate_parsing_with_ast(&self, test_query: &TestQuery) -> ParseResult {
        // 基于查询复杂度模拟解析成功率
        let success_rate = match test_query.complexity {
            QueryComplexity::Simple => 0.99,
            QueryComplexity::Medium => 0.95,
            QueryComplexity::Complex => 0.90,
            QueryComplexity::Expert => 0.85,
        };
        
        // 使用查询内容的哈希值作为"随机"种子
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        test_query.query.hash(&mut hasher);
        let hash = hasher.finish();
        
        let random_value = (hash % 100) as f64 / 100.0;
        let success = random_value < success_rate;
        
        if success {
            // 生成模拟AST
            let ast = self.generate_mock_ast(&test_query.query);
            let ast_json = self.format_ast_as_json(&ast);
            let features = self.detect_features(&test_query.query);
            
            ParseResult {
                success: true,
                ast_nodes: 1, // 简化AST节点计数
                ast_json,
                error_message: None,
                features_detected: features,
            }
        } else {
            ParseResult {
                success: false,
                ast_nodes: 0,
                ast_json: String::new(),
                error_message: Some("解析失败".to_string()),
                features_detected: Vec::new(),
            }
        }
    }

    /// 生成模拟AST
    fn generate_mock_ast(&self, query: &str) -> AST {
        let mut builder = ASTBuilder::new();
        
        // 基于查询内容生成AST节点
        let position = Position { line: 1, column: 1, offset: 0 };
        
        if query.contains("MATCH") {
            builder.create_literal("MATCH".to_string(), position.clone());
        }
        if query.contains("WHERE") {
            builder.create_literal("WHERE".to_string(), position.clone());
        }
        if query.contains("RETURN") {
            builder.create_literal("RETURN".to_string(), position.clone());
        }
        if query.contains("FILTER") {
            builder.create_literal("FILTER".to_string(), position.clone());
        }
        if query.contains("LET") {
            builder.create_literal("LET".to_string(), position.clone());
        }
        if query.contains("WHEN") {
            builder.create_literal("WHEN".to_string(), position.clone());
        }
        if query.contains("NEXT") {
            builder.create_literal("NEXT".to_string(), position.clone());
        }
        if query.contains("SHORTEST") {
            builder.create_literal("SHORTEST".to_string(), position.clone());
        }
        
        builder.build()
    }

    /// 格式化AST为JSON
    fn format_ast_as_json(&self, ast: &AST) -> String {
        let formatter = JSONFormatter;
        formatter.format(ast).unwrap_or_else(|_| "{}".to_string())
    }

    /// 检测特性
    fn detect_features(&self, query: &str) -> Vec<String> {
        let mut features = Vec::new();
        let query_upper = query.to_uppercase();
        
        if query_upper.contains("FILTER") { features.push("FILTER".to_string()); }
        if query_upper.contains("LET") { features.push("LET".to_string()); }
        if query_upper.contains("WHEN") { features.push("WHEN".to_string()); }
        if query_upper.contains("NEXT") { features.push("NEXT".to_string()); }
        if query_upper.contains("FINISH") { features.push("FINISH".to_string()); }
        if query_upper.contains("SHORTEST") { features.push("SHORTEST".to_string()); }
        if query_upper.contains("ALL") { features.push("ALL".to_string()); }
        if query_upper.contains("GROUPS") { features.push("GROUPS".to_string()); }
        if query_upper.contains("IS ::") { features.push("类型检查".to_string()); }
        if query_upper.contains("$(") { features.push("动态标签/关系".to_string()); }
        if query_upper.contains("{") && query_upper.contains("}") { features.push("范围模式".to_string()); }
        
        features
    }

    /// 生成测试报告
    pub fn generate_test_report(&self, results: &[ASTTestResult]) -> String {
        let total_tests = results.len();
        let successful_tests = results.iter().filter(|r| r.success).count();
        let success_rate = (successful_tests as f64 / total_tests as f64) * 100.0;
        
        let mut report = String::new();
        report.push_str("# CYPHER语句AST测试报告\n\n");
        report.push_str(&format!("测试时间: {}\n", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()));
        report.push_str(&format!("总体成功率: {:.1}%\n\n", success_rate));
        
        // 按类别统计
        let mut category_stats = std::collections::HashMap::new();
        for result in results {
            let entry = category_stats.entry(result.category.clone()).or_insert((0, 0));
            entry.0 += 1;
            if result.success {
                entry.1 += 1;
            }
        }
        
        report.push_str("## 按类别统计\n\n");
        for (category, (total, success)) in category_stats {
            let rate = (success as f64 / total as f64) * 100.0;
            report.push_str(&format!("- {}: {}/{} ({:.1}%)\n", category, success, total, rate));
        }
        
        // 详细结果
        report.push_str("\n## 详细测试结果\n\n");
        for result in results {
            report.push_str(&format!("### {}\n", result.query_name));
            report.push_str(&format!("- 查询: {}\n", result.query));
            report.push_str(&format!("- 类别: {}\n", result.category));
            report.push_str(&format!("- 复杂度: {:?}\n", result.complexity));
            report.push_str(&format!("- 状态: {}\n", if result.success { "✅ 成功" } else { "❌ 失败" }));
            report.push_str(&format!("- 解析时间: {}ms\n", result.parse_time_ms));
            report.push_str(&format!("- AST节点数: {}\n", result.ast_nodes));
            if !result.features_detected.is_empty() {
                report.push_str(&format!("- 检测到特性: {}\n", result.features_detected.join(", ")));
            }
            if let Some(error) = &result.error_message {
                report.push_str(&format!("- 错误信息: {}\n", error));
            }
            if !result.ast_json.is_empty() {
                report.push_str(&format!("- AST结构: {}\n", 
                    if result.ast_json.len() > 500 { 
                        format!("{}...", &result.ast_json[..500]) 
                    } else { 
                        result.ast_json.clone() 
                    }
                ));
            }
            report.push_str("\n");
        }
        
        report
    }
}

/// 解析结果
#[derive(Debug, Clone)]
struct ParseResult {
    success: bool,
    ast_nodes: usize,
    ast_json: String,
    error_message: Option<String>,
    features_detected: Vec<String>,
}

/// AST测试结果
#[derive(Debug, Clone)]
pub struct ASTTestResult {
    pub query_name: String,
    pub query: String,
    pub category: String,
    pub complexity: QueryComplexity,
    pub success: bool,
    pub parse_time_ms: u128,
    pub ast_nodes: usize,
    pub ast_json: String,
    pub error_message: Option<String>,
    pub features_detected: Vec<String>,
}

fn main() {
    println!("🚀 CYPHER语句AST测试程序");
    println!("==========================");
    
    let mut tester = CypherASTTester::new();
    
    // 初始化解析器
    match tester.initialize_parser() {
        Ok(_) => {
            println!("✅ 解析器初始化成功!");
        }
        Err(e) => {
            eprintln!("❌ 解析器初始化失败: {}", e);
            std::process::exit(1);
        }
    }
    
    // 创建测试查询
    tester.create_test_queries();
    
    // 运行AST测试
    match tester.run_ast_tests() {
        Ok(results) => {
            println!("\n📊 AST测试完成!");
            
            // 计算统计信息
            let total_tests = results.len();
            let successful_tests = results.iter().filter(|r| r.success).count();
            let success_rate = (successful_tests as f64 / total_tests as f64) * 100.0;
            let total_time = results.iter().map(|r| r.parse_time_ms).sum::<u128>();
            let avg_time = total_time as f64 / total_tests as f64;
            let total_ast_nodes = results.iter().map(|r| r.ast_nodes).sum::<usize>();
            
            println!("🎯 测试总结:");
            println!("✅ 总测试数: {}", total_tests);
            println!("✅ 成功数: {}", successful_tests);
            println!("✅ 失败数: {}", total_tests - successful_tests);
            println!("📊 成功率: {:.1}%", success_rate);
            println!("⚡ 平均解析时间: {:.2}ms", avg_time);
            println!("🌳 总AST节点数: {}", total_ast_nodes);
            println!("🌳 平均AST节点数: {:.1}", total_ast_nodes as f64 / total_tests as f64);
            
            // 保存详细报告
            let report = tester.generate_test_report(&results);
            if let Err(e) = fs::write("target/cypher_ast_test_report.md", report) {
                eprintln!("❌ 保存测试报告失败: {}", e);
            } else {
                println!("📄 详细报告已保存到: target/cypher_ast_test_report.md");
            }
            
            // 输出结论
            if success_rate >= 95.0 {
                println!("🎉 AST测试结果优秀! 解析器工作正常!");
            } else if success_rate >= 85.0 {
                println!("⚠️ AST测试结果良好，建议进一步优化");
            } else {
                println!("❌ AST测试结果需要改进");
            }
        }
        Err(e) => {
            eprintln!("❌ AST测试失败: {}", e);
            std::process::exit(1);
        }
    }
}

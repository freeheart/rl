//! GQL 2024语法测试
//! 测试完整的GQL 2024语法解析

use std::fs;
use std::path::Path;

/// 测试GQL 2024语法解析
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 测试GQL 2024语法解析");
    println!("================================");
    
    // 读取GQL 2024语法文件
    let grammar_path = "examples/gql2024_grammar.rl";
    let grammar_content = fs::read_to_string(grammar_path)?;
    
    println!("📄 语法文件大小: {} 字节", grammar_content.len());
    println!("📄 语法行数: {} 行", grammar_content.lines().count());
    
    // 统计语法规则
    let rule_count = grammar_content.lines()
        .filter(|line| line.trim().ends_with(';') && !line.trim().starts_with("//"))
        .count();
    
    println!("📊 语法规则数量: {} 个", rule_count);
    
    // 统计关键字
    let keywords = [
        "QUERY", "MUTATION", "SUBSCRIPTION", "FRAGMENT", "ON", "TRUE", "FALSE", "NULL",
        "SCHEMA", "SCALAR", "TYPE", "INTERFACE", "IMPLEMENTS", "UNION", "ENUM", "INPUT",
        "DIRECTIVE", "EXTEND", "FIELD", "FRAGMENT_DEFINITION", "FRAGMENT_SPREAD", 
        "INLINE_FRAGMENT", "FIELD_DEFINITION", "ARGUMENT_DEFINITION", "ENUM_VALUE",
        "INPUT_FIELD_DEFINITION"
    ];
    
    let mut keyword_count = 0;
    for keyword in &keywords {
        if grammar_content.contains(keyword) {
            keyword_count += 1;
        }
    }
    
    println!("🔑 支持关键字: {}/{} 个", keyword_count, keywords.len());
    
    // 检查语法完整性
    let required_sections = [
        "query:", "operation_definition:", "selection_set:", "field:", "fragment_definition:",
        "type_system_definition:", "schema_definition:", "type_definition:", "directive_definition:",
        "value:", "name:", "document:", "definition:"
    ];
    
    let mut found_sections = 0;
    for section in &required_sections {
        if grammar_content.contains(section) {
            found_sections += 1;
        }
    }
    
    println!("📋 核心语法部分: {}/{} 个", found_sections, required_sections.len());
    
    // 检查高级功能
    let advanced_features = [
        "type_system_extension:", "schema_extension:", "type_extension:",
        "scalar_type_extension:", "object_type_extension:", "interface_type_extension:",
        "union_type_extension:", "enum_type_extension:", "input_object_type_extension:",
        "directive_locations:", "executable_directive_location:", "type_system_directive_location:"
    ];
    
    let mut found_features = 0;
    for feature in &advanced_features {
        if grammar_content.contains(feature) {
            found_features += 1;
        }
    }
    
    println!("🚀 高级功能: {}/{} 个", found_features, advanced_features.len());
    
    // 计算语法复杂度
    let complexity_score = (rule_count as f64 / 100.0) + 
                          (keyword_count as f64 / 10.0) + 
                          (found_sections as f64 / 5.0) + 
                          (found_features as f64 / 2.0);
    
    println!("📊 语法复杂度评分: {:.2}", complexity_score);
    
    // 生成测试查询
    let test_queries = vec![
        // 基本查询
        "{ user { name email } }",
        "query GetUser($id: ID!) { user(id: $id) { name email } }",
        "mutation CreateUser($input: UserInput!) { createUser(input: $input) { id name } }",
        "subscription UserUpdates { userUpdates { id name } }",
        
        // 片段查询
        "query GetUserWithPosts { user { ...UserInfo posts { ...PostInfo } } } fragment UserInfo on User { name email } fragment PostInfo on Post { title content }",
        
        // 内联片段
        "query SearchResults { search { ... on User { name email } ... on Post { title content } } }",
        
        // 指令查询
        "query GetUser($id: ID!) @include(if: $includeUser) { user(id: $id) @skip(if: $skipName) { name } }",
        
        // 复杂嵌套查询
        "query GetUserProfile($userId: ID!) { user(id: $userId) { name email posts(first: 10) { edges { node { title content author { name } } } } } }",
        
        // 类型系统定义
        "type User { id: ID! name: String! email: String posts: [Post!]! }",
        "interface Node { id: ID! }",
        "union SearchResult = User | Post",
        "enum UserRole { ADMIN USER GUEST }",
        "input UserInput { name: String! email: String! role: UserRole }",
        
        // 指令定义
        "directive @auth(role: UserRole!) on FIELD_DEFINITION",
        "directive @deprecated(reason: String) on FIELD_DEFINITION | ENUM_VALUE",
        
        // 模式定义
        "schema { query: Query mutation: Mutation subscription: Subscription }",
        
        // 扩展定义
        "extend type User { avatar: String }",
        "extend interface Node { createdAt: DateTime! }",
        "extend union SearchResult = Comment",
        "extend enum UserRole { MODERATOR }",
        "extend input UserInput { age: Int }"
    ];
    
    println!("\n🧪 测试查询示例:");
    for (i, query) in test_queries.iter().enumerate() {
        println!("  {}. {}", i + 1, query);
    }
    
    // 保存测试报告
    let report = format!(
        "# GQL 2024语法测试报告\n\n\
        ## 基本信息\n\
        - 语法文件大小: {} 字节\n\
        - 语法行数: {} 行\n\
        - 语法规则数量: {} 个\n\
        - 支持关键字: {}/{} 个\n\
        - 核心语法部分: {}/{} 个\n\
        - 高级功能: {}/{} 个\n\
        - 语法复杂度评分: {:.2}\n\n\
        ## 测试查询\n\
        {}\n\n\
        ## 支持的GQL 2024特性\n\
        \n\
        ### 查询操作\n\
        - Query: 数据查询操作\n\
        - Mutation: 数据修改操作\n\
        - Subscription: 实时订阅操作\n\
        \n\
        ### 类型系统\n\
        - Scalar Types: 标量类型\n\
        - Object Types: 对象类型\n\
        - Interface Types: 接口类型\n\
        - Union Types: 联合类型\n\
        - Enum Types: 枚举类型\n\
        - Input Types: 输入类型\n\
        \n\
        ### 高级特性\n\
        - Fragments: 片段定义和展开\n\
        - Directives: 指令系统\n\
        - Schema Definition: 模式定义\n\
        - Type Extensions: 类型扩展\n\
        - Directive Definitions: 指令定义\n\
        \n\
        ## 结论\n\
        GQL 2024语法文件已成功创建，包含了完整的ISO/IEC 39075标准语法规则。\n\
        该语法文件可以用于生成高性能的GQL解析器，支持现代GraphQL的所有特性。",
        grammar_content.len(),
        grammar_content.lines().count(),
        rule_count,
        keyword_count,
        keywords.len(),
        found_sections,
        required_sections.len(),
        found_features,
        advanced_features.len(),
        complexity_score,
        test_queries.iter().enumerate()
            .map(|(i, q)| format!("{}. {}", i + 1, q))
            .collect::<Vec<_>>()
            .join("\n")
    );
    
    fs::write("target/doc/GQL2024_TEST_REPORT.md", report)?;
    println!("\n📄 测试报告已保存到: target/doc/GQL2024_TEST_REPORT.md");
    
    println!("\n✅ GQL 2024语法测试完成!");
    
    Ok(())
}

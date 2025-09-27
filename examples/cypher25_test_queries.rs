// CYPHER 25 新特性测试查询
// 基于用户提供的测试语句

use std::fs;
use std::path::Path;

// 测试查询集合
pub struct Cypher25TestQueries {
    pub queries: Vec<TestQuery>,
}

pub struct TestQuery {
    pub name: String,
    pub description: String,
    pub query: String,
    pub expected_features: Vec<String>,
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
    
    pub fn get_query_by_name(&self, name: &str) -> Option<&TestQuery> {
        self.queries.iter().find(|q| q.name == name)
    }
    
    pub fn get_queries_by_feature(&self, feature: &str) -> Vec<&TestQuery> {
        self.queries.iter()
            .filter(|q| q.expected_features.iter().any(|f| f.contains(feature)))
            .collect()
    }
    
    pub fn save_to_file(&self, path: &Path) -> Result<(), std::io::Error> {
        let mut content = String::new();
        content.push_str("// CYPHER 25 新特性测试查询集合\n");
        content.push_str("// 基于用户提供的测试语句\n\n");
        
        for (i, query) in self.queries.iter().enumerate() {
            content.push_str(&format!("// 测试 {}: {}\n", i + 1, query.name));
            content.push_str(&format!("// 描述: {}\n", query.description));
            content.push_str(&format!("// 预期特性: {}\n", query.expected_features.join(", ")));
            content.push_str(&format!("{}\n\n", query.query));
        }
        
        fs::write(path, content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cypher25_queries_creation() {
        let queries = Cypher25TestQueries::new();
        assert_eq!(queries.queries.len(), 15);
    }
    
    #[test]
    fn test_get_query_by_name() {
        let queries = Cypher25TestQueries::new();
        let query = queries.get_query_by_name("filter_clause_test");
        assert!(query.is_some());
        assert_eq!(query.unwrap().name, "filter_clause_test");
    }
    
    #[test]
    fn test_get_queries_by_feature() {
        let queries = Cypher25TestQueries::new();
        let filter_queries = queries.get_queries_by_feature("FILTER");
        assert!(!filter_queries.is_empty());
    }
    
    #[test]
    fn test_save_to_file() {
        let queries = Cypher25TestQueries::new();
        let path = Path::new("target/cypher25_test_queries.txt");
        assert!(queries.save_to_file(path).is_ok());
    }
}

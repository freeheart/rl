//! 知识图谱CLI功能模块
//! 
//! 提供知识图谱生成、分析和可视化功能

use std::path::PathBuf;
use std::fs;
use petgraph::visit::EdgeRef;
use petgraph::adj::IndexType;
use serde::{Deserialize, Serialize};
use crate::knowledge_graph::*;
use crate::error::Error;

/// 知识图谱生成器
pub struct KnowledgeGraphGenerator {
    kg_parser: KnowledgeGraphEnhancedParser,
    exporter: GraphExporter,
    visualizer: GraphVisualizer,
}

/// 图谱导出器
pub struct GraphExporter;

/// 图谱可视化器
pub struct GraphVisualizer;

/// 图谱格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GraphFormat {
    Json,
    Graphml,
    Dot,
    Csv,
    Neo4j,
    Rdf,
}

/// 图谱分析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphAnalysisResult {
    pub node_count: usize,
    pub edge_count: usize,
    pub complexity_score: f64,
    pub connectivity_score: f64,
    pub semantic_richness: f64,
    pub recommendations: Vec<GraphRecommendation>,
}

/// 图谱建议
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphRecommendation {
    pub category: RecommendationCategory,
    pub title: String,
    pub description: String,
    pub impact: ImpactLevel,
    pub implementation: String,
}

/// 建议类别
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationCategory {
    Structure,
    Semantics,
    Performance,
    Maintainability,
    Extensibility,
}

/// 影响级别
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl KnowledgeGraphGenerator {
    /// 创建知识图谱生成器
    pub fn new() -> Self {
        Self {
            kg_parser: KnowledgeGraphEnhancedParser::new(),
            exporter: GraphExporter,
            visualizer: GraphVisualizer,
        }
    }
    
    /// 生成知识图谱
    pub fn generate_knowledge_graph(
        &mut self,
        grammar: &str,
        format: GraphFormat,
    ) -> Result<KnowledgeGraph, Error> {
        // 使用知识图谱增强解析器解析语法
        let analysis = self.kg_parser.parse_with_knowledge_graph(grammar, grammar)
            .map_err(|_e| Error::ParseError { message: "Invalid grammar".to_string(), position: crate::error::Position { line: 1, column: 1, offset: 0 } })?;
        
        // 构建知识图谱
        let mut kg = KnowledgeGraph::new();
        
        // 添加语法规则节点
        self.add_grammar_rules(&mut kg, grammar)?;
        
        // 添加语义关系
        self.add_semantic_relationships(&mut kg, &analysis)?;
        
        // 添加上下文信息
        self.add_context_information(&mut kg, &analysis)?;
        
        Ok(kg)
    }
    
    /// 导出知识图谱
    pub fn export_graph(
        &self,
        kg: &KnowledgeGraph,
        format: GraphFormat,
        output_path: &PathBuf,
    ) -> Result<(), Error> {
        match format {
            GraphFormat::Json => self.export_to_json(kg, output_path),
            GraphFormat::Graphml => self.export_to_graphml(kg, output_path),
            GraphFormat::Dot => self.export_to_dot(kg, output_path),
            GraphFormat::Csv => self.export_to_csv(kg, output_path),
            GraphFormat::Neo4j => self.export_to_neo4j(kg, output_path),
            GraphFormat::Rdf => self.export_to_rdf(kg, output_path),
        }
    }
    
    /// 分析知识图谱
    pub fn analyze_graph(&self, kg: &KnowledgeGraph) -> Result<GraphAnalysisResult, Error> {
        let node_count = kg.graph.node_count();
        let edge_count = kg.graph.edge_count();
        
        // 计算复杂度分数
        let complexity_score = self.calculate_complexity_score(kg);
        
        // 计算连通性分数
        let connectivity_score = self.calculate_connectivity_score(kg);
        
        // 计算语义丰富度
        let semantic_richness = self.calculate_semantic_richness(kg);
        
        // 生成建议
        let recommendations = self.generate_recommendations(kg);
        
        Ok(GraphAnalysisResult {
            node_count,
            edge_count,
            complexity_score,
            connectivity_score,
            semantic_richness,
            recommendations,
        })
    }
    
    /// 可视化知识图谱
    pub fn visualize_graph(
        &self,
        kg: &KnowledgeGraph,
        output_path: &PathBuf,
        format: VisualizationFormat,
    ) -> Result<(), Error> {
        match format {
            VisualizationFormat::Html => self.visualize_as_html(kg, output_path),
            VisualizationFormat::Svg => self.visualize_as_svg(kg, output_path),
            VisualizationFormat::Png => self.visualize_as_png(kg, output_path),
            VisualizationFormat::Interactive => self.visualize_as_interactive(kg, output_path),
        }
    }
    
    /// 添加语法规则节点
    fn add_grammar_rules(&self, kg: &mut KnowledgeGraph, grammar: &str) -> Result<(), Error> {
        // 解析语法规则
        let rules = self.extract_grammar_rules(grammar)?;
        
        for rule in rules {
            let node = KnowledgeNode::GrammarRule {
                name: rule.name.clone(),
                pattern: rule.pattern.clone(),
                complexity: self.calculate_rule_complexity(&rule.pattern),
                usage_frequency: 0.0, // 初始值
            };
            
            let node_index = kg.add_node(node);
            kg.node_index.insert(rule.name, node_index);
        }
        
        Ok(())
    }
    
    /// 添加语义关系
    fn add_semantic_relationships(
        &self,
        kg: &mut KnowledgeGraph,
        analysis: &SemanticAnalysis,
    ) -> Result<(), Error> {
        for relationship in &analysis.relationships {
            if let (Some(&source_idx), Some(&target_idx)) = (
                kg.node_index.get(&relationship.source),
                kg.node_index.get(&relationship.target),
            ) {
                let edge = KnowledgeEdge::SemanticRelation {
                    relation_type: relationship.relation_type.clone(),
                    confidence: relationship.confidence,
                };
                
                kg.add_edge(source_idx, target_idx, edge);
            }
        }
        
        Ok(())
    }
    
    /// 添加上下文信息
    fn add_context_information(
        &self,
        kg: &mut KnowledgeGraph,
        analysis: &SemanticAnalysis,
    ) -> Result<(), Error> {
        for concept in &analysis.concepts {
            let node = KnowledgeNode::SemanticConcept {
                name: concept.name.clone(),
                description: concept.type_info.clone(),
                domain: concept.domain.clone(),
                confidence: concept.confidence,
            };
            
            let node_index = kg.add_node(node);
            kg.node_index.insert(concept.name.clone(), node_index);
        }
        
        Ok(())
    }
    
    /// 导出为JSON格式
    fn export_to_json(&self, kg: &KnowledgeGraph, output_path: &PathBuf) -> Result<(), Error> {
        // 简化的JSON导出，不直接序列化整个图谱
        let mut json_data = String::new();
        json_data.push_str("{\n");
        json_data.push_str("  \"node_count\": ");
        json_data.push_str(&kg.graph.node_count().to_string());
        json_data.push_str(",\n");
        json_data.push_str("  \"edge_count\": ");
        json_data.push_str(&kg.graph.edge_count().to_string());
        json_data.push_str(",\n");
        json_data.push_str("  \"nodes\": [\n");
        
        for (i, node) in kg.graph.node_indices().enumerate() {
            if i > 0 {
                json_data.push_str(",\n");
            }
            let node_data = &kg.graph[node];
            let node_json = match node_data {
                KnowledgeNode::GrammarRule { name, pattern, complexity, .. } => {
                    format!("    {{\"type\": \"GrammarRule\", \"name\": \"{}\", \"pattern\": \"{}\", \"complexity\": {}}}", 
                        name, pattern, complexity)
                }
                KnowledgeNode::SemanticConcept { name, description, domain, confidence } => {
                    format!("    {{\"type\": \"SemanticConcept\", \"name\": \"{}\", \"description\": \"{}\", \"domain\": \"{}\", \"confidence\": {}}}", 
                        name, description, domain, confidence)
                }
                KnowledgeNode::Context { name, scope, .. } => {
                    format!("    {{\"type\": \"Context\", \"name\": \"{}\", \"scope\": \"{}\"}}", 
                        name, scope)
                }
                KnowledgeNode::Optimization { rule, suggestion, impact, .. } => {
                    format!("    {{\"type\": \"Optimization\", \"rule\": \"{}\", \"suggestion\": \"{}\", \"impact\": {}}}", 
                        rule, suggestion, impact)
                }
            };
            json_data.push_str(&node_json);
        }
        
        json_data.push_str("\n  ]\n");
        json_data.push_str("}\n");
        
        fs::write(output_path, json_data)?;
        Ok(())
    }
    
    /// 导出为GraphML格式
    fn export_to_graphml(&self, kg: &KnowledgeGraph, output_path: &PathBuf) -> Result<(), Error> {
        // TODO: 实现GraphML导出
        todo!("实现GraphML导出")
    }
    
    /// 导出为DOT格式
    fn export_to_dot(&self, kg: &KnowledgeGraph, output_path: &PathBuf) -> Result<(), Error> {
        let mut dot_content = String::new();
        dot_content.push_str("digraph KnowledgeGraph {\n");
        dot_content.push_str("  rankdir=LR;\n");
        dot_content.push_str("  node [shape=box, style=filled];\n");
        
        // 添加节点
        for (i, node) in kg.graph.node_indices().enumerate() {
            let node_data = &kg.graph[node];
            let label = match node_data {
                KnowledgeNode::GrammarRule { name, .. } => format!("Rule: {}", name),
                KnowledgeNode::SemanticConcept { name, .. } => format!("Concept: {}", name),
                KnowledgeNode::Context { name, .. } => format!("Context: {}", name),
                KnowledgeNode::Optimization { rule, .. } => format!("Optimization: {}", rule),
            };
            
            dot_content.push_str(&format!("  {} [label=\"{}\"];\n", i.index(), label));
        }
        
        // 添加边
        for edge in kg.graph.edge_references() {
            let source = edge.source().index();
            let target = edge.target().index();
            let edge_label = match &edge.weight() {
                KnowledgeEdge::GrammarDependency { dependency_type, .. } => {
                    format!("{:?}", dependency_type)
                }
                KnowledgeEdge::SemanticRelation { relation_type, .. } => {
                    format!("{:?}", relation_type)
                }
                KnowledgeEdge::ContextRelation { relation_type, .. } => {
                    format!("{:?}", relation_type)
                }
                KnowledgeEdge::OptimizationRelation { optimization_type, .. } => {
                    format!("{:?}", optimization_type)
                }
            };
            
            dot_content.push_str(&format!("  {} -> {} [label=\"{}\"];\n", source, target, edge_label));
        }
        
        dot_content.push_str("}\n");
        
        fs::write(output_path, dot_content)?;
        Ok(())
    }
    
    /// 导出为CSV格式
    fn export_to_csv(&self, kg: &KnowledgeGraph, output_path: &PathBuf) -> Result<(), Error> {
        let mut csv_content = String::new();
        csv_content.push_str("Type,Name,Description,Domain,Confidence\n");
        
        for node in kg.graph.node_indices() {
            let node_data = &kg.graph[node];
            match node_data {
                KnowledgeNode::GrammarRule { name, pattern, complexity, .. } => {
                    csv_content.push_str(&format!("GrammarRule,{},{},{},{}\n", 
                        name, pattern, "grammar", complexity));
                }
                KnowledgeNode::SemanticConcept { name, description, domain, confidence } => {
                    csv_content.push_str(&format!("SemanticConcept,{},{},{},{}\n", 
                        name, description, domain, confidence));
                }
                KnowledgeNode::Context { name, scope, .. } => {
                    csv_content.push_str(&format!("Context,{},{},{},{}\n", 
                        name, scope, "context", 1.0));
                }
                KnowledgeNode::Optimization { rule, suggestion, impact, .. } => {
                    csv_content.push_str(&format!("Optimization,{},{},{},{}\n", 
                        rule, suggestion, "optimization", impact));
                }
            }
        }
        
        fs::write(output_path, csv_content)?;
        Ok(())
    }
    
    /// 导出为Neo4j格式
    fn export_to_neo4j(&self, kg: &KnowledgeGraph, output_path: &PathBuf) -> Result<(), Error> {
        // TODO: 实现Neo4j导出
        todo!("实现Neo4j导出")
    }
    
    /// 导出为RDF格式
    fn export_to_rdf(&self, kg: &KnowledgeGraph, output_path: &PathBuf) -> Result<(), Error> {
        // TODO: 实现RDF导出
        todo!("实现RDF导出")
    }
    
    /// 计算复杂度分数
    fn calculate_complexity_score(&self, kg: &KnowledgeGraph) -> f64 {
        let node_count = kg.graph.node_count() as f64;
        let edge_count = kg.graph.edge_count() as f64;
        
        // 基于节点和边的数量计算复杂度
        let density = edge_count / (node_count * (node_count - 1.0));
        let complexity = (node_count * 0.1 + edge_count * 0.05 + density * 10.0).min(10.0);
        
        complexity
    }
    
    /// 计算连通性分数
    fn calculate_connectivity_score(&self, kg: &KnowledgeGraph) -> f64 {
        // TODO: 实现连通性计算
        0.8 // 占位符
    }
    
    /// 计算语义丰富度
    fn calculate_semantic_richness(&self, kg: &KnowledgeGraph) -> f64 {
        let mut semantic_nodes = 0;
        let mut total_nodes = kg.graph.node_count();
        
        for node in kg.graph.node_indices() {
            if matches!(kg.graph[node], KnowledgeNode::SemanticConcept { .. }) {
                semantic_nodes += 1;
            }
        }
        
        if total_nodes == 0 {
            0.0
        } else {
            semantic_nodes as f64 / total_nodes as f64
        }
    }
    
    /// 生成建议
    fn generate_recommendations(&self, kg: &KnowledgeGraph) -> Vec<GraphRecommendation> {
        let mut recommendations = Vec::new();
        
        // 基于复杂度生成建议
        let complexity = self.calculate_complexity_score(kg);
        if complexity > 7.0 {
            recommendations.push(GraphRecommendation {
                category: RecommendationCategory::Structure,
                title: "简化图谱结构".to_string(),
                description: "图谱过于复杂，建议简化结构".to_string(),
                impact: ImpactLevel::High,
                implementation: "合并相关节点，减少不必要的连接".to_string(),
            });
        }
        
        // 基于连通性生成建议
        let connectivity = self.calculate_connectivity_score(kg);
        if connectivity < 0.3 {
            recommendations.push(GraphRecommendation {
                category: RecommendationCategory::Structure,
                title: "增强图谱连通性".to_string(),
                description: "图谱连通性较低，建议增加连接".to_string(),
                impact: ImpactLevel::Medium,
                implementation: "添加更多语义关系".to_string(),
            });
        }
        
        // 基于语义丰富度生成建议
        let semantic_richness = self.calculate_semantic_richness(kg);
        if semantic_richness < 0.5 {
            recommendations.push(GraphRecommendation {
                category: RecommendationCategory::Semantics,
                title: "增强语义信息".to_string(),
                description: "语义信息不足，建议增加语义节点".to_string(),
                impact: ImpactLevel::Medium,
                implementation: "添加更多语义概念和关系".to_string(),
            });
        }
        
        recommendations
    }
    
    /// 提取语法规则
    fn extract_grammar_rules(&self, grammar: &str) -> Result<Vec<GrammarRule>, Error> {
        // TODO: 实现语法规则提取
        Ok(vec![])
    }
    
    /// 计算规则复杂度
    fn calculate_rule_complexity(&self, pattern: &str) -> f64 {
        let length_factor = pattern.len() as f64 / 100.0;
        let nesting_factor = pattern.matches('(').count() as f64 / 10.0;
        let alternation_factor = pattern.matches('|').count() as f64 / 5.0;
        
        (length_factor + nesting_factor + alternation_factor).min(1.0)
    }
    
    /// 可视化为HTML
    fn visualize_as_html(&self, kg: &KnowledgeGraph, output_path: &PathBuf) -> Result<(), Error> {
        // TODO: 实现HTML可视化
        todo!("实现HTML可视化")
    }
    
    /// 可视化为SVG
    fn visualize_as_svg(&self, kg: &KnowledgeGraph, output_path: &PathBuf) -> Result<(), Error> {
        // TODO: 实现SVG可视化
        todo!("实现SVG可视化")
    }
    
    /// 可视化为PNG
    fn visualize_as_png(&self, kg: &KnowledgeGraph, output_path: &PathBuf) -> Result<(), Error> {
        // TODO: 实现PNG可视化
        todo!("实现PNG可视化")
    }
    
    /// 交互式可视化
    fn visualize_as_interactive(&self, kg: &KnowledgeGraph, output_path: &PathBuf) -> Result<(), Error> {
        // TODO: 实现交互式可视化
        todo!("实现交互式可视化")
    }
}

/// 可视化格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VisualizationFormat {
    Html,
    Svg,
    Png,
    Interactive,
}

/// 语法规则
#[derive(Debug, Clone)]
pub struct GrammarRule {
    pub name: String,
    pub pattern: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_kg_generator_creation() {
        let generator = KnowledgeGraphGenerator::new();
        // 测试生成器创建
    }
    
    #[test]
    fn test_complexity_calculation() {
        let generator = KnowledgeGraphGenerator::new();
        let kg = KnowledgeGraph::new();
        let complexity = generator.calculate_complexity_score(&kg);
        assert!(complexity >= 0.0 && complexity <= 10.0);
    }
}

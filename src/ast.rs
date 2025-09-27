//! AST模块

use std::collections::HashMap;

use crate::error::Position;

/// AST主结构
#[derive(Debug, Clone)]
pub struct AST {
    pub root: ASTNode,
    pub metadata: ASTMetadata,
}

/// AST节点
#[derive(Debug, Clone)]
pub enum ASTNode {
    // 基本节点
    Literal(LiteralNode),
    Identifier(IdentifierNode),
    BinaryOp(BinaryOpNode),
    UnaryOp(UnaryOpNode),
    TernaryOp(TernaryOpNode),
    
    // 复合节点
    Sequence(SequenceNode),
    Choice(ChoiceNode),
    Optional(OptionalNode),
    Repetition(RepetitionNode),
    
    // 高级节点
    Capture(CaptureNode),
    Reference(ReferenceNode),
    Action(ActionNode),
    
    // 声明节点
    Rule(RuleNode),
    Grammar(GrammarNode),
}

/// 字面量节点
#[derive(Debug, Clone)]
pub struct LiteralNode {
    pub value: String,
    pub position: Position,
}

/// 标识符节点
#[derive(Debug, Clone)]
pub struct IdentifierNode {
    pub name: String,
    pub position: Position,
}

/// 二元操作节点
#[derive(Debug, Clone)]
pub struct BinaryOpNode {
    pub operator: String,
    pub left: Box<ASTNode>,
    pub right: Box<ASTNode>,
    pub position: Position,
}

/// 一元操作节点
#[derive(Debug, Clone)]
pub struct UnaryOpNode {
    pub operator: String,
    pub operand: Box<ASTNode>,
    pub position: Position,
}

/// 三元操作节点
#[derive(Debug, Clone)]
pub struct TernaryOpNode {
    pub condition: Box<ASTNode>,
    pub true_value: Box<ASTNode>,
    pub false_value: Box<ASTNode>,
    pub position: Position,
}

/// 序列节点
#[derive(Debug, Clone)]
pub struct SequenceNode {
    pub children: Vec<ASTNode>,
    pub position: Position,
}

/// 选择节点
#[derive(Debug, Clone)]
pub struct ChoiceNode {
    pub alternatives: Vec<ASTNode>,
    pub position: Position,
}

/// 可选节点
#[derive(Debug, Clone)]
pub struct OptionalNode {
    pub child: Box<ASTNode>,
    pub position: Position,
}

/// 重复节点
#[derive(Debug, Clone)]
pub struct RepetitionNode {
    pub min: usize,
    pub max: Option<usize>,
    pub child: Box<ASTNode>,
    pub position: Position,
}

/// 捕获节点
#[derive(Debug, Clone)]
pub struct CaptureNode {
    pub name: String,
    pub child: Box<ASTNode>,
    pub position: Position,
}

/// 引用节点
#[derive(Debug, Clone)]
pub struct ReferenceNode {
    pub name: String,
    pub position: Position,
}

/// 动作节点
#[derive(Debug, Clone)]
pub struct ActionNode {
    pub code: String,
    pub language: String,
    pub child: Option<Box<ASTNode>>,
    pub position: Position,
}

/// 规则节点
#[derive(Debug, Clone)]
pub struct RuleNode {
    pub name: String,
    pub pattern: Box<ASTNode>,
    pub action: Option<ActionNode>,
    pub position: Position,
}

/// 语法节点
#[derive(Debug, Clone)]
pub struct GrammarNode {
    pub name: String,
    pub rules: Vec<RuleNode>,
    pub position: Position,
}

/// AST元数据
#[derive(Debug, Clone)]
pub struct ASTMetadata {
    pub source_file: Option<String>,
    pub generation_time: std::time::SystemTime,
    pub node_count: usize,
    pub max_depth: usize,
}

impl ASTMetadata {
    pub fn new() -> Self {
        Self {
            source_file: None,
            generation_time: std::time::SystemTime::now(),
            node_count: 0,
            max_depth: 0,
        }
    }
}

/// AST构建器
pub struct ASTBuilder {
    nodes: Vec<ASTNode>,
    current_rule: Option<String>,
    current_grammar: Option<String>,
}

impl ASTBuilder {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            current_rule: None,
            current_grammar: None,
        }
    }
    
    /// 创建字面量节点
    pub fn create_literal(&mut self, value: String, position: Position) -> ASTNode {
        ASTNode::Literal(LiteralNode { value, position })
    }
    
    /// 创建标识符节点
    pub fn create_identifier(&mut self, name: String, position: Position) -> ASTNode {
        ASTNode::Identifier(IdentifierNode { name, position })
    }
    
    /// 创建二元操作节点
    pub fn create_binary_op(&mut self, operator: String, left: ASTNode, right: ASTNode, position: Position) -> ASTNode {
        ASTNode::BinaryOp(BinaryOpNode {
            operator,
            left: Box::new(left),
            right: Box::new(right),
            position,
        })
    }
    
    /// 创建一元操作节点
    pub fn create_unary_op(&mut self, operator: String, operand: ASTNode, position: Position) -> ASTNode {
        ASTNode::UnaryOp(UnaryOpNode {
            operator,
            operand: Box::new(operand),
            position,
        })
    }
    
    /// 创建三元操作节点
    pub fn create_ternary_op(&mut self, condition: ASTNode, true_value: ASTNode, false_value: ASTNode, position: Position) -> ASTNode {
        ASTNode::TernaryOp(TernaryOpNode {
            condition: Box::new(condition),
            true_value: Box::new(true_value),
            false_value: Box::new(false_value),
            position,
        })
    }
    
    /// 创建序列节点
    pub fn create_sequence(&mut self, children: Vec<ASTNode>, position: Position) -> ASTNode {
        ASTNode::Sequence(SequenceNode { children, position })
    }
    
    /// 创建选择节点
    pub fn create_choice(&mut self, alternatives: Vec<ASTNode>, position: Position) -> ASTNode {
        ASTNode::Choice(ChoiceNode { alternatives, position })
    }
    
    /// 创建可选节点
    pub fn create_optional(&mut self, child: ASTNode, position: Position) -> ASTNode {
        ASTNode::Optional(OptionalNode {
            child: Box::new(child),
            position,
        })
    }
    
    /// 创建重复节点
    pub fn create_repetition(&mut self, min: usize, max: Option<usize>, child: ASTNode, position: Position) -> ASTNode {
        ASTNode::Repetition(RepetitionNode {
            min,
            max,
            child: Box::new(child),
            position,
        })
    }
    
    /// 创建捕获节点
    pub fn create_capture(&mut self, name: String, child: ASTNode, position: Position) -> ASTNode {
        ASTNode::Capture(CaptureNode {
            name,
            child: Box::new(child),
            position,
        })
    }
    
    /// 创建引用节点
    pub fn create_reference(&mut self, name: String, position: Position) -> ASTNode {
        ASTNode::Reference(ReferenceNode { name, position })
    }
    
    /// 创建动作节点
    pub fn create_action(&mut self, code: String, language: String, child: Option<ASTNode>, position: Position) -> ASTNode {
        ASTNode::Action(ActionNode {
            code,
            language,
            child: child.map(Box::new),
            position,
        })
    }
    
    /// 创建规则节点
    pub fn create_rule(&mut self, name: String, pattern: ASTNode, action: Option<ActionNode>, position: Position) -> ASTNode {
        ASTNode::Rule(RuleNode {
            name,
            pattern: Box::new(pattern),
            action,
            position,
        })
    }
    
    /// 创建语法节点
    pub fn create_grammar(&mut self, name: String, rules: Vec<RuleNode>, position: Position) -> ASTNode {
        ASTNode::Grammar(GrammarNode { name, rules, position })
    }
    
    /// 构建最终AST
    pub fn build(self) -> AST {
        let root = self.nodes.into_iter().next().unwrap_or(ASTNode::Literal(LiteralNode {
            value: "".to_string(),
            position: Position::new(0, 0, 0),
        }));
        
        let metadata = ASTMetadata::new();
        
        AST { root, metadata }
    }
}

/// AST访问者
pub trait ASTVisitor {
    type Result;
    
    fn visit_literal(&mut self, node: &LiteralNode) -> Self::Result;
    fn visit_identifier(&mut self, node: &IdentifierNode) -> Self::Result;
    fn visit_binary_op(&mut self, node: &BinaryOpNode) -> Self::Result;
    fn visit_unary_op(&mut self, node: &UnaryOpNode) -> Self::Result;
    fn visit_ternary_op(&mut self, node: &TernaryOpNode) -> Self::Result;
    fn visit_sequence(&mut self, node: &SequenceNode) -> Self::Result;
    fn visit_choice(&mut self, node: &ChoiceNode) -> Self::Result;
    fn visit_optional(&mut self, node: &OptionalNode) -> Self::Result;
    fn visit_repetition(&mut self, node: &RepetitionNode) -> Self::Result;
    fn visit_capture(&mut self, node: &CaptureNode) -> Self::Result;
    fn visit_reference(&mut self, node: &ReferenceNode) -> Self::Result;
    fn visit_action(&mut self, node: &ActionNode) -> Self::Result;
    fn visit_rule(&mut self, node: &RuleNode) -> Self::Result;
    fn visit_grammar(&mut self, node: &GrammarNode) -> Self::Result;
}

/// AST转换器
pub trait ASTTransformer {
    fn transform_literal(&self, node: &mut LiteralNode) -> Result<(), crate::error::Error>;
    fn transform_identifier(&self, node: &mut IdentifierNode) -> Result<(), crate::error::Error>;
    fn transform_binary_op(&self, node: &mut BinaryOpNode) -> Result<(), crate::error::Error>;
    fn transform_unary_op(&self, node: &mut UnaryOpNode) -> Result<(), crate::error::Error>;
    fn transform_ternary_op(&self, node: &mut TernaryOpNode) -> Result<(), crate::error::Error>;
    fn transform_sequence(&self, node: &mut SequenceNode) -> Result<(), crate::error::Error>;
    fn transform_choice(&self, node: &mut ChoiceNode) -> Result<(), crate::error::Error>;
    fn transform_optional(&self, node: &mut OptionalNode) -> Result<(), crate::error::Error>;
    fn transform_repetition(&self, node: &mut RepetitionNode) -> Result<(), crate::error::Error>;
    fn transform_capture(&self, node: &mut CaptureNode) -> Result<(), crate::error::Error>;
    fn transform_reference(&self, node: &mut ReferenceNode) -> Result<(), crate::error::Error>;
    fn transform_action(&self, node: &mut ActionNode) -> Result<(), crate::error::Error>;
    fn transform_rule(&self, node: &mut RuleNode) -> Result<(), crate::error::Error>;
    fn transform_grammar(&self, node: &mut GrammarNode) -> Result<(), crate::error::Error>;
}

/// AST序列化器
pub struct ASTSerializer {
    formatters: HashMap<String, Box<dyn ASTFormatter>>,
}

impl ASTSerializer {
    pub fn new() -> Self {
        Self {
            formatters: HashMap::new(),
        }
    }
    
    pub fn serialize(&self, ast: &AST, format: SerializationFormat) -> Result<String, crate::error::Error> {
        let formatter = self.formatters.get(&format.to_string())
            .ok_or_else(|| crate::error::Error::TemplateNotFound { template: format.to_string() })?;
        
        formatter.format(ast)
    }
    
    pub fn add_formatter(&mut self, format: SerializationFormat, formatter: Box<dyn ASTFormatter>) {
        self.formatters.insert(format.to_string(), formatter);
    }
}

/// 序列化格式
#[derive(Debug, Clone, PartialEq)]
pub enum SerializationFormat {
    JSON,
    XML,
    YAML,
    SExpression,
    Custom(String),
}

impl ToString for SerializationFormat {
    fn to_string(&self) -> String {
        match self {
            SerializationFormat::JSON => "json".to_string(),
            SerializationFormat::XML => "xml".to_string(),
            SerializationFormat::YAML => "yaml".to_string(),
            SerializationFormat::SExpression => "sexp".to_string(),
            SerializationFormat::Custom(name) => name.clone(),
        }
    }
}

/// AST格式化器
pub trait ASTFormatter {
    fn format(&self, ast: &AST) -> Result<String, crate::error::Error>;
}

/// JSON格式化器
pub struct JSONFormatter;

impl ASTFormatter for JSONFormatter {
    fn format(&self, ast: &AST) -> Result<String, crate::error::Error> {
        // 简化的JSON格式化，不依赖序列化
        let json = format!("{{\"root\": \"{:?}\", \"metadata\": \"{:?}\"}}", ast.root, ast.metadata);
        Ok(json)
    }
}

/// S表达式格式化器
pub struct SExpressionFormatter;

impl ASTFormatter for SExpressionFormatter {
    fn format(&self, ast: &AST) -> Result<String, crate::error::Error> {
        Ok(format!("(ast {:?})", ast.root))
    }
}

impl AST {
    /// 获取根节点
    pub fn get_root(&self) -> &ASTNode {
        &self.root
    }
    
    /// 获取元数据
    pub fn get_metadata(&self) -> &ASTMetadata {
        &self.metadata
    }
    
    /// 查找节点
    pub fn find_nodes<F>(&self, predicate: F) -> Vec<&ASTNode>
    where F: Fn(&ASTNode) -> bool {
        let mut result = Vec::new();
        self.collect_nodes(&self.root, &predicate, &mut result);
        result
    }
    
    fn collect_nodes<'a, F>(&self, node: &'a ASTNode, predicate: &F, result: &mut Vec<&'a ASTNode>)
    where F: Fn(&ASTNode) -> bool {
        if predicate(node) {
            result.push(node);
        }
        
        match node {
            ASTNode::BinaryOp(binary_op) => {
                self.collect_nodes(&binary_op.left, predicate, result);
                self.collect_nodes(&binary_op.right, predicate, result);
            }
            ASTNode::UnaryOp(unary_op) => {
                self.collect_nodes(&unary_op.operand, predicate, result);
            }
            ASTNode::TernaryOp(ternary_op) => {
                self.collect_nodes(&ternary_op.condition, predicate, result);
                self.collect_nodes(&ternary_op.true_value, predicate, result);
                self.collect_nodes(&ternary_op.false_value, predicate, result);
            }
            ASTNode::Sequence(sequence) => {
                for child in &sequence.children {
                    self.collect_nodes(child, predicate, result);
                }
            }
            ASTNode::Choice(choice) => {
                for alternative in &choice.alternatives {
                    self.collect_nodes(alternative, predicate, result);
                }
            }
            ASTNode::Optional(optional) => {
                self.collect_nodes(&optional.child, predicate, result);
            }
            ASTNode::Repetition(repetition) => {
                self.collect_nodes(&repetition.child, predicate, result);
            }
            ASTNode::Capture(capture) => {
                self.collect_nodes(&capture.child, predicate, result);
            }
            ASTNode::Action(action) => {
                if let Some(child) = &action.child {
                    self.collect_nodes(child, predicate, result);
                }
            }
            ASTNode::Rule(rule) => {
                self.collect_nodes(&rule.pattern, predicate, result);
            }
            ASTNode::Grammar(grammar) => {
                for rule in &grammar.rules {
                    self.collect_nodes(&rule.pattern, predicate, result);
                }
            }
            _ => {}
        }
    }
    
    /// 变换AST
    pub fn transform(&mut self, transformer: &dyn ASTTransformer) -> Result<(), crate::error::Error> {
        let mut root = std::mem::replace(&mut self.root, ASTNode::Literal(LiteralNode {
            value: "".to_string(),
            position: crate::error::Position::new(0, 0, 0),
        }));
        self.transform_node(&mut root, transformer)?;
        self.root = root;
        Ok(())
    }
    
    fn transform_node(&self, node: &mut ASTNode, transformer: &dyn ASTTransformer) -> Result<(), crate::error::Error> {
        match node {
            ASTNode::Literal(literal) => transformer.transform_literal(literal)?,
            ASTNode::Identifier(identifier) => transformer.transform_identifier(identifier)?,
            ASTNode::BinaryOp(binary_op) => {
                transformer.transform_binary_op(binary_op)?;
                self.transform_node(&mut binary_op.left, transformer)?;
                self.transform_node(&mut binary_op.right, transformer)?;
            }
            ASTNode::UnaryOp(unary_op) => {
                transformer.transform_unary_op(unary_op)?;
                self.transform_node(&mut unary_op.operand, transformer)?;
            }
            ASTNode::TernaryOp(ternary_op) => {
                transformer.transform_ternary_op(ternary_op)?;
                self.transform_node(&mut ternary_op.condition, transformer)?;
                self.transform_node(&mut ternary_op.true_value, transformer)?;
                self.transform_node(&mut ternary_op.false_value, transformer)?;
            }
            ASTNode::Sequence(sequence) => {
                transformer.transform_sequence(sequence)?;
                for child in &mut sequence.children {
                    self.transform_node(child, transformer)?;
                }
            }
            ASTNode::Choice(choice) => {
                transformer.transform_choice(choice)?;
                for alternative in &mut choice.alternatives {
                    self.transform_node(alternative, transformer)?;
                }
            }
            ASTNode::Optional(optional) => {
                transformer.transform_optional(optional)?;
                self.transform_node(&mut optional.child, transformer)?;
            }
            ASTNode::Repetition(repetition) => {
                transformer.transform_repetition(repetition)?;
                self.transform_node(&mut repetition.child, transformer)?;
            }
            ASTNode::Capture(capture) => {
                transformer.transform_capture(capture)?;
                self.transform_node(&mut capture.child, transformer)?;
            }
            ASTNode::Reference(reference) => transformer.transform_reference(reference)?,
            ASTNode::Action(action) => {
                transformer.transform_action(action)?;
                if let Some(child) = &mut action.child {
                    self.transform_node(child, transformer)?;
                }
            }
            ASTNode::Rule(rule) => {
                transformer.transform_rule(rule)?;
                self.transform_node(&mut rule.pattern, transformer)?;
            }
            ASTNode::Grammar(grammar) => {
                transformer.transform_grammar(grammar)?;
                for rule in &mut grammar.rules {
                    self.transform_node(&mut rule.pattern, transformer)?;
                }
            }
        }
        
        Ok(())
    }
}

impl Default for ASTBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ASTSerializer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ast_builder() {
        let mut builder = ASTBuilder::new();
        let position = Position::new(1, 1, 0);
        
        let literal = builder.create_literal("hello".to_string(), position.clone());
        let identifier = builder.create_identifier("world".to_string(), position.clone());
        let _binary_op = builder.create_binary_op("+".to_string(), literal, identifier, position);
        
        // Add the binary_op to the builder
        builder.nodes.push(_binary_op.clone());
        
        let ast = builder.build();
        
        match ast.root {
            ASTNode::BinaryOp(binary_op) => {
                assert_eq!(binary_op.operator, "+");
            }
            _ => panic!("Expected BinaryOp node"),
        }
    }
    
    #[test]
    fn test_ast_serialization() {
        let mut builder = ASTBuilder::new();
        let position = Position::new(1, 1, 0);
        
        let literal = builder.create_literal("test".to_string(), position);
        builder.nodes.push(literal);
        let ast = builder.build();
        
        let mut serializer = ASTSerializer::new();
        serializer.add_formatter(SerializationFormat::JSON, Box::new(JSONFormatter));
        let json = serializer.serialize(&ast, SerializationFormat::JSON).unwrap();
        
        assert!(json.contains("test"));
    }
    
    #[test]
    fn test_ast_find_nodes() {
        let mut builder = ASTBuilder::new();
        let position = Position::new(1, 1, 0);
        
        let literal1 = builder.create_literal("hello".to_string(), position.clone());
        let literal2 = builder.create_literal("world".to_string(), position.clone());
        let _binary_op = builder.create_binary_op("+".to_string(), literal1, literal2, position);
        
        // Add the binary_op to the builder
        builder.nodes.push(_binary_op.clone());
        
        let ast = builder.build();
        
        let literals = ast.find_nodes(|node| matches!(node, ASTNode::Literal(_)));
        assert_eq!(literals.len(), 2);
    }
    
    #[test]
    fn test_ternary_operator() {
        let mut builder = ASTBuilder::new();
        let position = Position::new(1, 1, 0);
        
        let condition = builder.create_literal("true".to_string(), position.clone());
        let true_value = builder.create_literal("yes".to_string(), position.clone());
        let false_value = builder.create_literal("no".to_string(), position.clone());
        let ternary_op = builder.create_ternary_op(condition, true_value, false_value, position);
        
        // Add the ternary_op to the builder
        builder.nodes.push(ternary_op.clone());
        
        let ast = builder.build();
        
        match ast.root {
            ASTNode::TernaryOp(ternary) => {
                assert!(matches!(*ternary.condition, ASTNode::Literal(_)));
                assert!(matches!(*ternary.true_value, ASTNode::Literal(_)));
                assert!(matches!(*ternary.false_value, ASTNode::Literal(_)));
            }
            _ => panic!("Expected TernaryOp node"),
        }
    }
}

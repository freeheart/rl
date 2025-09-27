//! 解析器模块

use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::grammar_parser::GrammarParser as RLGrammarParser;

/// 语法定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grammar {
    pub name: String,
    pub rules: Vec<Rule>,
    pub options: GrammarOptions,
}

/// 语法规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub name: String,
    pub pattern: Pattern,
    pub action: Option<Action>,
    pub attributes: RuleAttributes,
}

/// 模式定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Pattern {
    // 基本模式
    Literal(String),
    Identifier(String),
    CharacterClass(CharacterClass),
    
    // 组合模式
    Sequence(Vec<Pattern>),
    Choice(Vec<Pattern>),
    Optional(Box<Pattern>),
    ZeroOrMore(Box<Pattern>),
    OneOrMore(Box<Pattern>),
    
    // 高级模式
    Lookahead(Box<Pattern>),
    NegativeLookahead(Box<Pattern>),
    Capture(Box<Pattern>),
    Reference(String),
}

/// 字符类
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterClass {
    pub ranges: Vec<CharRange>,
    pub negated: bool,
}

/// 字符范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharRange {
    pub start: char,
    pub end: char,
}

/// 动作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub code: String,
    pub language: String,
}

/// 规则属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleAttributes {
    pub priority: Option<i32>,
    pub associativity: Option<Associativity>,
    pub visibility: Option<Visibility>,
}

/// 结合性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Associativity {
    Left,
    Right,
    NonAssociative,
}

/// 可见性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Private,
    Internal,
}

/// 语法选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarOptions {
    pub case_sensitive: bool,
    pub whitespace_mode: WhitespaceMode,
    pub error_recovery: bool,
    pub optimization_level: OptimizationLevel,
}

/// 空白字符模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WhitespaceMode {
    Skip,
    Preserve,
    Custom(String),
}

/// 优化级别
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationLevel {
    None,
    Basic,
    Aggressive,
    Maximum,
}

/// 解析算法
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ParserAlgorithm {
    LL1,
    LALR1,
    GLR,
    Hybrid,
}

/// 生成的解析器
#[derive(Debug, Clone)]
pub struct GeneratedParser {
    pub grammar: Grammar,
    pub parse_table: ParseTable,
    pub algorithm: ParserAlgorithm,
    pub code: String,
    pub performance_metrics: PerformanceMetrics,
}

/// 解析表
#[derive(Debug, Clone)]
pub struct ParseTable {
    pub actions: HashMap<(String, Token), ParseAction>,
    pub goto_table: HashMap<(String, String), String>,
}

/// 解析动作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParseAction {
    Shift(String),
    Reduce(Rule),
    Accept,
    Error,
}

/// Token
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Token {
    // 基本类型
    Literal(String),
    Identifier(String),
    Number(String),
    String(String),
    
    // 操作符
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
    
    // 分隔符
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,
    Colon,
    Dot,
    
    // 特殊
    Whitespace,
    Comment,
    Newline,
    EOF,
    Epsilon,
}

/// 性能指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub parse_time: std::time::Duration,
    pub memory_usage: usize,
    pub cache_hit_rate: f64,
    pub parallel_efficiency: f64,
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            parse_time: std::time::Duration::ZERO,
            memory_usage: 0,
            cache_hit_rate: 0.0,
            parallel_efficiency: 0.0,
        }
    }
}

/// 解析器生成器
pub struct ParserGenerator {
    algorithm_selector: AlgorithmSelector,
    table_builder: ParseTableBuilder,
    code_generator: ParserCodeGenerator,
}

impl ParserGenerator {
    pub fn new() -> Self {
        Self {
            algorithm_selector: AlgorithmSelector::new(),
            table_builder: ParseTableBuilder::new(),
            code_generator: ParserCodeGenerator::new(),
        }
    }
    
    /// 解析.rl格式的语法文件
    pub fn parse_rl_grammar(&self, input: &str) -> Result<Grammar, Error> {
        let mut parser = RLGrammarParser::new(input.to_string());
        let rl_grammar = parser.parse()?;
        
        // 转换为内部Grammar格式
        let mut grammar = Grammar {
            name: rl_grammar.name,
            rules: Vec::new(),
            options: GrammarOptions {
                case_sensitive: true,
                whitespace_mode: WhitespaceMode::Preserve,
                error_recovery: true,
                optimization_level: OptimizationLevel::Basic,
            },
        };

        // 转换规则
        for rl_rule in rl_grammar.rules {
            let rule = self.convert_rl_rule(rl_rule)?;
            grammar.rules.push(rule);
        }

        Ok(grammar)
    }

    /// 转换.rl规则为内部规则格式
    fn convert_rl_rule(&self, rl_rule: crate::grammar_parser::GrammarRule) -> Result<Rule, Error> {
        let pattern = self.convert_rule_definition(&rl_rule.definition)?;
        
        Ok(Rule {
            name: rl_rule.name,
            pattern,
            action: None,
            attributes: RuleAttributes {
                priority: Some(0),
                associativity: Some(Associativity::Left),
                visibility: Some(Visibility::Public),
            },
        })
    }

    /// 转换规则定义为内部模式格式
    fn convert_rule_definition(&self, definition: &crate::grammar_parser::RuleDefinition) -> Result<Pattern, Error> {
        match definition {
            crate::grammar_parser::RuleDefinition::Terminal(term) => {
                Ok(Pattern::Literal(term.clone()))
            }
            crate::grammar_parser::RuleDefinition::NonTerminal(nt) => {
                Ok(Pattern::Reference(nt.clone()))
            }
            crate::grammar_parser::RuleDefinition::Choice(choices) => {
                let mut patterns = Vec::new();
                for choice in choices {
                    patterns.push(self.convert_rule_definition(choice)?);
                }
                Ok(Pattern::Choice(patterns))
            }
            crate::grammar_parser::RuleDefinition::Sequence(sequence) => {
                let mut patterns = Vec::new();
                for element in sequence {
                    patterns.push(self.convert_rule_definition(element)?);
                }
                Ok(Pattern::Sequence(patterns))
            }
            crate::grammar_parser::RuleDefinition::Optional(opt) => {
                let inner = self.convert_rule_definition(opt)?;
                Ok(Pattern::Optional(Box::new(inner)))
            }
            crate::grammar_parser::RuleDefinition::ZeroOrMore(zom) => {
                let inner = self.convert_rule_definition(zom)?;
                Ok(Pattern::ZeroOrMore(Box::new(inner)))
            }
            crate::grammar_parser::RuleDefinition::OneOrMore(oom) => {
                let inner = self.convert_rule_definition(oom)?;
                Ok(Pattern::OneOrMore(Box::new(inner)))
            }
            crate::grammar_parser::RuleDefinition::Group(group) => {
                self.convert_rule_definition(group)
            }
            crate::grammar_parser::RuleDefinition::Comment(_) => {
                Ok(Pattern::Literal("".to_string()))
            }
        }
    }

    /// 分析语法文件
    pub fn analyze_grammar(&self, input: &str) -> Result<String, Error> {
        let mut parser = RLGrammarParser::new(input.to_string());
        let _rl_grammar = parser.parse()?;
        // 简化的语法分析
        Ok("Basic grammar analysis completed".to_string())
    }

    /// 生成解析器
    pub fn generate(&self, grammar: &Grammar) -> Result<GeneratedParser, Error> {
        // 选择最优算法
        let algorithm = self.algorithm_selector.select_algorithm(grammar);
        
        // 构建解析表
        let parse_table = self.table_builder.build_table(grammar, algorithm.clone())?;
        
        // 生成解析器代码
        let parser_code = self.code_generator.generate_parser_code(grammar, &parse_table, algorithm.clone())?;
        
        Ok(GeneratedParser {
            grammar: grammar.clone(),
            parse_table,
            algorithm,
            code: parser_code,
            performance_metrics: PerformanceMetrics::new(),
        })
    }
}

/// 算法选择器
pub struct AlgorithmSelector {
    complexity_analyzer: ComplexityAnalyzer,
}

impl AlgorithmSelector {
    pub fn new() -> Self {
        Self {
            complexity_analyzer: ComplexityAnalyzer::new(),
        }
    }
    
    pub fn select_algorithm(&self, grammar: &Grammar) -> ParserAlgorithm {
        let complexity = self.complexity_analyzer.analyze(grammar);
        
        match complexity {
            GrammarComplexity::Simple => ParserAlgorithm::LL1,
            GrammarComplexity::Medium => ParserAlgorithm::LALR1,
            GrammarComplexity::Complex => ParserAlgorithm::GLR,
            GrammarComplexity::Mixed => ParserAlgorithm::Hybrid,
        }
    }
}

/// 复杂度分析器
pub struct ComplexityAnalyzer;

impl ComplexityAnalyzer {
    pub fn new() -> Self {
        Self
    }
    
    pub fn analyze(&self, grammar: &Grammar) -> GrammarComplexity {
        let rule_count = grammar.rules.len();
        let avg_pattern_complexity = self.calculate_average_pattern_complexity(&grammar.rules);
        
        if rule_count < 10 && avg_pattern_complexity < 3.0 {
            GrammarComplexity::Simple
        } else if rule_count < 50 && avg_pattern_complexity < 5.0 {
            GrammarComplexity::Medium
        } else if rule_count < 100 && avg_pattern_complexity < 8.0 {
            GrammarComplexity::Complex
        } else {
            GrammarComplexity::Mixed
        }
    }
    
    fn calculate_average_pattern_complexity(&self, rules: &[Rule]) -> f64 {
        if rules.is_empty() {
            return 0.0;
        }
        
        let total_complexity: usize = rules.iter()
            .map(|rule| self.calculate_pattern_complexity(&rule.pattern))
            .sum();
        
        total_complexity as f64 / rules.len() as f64
    }
    
    fn calculate_pattern_complexity(&self, pattern: &Pattern) -> usize {
        match pattern {
            Pattern::Literal(_) => 1,
            Pattern::Identifier(_) => 1,
            Pattern::CharacterClass(_) => 2,
            Pattern::Sequence(patterns) => patterns.iter().map(|p| self.calculate_pattern_complexity(p)).sum(),
            Pattern::Choice(patterns) => patterns.iter().map(|p| self.calculate_pattern_complexity(p)).sum(),
            Pattern::Optional(p) => 1 + self.calculate_pattern_complexity(p),
            Pattern::ZeroOrMore(p) => 2 + self.calculate_pattern_complexity(p),
            Pattern::OneOrMore(p) => 2 + self.calculate_pattern_complexity(p),
            Pattern::Lookahead(p) => 1 + self.calculate_pattern_complexity(p),
            Pattern::NegativeLookahead(p) => 1 + self.calculate_pattern_complexity(p),
            Pattern::Capture(p) => 1 + self.calculate_pattern_complexity(p),
            Pattern::Reference(_) => 1,
        }
    }
}

/// 语法复杂度
#[derive(Debug, Clone)]
pub enum GrammarComplexity {
    Simple,
    Medium,
    Complex,
    Mixed,
}

/// 解析表构建器
pub struct ParseTableBuilder;

impl ParseTableBuilder {
    pub fn new() -> Self {
        Self
    }
    
    pub fn build_table(&self, grammar: &Grammar, algorithm: ParserAlgorithm) -> Result<ParseTable, Error> {
        match algorithm {
            ParserAlgorithm::LL1 => self.build_ll1_table(grammar),
            ParserAlgorithm::LALR1 => self.build_lalr1_table(grammar),
            ParserAlgorithm::GLR => self.build_glr_table(grammar),
            ParserAlgorithm::Hybrid => self.build_hybrid_table(grammar),
        }
    }
    
    fn build_ll1_table(&self, grammar: &Grammar) -> Result<ParseTable, Error> {
        let mut actions = HashMap::new();
        let goto_table = HashMap::new();
        
        // 简化的LL(1)表构建
        for rule in &grammar.rules {
            let first_set = self.compute_first_set(rule, grammar);
            for token in first_set {
                if token != Token::Epsilon {
                    actions.insert((rule.name.clone(), token), ParseAction::Reduce(rule.clone()));
                }
            }
        }
        
        Ok(ParseTable { actions, goto_table })
    }
    
    fn build_lalr1_table(&self, grammar: &Grammar) -> Result<ParseTable, Error> {
        // 简化的LALR(1)表构建
        self.build_ll1_table(grammar)
    }
    
    fn build_glr_table(&self, grammar: &Grammar) -> Result<ParseTable, Error> {
        // 简化的GLR表构建
        self.build_ll1_table(grammar)
    }
    
    fn build_hybrid_table(&self, grammar: &Grammar) -> Result<ParseTable, Error> {
        // 混合算法表构建
        self.build_ll1_table(grammar)
    }
    
    fn compute_first_set(&self, rule: &Rule, grammar: &Grammar) -> HashSet<Token> {
        let mut first_set = HashSet::new();
        
        match &rule.pattern {
            Pattern::Literal(s) => {
                first_set.insert(Token::Literal(s.clone()));
            }
            Pattern::Identifier(s) => {
                first_set.insert(Token::Identifier(s.clone()));
            }
            Pattern::Sequence(patterns) => {
                for pattern in patterns {
                    let pattern_first = self.compute_pattern_first(pattern, grammar);
                    let has_epsilon = pattern_first.contains(&Token::Epsilon);
                    first_set.extend(pattern_first);
                    if !has_epsilon {
                        break;
                    }
                }
            }
            Pattern::Choice(patterns) => {
                for pattern in patterns {
                    first_set.extend(self.compute_pattern_first(pattern, grammar));
                }
            }
            Pattern::Optional(_) => {
                first_set.insert(Token::Epsilon);
            }
            Pattern::ZeroOrMore(_) => {
                first_set.insert(Token::Epsilon);
            }
            Pattern::OneOrMore(_) => {
                // 不包含epsilon
            }
            _ => {
                first_set.insert(Token::Epsilon);
            }
        }
        
        first_set
    }
    
    fn compute_pattern_first(&self, pattern: &Pattern, grammar: &Grammar) -> HashSet<Token> {
        match pattern {
            Pattern::Literal(s) => {
                let mut set = HashSet::new();
                set.insert(Token::Literal(s.clone()));
                set
            }
            Pattern::Identifier(s) => {
                let mut set = HashSet::new();
                set.insert(Token::Identifier(s.clone()));
                set
            }
            Pattern::Reference(name) => {
                if let Some(rule) = grammar.rules.iter().find(|r| r.name == *name) {
                    self.compute_first_set(rule, grammar)
                } else {
                    HashSet::new()
                }
            }
            _ => {
                let mut set = HashSet::new();
                set.insert(Token::Epsilon);
                set
            }
        }
    }
}

/// 解析器代码生成器
pub struct ParserCodeGenerator;

impl ParserCodeGenerator {
    pub fn new() -> Self {
        Self
    }
    
    pub fn generate_parser_code(&self, grammar: &Grammar, parse_table: &ParseTable, algorithm: ParserAlgorithm) -> Result<String, Error> {
        let mut code = String::new();
        
        code.push_str(&format!("// Generated parser for grammar: {}\n", grammar.name));
        code.push_str(&format!("// Algorithm: {:?}\n", algorithm));
        code.push_str("\n");
        
        code.push_str("use std::collections::HashMap;\n");
        code.push_str("use std::collections::HashSet;\n");
        code.push_str("\n");
        
        // 生成Token定义
        code.push_str("#[derive(Debug, Clone, PartialEq, Eq, Hash)]\n");
        code.push_str("pub enum Token {\n");
        code.push_str("    Literal(String),\n");
        code.push_str("    Identifier(String),\n");
        code.push_str("    Number(String),\n");
        code.push_str("    String(String),\n");
        code.push_str("    EOF,\n");
        code.push_str("}\n\n");
        
        // 生成解析器结构
        code.push_str("pub struct Parser {\n");
        code.push_str("    tokens: Vec<Token>,\n");
        code.push_str("    position: usize,\n");
        code.push_str("}\n\n");
        
        // 生成解析器实现
        code.push_str("impl Parser {\n");
        code.push_str("    pub fn new(tokens: Vec<Token>) -> Self {\n");
        code.push_str("        Self { tokens, position: 0 }\n");
        code.push_str("    }\n\n");
        
        code.push_str("    pub fn parse(&mut self) -> Result<AST, ParseError> {\n");
        code.push_str("        self.parse_start()\n");
        code.push_str("    }\n\n");
        
        // 为每个规则生成解析方法
        for rule in &grammar.rules {
            code.push_str(&format!("    fn parse_{}(&mut self) -> Result<AST, ParseError> {{\n", rule.name));
            code.push_str("        // TODO: Implement parsing logic\n");
            code.push_str(&format!("        Ok(AST::Node(\"{}\".to_string()))\n", rule.name));
            code.push_str("    }\n\n");
        }
        
        code.push_str("}\n");
        
        Ok(code)
    }
}

/// 语法解析器
pub struct GrammarParser;

impl GrammarParser {
    pub fn parse(input: &str) -> Result<Grammar, Error> {
        let lines = input.lines();
        let mut grammar_name = String::new();
        let mut rules = Vec::new();
        
        // 简单的语法解析
        for line in lines {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") {
                continue;
            }
            
            if line.starts_with("grammar ") {
                grammar_name = line.replace("grammar ", "").replace(" {", "").trim().to_string();
            } else if line.contains(':') && !line.starts_with('}') {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() == 2 {
                    let rule_name = parts[0].trim().to_string();
                    let pattern_str = parts[1].trim().replace(';', "");
                    
                    let pattern = Self::parse_pattern(&pattern_str)?;
                    let rule = Rule {
                        name: rule_name,
                        pattern,
                        action: None,
                        attributes: RuleAttributes {
                            priority: None,
                            associativity: None,
                            visibility: None,
                        },
                    };
                    rules.push(rule);
                }
            }
        }
        
        Ok(Grammar {
            name: grammar_name,
            rules,
            options: GrammarOptions {
                case_sensitive: true,
                whitespace_mode: WhitespaceMode::Skip,
                error_recovery: true,
                optimization_level: OptimizationLevel::Basic,
            },
        })
    }
    
    fn parse_pattern(pattern_str: &str) -> Result<Pattern, Error> {
        let pattern_str = pattern_str.trim();
        
        if pattern_str.starts_with('\'') && pattern_str.ends_with('\'') && pattern_str.len() > 1 {
            let literal = pattern_str[1..pattern_str.len()-1].to_string();
            Ok(Pattern::Literal(literal))
        } else if pattern_str.starts_with('[') && pattern_str.ends_with(']') {
            let class_str = &pattern_str[1..pattern_str.len()-1];
            let character_class = CharacterClass {
                ranges: vec![CharRange { start: class_str.chars().next().unwrap(), end: class_str.chars().last().unwrap() }],
                negated: false,
            };
            Ok(Pattern::CharacterClass(character_class))
        } else if pattern_str.contains('|') {
            let choices: Vec<&str> = pattern_str.split('|').collect();
            let patterns: Result<Vec<Pattern>, Error> = choices.iter()
                .map(|choice| Self::parse_pattern(choice.trim()))
                .collect();
            Ok(Pattern::Choice(patterns?))
        } else if pattern_str.contains(' ') {
            let parts: Vec<&str> = pattern_str.split_whitespace().collect();
            let patterns: Result<Vec<Pattern>, Error> = parts.iter()
                .map(|part| Self::parse_pattern(part))
                .collect();
            Ok(Pattern::Sequence(patterns?))
        } else if pattern_str.ends_with('*') {
            let inner = &pattern_str[..pattern_str.len()-1];
            Ok(Pattern::ZeroOrMore(Box::new(Self::parse_pattern(inner)?)))
        } else if pattern_str.ends_with('+') {
            let inner = &pattern_str[..pattern_str.len()-1];
            Ok(Pattern::OneOrMore(Box::new(Self::parse_pattern(inner)?)))
        } else if pattern_str.ends_with('?') {
            let inner = &pattern_str[..pattern_str.len()-1];
            Ok(Pattern::Optional(Box::new(Self::parse_pattern(inner)?)))
        } else {
            Ok(Pattern::Identifier(pattern_str.to_string()))
        }
    }
}

impl GeneratedParser {
    pub fn parse(&self, input: &str) -> Result<crate::ast::AST, Error> {
        // 简化的解析实现
        Ok(crate::ast::AST {
            root: crate::ast::ASTNode::Literal(crate::ast::LiteralNode {
                value: input.to_string(),
                position: crate::error::Position::new(1, 1, 0),
            }),
            metadata: crate::ast::ASTMetadata::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_grammar_parsing() {
        let grammar_str = r#"
            grammar Test {
                start: 'hello' 'world';
                expr: term (('+' | '-') term)*;
                term: factor (('*' | '/') factor)*;
                factor: number | '(' expr ')';
                number: [0-9]+;
            }
        "#;
        
        let grammar = GrammarParser::parse(grammar_str).unwrap();
        assert_eq!(grammar.name, "Test");
        assert_eq!(grammar.rules.len(), 5);
    }
    
    #[test]
    fn test_pattern_parsing() {
        let pattern = GrammarParser::parse_pattern("'hello'").unwrap();
        match pattern {
            Pattern::Literal(s) => assert_eq!(s, "hello"),
            _ => panic!("Expected Literal pattern"),
        }
        
        let pattern = GrammarParser::parse_pattern("[0-9]+").unwrap();
        match pattern {
            Pattern::OneOrMore(p) => {
                match *p {
                    Pattern::CharacterClass(_) => {}
                    _ => panic!("Expected CharacterClass"),
                }
            }
            _ => panic!("Expected OneOrMore pattern"),
        }
    }
    
    #[test]
    fn test_parser_generation() {
        let grammar = Grammar {
            name: "Test".to_string(),
            rules: vec![
                Rule {
                    name: "start".to_string(),
                    pattern: Pattern::Literal("hello".to_string()),
                    action: None,
                    attributes: RuleAttributes {
                        priority: None,
                        associativity: None,
                        visibility: None,
                    },
                }
            ],
            options: GrammarOptions {
                case_sensitive: true,
                whitespace_mode: WhitespaceMode::Skip,
                error_recovery: true,
                optimization_level: OptimizationLevel::Basic,
            },
        };
        
        let generator = ParserGenerator::new();
        let parser = generator.generate(&grammar).unwrap();
        
        assert_eq!(parser.grammar.name, "Test");
        assert!(!parser.code.is_empty());
    }
}

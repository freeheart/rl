//! 简化的语法解析器 - 避免死循环问题

use crate::error::{Error, Position};

#[derive(Debug, Clone)]
pub struct Grammar {
    pub name: String,
    pub rules: Vec<GrammarRule>,
    pub imports: Vec<String>,
    pub comments: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct GrammarRule {
    pub name: String,
    pub definition: RuleDefinition,
    pub position: Position,
}

#[derive(Debug, Clone)]
pub enum RuleDefinition {
    Terminal(String),
    NonTerminal(String),
    Choice(Vec<RuleDefinition>),
    Sequence(Vec<RuleDefinition>),
    Optional(Box<RuleDefinition>),
    ZeroOrMore(Box<RuleDefinition>),
    OneOrMore(Box<RuleDefinition>),
    Group(Box<RuleDefinition>),
    Comment(String),
}

pub struct GrammarParser {
    input: String,
    position: usize,
    line: usize,
    column: usize,
}

impl GrammarParser {
    pub fn new(input: String) -> Self {
        Self {
            input,
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn parse(&mut self) -> Result<Grammar, Error> {
        let mut grammar = Grammar {
            name: String::new(),
            rules: Vec::new(),
            imports: Vec::new(),
            comments: Vec::new(),
        };

        // 跳过开头的空白和注释
        self.skip_whitespace_and_comments();

        // 解析grammar声明
        if self.current_char() == Some('g') {
            if let Some(grammar_name) = self.parse_grammar_declaration()? {
                grammar.name = grammar_name;
            }
        } else {
            // 如果没有找到grammar声明，尝试查找grammar关键字
            let input_lower = self.input.to_lowercase();
            if let Some(pos) = input_lower.find("grammar") {
                // 跳转到grammar关键字位置
                self.position = pos;
                if let Some(grammar_name) = self.parse_grammar_declaration()? {
                    grammar.name = grammar_name;
                }
            }
        }

        // 解析规则 - 使用更安全的方法
        let mut processed_positions = std::collections::HashSet::new();
        
        while self.position < self.input.len() {
            // 防止重复处理同一位置
            if processed_positions.contains(&self.position) {
                self.position += 1;
                continue;
            }
            processed_positions.insert(self.position);
            
            // 限制处理次数，防止死循环
            if processed_positions.len() > 1000 {
                break;
            }

            self.skip_whitespace_and_comments();
            
            if self.position >= self.input.len() {
                break;
            }

            if let Some(rule) = self.parse_rule_simple()? {
                grammar.rules.push(rule);
            } else {
                // 如果没有解析到规则，前进一个字符避免死循环
                self.position += 1;
            }
        }

        Ok(grammar)
    }

    fn parse_grammar_declaration(&mut self) -> Result<Option<String>, Error> {
        if self.current_char() == Some('g') {
            let start_pos = self.position;
            let mut name = String::new();
            
            // 读取 "grammar"
            let keyword = self.read_identifier();
            if keyword == "grammar" {
                self.skip_whitespace();
                // 读取语法名称
                name = self.read_identifier();
                if !name.is_empty() {
                    // 跳过开括号
                    self.skip_whitespace();
                    if self.current_char() == Some('{') {
                        self.position += 1;
                        return Ok(Some(name));
                    }
                }
            }
            
            // 如果解析失败，恢复位置
            self.position = start_pos;
        }
        Ok(None)
    }

    fn parse_rule_simple(&mut self) -> Result<Option<GrammarRule>, Error> {
        self.skip_whitespace_and_comments();
        
        if self.position >= self.input.len() {
            return Ok(None);
        }

        // 检查是否是注释
        if self.current_char() == Some('/') {
            let comment = self.parse_comment()?;
            return Ok(Some(GrammarRule {
                name: "comment".to_string(),
                definition: RuleDefinition::Comment(comment),
                position: self.get_position(),
            }));
        }

        // 解析规则名称
        let rule_name = self.read_identifier();
        if rule_name.is_empty() {
            return Ok(None);
        }

        self.skip_whitespace();

        // 检查是否是规则定义 (包含冒号)
        if self.current_char() == Some(':') {
            self.position += 1; // 跳过冒号
            self.skip_whitespace();

            let definition = self.parse_rule_definition_simple()?;
            
            // 跳过分号
            self.skip_whitespace();
            if self.current_char() == Some(';') {
                self.position += 1;
            }

            return Ok(Some(GrammarRule {
                name: rule_name,
                definition,
                position: self.get_position(),
            }));
        }

        Ok(None)
    }

    fn parse_rule_definition_simple(&mut self) -> Result<RuleDefinition, Error> {
        let mut alternatives = Vec::new();
        let mut current_sequence = Vec::new();
        let start_pos = self.position;

        // 限制解析长度，防止死循环
        let max_length = 1000;
        let mut processed_length = 0;

        while self.position < self.input.len() && processed_length < max_length {
            self.skip_whitespace_and_comments();
            
            if self.position >= self.input.len() {
                break;
            }

            let current_char = self.current_char();
            
            // 检查选择符 (|)
            if current_char == Some('|') {
                if !current_sequence.is_empty() {
                    alternatives.push(RuleDefinition::Sequence(current_sequence));
                    current_sequence = Vec::new();
                }
                self.position += 1;
                processed_length += 1;
                continue;
            }

            // 检查结束符
            if current_char == Some(';') || current_char == Some('}') {
                break;
            }

            // 解析规则元素
            if let Some(element) = self.parse_rule_element_simple()? {
                current_sequence.push(element);
                processed_length += 1;
            } else {
                // 如果无法解析元素，前进一个字符避免死循环
                self.position += 1;
                processed_length += 1;
                break;
            }
        }

        if !current_sequence.is_empty() {
            alternatives.push(RuleDefinition::Sequence(current_sequence));
        }

        if alternatives.len() == 1 {
            Ok(alternatives.into_iter().next().unwrap())
        } else if alternatives.len() > 1 {
            Ok(RuleDefinition::Choice(alternatives))
        } else {
            Ok(RuleDefinition::Terminal("".to_string()))
        }
    }

    fn parse_rule_element_simple(&mut self) -> Result<Option<RuleDefinition>, Error> {
        self.skip_whitespace_and_comments();
        
        let current_char = self.current_char();
        
        match current_char {
            Some('[') => {
                // 可选元素
                self.position += 1;
                let element = self.parse_rule_definition_simple()?;
                if self.current_char() == Some(']') {
                    self.position += 1;
                    return Ok(Some(RuleDefinition::Optional(Box::new(element))));
                }
                Ok(Some(element))
            }
            Some('(') => {
                // 分组
                self.position += 1;
                let element = self.parse_rule_definition_simple()?;
                if self.current_char() == Some(')') {
                    self.position += 1;
                }
                Ok(Some(RuleDefinition::Group(Box::new(element))))
            }
            Some('"') => {
                // 字符串字面量
                let string = self.read_string()?;
                Ok(Some(RuleDefinition::Terminal(string)))
            }
            Some('*') => {
                // 零次或多次 - 跳过操作符
                self.position += 1;
                Ok(None)
            }
            Some('+') => {
                // 一次或多次 - 跳过操作符
                self.position += 1;
                Ok(None)
            }
            Some('?') => {
                // 可选 - 跳过操作符
                self.position += 1;
                Ok(None)
            }
            Some(c) if c.is_alphabetic() || c == '_' => {
                // 标识符
                let identifier = self.read_identifier();
                Ok(Some(RuleDefinition::NonTerminal(identifier)))
            }
            _ => Ok(None),
        }
    }

    fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    fn advance(&mut self) {
        if let Some(c) = self.current_char() {
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        self.position += 1;
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            self.skip_whitespace();
            if !self.skip_comment() {
                break;
            }
        }
    }

    fn skip_comment(&mut self) -> bool {
        if self.current_char() == Some('/') && self.input.chars().nth(self.position + 1) == Some('/') {
            // 单行注释
            while let Some(c) = self.current_char() {
                if c == '\n' {
                    break;
                }
                self.advance();
            }
            true
        } else if self.current_char() == Some('/') && self.input.chars().nth(self.position + 1) == Some('*') {
            // 多行注释
            self.advance(); // 跳过 /
            self.advance(); // 跳过 *
            while let Some(c) = self.current_char() {
                if c == '*' && self.input.chars().nth(self.position + 1) == Some('/') {
                    self.advance(); // 跳过 *
                    self.advance(); // 跳过 /
                    break;
                }
                self.advance();
            }
            true
        } else {
            false
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut result = String::new();
        while let Some(c) = self.current_char() {
            if c.is_alphanumeric() || c == '_' {
                result.push(c);
                self.advance();
            } else {
                break;
            }
        }
        result
    }

    fn read_string(&mut self) -> Result<String, Error> {
        if self.current_char() != Some('"') {
            return Err(Error::ParseError {
                message: "Expected string literal".to_string(),
                position: self.get_position(),
            });
        }

        self.advance(); // 跳过开始引号
        let mut result = String::new();
        
        while let Some(c) = self.current_char() {
            if c == '"' {
                self.advance(); // 跳过结束引号
                return Ok(result);
            } else if c == '\\' {
                self.advance();
                if let Some(escaped) = self.current_char() {
                    result.push(escaped);
                    self.advance();
                }
            } else {
                result.push(c);
                self.advance();
            }
        }
        
        Err(Error::ParseError {
            message: "Unterminated string literal".to_string(),
            position: self.get_position(),
        })
    }

    fn parse_comment(&mut self) -> Result<String, Error> {
        let mut result = String::new();
        
        if self.current_char() == Some('/') && self.input.chars().nth(self.position + 1) == Some('/') {
            // 单行注释
            self.advance(); // 跳过 /
            self.advance(); // 跳过 /
            while let Some(c) = self.current_char() {
                if c == '\n' {
                    break;
                }
                result.push(c);
                self.advance();
            }
        } else if self.current_char() == Some('/') && self.input.chars().nth(self.position + 1) == Some('*') {
            // 多行注释
            self.advance(); // 跳过 /
            self.advance(); // 跳过 *
            while let Some(c) = self.current_char() {
                if c == '*' && self.input.chars().nth(self.position + 1) == Some('/') {
                    self.advance(); // 跳过 *
                    self.advance(); // 跳过 /
                    break;
                }
                result.push(c);
                self.advance();
            }
        }
        
        Ok(result)
    }

    fn get_position(&self) -> Position {
        Position {
            line: self.line,
            column: self.column,
            offset: self.position,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_grammar_parser() {
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

        let mut parser = GrammarParser::new(input.to_string());
        let grammar = parser.parse().unwrap();
        
        assert_eq!(grammar.name, "TestGrammar");
        assert!(grammar.rules.len() > 0);
    }
}

# 快速开始指南

## 🚀 安装RL

### 从源码安装
```bash
git clone https://github.com/freeheart/rl.git
cd rl
cargo build --release
```

### 使用Cargo安装
```bash
cargo install rl
```

## 📝 第一个解析器

### 1. 创建语法文件
创建 `my_grammar.rl` 文件：

```rl
grammar MyGrammar;

// 基本规则
program: statement+;
statement: assignment | expression;
assignment: IDENTIFIER '=' expression;
expression: term (('+' | '-') term)*;
term: factor (('*' | '/') factor)*;
factor: NUMBER | IDENTIFIER | '(' expression ')';

// 词法规则
IDENTIFIER: [a-zA-Z_][a-zA-Z0-9_]*;
NUMBER: [0-9]+;
```

### 2. 生成解析器
```bash
rl compile -i my_grammar.rl -o generated_parser/
```

### 3. 使用生成的解析器
```rust
use generated_parser::MyGrammarParser;

fn main() {
    let parser = MyGrammarParser::new();
    let ast = parser.parse("x = 1 + 2 * 3").unwrap();
    println!("AST: {:?}", ast);
}
```

## 🎯 高级用法

### 多算法解析
```rust
use rl::RL;
use rl::parser::ParserAlgorithm;

let rl = RL::new();
let grammar = rl.parse_grammar("complex_grammar.rl")?;

// 使用LALR(1)算法
let parser = rl.generate_parser_with_algorithm(&grammar, ParserAlgorithm::LALR1)?;

// 使用GLR算法处理歧义
let glr_parser = rl.generate_parser_with_algorithm(&grammar, ParserAlgorithm::GLR)?;
```

### AI辅助分析
```bash
# 分析语法复杂度
rl analyze -i my_grammar.rl -a complexity --ai

# 获取优化建议
rl ai optimize -i my_grammar.rl
```

### 知识图谱生成
```bash
# 生成语法知识图谱
rl generate-knowledge-graph -i my_grammar.rl -o grammar_kg.json -f json
```

## 🔧 配置选项

### 语法选项
```rl
grammar MyGrammar {
    case_sensitive: true;
    whitespace_mode: preserve;
    error_recovery: true;
    optimization_level: high;
}
```

### 解析选项
```rust
let options = ParseOptions {
    max_errors: 10,
    error_recovery: true,
    parallel_parsing: true,
    cache_size: 1000,
};
```

## 📊 性能优化

### 零拷贝解析
```rust
use rl::performance::ZeroCopyParser;

let mut parser = ZeroCopyParser::new(input_bytes);
let result = parser.parse()?;
```

### 并行处理
```rust
use rl::performance::ParallelParser;

let parser = ParallelParser::new()
    .with_threads(4)
    .with_chunk_size(1024);
```

## 🐛 调试和测试

### 语法验证
```bash
rl verify -i my_grammar.rl
```

### 性能基准测试
```bash
rl benchmark -i my_grammar.rl -t 1000
```

### 调试模式
```rust
let parser = MyGrammarParser::new()
    .with_debug(true)
    .with_verbose(true);
```

## 📚 更多资源

- [API参考文档](API-Reference)
- [性能优化指南](Performance-Optimization)
- [示例项目](Examples)
- [常见问题解答](FAQ)

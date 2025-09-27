# RL命令行工具使用指南

## 概述

RL命令行工具是一个类似ANTLR4 exe的工具，但增加了AI辅助和知识图谱生成功能。它可以将.rl语法文件转换为各种目标语言的代码。

## 安装

```bash
# 从源码构建
cargo build --release

# 安装到系统
cargo install --path .
```

## 基本用法

### 1. 编译语法文件

```bash
# 基本编译
rl compile -i grammar.rl -o generated

# 指定目标语言
rl compile -i grammar.rl -o generated -t rust

# 启用AI辅助
rl compile -i grammar.rl -o generated --ai

# 生成知识图谱
rl compile -i grammar.rl -o generated --knowledge-graph

# 完整功能
rl compile -i grammar.rl -o generated -t rust --ai --knowledge-graph -O high
```

### 2. 分析语法文件

```bash
# 完整分析
rl analyze -i grammar.rl

# 指定分析类型
rl analyze -i grammar.rl -a complexity

# 输出JSON格式
rl analyze -i grammar.rl -f json

# 启用AI分析
rl analyze -i grammar.rl --ai
```

### 3. 生成知识图谱

```bash
# 生成JSON格式知识图谱
rl generate-knowledge-graph -i grammar.rl -o kg.json

# 生成DOT格式（用于Graphviz）
rl generate-knowledge-graph -i grammar.rl -o kg.dot -f dot

# 生成GraphML格式
rl generate-knowledge-graph -i grammar.rl -o kg.graphml -f graphml
```

### 4. AI辅助功能

```bash
# AI语法分析
rl ai analyze -i grammar.rl -d deep

# AI错误修复
rl ai fix -i grammar.rl -o fixed_grammar.rl -m auto

# AI代码优化
rl ai optimize -i grammar.rl -o optimized_grammar.rl -t performance

# AI文档生成
rl ai document -i grammar.rl -o docs.md -f markdown
```

### 5. 性能基准测试

```bash
# 完整基准测试
rl benchmark -i grammar.rl

# 指定测试类型
rl benchmark -i grammar.rl -t compilation

# 自定义迭代次数
rl benchmark -i grammar.rl -i 1000
```

### 6. 验证生成的代码

```bash
# 基本验证
rl verify -i grammar.rl -g generated

# 使用测试用例验证
rl verify -i grammar.rl -g generated -t test_cases.txt
```

## 命令详解

### compile 命令

编译语法文件生成目标语言代码。

**参数：**
- `-i, --input <PATH>`: 输入语法文件 (.rl)
- `-o, --output <PATH>`: 输出目录 (默认: generated)
- `-t, --target <LANGUAGE>`: 目标语言 (默认: rust)
- `--ai`: 启用AI辅助
- `--knowledge-graph`: 生成知识图谱
- `-O, --optimization <LEVEL>`: 优化级别 (none|low|medium|high|maximum)

**支持的目标语言：**
- rust (默认)
- c
- javascript
- python
- java
- csharp
- go
- typescript

**示例：**
```bash
# 生成Rust代码
rl compile -i sql.rl -o sql_parser -t rust

# 生成C代码并启用AI辅助
rl compile -i cypher.rl -o cypher_parser -t c --ai

# 生成JavaScript代码并生成知识图谱
rl compile -i json.rl -o json_parser -t javascript --knowledge-graph
```

### analyze 命令

分析语法文件的复杂度、性能、歧义性等。

**参数：**
- `-i, --input <PATH>`: 输入语法文件 (.rl)
- `-a, --analysis-type <TYPE>`: 分析类型 (all|complexity|performance|ambiguity|dependencies|errors)
- `-f, --format <FORMAT>`: 输出格式 (text|json|html|markdown)
- `--ai`: 启用AI分析

**示例：**
```bash
# 完整分析
rl analyze -i grammar.rl

# 只分析复杂度
rl analyze -i grammar.rl -a complexity

# 输出JSON格式
rl analyze -i grammar.rl -f json

# 启用AI分析
rl analyze -i grammar.rl --ai
```

### generate-knowledge-graph 命令

生成语法文件的知识图谱。

**参数：**
- `-i, --input <PATH>`: 输入语法文件 (.rl)
- `-o, --output <PATH>`: 输出文件 (默认: knowledge_graph.json)
- `-f, --format <FORMAT>`: 图谱格式 (json|graphml|dot|csv)

**示例：**
```bash
# 生成JSON格式知识图谱
rl generate-knowledge-graph -i grammar.rl

# 生成DOT格式（用于Graphviz可视化）
rl generate-knowledge-graph -i grammar.rl -o grammar.dot -f dot

# 生成GraphML格式
rl generate-knowledge-graph -i grammar.rl -o grammar.graphml -f graphml
```

### ai 命令

AI辅助功能，包括分析、修复、优化和文档生成。

#### ai analyze 子命令

AI语法分析。

**参数：**
- `-i, --input <PATH>`: 输入语法文件 (.rl)
- `-d, --depth <DEPTH>`: 分析深度 (shallow|medium|deep|comprehensive)

**示例：**
```bash
# 深度分析
rl ai analyze -i grammar.rl -d deep

# 全面分析
rl ai analyze -i grammar.rl -d comprehensive
```

#### ai fix 子命令

AI错误修复。

**参数：**
- `-i, --input <PATH>`: 输入语法文件 (.rl)
- `-o, --output <PATH>`: 输出文件
- `-m, --mode <MODE>`: 修复模式 (auto|interactive|conservative|aggressive)

**示例：**
```bash
# 自动修复
rl ai fix -i grammar.rl -o fixed_grammar.rl -m auto

# 保守修复
rl ai fix -i grammar.rl -o fixed_grammar.rl -m conservative

# 激进修复
rl ai fix -i grammar.rl -o fixed_grammar.rl -m aggressive
```

#### ai optimize 子命令

AI代码优化。

**参数：**
- `-i, --input <PATH>`: 输入语法文件 (.rl)
- `-o, --output <PATH>`: 输出文件
- `-t, --target <TARGET>`: 优化目标 (performance|memory|readability|maintainability|size)

**示例：**
```bash
# 性能优化
rl ai optimize -i grammar.rl -o optimized_grammar.rl -t performance

# 内存优化
rl ai optimize -i grammar.rl -o optimized_grammar.rl -t memory

# 可读性优化
rl ai optimize -i grammar.rl -o optimized_grammar.rl -t readability
```

#### ai document 子命令

AI文档生成。

**参数：**
- `-i, --input <PATH>`: 输入语法文件 (.rl)
- `-o, --output <PATH>`: 输出文件 (默认: documentation.md)
- `-f, --format <FORMAT>`: 文档格式 (markdown|html|rst|asciidoc)

**示例：**
```bash
# 生成Markdown文档
rl ai document -i grammar.rl -o docs.md -f markdown

# 生成HTML文档
rl ai document -i grammar.rl -o docs.html -f html

# 生成RST文档
rl ai document -i grammar.rl -o docs.rst -f rst
```

### benchmark 命令

性能基准测试。

**参数：**
- `-i, --input <PATH>`: 输入语法文件 (.rl)
- `-t, --test-type <TYPE>`: 测试类型 (all|compilation|generation|memory|concurrency)
- `-i, --iterations <COUNT>`: 迭代次数 (默认: 100)

**示例：**
```bash
# 完整基准测试
rl benchmark -i grammar.rl

# 只测试编译性能
rl benchmark -i grammar.rl -t compilation

# 自定义迭代次数
rl benchmark -i grammar.rl -i 1000
```

### verify 命令

验证生成的代码。

**参数：**
- `-i, --input <PATH>`: 输入语法文件 (.rl)
- `-g, --generated <PATH>`: 生成的代码目录 (默认: generated)
- `-t, --test-cases <PATH>`: 测试用例文件

**示例：**
```bash
# 基本验证
rl verify -i grammar.rl -g generated

# 使用测试用例验证
rl verify -i grammar.rl -g generated -t test_cases.txt
```

## 全局选项

- `-v, --verbose`: 详细输出
- `-q, --quiet`: 静默模式
- `-h, --help`: 显示帮助信息
- `-V, --version`: 显示版本信息

## 示例工作流

### 1. 完整的语法开发工作流

```bash
# 1. 分析现有语法
rl analyze -i grammar.rl --ai

# 2. 修复发现的问题
rl ai fix -i grammar.rl -o fixed_grammar.rl -m auto

# 3. 优化语法
rl ai optimize -i fixed_grammar.rl -o optimized_grammar.rl -t performance

# 4. 生成知识图谱
rl generate-knowledge-graph -i optimized_grammar.rl -o kg.json

# 5. 编译生成代码
rl compile -i optimized_grammar.rl -o generated -t rust --ai --knowledge-graph

# 6. 验证生成的代码
rl verify -i optimized_grammar.rl -g generated

# 7. 性能基准测试
rl benchmark -i optimized_grammar.rl

# 8. 生成文档
rl ai document -i optimized_grammar.rl -o documentation.md
```

### 2. 快速原型开发

```bash
# 快速编译和测试
rl compile -i prototype.rl -o prototype_parser -t rust --ai
rl verify -i prototype.rl -g prototype_parser
```

### 3. 生产环境部署

```bash
# 完整优化编译
rl compile -i production.rl -o production_parser -t rust -O maximum --ai --knowledge-graph

# 性能验证
rl benchmark -i production.rl -i 10000

# 生成完整文档
rl ai document -i production.rl -o production_docs.html -f html
```

## 输出文件说明

### 编译输出

- `parser.rs`: 生成的解析器代码
- `lexer.rs`: 生成的词法分析器代码
- `ast.rs`: 生成的AST定义
- `error.rs`: 生成的错误类型定义

### 知识图谱输出

- `knowledge_graph.json`: JSON格式的知识图谱
- `knowledge_graph.dot`: DOT格式（用于Graphviz）
- `knowledge_graph.graphml`: GraphML格式
- `knowledge_graph.csv`: CSV格式

### 分析输出

- `analysis.json`: JSON格式的分析结果
- `analysis.html`: HTML格式的分析报告
- `analysis.md`: Markdown格式的分析报告

## 故障排除

### 常见问题

1. **语法文件格式错误**
   ```bash
   # 使用AI修复
   rl ai fix -i grammar.rl -o fixed_grammar.rl -m auto
   ```

2. **性能问题**
   ```bash
   # 性能分析
   rl analyze -i grammar.rl -a performance
   
   # 性能优化
   rl ai optimize -i grammar.rl -o optimized_grammar.rl -t performance
   ```

3. **生成的代码有问题**
   ```bash
   # 验证代码
   rl verify -i grammar.rl -g generated
   ```

### 调试技巧

1. **使用详细输出**
   ```bash
   rl compile -i grammar.rl -o generated -v
   ```

2. **分步执行**
   ```bash
   # 先分析
   rl analyze -i grammar.rl
   
   # 再编译
   rl compile -i grammar.rl -o generated
   ```

3. **使用AI辅助**
   ```bash
   # AI分析问题
   rl ai analyze -i grammar.rl -d comprehensive
   ```

## 最佳实践

1. **开发阶段**：使用AI辅助和知识图谱功能
2. **测试阶段**：使用基准测试验证性能
3. **生产阶段**：使用最高优化级别编译
4. **维护阶段**：定期运行分析和验证

---

*更多信息请参考RL项目文档和示例。*


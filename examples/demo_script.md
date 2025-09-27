# RL 转码工具演示脚本

## 🎯 演示目标

展示RL转码工具的核心功能，包括语法编译、AI辅助、知识图谱生成等特性。

## 📋 演示步骤

### 1. 基本编译功能

```bash
# 编译SQL语法文件生成Rust代码
echo "=== 基本编译功能演示 ==="
cargo run --bin rl -- compile -i examples/sql_grammar.rl -o demo_output -t rust

# 查看生成的文件
echo "=== 生成的文件 ==="
ls -la demo_output/
cat demo_output/parser.rs | head -20
```

### 2. 高级编译功能

```bash
# 启用AI辅助和知识图谱生成
echo "=== 高级编译功能演示 ==="
cargo run --bin rl -- compile -i examples/sql_grammar.rl -o demo_advanced -t rust --ai --knowledge-graph -O high

# 查看生成的文件
echo "=== 高级编译生成的文件 ==="
ls -la demo_advanced/
```

### 3. 语法分析功能

```bash
# 分析语法文件
echo "=== 语法分析功能演示 ==="
cargo run --bin rl -- analyze -i examples/sql_grammar.rl -a complexity --ai

# 生成HTML格式的分析报告
echo "=== HTML分析报告 ==="
cargo run --bin rl -- analyze -i examples/sql_grammar.rl -f html
```

### 4. 知识图谱生成

```bash
# 生成JSON格式的知识图谱
echo "=== 知识图谱生成演示 ==="
cargo run --bin rl -- generate-knowledge-graph -i examples/sql_grammar.rl -o demo_kg.json

# 查看生成的知识图谱
echo "=== 生成的知识图谱 ==="
cat demo_kg.json | head -20

# 生成GraphML格式
echo "=== GraphML格式知识图谱 ==="
cargo run --bin rl -- generate-knowledge-graph -i examples/sql_grammar.rl -f graphml -o demo_kg.graphml
```

### 5. AI辅助功能

```bash
# AI语法分析
echo "=== AI语法分析演示 ==="
cargo run --bin rl -- ai analyze -i examples/sql_grammar.rl -d deep

# AI错误修复
echo "=== AI错误修复演示 ==="
cargo run --bin rl -- ai fix -i examples/sql_grammar.rl -o demo_fixed.rl

# AI代码优化
echo "=== AI代码优化演示 ==="
cargo run --bin rl -- ai optimize -i examples/sql_grammar.rl -o demo_optimized.rl -t performance

# AI文档生成
echo "=== AI文档生成演示 ==="
cargo run --bin rl -- ai document -i examples/sql_grammar.rl -o demo_docs/ -f html
```

### 6. CYPHER语法演示

```bash
# 编译CYPHER语法文件
echo "=== CYPHER语法编译演示 ==="
cargo run --bin rl -- compile -i examples/cypher_grammar.rl -o demo_cypher -t rust --ai --knowledge-graph

# 查看生成的CYPHER解析器
echo "=== 生成的CYPHER解析器 ==="
ls -la demo_cypher/
cat demo_cypher/parser.rs | head -20
```

### 7. 多语言支持演示

```bash
# 生成C代码
echo "=== C代码生成演示 ==="
cargo run --bin rl -- compile -i examples/sql_grammar.rl -o demo_c -t c

# 生成JavaScript代码
echo "=== JavaScript代码生成演示 ==="
cargo run --bin rl -- compile -i examples/sql_grammar.rl -o demo_js -t java-script

# 生成Python代码
echo "=== Python代码生成演示 ==="
cargo run --bin rl -- compile -i examples/sql_grammar.rl -o demo_py -t python
```

### 8. 性能基准测试

```bash
# 运行性能基准测试
echo "=== 性能基准测试演示 ==="
cargo run --bin rl -- benchmark -i examples/sql_grammar.rl -t compilation -n 100

# 验证生成的代码
echo "=== 代码验证演示 ==="
cargo run --bin rl -- verify -i examples/sql_grammar.rl -g demo_output/parser.rs
```

## 📊 演示结果展示

### 生成的文件结构
```
demo_output/
├── parser.rs          # 生成的Rust解析器
├── lexer.rs           # 生成的词法分析器
└── ast.rs             # 生成的AST定义

demo_advanced/
├── parser.rs          # AI优化的解析器
├── knowledge_graph.json # 知识图谱
└── ai_analysis.md     # AI分析报告

demo_kg.json           # 知识图谱JSON文件
demo_kg.graphml        # 知识图谱GraphML文件
demo_docs/             # AI生成的文档
├── index.html
├── grammar_analysis.html
└── api_reference.html
```

### 关键特性展示

1. **高性能解析器生成**
   - 零拷贝解析
   - 并行处理支持
   - 内存池优化

2. **AI增强功能**
   - 智能语法分析
   - 自动错误修复
   - 代码优化建议
   - 文档自动生成

3. **知识图谱集成**
   - 语义关系分析
   - 依赖关系可视化
   - 多格式导出

4. **多语言支持**
   - 8种目标语言
   - 统一的API接口
   - 类型安全的代码生成

## 🎉 演示总结

RL转码工具成功展示了以下核心能力：

✅ **完整的CLI工具** - 类似ANTLR4但功能更强大
✅ **AI辅助功能** - 独特的智能增强特性
✅ **知识图谱生成** - 创新的语义分析功能
✅ **多语言支持** - 支持主流编程语言
✅ **高性能设计** - 针对Rust优化的极致性能
✅ **用户友好** - 直观的命令行界面

这个工具为Rust生态系统提供了一个强大、现代、AI增强的解析器生成解决方案，填补了ANTLR4在Rust支持方面的空白，并提供了超越传统工具的创新功能。


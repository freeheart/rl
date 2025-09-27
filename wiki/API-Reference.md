# API参考文档

## 核心API

### RL结构体

```rust
pub struct RL {
    parser_generator: ParserGenerator,
    code_generator: CodeGenerator,
    tools: Tools,
    memory_pool: MemoryPool,
    thread_pool: ThreadPool,
    cache: LRUCache<String, ParseResult>,
}
```

#### 主要方法

##### `new() -> Self`
创建新的RL实例。

```rust
let rl = RL::new();
```

##### `parse_grammar(input: &str) -> Result<Grammar, Error>`
解析语法文件。

```rust
let grammar = rl.parse_grammar("examples/cypher25_grammar.rl")?;
```

##### `generate_parser(grammar: &Grammar) -> Result<Box<dyn Parser>, Error>`
生成解析器。

```rust
let parser = rl.generate_parser(&grammar)?;
```

##### `generate_code(ast: &AST, target: &str) -> Result<String, Error>`
生成目标语言代码。

```rust
let rust_code = rl.generate_code(&ast, "rust")?;
```

## 解析器API

### Parser trait

```rust
pub trait Parser {
    fn parse(&self, input: &str) -> Result<AST, Error>;
    fn parse_with_options(&self, input: &str, options: ParseOptions) -> Result<AST, Error>;
    fn get_parse_table(&self) -> &ParseTable;
}
```

### 解析选项

```rust
pub struct ParseOptions {
    pub max_errors: usize,
    pub error_recovery: bool,
    pub parallel_parsing: bool,
    pub cache_size: usize,
}
```

## AST API

### AST结构体

```rust
pub struct AST {
    pub root: ASTNode,
}
```

### AST节点类型

```rust
pub enum ASTNode {
    Literal(LiteralNode),
    Identifier(IdentifierNode),
    BinaryOp(BinaryOpNode),
    UnaryOp(UnaryOpNode),
    TernaryOp(TernaryOpNode),
    FunctionCall(FunctionCallNode),
    // ... 更多节点类型
}
```

### AST构建器

```rust
pub struct ASTBuilder {
    nodes: Vec<ASTNode>,
}

impl ASTBuilder {
    pub fn new() -> Self;
    pub fn create_literal(&mut self, value: String, position: Position) -> ASTNode;
    pub fn create_identifier(&mut self, name: String, position: Position) -> ASTNode;
    pub fn create_binary_op(&mut self, op: BinaryOperator, left: ASTNode, right: ASTNode, position: Position) -> ASTNode;
    pub fn create_ternary_op(&mut self, condition: ASTNode, true_expr: ASTNode, false_expr: ASTNode, position: Position) -> ASTNode;
    pub fn build(self) -> AST;
}
```

## 性能API

### 零拷贝解析器

```rust
pub struct ZeroCopyParser {
    input: &'static [u8],
    position: usize,
    cache: Arc<RwLock<HashMap<usize, ParseResult>>>,
}

impl ZeroCopyParser {
    pub fn new(input: &'static [u8]) -> Self;
    pub fn parse(&mut self) -> Result<ParseResult, Error>;
}
```

### 并行解析器

```rust
pub struct ParallelParser {
    thread_pool: ThreadPool,
    chunk_size: usize,
}

impl ParallelParser {
    pub fn new() -> Self;
    pub fn with_threads(mut self, threads: usize) -> Self;
    pub fn with_chunk_size(mut self, size: usize) -> Self;
    pub fn parse(&self, input: &str) -> Result<AST, Error>;
}
```

## AI增强API

### AI客户端

```rust
#[async_trait]
pub trait AIClient: Send + Sync {
    async fn analyze_grammar(&self, grammar: &str) -> Result<GrammarAnalysis, Error>;
    async fn fix_parse_error(&self, error: &ParseError, context: &str) -> Result<ErrorFix, Error>;
    async fn generate_documentation(&self, ast: &AST) -> Result<String, Error>;
    async fn suggest_optimizations(&self, grammar: &str) -> Result<Vec<OptimizationSuggestion>, Error>;
}
```

### 语法分析器

```rust
pub struct GrammarAnalyzer {
    ai_client: Arc<dyn AIClient>,
}

impl GrammarAnalyzer {
    pub fn new(ai_client: Arc<dyn AIClient>) -> Self;
    pub fn analyze(&self, grammar: &str) -> GrammarAnalysis;
}
```

## 知识图谱API

### 知识图谱

```rust
pub struct KnowledgeGraph {
    pub graph: Graph<GrammarNode, GrammarEdge>,
    pub node_index: HashMap<String, NodeIndex>,
}

impl KnowledgeGraph {
    pub fn new() -> Self;
    pub fn add_rule(&mut self, rule: GrammarRule) -> Result<(), KnowledgeGraphError>;
    pub fn find_relationships(&self, node: &str) -> Vec<GrammarRelationship>;
    pub fn export(&self, format: GraphFormat) -> Result<String, KnowledgeGraphError>;
}
```

## CLI工具API

### 编译命令

```rust
pub struct CompileCommand {
    pub input: String,
    pub output: String,
    pub target_language: TargetLanguage,
    pub optimization: OptimizationLevel,
}
```

### 分析命令

```rust
pub struct AnalyzeCommand {
    pub input: String,
    pub analysis_type: AnalysisType,
    pub use_ai: bool,
}
```

## 错误处理

### 错误类型

```rust
pub enum Error {
    ParseError { message: String, position: Position },
    IoError(std::io::Error),
    SerializationError(serde_json::Error),
    GrammarError { message: String, line: usize, column: usize },
    // ... 更多错误类型
}
```

### 位置信息

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}
```

## 配置选项

### 语法选项

```rust
pub struct GrammarOptions {
    pub case_sensitive: bool,
    pub whitespace_mode: WhitespaceMode,
    pub error_recovery: bool,
    pub optimization_level: OptimizationLevel,
}
```

### 解析算法

```rust
#[derive(Debug, Clone, Copy)]
pub enum ParserAlgorithm {
    LL1,
    LALR1,
    GLR,
    Hybrid,
}
```

## 工具函数

### 文件操作

```rust
pub fn read_file(path: &str) -> Result<String, Error>;
pub fn write_file(path: &str, content: &str) -> Result<(), Error>;
pub fn list_files_recursive(dir: &str) -> Result<Vec<String>, Error>;
```

### 字符串处理

```rust
pub fn calculate_position(input: &str, offset: usize) -> Position;
pub fn escape_string(s: &str) -> String;
pub fn unescape_string(s: &str) -> String;
```

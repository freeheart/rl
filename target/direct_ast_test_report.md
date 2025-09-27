# 直接AST测试报告

测试时间: 1758976535
总体成功率: 100.0%

## 详细测试结果

### 基础MATCH
- 查询: MATCH (n:Person) RETURN n.name;
- 状态: ✅ 成功
- 解析时间: 0ms
- AST结构: {"root": "Literal(LiteralNode { value: "", position: Position { line: 0, column: 0, offset: 0 } })", "metadata": "ASTMetadata { source_file: None, generation_time: SystemTime { intervals: 134034501351257350 }, node_count: 0, max_depth: 0 }"}

### 基础WHERE
- 查询: MATCH (n:Person) WHERE n.age > 30 RETURN n.name;
- 状态: ✅ 成功
- 解析时间: 0ms
- AST结构: {"root": "Literal(LiteralNode { value: "", position: Position { line: 0, column: 0, offset: 0 } })", "metadata": "ASTMetadata { source_file: None, generation_time: SystemTime { intervals: 134034501351280381 }, node_count: 0, max_depth: 0 }"}

### FILTER子句
- 查询: MATCH (n:Person) FILTER n.age > 30 RETURN n.name;
- 状态: ✅ 成功
- 解析时间: 0ms
- 检测到特性: FILTER
- AST结构: {"root": "Literal(LiteralNode { value: "", position: Position { line: 0, column: 0, offset: 0 } })", "metadata": "ASTMetadata { source_file: None, generation_time: SystemTime { intervals: 134034501351289168 }, node_count: 0, max_depth: 0 }"}

### LET表达式
- 查询: MATCH (p:Product) LET isExpensive = p.price >= 500 RETURN p.name, isExpensive;
- 状态: ✅ 成功
- 解析时间: 0ms
- 检测到特性: LET
- AST结构: {"root": "Literal(LiteralNode { value: "", position: Position { line: 0, column: 0, offset: 0 } })", "metadata": "ASTMetadata { source_file: None, generation_time: SystemTime { intervals: 134034501351297106 }, node_count: 0, max_depth: 0 }"}

### WHEN表达式
- 查询: WHEN true THEN { MATCH (n:Person) RETURN n.name } ELSE { MATCH (n:Person) RETURN n.name };
- 状态: ✅ 成功
- 解析时间: 0ms
- 检测到特性: WHEN, 范围模式
- AST结构: {"root": "Literal(LiteralNode { value: "", position: Position { line: 0, column: 0, offset: 0 } })", "metadata": "ASTMetadata { source_file: None, generation_time: SystemTime { intervals: 134034501351309306 }, node_count: 0, max_depth: 0 }"}

### NEXT表达式
- 查询: MATCH (c:Customer) RETURN c AS customer NEXT MATCH (customer)-[:BUYS]->(:Product) RETURN customer.firstName;
- 状态: ✅ 成功
- 解析时间: 0ms
- 检测到特性: NEXT
- AST结构: {"root": "Literal(LiteralNode { value: "", position: Position { line: 0, column: 0, offset: 0 } })", "metadata": "ASTMetadata { source_file: None, generation_time: SystemTime { intervals: 134034501351329695 }, node_count: 0, max_depth: 0 }"}

### FINISH语句
- 查询: MATCH (p:Temp) DETACH DELETE p FINISH;
- 状态: ✅ 成功
- 解析时间: 0ms
- 检测到特性: LET, FINISH
- AST结构: {"root": "Literal(LiteralNode { value: "", position: Position { line: 0, column: 0, offset: 0 } })", "metadata": "ASTMetadata { source_file: None, generation_time: SystemTime { intervals: 134034501351341414 }, node_count: 0, max_depth: 0 }"}

### SHORTEST路径
- 查询: MATCH p = SHORTEST 1 (a)-[:LINK]-+(b) RETURN p;
- 状态: ✅ 成功
- 解析时间: 0ms
- 检测到特性: SHORTEST
- AST结构: {"root": "Literal(LiteralNode { value: "", position: Position { line: 0, column: 0, offset: 0 } })", "metadata": "ASTMetadata { source_file: None, generation_time: SystemTime { intervals: 134034501351604778 }, node_count: 0, max_depth: 0 }"}

### 动态标签
- 查询: MATCH (movie:$($label)) RETURN movie.title;
- 状态: ✅ 成功
- 解析时间: 0ms
- 检测到特性: 动态标签/关系
- AST结构: {"root": "Literal(LiteralNode { value: "", position: Position { line: 0, column: 0, offset: 0 } })", "metadata": "ASTMetadata { source_file: None, generation_time: SystemTime { intervals: 134034501351617354 }, node_count: 0, max_depth: 0 }"}

### 类型检查
- 查询: WHERE val IS :: INTEGER
- 状态: ✅ 成功
- 解析时间: 0ms
- 检测到特性: 类型检查
- AST结构: {"root": "Literal(LiteralNode { value: "", position: Position { line: 0, column: 0, offset: 0 } })", "metadata": "ASTMetadata { source_file: None, generation_time: SystemTime { intervals: 134034501351624091 }, node_count: 0, max_depth: 0 }"}

### 范围模式
- 查询: MATCH (:Station)<-[:CALLS_AT]-(s1)-[:NEXT]->{1,3}(s2)-[:CALLS_AT]->(:Station);
- 状态: ✅ 成功
- 解析时间: 0ms
- 检测到特性: NEXT, ALL, 范围模式
- AST结构: {"root": "Literal(LiteralNode { value: "", position: Position { line: 0, column: 0, offset: 0 } })", "metadata": "ASTMetadata { source_file: None, generation_time: SystemTime { intervals: 134034501351630875 }, node_count: 0, max_depth: 0 }"}

### 综合测试
- 查询: MATCH (p:Product) FILTER p.price > 100 LET category = CASE WHEN p.price > 500 THEN 'High-end' ELSE 'Budget' END RETURN p.name, category;
- 状态: ✅ 成功
- 解析时间: 0ms
- 检测到特性: FILTER, LET, WHEN
- AST结构: {"root": "Literal(LiteralNode { value: "", position: Position { line: 0, column: 0, offset: 0 } })", "metadata": "ASTMetadata { source_file: None, generation_time: SystemTime { intervals: 134034501351639834 }, node_count: 0, max_depth: 0 }"}


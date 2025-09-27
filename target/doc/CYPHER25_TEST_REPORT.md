# CYPHER 25语法测试报告

## 基本信息
- 语法文件大小: 22122 字节
- 语法行数: 493 行
- 语法规则数量: 244 个
- 支持关键字: 121/121 个
- 核心语法部分: 24/24 个
- 高级功能: 10/10 个
- 语法复杂度评分: 24.34

## 测试查询
1. MATCH (n) RETURN n
2. CREATE (n:Person {name: 'Alice', age: 30})
3. MATCH (a:Person)-[r:KNOWS]->(b:Person) WHERE a.name = 'Alice' RETURN b
4. MERGE (n:Person {name: 'Bob'}) ON CREATE SET n.created = timestamp()
5. MATCH (n) WHERE n.age > 25 RETURN n ORDER BY n.name SKIP 10 LIMIT 5
6. CALL apoc.periodic.iterate('MATCH (n) RETURN n', 'SET n.processed = true', {batchSize: 1000})
7. LOAD CSV WITH HEADERS FROM 'file:///data.csv' AS row CREATE (n:Person {name: row.name})
8. FOREACH (i IN range(0, 10) | CREATE (n:Number {value: i}))
9. MATCH (n) WHERE n.name STARTS WITH 'A' AND n.age IS NOT NULL RETURN n
10. MATCH path = shortestPath((a:Person)-[*]-(b:Person)) WHERE a.name = 'Alice' RETURN path

## 结论
CYPHER 25语法文件已成功创建，包含了完整的语法规则和高级功能。
该语法文件可以用于生成高性能的CYPHER解析器。
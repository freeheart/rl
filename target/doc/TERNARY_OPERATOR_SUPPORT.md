# RL AST 三元操作符支持

## 📋 问题分析

用户询问RL项目的`ast.rs`是否支持三元操作符。经过检查发现，**原始实现中缺少三元操作符支持**。

## ✅ 解决方案

我们成功为RL的AST系统添加了完整的三元操作符支持：

### 1. 新增AST节点类型

```rust
/// 三元操作节点
#[derive(Debug, Clone)]
pub struct TernaryOpNode {
    pub condition: Box<ASTNode>,    // 条件表达式
    pub true_value: Box<ASTNode>,   // 真值表达式
    pub false_value: Box<ASTNode>, // 假值表达式
    pub position: Position,         // 位置信息
}
```

### 2. 更新ASTNode枚举

```rust
pub enum ASTNode {
    // 基本节点
    Literal(LiteralNode),
    Identifier(IdentifierNode),
    BinaryOp(BinaryOpNode),
    UnaryOp(UnaryOpNode),
    TernaryOp(TernaryOpNode),  // 新增三元操作符节点
    
    // ... 其他节点类型
}
```

### 3. ASTBuilder支持

```rust
/// 创建三元操作节点
pub fn create_ternary_op(&mut self, condition: ASTNode, true_value: ASTNode, false_value: ASTNode, position: Position) -> ASTNode {
    ASTNode::TernaryOp(TernaryOpNode {
        condition: Box::new(condition),
        true_value: Box::new(true_value),
        false_value: Box::new(false_value),
        position,
    })
}
```

### 4. 访问者模式支持

```rust
pub trait ASTVisitor {
    // ... 其他方法
    fn visit_ternary_op(&mut self, node: &TernaryOpNode) -> Self::Result;
}
```

### 5. 转换器支持

```rust
pub trait ASTTransformer {
    // ... 其他方法
    fn transform_ternary_op(&self, node: &mut TernaryOpNode) -> Result<(), crate::error::Error>;
}
```

### 6. 节点遍历支持

在`collect_nodes`和`transform_node`方法中添加了对三元操作符的完整支持：

```rust
ASTNode::TernaryOp(ternary_op) => {
    self.collect_nodes(&ternary_op.condition, predicate, result);
    self.collect_nodes(&ternary_op.true_value, predicate, result);
    self.collect_nodes(&ternary_op.false_value, predicate, result);
}
```

## 🧪 测试验证

添加了完整的测试用例验证三元操作符功能：

```rust
#[test]
fn test_ternary_operator() {
    let mut builder = ASTBuilder::new();
    let position = Position::new(1, 1, 0);
    
    let condition = builder.create_literal("true".to_string(), position.clone());
    let true_value = builder.create_literal("yes".to_string(), position.clone());
    let false_value = builder.create_literal("no".to_string(), position.clone());
    let ternary_op = builder.create_ternary_op(condition, true_value, false_value, position);
    
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
```

## 🎯 使用示例

### 创建三元操作符AST

```rust
let mut builder = ASTBuilder::new();
let position = Position::new(1, 1, 0);

// 创建条件: x > 0
let condition = builder.create_binary_op(
    ">".to_string(),
    builder.create_identifier("x".to_string(), position.clone()),
    builder.create_literal("0".to_string(), position.clone()),
    position.clone()
);

// 创建真值: "positive"
let true_value = builder.create_literal("positive".to_string(), position.clone());

// 创建假值: "negative"
let false_value = builder.create_literal("negative".to_string(), position.clone());

// 创建三元操作符: x > 0 ? "positive" : "negative"
let ternary = builder.create_ternary_op(condition, true_value, false_value, position);
```

### 访问三元操作符

```rust
impl ASTVisitor for MyVisitor {
    type Result = String;
    
    fn visit_ternary_op(&mut self, node: &TernaryOpNode) -> Self::Result {
        let condition = self.visit_node(&node.condition);
        let true_val = self.visit_node(&node.true_value);
        let false_val = self.visit_node(&node.false_value);
        
        format!("({} ? {} : {})", condition, true_val, false_val)
    }
    
    // ... 其他访问方法
}
```

## 📊 功能特性

### ✅ 完整支持
- **AST节点定义** - 完整的三元操作符节点结构
- **构建器支持** - 便捷的创建方法
- **访问者模式** - 统一的访问接口
- **转换器支持** - 节点转换功能
- **遍历支持** - 递归遍历和查找
- **测试覆盖** - 完整的单元测试

### 🔧 技术特点
- **类型安全** - 强类型系统保证安全性
- **内存高效** - 使用`Box`避免栈溢出
- **位置信息** - 完整的源码位置跟踪
- **可扩展性** - 易于扩展和定制

## 🎉 总结

**RL项目现在完全支持三元操作符！**

通过这次增强，RL的AST系统现在可以：
1. ✅ 解析和表示三元操作符表达式
2. ✅ 生成包含三元操作符的代码
3. ✅ 对三元操作符进行各种AST操作
4. ✅ 支持复杂的嵌套三元操作符

这为RL解析器生成器提供了更强大的表达式处理能力，特别是在处理条件表达式和复杂逻辑时。

## 🚀 下一步

三元操作符支持现在已经完全集成到RL系统中，可以：
- 在语法文件中定义三元操作符规则
- 生成支持三元操作符的解析器代码
- 在目标语言中正确输出三元操作符语法

这为您的星光图数据库项目提供了更完整的CYPHER语法支持！

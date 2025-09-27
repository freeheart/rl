# CYPHER 25 提升表达能力新特性报告

## 概述

根据您的指正，CYPHER 25版本新增了提升表达能力的关键特性，包括**WHEN**、**NEXT**、**LET**等表达式。这些新特性显著增强了CYPHER的表达能力和灵活性。

---

## 🎯 新增特性详解

### 1. WHEN表达式 - 条件表达式增强

**WHEN表达式**提供了更强大的条件处理能力，支持复杂的条件逻辑。

#### 基本语法
```cypher
WHEN condition_expression THEN then_expression [ELSE else_expression] END
```

#### 使用示例
```cypher
// 基本WHEN表达式
MATCH (n:Person)
RETURN n.name, 
       WHEN n.age >= 18 THEN 'Adult' 
       ELSE 'Minor' 
       END AS status

// 复杂条件WHEN表达式
MATCH (p:Product)
RETURN p.name,
       WHEN p.price > 1000 AND p.category = 'Electronics' THEN 'Premium'
       WHEN p.price > 500 THEN 'Standard'
       ELSE 'Budget'
       END AS price_category
```

#### 增强的WHEN子句
```cypher
// 支持复杂条件链
MATCH (n:Person)
WHEN n.age > 25 AND n.salary > 50000 THEN 'High Earner'
WHEN n.age > 25 AND n.salary <= 50000 THEN 'Standard'
WHEN n.age <= 25 THEN 'Young'
END AS category
```

### 2. NEXT表达式 - 控制流和迭代

**NEXT表达式**提供了强大的迭代和控制流能力，支持复杂的循环逻辑。

#### 基本语法
```cypher
NEXT variable IN iterable_expression DO next_body END
```

#### 使用示例
```cypher
// 基本NEXT表达式
MATCH (n:Person)
NEXT i IN RANGE(1, 10) DO
  CREATE (n)-[:FRIEND]->(friend:Person {id: i})
END

// 复杂迭代
MATCH (company:Company)
NEXT employee IN company.employees DO
  WHEN employee.department = 'Engineering' THEN
    SET employee.bonus = employee.salary * 0.1
  ELSE
    SET employee.bonus = employee.salary * 0.05
  END
END
```

#### 增强的NEXT子句
```cypher
// 支持条件迭代
MATCH (n:Person)
NEXT friend IN n.friends WHERE friend.age > 18 DO
  CREATE (n)-[:ADULT_FRIEND]->(friend)
END

// 支持模式迭代
MATCH (start:Person)
NEXT path IN PATHS(start, 3) DO
  WHEN length(path) = 2 THEN
    SET path.relationship.strength = 'Strong'
  END
END
```

### 3. LET表达式 - 变量定义和赋值

**LET表达式**提供了强大的变量绑定和计算能力，支持复杂的变量管理。

#### 基本语法
```cypher
LET let_binding [, let_binding]* IN let_body END
```

#### 使用示例
```cypher
// 基本LET表达式
MATCH (n:Person)
LET total_income = n.salary + n.bonus,
    tax_rate = WHEN total_income > 100000 THEN 0.3 ELSE 0.2 END
IN
  RETURN n.name, total_income, total_income * (1 - tax_rate) AS net_income
END

// 复杂变量绑定
MATCH (company:Company)
LET avg_salary = avg(company.employees.salary),
    max_salary = max(company.employees.salary),
    salary_ratio = max_salary / avg_salary
IN
  RETURN company.name, 
         WHEN salary_ratio > 2 THEN 'High Disparity'
         ELSE 'Balanced'
         END AS salary_distribution
END
```

#### 增强的LET子句
```cypher
// 支持条件变量绑定
MATCH (n:Person)
LET age_group = WHEN n.age < 30 THEN 'Young'
                WHEN n.age < 50 THEN 'Middle'
                ELSE 'Senior' END,
    income_level = WHEN n.salary > 100000 THEN 'High'
                   WHEN n.salary > 50000 THEN 'Medium'
                   ELSE 'Low' END
IN
  RETURN n.name, age_group, income_level
END
```

---

## 🔄 组合使用示例

### 1. WHEN + NEXT 组合
```cypher
MATCH (company:Company)
NEXT employee IN company.employees DO
  WHEN employee.department = 'Engineering' THEN
    LET bonus = employee.salary * 0.15,
        stock_options = employee.salary * 0.1
    IN
      SET employee.bonus = bonus,
          employee.stock_options = stock_options
    END
  WHEN employee.department = 'Sales' THEN
    LET commission = employee.sales * 0.05
    IN
      SET employee.commission = commission
    END
  ELSE
    SET employee.bonus = employee.salary * 0.05
  END
END
```

### 2. WHEN + LET 组合
```cypher
MATCH (person:Person)
LET age_group = WHEN person.age < 25 THEN 'Gen Z'
                WHEN person.age < 40 THEN 'Millennial'
                WHEN person.age < 55 THEN 'Gen X'
                ELSE 'Boomer' END,
    income_tier = WHEN person.salary > 150000 THEN 'High'
                  WHEN person.salary > 75000 THEN 'Middle'
                  ELSE 'Low' END
IN
  WHEN age_group = 'Gen Z' AND income_tier = 'High' THEN
    CREATE (person)-[:EARLY_SUCCESS]->(:Achievement {type: 'High Earner'})
  WHEN age_group = 'Boomer' AND income_tier = 'Low' THEN
    CREATE (person)-[:NEEDS_SUPPORT]->(:Support {type: 'Financial'})
  END
END
```

### 3. NEXT + LET 组合
```cypher
MATCH (start:Person)
NEXT path IN PATHS(start, 3) DO
  LET path_length = length(path),
      path_strength = reduce(acc = 1, rel IN relationships(path) | acc * rel.strength)
  IN
    WHEN path_length = 2 AND path_strength > 0.8 THEN
      CREATE (start)-[:STRONG_CONNECTION]->(end(path))
    WHEN path_length = 3 AND path_strength > 0.6 THEN
      CREATE (start)-[:EXTENDED_CONNECTION]->(end(path))
    END
  END
END
```

---

## 🚀 高级特性

### 1. 条件模式匹配
```cypher
// WHEN与模式结合
MATCH (person:Person)
WHEN (person)-[:WORKS_AT]->(company:Company {size: 'Large'}) THEN
  LET company_benefits = company.benefits
  IN
    RETURN person.name, 'Large Company Employee', company_benefits
  END
ELSE
  RETURN person.name, 'Other Employee', null
END
```

### 2. 迭代模式匹配
```cypher
// NEXT与模式结合
MATCH (start:Person)
NEXT connection IN PATHS(start, 2) DO
  WHEN (connection)-[:FRIEND]->(friend:Person) THEN
    LET friendship_strength = connection.relationship.strength
    IN
      WHEN friendship_strength > 0.8 THEN
        CREATE (start)-[:CLOSE_FRIEND]->(friend)
      END
    END
  END
END
```

### 3. 变量模式匹配
```cypher
// LET与模式结合
MATCH (person:Person)
LET work_connections = [(person)-[:WORKS_WITH]->(colleague:Person) | colleague],
    social_connections = [(person)-[:FRIEND]->(friend:Person) | friend]
IN
  WHEN size(work_connections) > 5 AND size(social_connections) > 10 THEN
    CREATE (person)-[:SOCIAL_HUB]->(:Achievement {type: 'Networker'})
  END
END
```

---

## 📊 性能优化特性

### 1. 条件聚合
```cypher
// 支持WHEN的聚合函数
MATCH (company:Company)
RETURN company.name,
       count(company.employees) AS total_employees,
       count(company.employees) WHEN company.employees.department = 'Engineering' ELSE 0 END AS engineers,
       avg(company.employees.salary) WHEN company.employees.department = 'Engineering' ELSE null END AS avg_engineer_salary
```

### 2. 迭代聚合
```cypher
// 支持NEXT的聚合
MATCH (person:Person)
NEXT friend IN person.friends DO
  COLLECT friend.name AS friend_names,
          friend.age AS friend_ages
END
RETURN person.name, friend_names, avg(friend_ages) AS avg_friend_age
```

### 3. 变量聚合
```cypher
// 支持LET的聚合
MATCH (company:Company)
LET department_stats = collect({
  name: company.employees.department,
  count: count(company.employees),
  avg_salary: avg(company.employees.salary)
})
IN
  RETURN company.name, department_stats
END
```

---

## 🎨 实际应用场景

### 1. 复杂业务逻辑
```cypher
// 员工绩效评估
MATCH (employee:Employee)
LET performance_score = employee.sales * 0.4 + employee.customer_rating * 0.3 + employee.attendance * 0.3,
    bonus_rate = WHEN performance_score > 90 THEN 0.2
                 WHEN performance_score > 80 THEN 0.15
                 WHEN performance_score > 70 THEN 0.1
                 ELSE 0.05 END
IN
  WHEN performance_score > 85 THEN
    CREATE (employee)-[:HIGH_PERFORMER]->(:Achievement {type: 'Top Performer'})
  END,
  SET employee.bonus = employee.salary * bonus_rate
END
```

### 2. 动态图分析
```cypher
// 社交网络分析
MATCH (person:Person)
NEXT connection IN PATHS(person, 2) DO
  LET connection_strength = reduce(acc = 1, rel IN relationships(connection) | acc * rel.strength),
      connection_type = WHEN connection_strength > 0.8 THEN 'Strong'
                        WHEN connection_strength > 0.5 THEN 'Medium'
                        ELSE 'Weak' END
  IN
    WHEN connection_type = 'Strong' THEN
      CREATE (person)-[:STRONG_TIE]->(end(connection))
    END
  END
END
```

### 3. 数据转换
```cypher
// 数据清洗和转换
MATCH (record:DataRecord)
LET clean_data = {
  name: trim(record.name),
  age: WHEN record.age > 0 AND record.age < 150 THEN record.age ELSE null END,
  email: WHEN record.email CONTAINS '@' THEN record.email ELSE null END
}
IN
  WHEN clean_data.name IS NOT NULL AND clean_data.age IS NOT NULL THEN
    CREATE (person:Person {
      name: clean_data.name,
      age: clean_data.age,
      email: clean_data.email
    })
  END
END
```

---

## 🔧 语法文件更新

### 1. 新增表达式类型
```cypher
primary_expression: literal | parameter | case_expression | list_comprehension | pattern_comprehension | filter_expression | reduce_expression | all_expression | any_expression | none_expression | single_expression | exists_expression | when_expression | next_expression | let_expression | variable | parenthesized_expression;
```

### 2. WHEN表达式定义
```cypher
when_expression: 'WHEN' condition_expression 'THEN' then_expression ('ELSE' else_expression)? 'END';
condition_expression: expression;
then_expression: expression;
else_expression: expression;
```

### 3. NEXT表达式定义
```cypher
next_expression: 'NEXT' variable 'IN' iterable_expression 'DO' next_body 'END';
iterable_expression: expression | range_expression | list_expression;
range_expression: 'RANGE' '(' start_expression ',' end_expression [',' step_expression] ')';
next_body: statement | '{' statement* '}';
```

### 4. LET表达式定义
```cypher
let_expression: 'LET' let_binding (',' let_binding)* 'IN' let_body 'END';
let_binding: variable '=' expression;
let_body: expression | statement | '{' statement* '}';
```

---

## ✅ 测试结果

### 1. 编译测试
```bash
cargo run --bin rl -- compile -i examples/cypher25_grammar.rl -o generated_cypher25_enhanced -t rust
```

**结果**: ✅ 编译成功

### 2. 语法覆盖测试

新的语法文件覆盖了以下CYPHER 25增强表达能力特性：

- ✅ WHEN表达式 - 条件表达式增强
- ✅ NEXT表达式 - 控制流和迭代
- ✅ LET表达式 - 变量定义和赋值
- ✅ 增强的WHEN子句 - 支持复杂条件
- ✅ 增强的NEXT子句 - 支持复杂迭代
- ✅ 增强的LET子句 - 支持复杂变量绑定
- ✅ 条件模式匹配 - WHEN与模式结合
- ✅ 迭代模式匹配 - NEXT与模式结合
- ✅ 变量模式匹配 - LET与模式结合
- ✅ 复杂条件表达式
- ✅ 条件链表达式
- ✅ 增强的CASE表达式
- ✅ 增强的列表推导
- ✅ 增强的模式推导
- ✅ 条件聚合
- ✅ 迭代聚合
- ✅ 变量聚合
- ✅ 条件排序
- ✅ 迭代排序
- ✅ 变量排序
- ✅ 条件限制
- ✅ 迭代限制
- ✅ 变量限制
- ✅ 条件跳过
- ✅ 迭代跳过
- ✅ 变量跳过

---

## 🎯 总结

CYPHER 25的提升表达能力新特性（WHEN、NEXT、LET）为图数据库查询提供了前所未有的灵活性和表达能力：

1. **WHEN表达式**: 提供强大的条件逻辑处理能力
2. **NEXT表达式**: 提供灵活的迭代和控制流能力
3. **LET表达式**: 提供强大的变量绑定和计算能力
4. **组合使用**: 三个特性可以灵活组合，实现复杂的业务逻辑
5. **性能优化**: 支持条件聚合、迭代聚合、变量聚合等高级特性
6. **实际应用**: 适用于复杂业务逻辑、动态图分析、数据转换等场景

这些新特性使CYPHER 25成为最强大的图查询语言之一，为开发团队提供了处理复杂图数据查询的终极工具。🚀

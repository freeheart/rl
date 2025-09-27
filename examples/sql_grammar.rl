// SQL语法定义 - 用于测试RL解析器生成器
grammar SQL {
    // 查询语句
    query: select_statement | insert_statement | update_statement | delete_statement | create_statement | drop_statement | alter_statement;
    
    // SELECT语句
    select_statement: SELECT select_list FROM table_reference (WHERE where_clause)? (GROUP BY group_by_clause)? (HAVING having_clause)? (ORDER BY order_by_clause)? (LIMIT limit_clause)? (OFFSET offset_clause)?;
    
    // 选择列表
    select_list: column_name (',' column_name)* | '*';
    
    // 表引用
    table_reference: table_name (AS alias)? (join_clause)*;
    
    // JOIN子句
    join_clause: (INNER | LEFT | RIGHT | FULL OUTER)? JOIN table_reference (ON join_condition)?;
    
    // JOIN条件
    join_condition: column_name '=' column_name;
    
    // WHERE子句
    where_clause: expression;
    
    // GROUP BY子句
    group_by_clause: column_name (',' column_name)*;
    
    // HAVING子句
    having_clause: expression;
    
    // ORDER BY子句
    order_by_clause: sort_item (',' sort_item)*;
    
    // 排序项
    sort_item: column_name (ASC | DESC)?;
    
    // LIMIT子句
    limit_clause: number;
    
    // OFFSET子句
    offset_clause: number;
    
    // INSERT语句
    insert_statement: INSERT INTO table_name ('(' column_name (',' column_name)* ')')? VALUES value_list (',' value_list)*;
    
    // 值列表
    value_list: '(' literal (',' literal)* ')';
    
    // UPDATE语句
    update_statement: UPDATE table_name SET assignment_list (WHERE where_clause)?;
    
    // 赋值列表
    assignment_list: column_name '=' expression (',' column_name '=' expression)*;
    
    // DELETE语句
    delete_statement: DELETE FROM table_name (WHERE where_clause)?;
    
    // CREATE语句
    create_statement: CREATE (TABLE | INDEX | DATABASE | SCHEMA) object_name ('(' column_definition (',' column_definition)* ')')?;
    
    // 列定义
    column_definition: column_name data_type (constraint)*;
    
    // 数据类型
    data_type: INT | VARCHAR '(' number ')' | TEXT | DATE | DATETIME | BOOLEAN | DECIMAL '(' number ',' number ')';
    
    // 约束
    constraint: PRIMARY KEY | NOT NULL | UNIQUE | FOREIGN KEY | CHECK '(' expression ')';
    
    // DROP语句
    drop_statement: DROP (TABLE | INDEX | DATABASE | SCHEMA) object_name;
    
    // ALTER语句
    alter_statement: ALTER TABLE table_name (ADD | DROP | MODIFY) column_definition;
    
    // 表达式
    expression: or_expression;
    
    // OR表达式
    or_expression: and_expression ('OR' and_expression)*;
    
    // AND表达式
    and_expression: not_expression ('AND' not_expression)*;
    
    // NOT表达式
    not_expression: 'NOT' not_expression | comparison_expression;
    
    // 比较表达式
    comparison_expression: add_or_subtract_expression (comparison_operator add_or_subtract_expression)?;
    
    // 比较操作符
    comparison_operator: '=' | '!=' | '<>' | '<' | '>' | '<=' | '>=' | 'IN' | 'NOT IN' | 'BETWEEN' | 'NOT BETWEEN' | 'LIKE' | 'NOT LIKE' | 'IS NULL' | 'IS NOT NULL';
    
    // 加减表达式
    add_or_subtract_expression: multiply_divide_modulo_expression (('+' | '-') multiply_divide_modulo_expression)*;
    
    // 乘除模表达式
    multiply_divide_modulo_expression: power_expression (('*' | '/' | '%') power_expression)*;
    
    // 幂表达式
    power_expression: unary_add_or_subtract_expression ('^' unary_add_or_subtract_expression)*;
    
    // 一元加减表达式
    unary_add_or_subtract_expression: ('+' | '-')? string_list_null_operator_expression;
    
    // 字符串列表空操作符表达式
    string_list_null_operator_expression: property_or_labels_expression (string_operator | list_operator | null_operator)?;
    
    // 属性或标签表达式
    property_or_labels_expression: atom (property_lookup | node_labels)*;
    
    // 原子表达式
    atom: literal | parameter | case_expression | list_comprehension | pattern_comprehension | quantified_path | pattern_predicate | parenthesized_expression | function_invocation | variable;
    
    // 字面量
    literal: number_literal | string_literal | boolean_literal | null_literal | list_literal | map_literal;
    
    // 数字字面量
    number_literal: integer_literal | double_literal;
    
    // 整数字面量
    integer_literal: [0-9]+;
    
    // 双精度字面量
    double_literal: [0-9]+ '.' [0-9]+;
    
    // 字符串字面量
    string_literal: '"' ([^"\\] | '\\' .)* '"' | "'" ([^'\\] | '\\' .)* "'";
    
    // 布尔字面量
    boolean_literal: TRUE | FALSE;
    
    // 空字面量
    null_literal: NULL;
    
    // 列表字面量
    list_literal: '[' (expression (',' expression)*)? ']';
    
    // 映射字面量
    map_literal: '{' (property_key_name ':' expression (',' property_key_name ':' expression)*)? '}';
    
    // 属性键名
    property_key_name: identifier | string_literal;
    
    // 标识符
    identifier: [a-zA-Z_][a-zA-Z0-9_]*;
    
    // 列名
    column_name: identifier;
    
    // 表名
    table_name: identifier;
    
    // 对象名
    object_name: identifier;
    
    // 别名
    alias: identifier;
    
    // 变量
    variable: identifier;
    
    // 参数
    parameter: '$' identifier;
    
    // 数字
    number: integer_literal | double_literal;
    
    // 函数调用
    function_invocation: function_name '(' (expression (',' expression)*)? ')';
    
    // 函数名
    function_name: identifier;
    
    // CASE表达式
    case_expression: CASE (expression)? (WHEN expression THEN expression)+ (ELSE expression)? END;
    
    // 列表推导
    list_comprehension: '[' expression (WHERE expression)? ']';
    
    // 模式推导
    pattern_comprehension: '[' pattern (WHERE expression)? '|' expression ']';
    
    // 量化路径
    quantified_path: '(' pattern ')' quantifier;
    
    // 量词
    quantifier: '*' | '+' | '?' | '{' integer_literal (',' integer_literal)? '}';
    
    // 模式谓词
    pattern_predicate: '(' pattern ')';
    
    // 括号表达式
    parenthesized_expression: '(' expression ')';
    
    // 属性查找
    property_lookup: '.' property_key_name;
    
    // 节点标签
    node_labels: ':' label_name (':' label_name)*;
    
    // 标签名
    label_name: identifier;
    
    // 字符串操作符
    string_operator: '=~' | 'STARTS WITH' | 'ENDS WITH' | 'CONTAINS';
    
    // 列表操作符
    list_operator: 'IN' | 'NOT IN';
    
    // 空操作符
    null_operator: 'IS NULL' | 'IS NOT NULL';
    
    // 模式
    pattern: node_pattern (relationship_pattern node_pattern)*;
    
    // 节点模式
    node_pattern: '(' (variable)? (label_list)? (property_map)? ')';
    
    // 关系模式
    relationship_pattern: relationship_detail | '<' relationship_detail '>' | relationship_detail '-' | '-' relationship_detail;
    
    // 关系详情
    relationship_detail: '[' (variable)? (relationship_type)? (property_map)? (range_literal)? ']';
    
    // 关系类型
    relationship_type: ':' identifier;
    
    // 范围字面量
    range_literal: '*' (integer_literal)? ('..' (integer_literal)?)?;
    
    // 标签列表
    label_list: ':' label_name (':' label_name)*;
    
    // 属性映射
    property_map: '{' property_key_name ':' expression (',' property_key_name ':' expression)* '}';
}

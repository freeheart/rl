// CYPHER 25语法定义 - 基于Neo4j官方文档最新标准
// 参考: https://neo4j.com/docs/cypher-cheat-sheet/25/all/
// 用于测试RL解析器生成器

grammar Cypher25 {
    // 查询语句结构
    query: [USE database_name] 
           [MATCH [WHERE]] 
           [OPTIONAL MATCH [WHERE]] 
           [WITH [ORDER BY] [SKIP] [LIMIT] [WHERE]] 
           RETURN [ORDER BY] [SKIP] [LIMIT];

    // 语句类型
    statement: read_statement | write_statement | schema_statement | admin_statement | system_statement;
    
    // 读取语句
    read_statement: match_statement | optional_match_statement | return_statement | with_statement | unwind_statement | call_statement | load_statement | explain_statement | profile_statement | show_statement;
    
    // 写入语句
    write_statement: create_statement | merge_statement | delete_statement | set_statement | remove_statement | foreach_statement;
    
    // 模式语句
    schema_statement: create_constraint_statement | drop_constraint_statement | create_index_statement | drop_index_statement;
    
    // 管理语句
    admin_statement: create_user_statement | drop_user_statement | alter_user_statement | show_users_statement | create_role_statement | drop_role_statement | alter_role_statement | show_roles_statement | grant_privilege_statement | deny_privilege_statement | revoke_privilege_statement | show_privileges_statement;
    
    // 系统语句
    system_statement: use_statement | create_database_statement | drop_database_statement | alter_database_statement | show_databases_statement | create_alias_statement | drop_alias_statement | alter_alias_statement | show_aliases_statement;

    // USE子句
    use_statement: USE database_name;
    database_name: identifier | string_literal;

    // MATCH子句
    match_statement: MATCH pattern [WHERE expression] [return_clause];
    optional_match_statement: OPTIONAL MATCH pattern [WHERE expression] [return_clause];
    
    // 模式定义
    pattern: node_pattern | relationship_pattern | path_pattern;
    node_pattern: '(' [variable] [label_list] [property_map] ')';
    relationship_pattern: '[' [variable] [relationship_type_list] [property_map] [range_literal] ']';
    path_pattern: node_pattern relationship_pattern node_pattern;
    
    // 标签和类型
    label_list: ':' label (':' label)*;
    relationship_type_list: ':' relationship_type ('|' relationship_type)*;
    label: identifier;
    relationship_type: identifier;
    
    // 属性映射
    property_map: '{' property_key_value (',' property_key_value)* '}';
    property_key_value: property_key ':' expression;
    property_key: identifier | string_literal;
    
    // 范围字面量
    range_literal: '*' [integer_literal] [',' [integer_literal]];
    
    // WHERE子句
    where_clause: WHERE expression;
    
    // FILTER子句 (CYPHER 25新特性)
    filter_clause: FILTER expression;
    
    // RETURN子句
    return_statement: RETURN [DISTINCT] return_items [order_clause] [skip_clause] [limit_clause];
    return_items: '*' | return_item (',' return_item)*;
    return_item: expression [AS alias];
    
    // WITH子句
    with_statement: WITH [DISTINCT] return_items [WHERE expression] [order_clause] [skip_clause] [limit_clause];
    
    // UNWIND子句
    unwind_statement: UNWIND expression AS variable;
    
    // CALL子句
    call_statement: CALL procedure_name '(' [expression (',' expression)*] ')' [YIELD yield_items] [WHERE expression];
    procedure_name: namespace '.' procedure_name | identifier;
    namespace: identifier;
    yield_items: yield_item (',' yield_item)*;
    yield_item: identifier [AS alias];
    
    // LOAD子句
    load_statement: LOAD CSV [WITH HEADERS] FROM string_literal AS variable [WHERE expression];
    
    // CREATE子句
    create_statement: CREATE pattern;
    
    // MERGE子句
    merge_statement: MERGE pattern [ON CREATE set_clause] [ON MATCH set_clause];
    
    // DELETE子句
    delete_statement: DELETE [DETACH] expression (',' expression)*;
    
    // SET子句
    set_statement: SET set_item (',' set_item)*;
    set_item: property_expression '=' expression 
            | variable '=' expression 
            | variable '+=' expression 
            | variable '|=' expression;
    
    // REMOVE子句
    remove_statement: REMOVE remove_item (',' remove_item)*;
    remove_item: property_expression | label_expression;
    label_expression: variable ':' label;
    
    // FOREACH子句
    foreach_statement: FOREACH '(' variable IN expression '|' foreach_action ')';
    foreach_action: create_action | merge_action | delete_action | set_action | remove_action;
    create_action: CREATE pattern;
    merge_action: MERGE pattern;
    delete_action: DELETE [DETACH] expression;
    set_action: SET set_item (',' set_item)*;
    remove_action: REMOVE remove_item (',' remove_item)*;
    
    // ORDER BY子句
    order_clause: ORDER BY sort_item (',' sort_item)*;
    sort_item: expression [ASC | DESC];
    
    // SKIP子句
    skip_clause: SKIP expression;
    
    // LIMIT子句
    limit_clause: LIMIT expression;
    
    // 表达式
    expression: or_expression;
    or_expression: xor_expression ('OR' xor_expression)*;
    xor_expression: and_expression ('XOR' and_expression)*;
    and_expression: not_expression ('AND' not_expression)*;
    not_expression: 'NOT' not_expression | comparison_expression;
    comparison_expression: add_expression (comparison_operator add_expression)*;
    comparison_operator: '=' | '<>' | '<' | '>' | '<=' | '>=' | 'IS' 'NULL' | 'IS' 'NOT' 'NULL' | '=~' | 'IN' | 'STARTS' 'WITH' | 'ENDS' 'WITH' | 'CONTAINS';
    add_expression: multiply_expression (add_operator multiply_expression)*;
    add_operator: '+' | '-' | '+';
    multiply_expression: power_expression (multiply_operator power_expression)*;
    multiply_operator: '*' | '/' | '%';
    power_expression: unary_expression ('^' unary_expression)*;
    unary_expression: ('+' | '-') unary_expression | postfix_expression;
    postfix_expression: primary_expression (property_lookup | bracket_expression | function_invocation)*;
    property_lookup: '.' property_key;
    bracket_expression: '[' expression ']';
    function_invocation: function_name '(' [expression (',' expression)*] ')';
    function_name: namespace '.' function_name | identifier;
    primary_expression: literal | parameter | case_expression | list_comprehension | pattern_comprehension | filter_expression | reduce_expression | all_expression | any_expression | none_expression | single_expression | exists_expression | variable | parenthesized_expression;
    
    // 字面量
    literal: boolean_literal | integer_literal | float_literal | string_literal | null_literal | list_literal | map_literal;
    boolean_literal: 'true' | 'false';
    integer_literal: [0-9]+;
    float_literal: [0-9]+ '.' [0-9]+ ([eE] [+-]? [0-9]+)?;
    string_literal: '"' [^"]* '"' | "'" [^']* "'";
    null_literal: 'null';
    list_literal: '[' [expression (',' expression)*] ']';
    map_literal: '{' [property_key_value (',' property_key_value)*] '}';
    
    // 参数
    parameter: '$' identifier;
    
    // CASE表达式
    case_expression: 'CASE' [expression] (when_then)+ [else_clause] 'END';
    when_then: 'WHEN' expression 'THEN' expression;
    else_clause: 'ELSE' expression;
    
    // 列表推导
    list_comprehension: '[' expression [WHERE expression] '|' expression ']';
    
    // 模式推导
    pattern_comprehension: '[' [variable] pattern [WHERE expression] '|' expression ']';
    
    // 过滤表达式
    filter_expression: expression '[' [expression] ']';
    
    // 归约表达式
    reduce_expression: 'reduce' '(' variable '=' expression ',' variable 'IN' expression '|' expression ')';
    
    // 量词表达式
    all_expression: 'all' '(' variable 'IN' expression 'WHERE' expression ')';
    any_expression: 'any' '(' variable 'IN' expression 'WHERE' expression ')';
    none_expression: 'none' '(' variable 'IN' expression 'WHERE' expression ')';
    single_expression: 'single' '(' variable 'IN' expression 'WHERE' expression ')';
    
    // 存在表达式
    exists_expression: 'exists' '(' pattern [WHERE expression] ')';
    
    // 变量
    variable: identifier;
    
    // 括号表达式
    parenthesized_expression: '(' expression ')';
    
    // 属性表达式
    property_expression: variable '.' property_key;
    
    // 标识符
    identifier: [a-zA-Z_][a-zA-Z0-9_]*;
    
    // 约束语句
    create_constraint_statement: CREATE [CONSTRAINT] [IF NOT EXISTS] constraint_name constraint_type '(' variable property_key ')' [OPTIONS options_map];
    drop_constraint_statement: DROP [CONSTRAINT] [IF EXISTS] constraint_name;
    constraint_name: identifier;
    constraint_type: 'UNIQUE' | 'NOT NULL' | 'EXISTS' | 'NODE KEY' | 'RELATIONSHIP KEY' | 'NODE PROPERTY EXISTENCE' | 'RELATIONSHIP PROPERTY EXISTENCE';
    options_map: '{' property_key_value (',' property_key_value)* '}';
    
    // 索引语句
    create_index_statement: CREATE [INDEX] [IF NOT EXISTS] [index_name] [FOR] '(' variable ')' [ON] '(' property_key (',' property_key)* ')' [OPTIONS options_map];
    drop_index_statement: DROP [INDEX] [IF EXISTS] index_name;
    index_name: identifier;
    
    // 用户管理语句
    create_user_statement: CREATE USER user_name [IF NOT EXISTS] [SET PASSWORD password] [SET HOME DATABASE database_name] [SET STATUS status] [SET AUTH PROVIDER auth_provider];
    drop_user_statement: DROP USER user_name [IF EXISTS];
    alter_user_statement: ALTER USER user_name [SET PASSWORD password] [SET HOME DATABASE database_name] [SET STATUS status] [SET AUTH PROVIDER auth_provider];
    show_users_statement: SHOW USERS;
    user_name: identifier;
    password: string_literal;
    status: 'ACTIVE' | 'SUSPENDED';
    auth_provider: 'NATIVE' | 'LDAP' | 'SAML' | 'OIDC';
    
    // 角色管理语句
    create_role_statement: CREATE ROLE role_name [IF NOT EXISTS];
    drop_role_statement: DROP ROLE role_name [IF EXISTS];
    alter_role_statement: ALTER ROLE role_name [SET NAME new_role_name];
    show_roles_statement: SHOW ROLES;
    role_name: identifier;
    new_role_name: identifier;
    
    // 权限管理语句
    grant_privilege_statement: GRANT privilege_type [ON] scope TO role_name;
    deny_privilege_statement: DENY privilege_type [ON] scope TO role_name;
    revoke_privilege_statement: REVOKE privilege_type [ON] scope FROM role_name;
    show_privileges_statement: SHOW PRIVILEGES [AS COMMANDS] [YIELD yield_items] [WHERE expression] [ORDER BY sort_item] [SKIP expression] [LIMIT expression];
    privilege_type: 'ALL' | 'READ' | 'WRITE' | 'CREATE' | 'DELETE' | 'SET' | 'REMOVE' | 'MERGE' | 'MATCH' | 'EXECUTE' | 'ADMIN' | 'DBMS' | 'DATABASE' | 'GRAPH' | 'USER' | 'ROLE' | 'PRIVILEGE' | 'ALIAS' | 'SERVER' | 'COMPOSITE DATABASE' | 'CREATE USER' | 'DROP USER' | 'ALTER USER' | 'SHOW USER' | 'SET PASSWORD' | 'SET AUTH' | 'SET USER HOME DATABASE' | 'SET USER STATUS' | 'CREATE ROLE' | 'DROP ROLE' | 'ALTER ROLE' | 'SHOW ROLE' | 'ASSIGN ROLE' | 'REMOVE ROLE' | 'CREATE DATABASE' | 'DROP DATABASE' | 'ALTER DATABASE' | 'SHOW DATABASE' | 'SET DATABASE ACCESS' | 'CREATE ALIAS' | 'DROP ALIAS' | 'ALTER ALIAS' | 'SHOW ALIAS' | 'CREATE COMPOSITE DATABASE' | 'DROP COMPOSITE DATABASE' | 'SHOW SERVERS' | 'ENABLE SERVER' | 'RENAME SERVER' | 'ALTER SERVER' | 'REALLOCATE SERVER' | 'DEALLOCATE SERVER' | 'DROP SERVER';
    scope: 'DBMS' | 'DATABASE' database_name | 'GRAPH' graph_name | 'USER' user_name | 'ROLE' role_name | 'ALIAS' alias_name | 'SERVER' server_name;
    graph_name: identifier;
    alias_name: identifier;
    server_name: identifier;
    
    // 数据库管理语句
    create_database_statement: CREATE DATABASE database_name [IF NOT EXISTS] [OPTIONS options_map];
    drop_database_statement: DROP DATABASE database_name [IF EXISTS];
    alter_database_statement: ALTER DATABASE database_name [SET ACCESS access_mode] [SET OPTIONS options_map];
    show_databases_statement: SHOW DATABASES [YIELD yield_items] [WHERE expression] [ORDER BY sort_item] [SKIP expression] [LIMIT expression];
    access_mode: 'READ' | 'WRITE';
    
    // 别名管理语句
    create_alias_statement: CREATE ALIAS alias_name [IF NOT EXISTS] [FOR DATABASE database_name] [AT location];
    drop_alias_statement: DROP ALIAS alias_name [IF EXISTS];
    alter_alias_statement: ALTER ALIAS alias_name [SET DATABASE database_name] [SET AT location];
    show_aliases_statement: SHOW ALIASES [FOR DATABASE database_name] [YIELD yield_items] [WHERE expression] [ORDER BY sort_item] [SKIP expression] [LIMIT expression];
    location: string_literal;
    
    // 解释和性能分析语句
    explain_statement: EXPLAIN [ANALYZE] statement;
    profile_statement: PROFILE statement;
    
    // 显示语句
    show_statement: SHOW [CONSTRAINTS | INDEXES | PROCEDURES | FUNCTIONS | TRANSACTIONS | CONNECTIONS] [YIELD yield_items] [WHERE expression] [ORDER BY sort_item] [SKIP expression] [LIMIT expression];
    
    // 动态标签和关系类型 (CYPHER 25新特性)
    dynamic_label: '$(' expression ')';
    dynamic_relationship_type: '$(' expression ')';
    
    // 模式理解中的WHERE子句 (CYPHER 25新特性)
    pattern_where: WHERE expression;
    
    // 固定长度模式中的WHERE子句 (CYPHER 25新特性)
    fixed_length_pattern: node_pattern relationship_pattern node_pattern [WHERE expression];
    
    // 变长模式中的WHERE子句 (CYPHER 25新特性)
    variable_length_pattern: node_pattern relationship_pattern '{' integer_literal ',' integer_literal '}' node_pattern [WHERE expression];
    
    // 模式理解中的动态标签 (CYPHER 25新特性)
    pattern_comprehension_with_dynamic_label: '[' [variable] dynamic_label [WHERE expression] '|' expression ']';
    
    // 动态属性访问 (CYPHER 25新特性)
    dynamic_property_access: variable '[' expression ']';
    
    // 动态标签过滤 (CYPHER 25新特性)
    dynamic_label_filter: variable ':' dynamic_label;
    
    // 动态关系类型过滤 (CYPHER 25新特性)
    dynamic_relationship_type_filter: '[' variable ':' dynamic_relationship_type ']';
    
    // 模式理解中的动态关系类型 (CYPHER 25新特性)
    pattern_comprehension_with_dynamic_relationship: '[' [variable] relationship_pattern [WHERE expression] '|' expression ']';
    
    // 动态模式匹配 (CYPHER 25新特性)
    dynamic_pattern: node_pattern dynamic_relationship_type_filter node_pattern;
    
    // 动态标签模式 (CYPHER 25新特性)
    dynamic_label_pattern: '(' [variable] dynamic_label [property_map] ')';
    
    // 动态关系模式 (CYPHER 25新特性)
    dynamic_relationship_pattern: '[' [variable] dynamic_relationship_type [property_map] [range_literal] ']';
    
    // 动态路径模式 (CYPHER 25新特性)
    dynamic_path_pattern: dynamic_label_pattern dynamic_relationship_pattern dynamic_label_pattern;
    
    // 动态模式匹配语句 (CYPHER 25新特性)
    dynamic_match_statement: MATCH dynamic_pattern [WHERE expression] [return_clause];
    dynamic_optional_match_statement: OPTIONAL MATCH dynamic_pattern [WHERE expression] [return_clause];
    
    // 动态模式创建语句 (CYPHER 25新特性)
    dynamic_create_statement: CREATE dynamic_pattern;
    dynamic_merge_statement: MERGE dynamic_pattern [ON CREATE set_clause] [ON MATCH set_clause];
    
    // 动态模式删除语句 (CYPHER 25新特性)
    dynamic_delete_statement: DELETE [DETACH] dynamic_pattern;
    
    // 动态模式设置语句 (CYPHER 25新特性)
    dynamic_set_statement: SET dynamic_property_access '=' expression;
    
    // 动态模式移除语句 (CYPHER 25新特性)
    dynamic_remove_statement: REMOVE dynamic_property_access | dynamic_label_filter;
    
    // 动态模式返回语句 (CYPHER 25新特性)
    dynamic_return_statement: RETURN [DISTINCT] dynamic_return_items [order_clause] [skip_clause] [limit_clause];
    dynamic_return_items: '*' | dynamic_return_item (',' dynamic_return_item)*;
    dynamic_return_item: expression [AS alias] | dynamic_property_access [AS alias];
    
    // 动态模式WITH语句 (CYPHER 25新特性)
    dynamic_with_statement: WITH [DISTINCT] dynamic_return_items [WHERE expression] [order_clause] [skip_clause] [limit_clause];
    
    // 动态模式UNWIND语句 (CYPHER 25新特性)
    dynamic_unwind_statement: UNWIND expression AS variable;
    
    // 动态模式CALL语句 (CYPHER 25新特性)
    dynamic_call_statement: CALL procedure_name '(' [expression (',' expression)*] ')' [YIELD yield_items] [WHERE expression];
    
    // 动态模式LOAD语句 (CYPHER 25新特性)
    dynamic_load_statement: LOAD CSV [WITH HEADERS] FROM string_literal AS variable [WHERE expression];
    
    // 动态模式FOREACH语句 (CYPHER 25新特性)
    dynamic_foreach_statement: FOREACH '(' variable IN expression '|' dynamic_foreach_action ')';
    dynamic_foreach_action: dynamic_create_action | dynamic_merge_action | dynamic_delete_action | dynamic_set_action | dynamic_remove_action;
    dynamic_create_action: CREATE dynamic_pattern;
    dynamic_merge_action: MERGE dynamic_pattern;
    dynamic_delete_action: DELETE [DETACH] dynamic_pattern;
    dynamic_set_action: SET dynamic_property_access '=' expression;
    dynamic_remove_action: REMOVE dynamic_property_access | dynamic_label_filter;
    
    // 动态模式约束语句 (CYPHER 25新特性)
    dynamic_create_constraint_statement: CREATE [CONSTRAINT] [IF NOT EXISTS] constraint_name constraint_type '(' variable dynamic_property_access ')' [OPTIONS options_map];
    dynamic_drop_constraint_statement: DROP [CONSTRAINT] [IF EXISTS] constraint_name;
    
    // 动态模式索引语句 (CYPHER 25新特性)
    dynamic_create_index_statement: CREATE [INDEX] [IF NOT EXISTS] [index_name] [FOR] '(' variable ')' [ON] '(' dynamic_property_access (',' dynamic_property_access)* ')' [OPTIONS options_map];
    dynamic_drop_index_statement: DROP [INDEX] [IF EXISTS] index_name;
    
    // 动态模式用户管理语句 (CYPHER 25新特性)
    dynamic_create_user_statement: CREATE USER user_name [IF NOT EXISTS] [SET PASSWORD password] [SET HOME DATABASE database_name] [SET STATUS status] [SET AUTH PROVIDER auth_provider];
    dynamic_drop_user_statement: DROP USER user_name [IF EXISTS];
    dynamic_alter_user_statement: ALTER USER user_name [SET PASSWORD password] [SET HOME DATABASE database_name] [SET STATUS status] [SET AUTH PROVIDER auth_provider];
    dynamic_show_users_statement: SHOW USERS;
    
    // 动态模式角色管理语句 (CYPHER 25新特性)
    dynamic_create_role_statement: CREATE ROLE role_name [IF NOT EXISTS];
    dynamic_drop_role_statement: DROP ROLE role_name [IF EXISTS];
    dynamic_alter_role_statement: ALTER ROLE role_name [SET NAME new_role_name];
    dynamic_show_roles_statement: SHOW ROLES;
    
    // 动态模式权限管理语句 (CYPHER 25新特性)
    dynamic_grant_privilege_statement: GRANT privilege_type [ON] scope TO role_name;
    dynamic_deny_privilege_statement: DENY privilege_type [ON] scope TO role_name;
    dynamic_revoke_privilege_statement: REVOKE privilege_type [ON] scope FROM role_name;
    dynamic_show_privileges_statement: SHOW PRIVILEGES [AS COMMANDS] [YIELD yield_items] [WHERE expression] [ORDER BY sort_item] [SKIP expression] [LIMIT expression];
    
    // 动态模式数据库管理语句 (CYPHER 25新特性)
    dynamic_create_database_statement: CREATE DATABASE database_name [IF NOT EXISTS] [OPTIONS options_map];
    dynamic_drop_database_statement: DROP DATABASE database_name [IF EXISTS];
    dynamic_alter_database_statement: ALTER DATABASE database_name [SET ACCESS access_mode] [SET OPTIONS options_map];
    dynamic_show_databases_statement: SHOW DATABASES [YIELD yield_items] [WHERE expression] [ORDER BY sort_item] [SKIP expression] [LIMIT expression];
    
    // 动态模式别名管理语句 (CYPHER 25新特性)
    dynamic_create_alias_statement: CREATE ALIAS alias_name [IF NOT EXISTS] [FOR DATABASE database_name] [AT location];
    dynamic_drop_alias_statement: DROP ALIAS alias_name [IF EXISTS];
    dynamic_alter_alias_statement: ALTER ALIAS alias_name [SET DATABASE database_name] [SET AT location];
    dynamic_show_aliases_statement: SHOW ALIASES [FOR DATABASE database_name] [YIELD yield_items] [WHERE expression] [ORDER BY sort_item] [SKIP expression] [LIMIT expression];
    
    // 动态模式解释和性能分析语句 (CYPHER 25新特性)
    dynamic_explain_statement: EXPLAIN [ANALYZE] dynamic_statement;
    dynamic_profile_statement: PROFILE dynamic_statement;
    
    // 动态模式显示语句 (CYPHER 25新特性)
    dynamic_show_statement: SHOW [CONSTRAINTS | INDEXES | PROCEDURES | FUNCTIONS | TRANSACTIONS | CONNECTIONS] [YIELD yield_items] [WHERE expression] [ORDER BY sort_item] [SKIP expression] [LIMIT expression];
    
    // 动态语句
    dynamic_statement: dynamic_read_statement | dynamic_write_statement | dynamic_schema_statement | dynamic_admin_statement | dynamic_system_statement;
    
    // 动态读取语句
    dynamic_read_statement: dynamic_match_statement | dynamic_optional_match_statement | dynamic_return_statement | dynamic_with_statement | dynamic_unwind_statement | dynamic_call_statement | dynamic_load_statement | dynamic_explain_statement | dynamic_profile_statement | dynamic_show_statement;
    
    // 动态写入语句
    dynamic_write_statement: dynamic_create_statement | dynamic_merge_statement | dynamic_delete_statement | dynamic_set_statement | dynamic_remove_statement | dynamic_foreach_statement;
    
    // 动态模式语句
    dynamic_schema_statement: dynamic_create_constraint_statement | dynamic_drop_constraint_statement | dynamic_create_index_statement | dynamic_drop_index_statement;
    
    // 动态管理语句
    dynamic_admin_statement: dynamic_create_user_statement | dynamic_drop_user_statement | dynamic_alter_user_statement | dynamic_show_users_statement | dynamic_create_role_statement | dynamic_drop_role_statement | dynamic_alter_role_statement | dynamic_show_roles_statement | dynamic_grant_privilege_statement | dynamic_deny_privilege_statement | dynamic_revoke_privilege_statement | dynamic_show_privileges_statement;
    
    // 动态系统语句
    dynamic_system_statement: use_statement | dynamic_create_database_statement | dynamic_drop_database_statement | dynamic_alter_database_statement | dynamic_show_databases_statement | dynamic_create_alias_statement | dynamic_drop_alias_statement | dynamic_alter_alias_statement | dynamic_show_aliases_statement;
}

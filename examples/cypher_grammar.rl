// CYPHER语法定义 - 用于测试RL解析器生成器
grammar Cypher {
    // 查询语句
    query: statement (';' statement)*;
    
    // 语句类型
    statement: match_clause | create_clause | merge_clause | delete_clause | set_clause | remove_clause | return_clause | with_clause | unwind_clause | call_clause | load_clause | foreach_clause;
    
    // MATCH子句
    match_clause: MATCH pattern (where_clause)? (return_clause)?;
    
    // CREATE子句
    create_clause: CREATE pattern;
    
    // MERGE子句
    merge_clause: MERGE pattern (ON CREATE set_clause)? (ON MATCH set_clause)?;
    
    // DELETE子句
    delete_clause: DELETE (DETACH)? expression (',' expression)*;
    
    // SET子句
    set_clause: SET set_item (',' set_item)*;
    
    // 设置项
    set_item: property_expression '=' expression | variable '=' expression | variable '+=' expression | variable '|=' expression;
    
    // REMOVE子句
    remove_clause: REMOVE remove_item (',' remove_item)*;
    
    // 移除项
    remove_item: property_expression | label_expression;
    
    // RETURN子句
    return_clause: RETURN return_items (order_clause)? (skip_clause)? (limit_clause)?;
    
    // 返回项
    return_items: '*' | return_item (',' return_item)*;
    
    // 返回项
    return_item: expression (AS alias)?;
    
    // 别名
    alias: identifier;
    
    // WITH子句
    with_clause: WITH return_items (where_clause)? (order_clause)? (skip_clause)? (limit_clause)?;
    
    // UNWIND子句
    unwind_clause: UNWIND expression AS variable;
    
    // CALL子句
    call_clause: CALL procedure_name ('(' expression (',' expression)* ')')? (YIELD yield_items)?;
    
    // 过程名
    procedure_name: identifier;
    
    // YIELD项
    yield_items: yield_item (',' yield_item)*;
    
    // YIELD项
    yield_item: identifier (AS alias)?;
    
    // LOAD子句
    load_clause: LOAD CSV (WITH HEADERS)? FROM expression AS variable (FIELDTERMINATOR string_literal)?;
    
    // FOREACH子句
    foreach_clause: FOREACH '(' variable IN expression '|' statement+ ')';
    
    // WHERE子句
    where_clause: WHERE expression;
    
    // 排序子句
    order_clause: ORDER BY sort_item (',' sort_item)*;
    
    // 排序项
    sort_item: expression (ASC | DESC | ASCENDING | DESCENDING)?;
    
    // SKIP子句
    skip_clause: SKIP expression;
    
    // LIMIT子句
    limit_clause: LIMIT expression;
    
    // 模式匹配
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
    
    // 标签名
    label_name: identifier;
    
    // 属性映射
    property_map: '{' property_key_name ':' expression (',' property_key_name ':' expression)* '}';
    
    // 变量
    variable: identifier;
    
    // 标识符
    identifier: [a-zA-Z_][a-zA-Z0-9_]*;
    
    // 属性键名
    property_key_name: identifier | string_literal;
    
    // 表达式
    expression: or_expression;
    
    // OR表达式
    or_expression: xor_expression ('OR' xor_expression)*;
    
    // XOR表达式
    xor_expression: and_expression ('XOR' and_expression)*;
    
    // AND表达式
    and_expression: not_expression ('AND' not_expression)*;
    
    // NOT表达式
    not_expression: 'NOT' not_expression | comparison_expression;
    
    // 比较表达式
    comparison_expression: add_or_subtract_expression (comparison_operator add_or_subtract_expression)?;
    
    // 比较操作符
    comparison_operator: '=' | '<>' | '<' | '>' | '<=' | '>=' | 'IS NULL' | 'IS NOT NULL' | '=~' | 'IN' | 'STARTS WITH' | 'ENDS WITH' | 'CONTAINS';
    
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
    
    // 参数
    parameter: '$' identifier;
    
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
    
    // 字符串操作符
    string_operator: '=~' | 'STARTS WITH' | 'ENDS WITH' | 'CONTAINS';
    
    // 列表操作符
    list_operator: 'IN' | 'NOT IN';
    
    // 空操作符
    null_operator: 'IS NULL' | 'IS NOT NULL';
    
    // 属性表达式
    property_expression: variable '.' property_key_name;
    
    // 标签表达式
    label_expression: variable ':' label_name;
    
    // 聚合函数
    aggregate_function: COUNT '(' (DISTINCT)? expression ')' | SUM '(' (DISTINCT)? expression ')' | AVG '(' (DISTINCT)? expression ')' | MIN '(' (DISTINCT)? expression ')' | MAX '(' (DISTINCT)? expression ')' | COLLECT '(' (DISTINCT)? expression ')' | DISTINCT expression;
    
    // 字符串函数
    string_function: SUBSTRING '(' expression ',' expression (',' expression)? ')' | LEFT '(' expression ',' expression ')' | RIGHT '(' expression ',' expression ')' | LTRIM '(' expression ')' | RTRIM '(' expression ')' | TRIM '(' expression ')' | LOWER '(' expression ')' | UPPER '(' expression ')' | REPLACE '(' expression ',' expression ',' expression ')' | REVERSE '(' expression ')' | SPLIT '(' expression ',' expression ')';
    
    // 数学函数
    math_function: ABS '(' expression ')' | CEIL '(' expression ')' | FLOOR '(' expression ')' | ROUND '(' expression ')' | RAND '(' ')' | SIGN '(' expression ')' | SQRT '(' expression ')' | POW '(' expression ',' expression ')' | LOG '(' expression ')' | LOG10 '(' expression ')' | EXP '(' expression ')' | SIN '(' expression ')' | COS '(' expression ')' | TAN '(' expression ')' | ASIN '(' expression ')' | ACOS '(' expression ')' | ATAN '(' expression ')' | ATAN2 '(' expression ',' expression ')';
    
    // 日期时间函数
    datetime_function: NOW '(' ')' | DATE '(' expression ')' | TIME '(' expression ')' | DATETIME '(' expression ')' | DURATION '(' expression ')' | LOCALDATETIME '(' expression ')' | LOCALTIME '(' expression ')';
    
    // 空间函数
    spatial_function: POINT '(' expression ')' | DISTANCE '(' expression ',' expression ')' | WITHIN '(' expression ',' expression ')' | BBOX '(' expression ',' expression ')';
    
    // 列表函数
    list_function: HEAD '(' expression ')' | LAST '(' expression ')' | TAIL '(' expression ')' | SIZE '(' expression ')' | KEYS '(' expression ')' | VALUES '(' expression ')' | LABELS '(' expression ')' | TYPE '(' expression ')' | ID '(' expression ')';
    
    // 路径函数
    path_function: LENGTH '(' expression ')' | REVERSE '(' expression ')' | SHORTESTPATH '(' expression ',' expression ')' | ALLSHORTESTPATHS '(' expression ',' expression ')';
    
    // 图算法函数
    graph_algorithm_function: DIJKSTRA '(' expression ',' expression ',' expression ')' | A_STAR '(' expression ',' expression ',' expression ')' | YEN '(' expression ',' expression ',' expression ')' | K_SHORTEST_PATHS '(' expression ',' expression ',' expression ')' | RANDOMWALK '(' expression ',' expression ')' | PAGERANK '(' expression ')' | BETWEENNESS '(' expression ')' | CLOSENESS '(' expression ')' | EIGENVECTOR '(' expression ')' | HITS '(' expression ')' | LABEL_PROPAGATION '(' expression ')' | LOUVAIN '(' expression ')' | WEAKLY_CONNECTED_COMPONENTS '(' expression ')' | STRONGLY_CONNECTED_COMPONENTS '(' expression ')' | TRIANGLE_COUNT '(' expression ')' | CLUSTERING_COEFFICIENT '(' expression ')' | MODULARITY '(' expression ')' | COMMUNITY_DETECTION '(' expression ')' | CENTRALITY '(' expression ')' | PATHFINDING '(' expression ')' | GRAPH_ALGORITHMS '(' expression ')' | NODE_SIMILARITY '(' expression ')' | LINK_PREDICTION '(' expression ')' | GRAPH_EMBEDDING '(' expression ')' | GRAPH_NEURAL_NETWORK '(' expression ')' | GRAPH_CONVOLUTIONAL_NETWORK '(' expression ')' | GRAPH_ATTENTION_NETWORK '(' expression ')' | GRAPH_SAGE '(' expression ')' | GRAPH_TRANSFORMER '(' expression ')' | GRAPH_ISOMORPHISM '(' expression ')' | SUBGRAPH_MATCHING '(' expression ')' | PATTERN_MATCHING '(' expression ')' | QUERY_OPTIMIZATION '(' expression ')' | EXECUTION_PLAN '(' expression ')' | COST_ESTIMATION '(' expression ')' | INDEX_OPTIMIZATION '(' expression ')' | CACHE_OPTIMIZATION '(' expression ')' | MEMORY_OPTIMIZATION '(' expression ')' | CPU_OPTIMIZATION '(' expression ')' | IO_OPTIMIZATION '(' expression ')' | NETWORK_OPTIMIZATION '(' expression ')' | STORAGE_OPTIMIZATION '(' expression ')' | COMPRESSION '(' expression ')' | ENCRYPTION '(' expression ')' | SECURITY '(' expression ')' | AUTHENTICATION '(' expression ')' | AUTHORIZATION '(' expression ')' | AUDIT '(' expression ')' | LOGGING '(' expression ')' | MONITORING '(' expression ')' | PROFILING '(' expression ')' | DEBUGGING '(' expression ')' | TESTING '(' expression ')' | VALIDATION '(' expression ')' | VERIFICATION '(' expression ')' | QUALITY_ASSURANCE '(' expression ')' | PERFORMANCE_TUNING '(' expression ')' | SCALABILITY '(' expression ')' | AVAILABILITY '(' expression ')' | RELIABILITY '(' expression ')' | FAULT_TOLERANCE '(' expression ')' | DISASTER_RECOVERY '(' expression ')' | BACKUP '(' expression ')' | RESTORE '(' expression ')' | REPLICATION '(' expression ')' | SHARDING '(' expression ')' | PARTITIONING '(' expression ')' | CLUSTERING '(' expression ')' | LOAD_BALANCING '(' expression ')' | FAILOVER '(' expression ')' | HIGH_AVAILABILITY '(' expression ')' | BUSINESS_CONTINUITY '(' expression ')' | SERVICE_LEVEL_AGREEMENT '(' expression ')' | SLA '(' expression ')' | KPI '(' expression ')' | METRICS '(' expression ')' | DASHBOARD '(' expression ')' | REPORTING '(' expression ')' | ANALYTICS '(' expression ')' | BUSINESS_INTELLIGENCE '(' expression ')' | DATA_WAREHOUSE '(' expression ')' | DATA_LAKE '(' expression ')' | DATA_MART '(' expression ')' | ETL '(' expression ')' | ELT '(' expression ')' | DATA_PIPELINE '(' expression ')' | DATA_STREAMING '(' expression ')' | REAL_TIME '(' expression ')' | BATCH_PROCESSING '(' expression ')' | STREAM_PROCESSING '(' expression ')' | EVENT_PROCESSING '(' expression ')' | COMPLEX_EVENT_PROCESSING '(' expression ')' | CEP '(' expression ')' | PATTERN_DETECTION '(' expression ')' | ANOMALY_DETECTION '(' expression ')' | FRAUD_DETECTION '(' expression ')' | RISK_ASSESSMENT '(' expression ')' | COMPLIANCE '(' expression ')' | GOVERNANCE '(' expression ')' | DATA_QUALITY '(' expression ')' | DATA_LINEAGE '(' expression ')' | DATA_CATALOG '(' expression ')' | METADATA '(' expression ')' | SCHEMA '(' expression ')' | ONTOLOGY '(' expression ')' | TAXONOMY '(' expression ')' | VOCABULARY '(' expression ')' | GLOSSARY '(' expression ')' | DICTIONARY '(' expression ')' | THESAURUS '(' expression ')' | KNOWLEDGE_BASE '(' expression ')' | EXPERT_SYSTEM '(' expression ')' | RULE_ENGINE '(' expression ')' | DECISION_TREE '(' expression ')' | NEURAL_NETWORK '(' expression ')' | DEEP_LEARNING '(' expression ')' | MACHINE_LEARNING '(' expression ')' | ARTIFICIAL_INTELLIGENCE '(' expression ')' | AI '(' expression ')' | ML '(' expression ')' | DL '(' expression ')' | NLP '(' expression ')' | NATURAL_LANGUAGE_PROCESSING '(' expression ')' | COMPUTER_VISION '(' expression ')' | CV '(' expression ')' | SPEECH_RECOGNITION '(' expression ')' | SPEECH_SYNTHESIS '(' expression ')' | TEXT_TO_SPEECH '(' expression ')' | TTS '(' expression ')' | SPEECH_TO_TEXT '(' expression ')' | STT '(' expression ')' | VOICE_RECOGNITION '(' expression ')' | VOICE_SYNTHESIS '(' expression ')' | VOICE_CLONING '(' expression ')' | VOICE_CONVERSION '(' expression ')' | VOICE_ENHANCEMENT '(' expression ')' | NOISE_REDUCTION '(' expression ')';
}

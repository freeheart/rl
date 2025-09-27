// GQL 2024语法定义 - 基于ISO/IEC 39075标准
// 用于测试RL解析器生成器
grammar GQL2024 {
    // 查询语句
    query: operation_definition | selection_set;
    
    // 操作定义
    operation_definition: operation_type operation_name? variable_definitions? directives? selection_set;
    
    // 操作类型
    operation_type: QUERY | MUTATION | SUBSCRIPTION;
    
    // 操作名称
    operation_name: name;
    
    // 变量定义
    variable_definitions: '(' variable_definition+ ')';
    
    // 变量定义项
    variable_definition: variable ':' type_ default_value?;
    
    // 变量
    variable: '$' name;
    
    // 类型
    type_: named_type | list_type | non_null_type;
    
    // 命名类型
    named_type: name;
    
    // 列表类型
    list_type: '[' type_ ']';
    
    // 非空类型
    non_null_type: named_type '!' | list_type '!';
    
    // 默认值
    default_value: '=' value;
    
    // 选择集
    selection_set: '{' selection+ '}';
    
    // 选择
    selection: field | fragment_spread | inline_fragment;
    
    // 字段
    field: alias? name arguments? directives? selection_set?;
    
    // 别名
    alias: name ':';
    
    // 参数
    arguments: '(' argument+ ')';
    
    // 参数项
    argument: name ':' value;
    
    // 片段展开
    fragment_spread: '...' fragment_name directives?;
    
    // 内联片段
    inline_fragment: '...' type_condition? directives? selection_set;
    
    // 类型条件
    type_condition: 'on' named_type;
    
    // 片段定义
    fragment_definition: 'fragment' fragment_name type_condition directives? selection_set;
    
    // 片段名称
    fragment_name: name;
    
    // 指令
    directives: directive+;
    
    // 指令项
    directive: '@' name arguments?;
    
    // 值
    value: variable | int_value | float_value | string_value | boolean_value | null_value | enum_value | list_value | object_value;
    
    // 整数值
    int_value: integer_part;
    
    // 浮点值
    float_value: integer_part '.' digit+ exponent_part? | integer_part exponent_part;
    
    // 整数字面量
    integer_part: negative_sign? digit+;
    
    // 负号
    negative_sign: '-';
    
    // 指数部分
    exponent_part: exponent_indicator sign? digit+;
    
    // 指数指示符
    exponent_indicator: 'e' | 'E';
    
    // 符号
    sign: '+' | '-';
    
    // 字符串值
    string_value: '"' string_character* '"' | "'" string_character* "'";
    
    // 字符串字符
    string_character: source_character | escape_sequence;
    
    // 转义序列
    escape_sequence: '\\' escape_character;
    
    // 转义字符
    escape_character: '"' | '\\' | '/' | 'b' | 'f' | 'n' | 'r' | 't' | 'u' hex_digit hex_digit hex_digit hex_digit;
    
    // 十六进制数字
    hex_digit: digit | 'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'a' | 'b' | 'c' | 'd' | 'e' | 'f';
    
    // 布尔值
    boolean_value: TRUE | FALSE;
    
    // 空值
    null_value: NULL;
    
    // 枚举值
    enum_value: name;
    
    // 列表值
    list_value: '[' value* ']';
    
    // 对象值
    object_value: '{' object_field* '}';
    
    // 对象字段
    object_field: name ':' value;
    
    // 名称
    name: name_start name_character*;
    
    // 名称开始字符
    name_start: letter | '_';
    
    // 名称字符
    name_character: letter | digit | '_';
    
    // 字母
    letter: 'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' | 'K' | 'L' | 'M' | 'N' | 'O' | 'P' | 'Q' | 'R' | 'S' | 'T' | 'U' | 'V' | 'W' | 'X' | 'Y' | 'Z' | 'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n' | 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' | 'v' | 'w' | 'x' | 'y' | 'z';
    
    // 数字
    digit: '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9';
    
    // 源字符
    source_character: unicode_character;
    
    // Unicode字符
    unicode_character: unicode_letter | unicode_digit | unicode_punctuation | unicode_symbol | unicode_separator | unicode_other;
    
    // Unicode字母
    unicode_letter: unicode_uppercase_letter | unicode_lowercase_letter | unicode_titlecase_letter | unicode_modifier_letter | unicode_other_letter;
    
    // Unicode数字
    unicode_digit: unicode_decimal_digit | unicode_letter_number | unicode_other_number;
    
    // Unicode标点符号
    unicode_punctuation: unicode_connector_punctuation | unicode_dash_punctuation | unicode_open_punctuation | unicode_close_punctuation | unicode_initial_punctuation | unicode_final_punctuation | unicode_other_punctuation;
    
    // Unicode符号
    unicode_symbol: unicode_math_symbol | unicode_currency_symbol | unicode_modifier_symbol | unicode_other_symbol;
    
    // Unicode分隔符
    unicode_separator: unicode_space_separator | unicode_line_separator | unicode_paragraph_separator;
    
    // Unicode其他
    unicode_other: unicode_control | unicode_format | unicode_surrogate | unicode_private_use | unicode_not_assigned;
    
    // 类型系统定义
    type_system_definition: schema_definition | type_definition | directive_definition;
    
    // 模式定义
    schema_definition: 'schema' directives? '{' operation_type_definition+ '}';
    
    // 操作类型定义
    operation_type_definition: operation_type ':' named_type;
    
    // 类型定义
    type_definition: scalar_type_definition | object_type_definition | interface_type_definition | union_type_definition | enum_type_definition | input_object_type_definition;
    
    // 标量类型定义
    scalar_type_definition: 'scalar' name directives?;
    
    // 对象类型定义
    object_type_definition: 'type' name implements_interfaces? directives? '{' field_definition+ '}';
    
    // 实现接口
    implements_interfaces: 'implements' '&'? named_type;
    
    // 字段定义
    field_definition: name arguments_definition? ':' type_ directives?;
    
    // 参数定义
    arguments_definition: '(' input_value_definition+ ')';
    
    // 输入值定义
    input_value_definition: name ':' type_ default_value? directives?;
    
    // 接口类型定义
    interface_type_definition: 'interface' name directives? '{' field_definition+ '}';
    
    // 联合类型定义
    union_type_definition: 'union' name directives? '=' union_members;
    
    // 联合成员
    union_members: named_type | union_members '|' named_type;
    
    // 枚举类型定义
    enum_type_definition: 'enum' name directives? '{' enum_value_definition+ '}';
    
    // 枚举值定义
    enum_value_definition: name directives?;
    
    // 输入对象类型定义
    input_object_type_definition: 'input' name directives? '{' input_value_definition+ '}';
    
    // 指令定义
    directive_definition: 'directive' '@' name arguments_definition? 'on' directive_locations;
    
    // 指令位置
    directive_locations: directive_location | directive_locations '|' directive_location;
    
    // 指令位置项
    directive_location: executable_directive_location | type_system_directive_location;
    
    // 可执行指令位置
    executable_directive_location: QUERY | MUTATION | SUBSCRIPTION | FIELD | FRAGMENT_DEFINITION | FRAGMENT_SPREAD | INLINE_FRAGMENT;
    
    // 类型系统指令位置
    type_system_directive_location: SCHEMA | SCALAR | OBJECT | FIELD_DEFINITION | ARGUMENT_DEFINITION | INTERFACE | UNION | ENUM | ENUM_VALUE | INPUT_OBJECT | INPUT_FIELD_DEFINITION;
    
    // 扩展定义
    type_system_extension: schema_extension | type_extension;
    
    // 模式扩展
    schema_extension: 'extend' 'schema' directives? '{' operation_type_definition+ '}';
    
    // 类型扩展
    type_extension: scalar_type_extension | object_type_extension | interface_type_extension | union_type_extension | enum_type_extension | input_object_type_extension;
    
    // 标量类型扩展
    scalar_type_extension: 'extend' 'scalar' name directives?;
    
    // 对象类型扩展
    object_type_extension: 'extend' 'type' name implements_interfaces? directives? '{' field_definition+ '}';
    
    // 接口类型扩展
    interface_type_extension: 'extend' 'interface' name directives? '{' field_definition+ '}';
    
    // 联合类型扩展
    union_type_extension: 'extend' 'union' name directives? '=' union_members;
    
    // 枚举类型扩展
    enum_type_extension: 'extend' 'enum' name directives? '{' enum_value_definition+ '}';
    
    // 输入对象类型扩展
    input_object_type_extension: 'extend' 'input' name directives? '{' input_value_definition+ '}';
    
    // 文档
    document: definition+;
    
    // 定义
    definition: executable_definition | type_system_definition | type_system_extension;
    
    // 可执行定义
    executable_definition: operation_definition | fragment_definition;
    
    // 关键字
    QUERY: 'query';
    MUTATION: 'mutation';
    SUBSCRIPTION: 'subscription';
    FRAGMENT: 'fragment';
    ON: 'on';
    TRUE: 'true';
    FALSE: 'false';
    NULL: 'null';
    SCHEMA: 'schema';
    SCALAR: 'scalar';
    TYPE: 'type';
    INTERFACE: 'interface';
    IMPLEMENTS: 'implements';
    UNION: 'union';
    ENUM: 'enum';
    INPUT: 'input';
    DIRECTIVE: 'directive';
    EXTEND: 'extend';
    FIELD: 'field';
    FRAGMENT_DEFINITION: 'fragment_definition';
    FRAGMENT_SPREAD: 'fragment_spread';
    INLINE_FRAGMENT: 'inline_fragment';
    FIELD_DEFINITION: 'field_definition';
    ARGUMENT_DEFINITION: 'argument_definition';
    ENUM_VALUE: 'enum_value';
    INPUT_FIELD_DEFINITION: 'input_field_definition';
}

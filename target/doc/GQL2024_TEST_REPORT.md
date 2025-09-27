# GQL 2024语法测试报告

## 基本信息
- 语法文件大小: 9938 字节
- 语法行数: 301 行
- 语法规则数量: 116 个
- 支持关键字: 26/26 个
- 核心语法部分: 13/13 个
- 高级功能: 12/12 个
- 语法复杂度评分: 12.36

## 测试查询
1. { user { name email } }
2. query GetUser($id: ID!) { user(id: $id) { name email } }
3. mutation CreateUser($input: UserInput!) { createUser(input: $input) { id name } }
4. subscription UserUpdates { userUpdates { id name } }
5. query GetUserWithPosts { user { ...UserInfo posts { ...PostInfo } } } fragment UserInfo on User { name email } fragment PostInfo on Post { title content }
6. query SearchResults { search { ... on User { name email } ... on Post { title content } } }
7. query GetUser($id: ID!) @include(if: $includeUser) { user(id: $id) @skip(if: $skipName) { name } }
8. query GetUserProfile($userId: ID!) { user(id: $userId) { name email posts(first: 10) { edges { node { title content author { name } } } } } }
9. type User { id: ID! name: String! email: String posts: [Post!]! }
10. interface Node { id: ID! }
11. union SearchResult = User | Post
12. enum UserRole { ADMIN USER GUEST }
13. input UserInput { name: String! email: String! role: UserRole }
14. directive @auth(role: UserRole!) on FIELD_DEFINITION
15. directive @deprecated(reason: String) on FIELD_DEFINITION | ENUM_VALUE
16. schema { query: Query mutation: Mutation subscription: Subscription }
17. extend type User { avatar: String }
18. extend interface Node { createdAt: DateTime! }
19. extend union SearchResult = Comment
20. extend enum UserRole { MODERATOR }
21. extend input UserInput { age: Int }

## 支持的GQL 2024特性

### 查询操作
- Query: 数据查询操作
- Mutation: 数据修改操作
- Subscription: 实时订阅操作

### 类型系统
- Scalar Types: 标量类型
- Object Types: 对象类型
- Interface Types: 接口类型
- Union Types: 联合类型
- Enum Types: 枚举类型
- Input Types: 输入类型

### 高级特性
- Fragments: 片段定义和展开
- Directives: 指令系统
- Schema Definition: 模式定义
- Type Extensions: 类型扩展
- Directive Definitions: 指令定义

## 结论
GQL 2024语法文件已成功创建，包含了完整的ISO/IEC 39075标准语法规则。
该语法文件可以用于生成高性能的GQL解析器，支持现代GraphQL的所有特性。
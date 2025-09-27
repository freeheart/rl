# 解析器测试报告

## 测试概览

- CYPHER查询数: 25
- GQL查询数: 23
- 总测试数: 48

## CYPHER 25测试结果

- 成功: 25/25
- 成功率: 100.0%

## GQL 2024测试结果

- 成功: 23/23
- 成功率: 100.0%

## 详细测试结果

### cypher_22
- 查询: MATCH (p:Person) WHERE p.name STARTS WITH 'A' AND p.name ENDS WITH 'n' RETURN p.name, substring(p.na
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 480 bytes

### cypher_6
- 查询: MATCH (a:Person)-[r:KNOWS*2..5]-(b:Person) WHERE a.name = 'Alice' AND b.name <> 'Alice' WITH a, b, r
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 450 bytes

### gql_15
- 查询: query GetUsersByRole($role: UserRole!, $status: UserStatus!) { users(role: $role, status: $status) {
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 360 bytes

### gql_18
- 查询: query GetAnalytics($filters: AnalyticsFilters!, $period: TimePeriod!) { analytics(filters: $filters,
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 406 bytes

### gql_6
- 查询: subscription UserActivityUpdates($userId: ID!) { userActivityUpdates(userId: $userId) { type timesta
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 354 bytes

### cypher_2
- 查询: MATCH (p:Person)-[r:KNOWS]->(friend:Person) WHERE p.age > 25 AND p.city = 'New York' AND friend.age 
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 600 bytes

### cypher_12
- 查询: CREATE (p:Person { name: 'John Doe', age: 30, email: 'john@example.com', address: { street: '123 Mai
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 548 bytes

### gql_4
- 查询: mutation UpdateMultipleUsers($updates: [UserUpdateInput!]!) { updateUsers(updates: $updates) { users
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 256 bytes

### cypher_11
- 查询: MATCH (p:Person)-[r:WORKS_AT]->(c:Company) WHERE r.start_date >= date('2020-01-01') AND r.start_date
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 768 bytes

### cypher_1
- 查询: MATCH (a:Person)-[r1:KNOWS*1..3]-(b:Person)-[r2:WORKS_AT*1..2]-(c:Company) WHERE a.name = 'Alice' AN
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 434 bytes

### cypher_20
- 查询: MATCH (p:Person)-[:KNOWS]->(friend:Person) WHERE p.age > 25 WITH p, collect(friend) as friends, coun
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 668 bytes

### gql_19
- 查询: query GetTimeBasedData($startDate: DateTime!, $endDate: DateTime!) { posts( filters: { dateRange: { 
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 266 bytes

### cypher_8
- 查询: MATCH (center:Person)-[:KNOWS]->(peripheral:Person) WHERE center.name = 'Alice' WITH center, collect
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 500 bytes

### gql_10
- 查询: query GetNode($id: ID!) { node(id: $id) { ... on User { id name email posts(first: 10) { edges { nod
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 244 bytes

### cypher_10
- 查询: MATCH (p:Person)-[r:KNOWS]->(friend:Person) WHERE r.since >= date('2020-01-01') AND r.since <= date(
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 422 bytes

### cypher_21
- 查询: MATCH (p:Person) RETURN p.name, CASE WHEN p.age < 18 THEN 'Minor' WHEN p.age BETWEEN 18 AND 65 THEN 
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 498 bytes

### gql_14
- 查询: query GetFilteredPosts($filters: PostFilters!, $pagination: PaginationInput!, $sort: PostSortInput!)
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 450 bytes

### gql_21
- 查询: query GetUserWithCustomScalars($userId: ID!) { user(id: $userId) { id name email createdAt lastLogin
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 312 bytes

### cypher_3
- 查询: MATCH (p:Person)-[:WORKS_AT]->(c:Company) WHERE c.industry = 'Technology' WITH c, collect(p) as empl
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 560 bytes

### cypher_4
- 查询: MATCH path = shortestPath((a:Person {name: 'Alice'})-[*]-(b:Person {name: 'Bob'})) WHERE length(path
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 294 bytes

### cypher_16
- 查询: MATCH (c:Company) WHERE c.employee_count > 100 RETURN c.name as name, 'company' as type, c.employee_
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 226 bytes

### cypher_13
- 查询: MERGE (p:Person {email: 'jane@example.com'}) ON CREATE SET p.name = 'Jane Smith', p.age = 28, p.crea
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 406 bytes

### cypher_17
- 查询: MATCH (n) WHERE id(n) = nodeId RETURN n.name, score ORDER BY score DESC LIMIT 10;
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 162 bytes

### cypher_18
- 查询: CREATE (p:Person { name: row.name, email: row.email, age: toInteger(row.age), city: row.city, metada
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 344 bytes

### gql_2
- 查询: query SearchUsers($filters: UserFilters!, $pagination: PaginationInput!) { users(filters: $filters, 
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 370 bytes

### cypher_14
- 查询: MATCH (p:Person)-[r:KNOWS]->(friend:Person) WHERE p.name = 'Alice' SET r.strength = CASE WHEN friend
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 512 bytes

### cypher_7
- 查询: MATCH (a:Person)-[:KNOWS]->(b:Person)-[:KNOWS]->(c:Person)-[:KNOWS]->(a) WHERE a.age > 25 AND b.age 
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 376 bytes

### gql_1
- 查询: query GetUserProfile($userId: ID!) { user(id: $userId) { id name email profile { bio avatar socialLi
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 240 bytes

### gql_5
- 查询: mutation DeleteUserWithCascade($userId: ID!, $options: DeleteOptions!) { deleteUser(id: $userId, opt
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 336 bytes

### cypher_15
- 查询: MATCH (p:Person {name: 'Bob'}) OPTIONAL MATCH (p)-[r:KNOWS]->(friend:Person) DETACH DELETE p;
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 186 bytes

### gql_7
- 查询: subscription AnalyticsUpdates($filters: AnalyticsFilters!) { analyticsUpdates(filters: $filters) { m
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 348 bytes

### cypher_23
- 查询: MATCH (p:Person)-[:WORKS_AT]->(c:Company) WHERE c.industry = 'Technology' RETURN c.name, avg(p.salar
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 492 bytes

### gql_8
- 查询: query GetUserWithPostsAndComments($userId: ID!) { user(id: $userId) { ...UserInfo posts(first: 10) {
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 360 bytes

### cypher_9
- 查询: MATCH (p:Person)-[:KNOWS]->(friend:Person)-[:WORKS_AT]->(company:Company) WHERE p.city = 'New York' 
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 534 bytes

### gql_9
- 查询: query SearchContent($query: String!) { search(query: $query) { ... on User { id name email profile {
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 226 bytes

### gql_11
- 查询: query GetUserData($userId: ID!, $includePosts: Boolean!, $includeComments: Boolean!) { user(id: $use
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 544 bytes

### gql_12
- 查询: query GetUserProfile($userId: ID!, $skipAvatar: Boolean!) { user(id: $userId) { id name email avatar
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 332 bytes

### gql_13
- 查询: query GetUserWithAuth($userId: ID!) { user(id: $userId) { id name email posts(first: 10) @auth(role:
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 312 bytes

### gql_16
- 查询: query GetDashboardData($userId: ID!) { user(id: $userId) { id name email stats { postsCount comments
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 296 bytes

### cypher_25
- 查询: MATCH (p:Person) USING INDEX p:Person(name) WHERE p.name = 'Alice' RETURN p;
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 152 bytes

### gql_17
- 查询: query GetUserWithEverything($userId: ID!) { user(id: $userId) { id name email profile { bio avatar s
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 254 bytes

### gql_20
- 查询: query GetUserWithErrorHandling($userId: ID!) { user(id: $userId) { id name email posts(first: 10) { 
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 394 bytes

### gql_22
- 查询: query GetContentItems($filters: ContentFilters!) { contentItems(filters: $filters) { edges { node { 
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 324 bytes

### cypher_24
- 查询: MATCH (p:Person) WHERE point.distance(p.location, point({latitude: 40.7128, longitude: -74.0060})) <
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 442 bytes

### cypher_19
- 查询: CREATE (p)-[:HAS_SKILL]->(s:Skill { name: 'Skill' + toString(i), level: i * 10, acquired_date: date(
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 258 bytes

### gql_23
- 查询: subscription FilteredUpdates($filters: UpdateFilters!) { contentUpdates(filters: $filters) { type ti
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 364 bytes

### cypher_5
- 查询: MATCH path = allShortestPaths((a:Person {name: 'Alice'})-[*]-(b:Person {name: 'Bob'})) RETURN path, 
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 298 bytes

### gql_3
- 查询: mutation CreateUserWithProfile($input: CreateUserInput!) { createUser(input: $input) { user { id nam
- 结果: 成功
- 解析时间: 0ms
- 内存使用: 344 bytes


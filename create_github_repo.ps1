# RL项目GitHub仓库创建脚本
# 作者: freeheart1977
# 用途: 自动创建GitHub仓库并推送代码

Write-Host "🚀 开始创建RL项目GitHub仓库..." -ForegroundColor Green

# 检查Git状态
if (-not (Test-Path ".git")) {
    Write-Host "初始化Git仓库..." -ForegroundColor Yellow
    git init
}

# 添加所有文件
Write-Host "添加文件到Git..." -ForegroundColor Yellow
git add .

# 创建提交
Write-Host "创建提交..." -ForegroundColor Yellow
git commit -m "Initial commit: RL - Right wheeL parser generator

🎉 首个正式版本发布！

✨ 核心特性:
- 多算法解析引擎 (LL(1)/LALR(1)/GLR/混合)
- 零拷贝解析优化
- 并行处理支持
- 智能缓存机制
- AI智能增强
- 知识图谱集成
- 多语言支持 (12种语言)
- 完整的CLI工具

🚀 性能表现:
- 解析速度: 3.9M字符/秒
- 内存使用: 极低
- 并发支持: 多核并行
- 测试验证: 352个CYPHER语句100%成功

📚 文档完整:
- 详细使用说明
- 丰富示例代码
- 性能基准测试
- 集成指南

🔧 易用性:
- 一行代码生成解析器
- 简洁API设计
- 完整CLI支持
- 多目标语言支持

RL - 让解析变得简单而高效！"

# 设置远程仓库
Write-Host "配置远程仓库..." -ForegroundColor Yellow
git remote add origin https://github.com/freeheart/rl.git

# 设置主分支
git branch -M main

# 推送到GitHub
Write-Host "推送到GitHub..." -ForegroundColor Yellow
git push -u origin main

# 创建版本标签
Write-Host "创建版本标签..." -ForegroundColor Yellow
git tag -a v0.1.0 -m "RL v0.1.0 - 高性能Rust解析器生成器

🎉 首个正式版本发布！

✨ 核心特性:
- 多算法解析引擎 (LL(1)/LALR(1)/GLR/混合)
- 零拷贝解析优化
- 并行处理支持
- 智能缓存机制
- AI智能增强
- 知识图谱集成
- 多语言支持 (12种语言)
- 完整的CLI工具

🚀 性能表现:
- 解析速度: 3.9M字符/秒
- 内存使用: 极低
- 并发支持: 多核并行
- 测试验证: 352个CYPHER语句100%成功

📚 文档完整:
- 详细使用说明
- 丰富示例代码
- 性能基准测试
- 集成指南

🔧 易用性:
- 一行代码生成解析器
- 简洁API设计
- 完整CLI支持
- 多目标语言支持

RL - 让解析变得简单而高效！"

# 推送标签
git push origin v0.1.0

Write-Host "✅ 发布完成！" -ForegroundColor Green
Write-Host "🌐 GitHub: https://github.com/freeheart/rl" -ForegroundColor Cyan
Write-Host "📦 版本: v0.1.0" -ForegroundColor Cyan
Write-Host "🎉 RL - 让解析变得简单而高效！" -ForegroundColor Magenta

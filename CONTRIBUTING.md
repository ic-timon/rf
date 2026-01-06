# 贡献指南

感谢您对 RF 框架的关注！我们欢迎所有形式的贡献。

## 如何贡献

### 报告问题

如果您发现了 bug 或有功能建议，请：

1. 在 [GitHub Issues](https://github.com/ic-timon/rf/issues) 中搜索是否已有相关问题
2. 如果没有，请创建新的 Issue，并包含：
   - 清晰的问题描述
   - 复现步骤
   - 预期行为和实际行为
   - 环境信息（Rust 版本、操作系统等）

### 提交代码

#### 1. Fork 仓库

在 GitHub 上 Fork [RF 仓库](https://github.com/ic-timon/rf)。

#### 2. 创建分支

```bash
git checkout -b feature/your-feature-name
# 或
git checkout -b fix/your-bug-fix
```

#### 3. 开发

- 遵循项目的代码风格
- 添加必要的测试
- 更新相关文档
- 确保所有测试通过：`cargo test`
- 确保代码可以编译：`cargo build`

#### 4. 提交代码

使用清晰的提交信息：

```bash
git commit -m "feat: 添加新功能描述"
# 或
git commit -m "fix: 修复问题描述"
```

提交信息格式：
- `feat:` - 新功能
- `fix:` - Bug 修复
- `docs:` - 文档更新
- `style:` - 代码格式调整
- `refactor:` - 代码重构
- `test:` - 测试相关
- `chore:` - 构建/工具相关

#### 5. 推送并创建 Pull Request

```bash
git push origin feature/your-feature-name
```

然后在 GitHub 上创建 Pull Request。

## 代码规范

### Rust 代码风格

- 遵循 Rust 官方代码风格指南
- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量

### 文档

- 所有公共 API 必须有文档注释
- 使用中文编写文档
- 代码示例应该可以编译运行

### 测试

- 为新功能添加单元测试
- 确保测试覆盖主要功能
- 测试应该清晰易懂

## 项目结构

```
rf/
├── core/           # 核心模块
├── container/      # 容器模块
├── os/             # 操作系统模块
├── net/            # 网络模块
├── database/       # 数据库模块
├── encoding/       # 编码模块
├── crypto/         # 加密模块
├── util/           # 工具模块
├── errors/         # 错误处理模块
├── text/           # 文本处理模块
├── i18n/           # 国际化模块
├── debug/          # 调试模块
├── test/           # 测试工具模块
├── frame/          # 框架实例管理
├── contrib/        # Contrib 模块
├── cmd/            # CLI 工具
├── docs/           # 文档
└── tests/          # 集成测试
```

## 开发环境设置

### 1. 克隆仓库

```bash
git clone https://github.com/ic-timon/rf.git
cd rf
```

### 2. 安装依赖

确保已安装 Rust（推荐使用 [rustup](https://rustup.rs/)）：

```bash
rustup update
```

### 3. 构建项目

```bash
cargo build
```

### 4. 运行测试

```bash
cargo test
```

### 5. 生成文档

```bash
cargo doc --open
```

## Pull Request 流程

1. **确保代码质量**
   - 所有测试通过
   - 代码格式化：`cargo fmt`
   - 代码检查：`cargo clippy`
   - 文档完整

2. **创建 Pull Request**
   - 填写清晰的标题和描述
   - 说明变更的原因和影响
   - 关联相关的 Issue（如果有）

3. **代码审查**
   - 维护者会审查您的代码
   - 根据反馈进行修改
   - 保持友好的沟通

4. **合并**
   - 审查通过后，维护者会合并您的 PR
   - 感谢您的贡献！

## 行为准则

- 保持友好和尊重
- 欢迎不同观点和经验
- 专注于对项目最有利的事情
- 尊重其他贡献者

## 获取帮助

如果您在贡献过程中遇到问题：

- 查看 [文档](docs/INDEX.md)
- 在 [GitHub Discussions](https://github.com/ic-timon/rf/discussions) 提问
- 创建 Issue 寻求帮助

## 许可证

通过贡献代码，您同意您的贡献将在与项目相同的许可证（MIT License）下发布。

再次感谢您对 RF 框架的贡献！


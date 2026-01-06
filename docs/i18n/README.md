# I18n 模块教程

I18n 模块提供国际化支持，支持多语言切换和本地化。

## 模块概述

I18n 模块功能：

- 多语言支持
- 语言切换
- 本地化文本

## 快速开始

```rust
use rf_i18n::I18n;

let i18n = I18n::new("zh-CN")?;
let text = i18n.t("welcome")?;
```

## 相关链接

- [os 模块](../os/README.md) - 配置管理


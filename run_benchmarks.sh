#!/bin/bash

# Benchmark 测试运行脚本
# 运行所有 benchmark 测试并生成结果报告

set -e

echo "开始运行 RF 框架的 Benchmark 测试..."
echo "=========================================="

# 创建结果目录
RESULTS_DIR="docs/benchmark_results"
mkdir -p "$RESULTS_DIR"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
REPORT_FILE="$RESULTS_DIR/benchmark_report_${TIMESTAMP}.md"

# 写入报告头部
cat > "$REPORT_FILE" << EOF
# RF 框架 Benchmark 测试报告

生成时间: $(date)

## 测试环境

- 操作系统: $(uname -s)
- 架构: $(uname -m)
- Rust 版本: $(rustc --version)

## 测试结果

EOF

# 运行 container 模块的 benchmark
echo "运行 container 模块 benchmark..."
echo "### Container 模块" >> "$REPORT_FILE"
echo "\`\`\`" >> "$REPORT_FILE"
cargo bench --bench array_bench --bench list_bench --bench queue_bench --bench ring_bench --bench map_bench --bench set_bench --bench pool_bench 2>&1 | tee -a "$REPORT_FILE"
echo "\`\`\`" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

# 运行 os 模块的 benchmark
echo "运行 os 模块 benchmark..."
echo "### OS 模块" >> "$REPORT_FILE"
echo "\`\`\`" >> "$REPORT_FILE"
cargo bench --bench cache_bench --bench mutex_bench 2>&1 | tee -a "$REPORT_FILE"
echo "\`\`\`" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

# 运行 encoding 模块的 benchmark
echo "运行 encoding 模块 benchmark..."
echo "### Encoding 模块" >> "$REPORT_FILE"
echo "\`\`\`" >> "$REPORT_FILE"
cargo bench --bench json_bench --bench hash_bench 2>&1 | tee -a "$REPORT_FILE"
echo "\`\`\`" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

# 运行 util 模块的 benchmark
echo "运行 util 模块 benchmark..."
echo "### Util 模块" >> "$REPORT_FILE"
echo "\`\`\`" >> "$REPORT_FILE"
cargo bench --bench guid_bench --bench rand_bench 2>&1 | tee -a "$REPORT_FILE"
echo "\`\`\`" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

echo "=========================================="
echo "Benchmark 测试完成！"
echo "结果已保存到: $REPORT_FILE"


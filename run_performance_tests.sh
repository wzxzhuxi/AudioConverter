#!/bin/bash

echo "⚡ AudioConverter 性能测试套件"
echo "=================================="

echo "🔥 编译 Release 版本..."
cargo build --release

echo "🧪 运行性能测试（Release模式）..."
cargo test --release --test performance_tests -- --nocapture

echo "📈 运行基准测试..."
cargo bench

echo "📊 性能分析完成！"

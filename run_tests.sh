#!/bin/bash

echo "🎵 AudioConverter 完整测试套件"
echo "=================================="

# 生成测试音频文件
echo "📁 生成测试音频文件..."
cd test_data
if [ ! -f "generate_test_audio.rs" ]; then
  echo "⚠️  请先创建 generate_test_audio.rs 文件"
fi
cd ..

echo "🧪 运行所有测试..."

echo "1️⃣  单元测试（Debug模式）..."
cargo test --lib

echo "2️⃣  集成测试..."
cargo test --test integration_tests

echo "3️⃣  CLI测试..."
cargo test --test cli_tests

echo "4️⃣  音频格式测试..."
cargo test --test audio_format_tests

echo "5️⃣  性能测试（Release模式）..."
echo "⚡ 使用 Release 模式运行性能测试以获得准确结果..."
cargo test --release --test performance_tests

echo "6️⃣  属性测试..."
cargo test --test property_tests

echo "7️⃣  错误处理测试..."
cargo test --test error_handling_tests

echo "🏆 基准测试（最高优化）..."
cargo bench

echo "📊 生成测试覆盖率报告..."
cargo tarpaulin --out html --output-dir coverage

echo "🚀 编译 Release 版本..."
cargo build --release

echo "✅ 所有测试完成！"
echo "📋 覆盖率报告: coverage/tarpaulin-report.html"
echo "🎯 优化后的二进制文件: target/release/audio-converter"

# una-bakery — 解析 Unicode 数据并生成规范化表

[Русский](README.ru.md) | [English](README.md) | 中文

----

该 crate 用于解析 Unicode 标准数据（当前版本 — 17.0.0），并生成用于规范化算法的优化数据表。

该库是辅助性的；生成的数据在规范化 crate [una-normalization](https://github.com/una-rs/una-normalization) 中使用。

## 使用

```
cargo run
```

执行后将生成：

- 位于 `output/tables` 的规范化表。
- 位于 `output/stats` 的码点分解/组合特性统计信息。

## 信任与验证

本 crate 直接从官方 Unicode 17.0.0 数据文件生成表。无硬编码表，无隐藏逻辑。

**验证步骤：**

- 下载 [input/NOTES.md](input/NOTES.md) 中列出的源文件。
- 运行 `cargo run`。
- 比对生成文件的哈希值与发布版本。

## 结构

*主要模块：*

- `src/bin` — 表生成。
- `src/data` — 解析由 Unicode 提供的文件中的数据。
- `src/normalization` — 将码点的规范化数据编码为表格式。

*其他模块：*

- `src/codepoint` — 码点类及其属性；数据来自 `UCD/UnicodeData.txt`。
- `src/hangul` — 韩文（Hangul），码点的一个特殊情况。
- `src/errors` — 错误。
- `src/tests` — 测试和可选的断言检查，对希望更深入理解代码和标准的人会有帮助。

**输入和输出数据：**

- `input` — Unicode 源数据。
- `output/tables` — 生成的规范化表。
- `output/stats` — 编码过程中生成的统计信息和附加信息。

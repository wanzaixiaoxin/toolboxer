# Toolboxer

Toolboxer 是一个用 Rust 编写的命令行工具集，旨在提供一系列实用的文件系统操作功能。目前，它包含一个强大的 `tree` 命令，用于以树形结构显示目录内容。

## 功能

- **树形目录显示**：以直观的树形结构展示目录内容
- **灵活的排序选项**：支持按名称、类型、大小或修改日期排序
- **文件过滤**：可以显示或隐藏隐藏文件，支持模式匹配过滤
- **详细信息显示**：可选择性地显示文件权限、大小和修改时间
- **彩色输出**：不同类型的文件使用不同的颜色标识，提高可读性
- **可控深度**：允许用户指定目录遍历的最大深度

## 安装

确保您的系统中已安装 Rust 和 Cargo。然后，按照以下步骤安装 Toolboxer：

```bash
git clone https://github.com/wanzaixiaoxin/toolboxer.git
cd toolboxer
cargo build --release
```

编译后的可执行文件将位于 `target/release` 目录中。

## 使用方法

### tree 命令

```
toolboxer tree [选项] [路径]
```

#### 选项：

- `-d, --depth <DEPTH>`：指定遍历的最大深度
- `-a, --all`：显示隐藏文件
- `-t, --sort-type`：按类型排序（目录优先）
- `-s, --sort-size`：按文件大小排序
- `-D, --sort-date`：按修改日期排序
- `-p, --permissions`：显示文件权限
- `-S, --human-size`：显示文件大小
- `-m, --modified`：显示文件的修改时间
- `-f, --filter <PATTERN>`：按模式过滤文件名

#### 示例：

```bash
# 显示当前目录的树形结构
toolboxer tree

# 显示指定目录，包含隐藏文件，最大深度为2层
toolboxer tree /path/to/dir -a -d 2

# 显示目录，包含文件大小和修改时间
toolboxer tree -S -m

# 按大小排序并显示权限
toolboxer tree -s -p

# 仅显示包含特定字符串的文件
toolboxer tree -f "test"
```

## 贡献

欢迎贡献！请随时提交问题或拉取请求。

## 许可证

本项目采用 MIT 许可证。详情请参见 [LICENSE](LICENSE) 文件。

## 作者

[wanzaixiaoxin]

## 致谢

感谢所有为这个项目做出贡献的开发者和用户。

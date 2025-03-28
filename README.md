# Toolboxer
 --
一个高效的Rust命令行工具集，提供系统管理和开发辅助功能。

## ✨ 功能特性

### 📂 目录分析
- 树形结构展示目录内容
- 支持文件大小、权限、修改时间显示
- 支持深度控制与模式过滤
- 可定制输出格式(ASCII/Unicode)

### 🔌 端口查看
- 显示端口占用进程信息
- 支持显示所有TCP/UDP端口
- 支持过滤监听状态端口
- 可终止占用端口的进程


## 🚀 快速开始

### 安装
```bash
# 通过cargo安装
cargo install --path .
```

### 使用示例
```bash
# 查看当前目录结构(2层深度)
toolboxer tree -d 2

# 查看所有监听端口
toolboxer portown --listen

# 查看TCP连接并显示进程树(3层深度)
toolboxer portown --tcp-only -d 3
```

## ⚙️ 命令参考

### tree 命令
| 参数 | 简写 | 说明 |
|------|------|-----|
| --depth | -d | 设置显示深度 |
| --size | -s | 显示文件大小 |
| --time | -t | 显示修改时间 |
| --pattern | -p | 按模式过滤文件 |

### portown 命令
| 参数 | 简写 | 说明 |
|------|------|-----|
| --tcp-only |  | 仅显示TCP连接 |
| --udp-only |  | 仅显示UDP连接 |
| --depth | -d | 设置显示深度（进程树层级） |
| --established-only | -e | 仅显示已建立的连接 |
| --kill | -k | 终止占用端口的进程 |

## 🤝 参与贡献

欢迎通过以下方式参与项目：
- 提交issue报告问题或建议
- 提交pull request贡献代码
- 完善文档和测试用例

请确保：
1. 代码符合Rust官方编码规范
2. 提交前运行`cargo fmt`和`cargo clippy`
3. 为新功能添加测试用例

---

📜 *许可证：MIT*

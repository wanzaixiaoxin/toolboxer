# Toolboxer

## 核心功能

### 1. 目录分析
- 树形结构展示目录内容
- 支持文件大小、权限、修改时间显示
- 支持深度控制与模式过滤

### 2. 端口查看
- 显示端口占用进程信息
- 支持显示所有TCP/UDP端口
- 支持过滤监听状态端口


## 使用示例

```bash
# 快速查看当前目录结构（2层深度）
toolboxer tree -d 2

# 端口占用
toolboxer portown --listen

```

## 参数说明

### portown 命令
| 参数 | 简写 | 说明 |
|------|------|-----|

| --tcp-only |  | 仅显示TCP连接 |
| --udp-only |  | 仅显示UDP连接 |
| --depth | -d | 设置显示深度（进程树层级） |
| --established-only | -e | 仅显示已建立的连接 |

## 贡献指南

欢迎通过 issue 或 pull request 参与贡献，请确保代码符合Rust官方编码规范。

---

*许可证：MIT*

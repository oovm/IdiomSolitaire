Idiom Solitaire
===============

## Usage

### Web

- 在线成语接龙:
- 在线成语接龙(Gitee):
- 成语接龙杀手:
- 成语接龙杀手(Gitee): 

### 数据库

- 数据库下载:

### Rust

- 查看示例: 

### Python

```py
pyo3 bind
```

## 算法

### 成语接龙模式

```rust
pub enum SolitaireMode {
    /// 同字模式, 需要字符完全相同, 允许多音字
    Character = 0,
    /// 同调模式, 需要发音以及音调相同
    Tone = 1,
    /// 同音模式, 允许音调不同
    Sound = 2,
}
```

### 自由成语接龙

- 贪婪算法: `solver.solve_greedy(input: &str) -> Result<Idiom>`
- 随机算法: `solver.solve_random(input: &str) -> Result<Idiom>`

### 目标接龙搜索

- BFS: `solver.solve_target(input: &str, target: &str) -> Result<Vec<Idiom>>`
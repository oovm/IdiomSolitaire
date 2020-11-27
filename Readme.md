Idiom Solitaire
===============

## Usage

### Web

- 在线成语接龙: [Idiom Solitaire](https://galaster.github.io/IdiomSolitaire) / [Idiom Solitaire(Gitee)](https://notedge.gitee.io/IdiomSolitaire)
- 成语接龙杀手: [Idiom Solitaire Killer](https://galaster.github.io/IdiomSolitaireFuck) / [Idiom Solitaire Killer(Gitee)](https://notedge.gitee.io/IdiomSolitaireFuck)

### 数据库

- 数据库下载: [database.csv](https://github.com/GalAster/IdiomSolitaire/blob/master/projects/external/database.csv)

### Rust

- 查看示例: [idiom-solitaire-core/tests](https://github.com/GalAster/IdiomSolitaire/blob/master/projects/idiom-solitaire-core/tests/main.rs)

### Python

```py
pyo3 bind
```

### Typescript/Javascript

```js
wasm bind
```

## Algorithm

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
# [![Bevy](assets/branding/bevy_logo_light_dark_and_dimmed.svg)](https://bevyengine.org)

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/bevyengine/bevy#license)
[![Crates.io](https://img.shields.io/crates/v/bevy.svg)](https://crates.io/crates/bevy)
[![Downloads](https://img.shields.io/crates/d/bevy.svg)](https://crates.io/crates/bevy)
[![Docs](https://docs.rs/bevy/badge.svg)](https://docs.rs/bevy/latest/bevy/)
[![CI](https://github.com/bevyengine/bevy/workflows/CI/badge.svg)](https://github.com/bevyengine/bevy/actions)
[![Discord](https://img.shields.io/discord/691052431525675048.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/bevy)

## 本分支是Bevy的汉化版本，包括项目内的所有注释和文档，但不包含外链内容。
## Bevy是什么?

Bevy是一个用Rust构建的、全新的简单数据驱动的游戏引擎。永久免费且开源！

## 警告

Bevy还在开发的早期阶段。许多重要特性还缺失着，文档也很少。一个新的Bevy版本发布可能会带来对API破坏性的改变[发布约每三个月一次](https://bevyengine.org/news/bevy-0-6/#the-train-release-schedule)。我们会提供[迁移指引](https://bevyengine.org/learn/migration-guides/)，但我们无法保证每次都可以简单地迁移。仅在你愿意在此前提下工作使用它。

**最小支持Rust版本:** Bevy严重依赖着Rust和编译器的演进。因此，Bevy的最小支持Rust版本一般接近于Rust的“最新的稳定发布版本”。

## 设计目标

* **能力**: 提供一个完整的2D和3D的特性集合
* **简单性**: 新手易入门，老手可以拥有无限的灵活性
* **以数据为重心**: 使用ECS模式的数据驱动结构
* **模块化**: 使用你需要的，替代你不喜欢的
* **快速**: 应用逻辑需要快速运行并尽可能并行化
* **高效性**: 变动需要尽可能快，等待并无乐趣

## 关于

* **[特性](https://bevyengine.org):** Bevy特性的概述。
* **[新闻](https://bevyengine.org/news/)**: 一个包含我们的进度、计划和闪亮的新特性的开发博客。

## 文档

* **[快速开始](https://bevyengine.org/learn/quick-start/introduction):** Bevy官方的快速入门指引。开始学习Bevy最佳的地方。
* **[Bevy Rust API 文档](https://docs.rs/bevy):** Bevy的RustAPI文档，自动生成于本仓库的注释。
* **[官方示例](https://github.com/bevyengine/bevy/tree/latest/examples):** Bevy专门的可运行示例，非常适合深究特定的概念。
* **[社区制作的学习资源](https://bevyengine.org/assets/#learning)**: 更多Bevy社区制作的教程、文档和示例。

## 社区

在参与社区贡献或讨论之前，你应该自我熟悉一下我们的[**行为准则**](./CODE_OF_CONDUCT.md).

* **[Discord](https://discord.gg/bevy):** Bevy的官方Discord。
* **[Reddit](https://reddit.com/r/bevy):** Bevy的官方Reddit。
* **[GitHub Discussions](https://github.com/bevyengine/bevy/discussions):** 关于Bevy的疑问的最佳询问地。
* **[Bevy Assets](https://bevyengine.org/assets/):** 一系列很棒的Bevy项目、工具、插件和学习材料。

### 贡献

如果你想要对Bevy做出贡献，参阅 **[贡献者指南](https://github.com/bevyengine/bevy/blob/main/CONTRIBUTING.md)**.
对于简单的问题，随意 [开个issue](https://github.com/bevyengine/bevy/issues) 或者
[PR](https://github.com/bevyengine/bevy/pulls) 您自己解决!

对于更加复杂的架构决策和实验性质的疯狂科学想法，请开个[RFC](https://github.com/bevyengine/rfcs)（讨论请求），然后我们可以一起进行高效的头脑风暴。
For more complex architecture decisions and experimental mad science, please open an [RFC](https://github.com/bevyengine/rfcs) (Request For Comments) so we can brainstorm together effectively!

## 开始

我们推荐看下 [快速入门指南](https://bevyengine.org/learn/quick-start/introduction) 作为简要的介绍。

沿着 [配置指南](https://bevyengine.org/learn/quick-start/getting-started/setup) 保证你的开发环境得到正确配置。
配置完成后，你可以尝试下[示例](https://github.com/bevyengine/bevy/tree/latest/examples) 通过克隆本仓库并执行一下命令：

```sh
# 切换到正确的版本 (最新发布版，默认开发分支*main*)
git checkout latest
# 运行 "breakout" 示例
cargo run --example breakout
```

用以下代码启用标准功能绘制一个窗口：

```rust
use bevy::prelude::*;

fn main(){
  App::new()
    .add_plugins(DefaultPlugins)
    .run();
}
```

### 快速编译

Bevy可以用默认配置在稳定版本Rust挺好地构建。然而对于非常快速的迭代编译，你需要通过[following the instructions here](https://bevyengine.org/learn/quick-start/getting-started/setup)启用“快速编译”设置。

## [Bevy的cargo特性][cargo_features]

这个 [列表][cargo_features] 概括了Bevy支持的不同cargo特性。它们允许你在你自己的使用场景中自定义Bevy特性集。

[cargo_features]: docs/cargo_features.md

## 致谢

Bevy是很多人努力工作的成果。非常非常感谢所有的Bevy贡献者、许多于我们之前的开源项目、[Rust 游戏开发生态](https://arewegameyet.rs/)，以及我们所依赖的许多库。

非常非常感谢Bevy的[慷慨的赞助者]。Bevy将永远免费且开源，但制作它并非免费。如果你喜欢我们所构建的内容，请考虑[支持我们的工作](https://bevyengine.org/donate/)。

<!-- This next line need to stay exactly as is. It is required for BrowserStack sponsorship. -->
This project is tested with BrowserStack.

## 许可证

Bevy是免费、开源且宽松许可的！
除非另有说明（在下面和/或在单个文件中），此存储库中的所有代码均根据以下任一方式获得双重许可：

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

这意味着你可以选择你倾向的许可证！
这种双重许可方法是 Rust 生态系统中事实上的标准，并且有[非常充分的理由](https://github.com/bevyengine/bevy/issues/2373) 包含这两种方法。

由于其外部来源，某些引擎的代码带有额外的版权声明和许可条款。
这些通常与 BSD 类似，但具体细节因板条箱而异：
如果包的自述文件包含“许可证”标题（或类似的），则将列出适用于该包的附加版权声明和许可条款。
上述许可要求仍然适用于对这些 crate 的贡献，并且这些 crate 的各个部分将带有这些许可条款。
每个板条箱的 [license](https://doc.rust-lang.org/cargo/reference/manifest.html#the-license-and-license-file-fields) 字段也会反映这一点。
例如，[`bevy_mikktspace`](./crates/bevy_mikktspace/README.md#license-agreement) 具有 Zlib 许可证下的代码（以及选择 MIT 许可证时的版权声明）。

此存储库中包含的[资产]（资产）（对于我们的[示例](./examples/README.md)）通常属于不同的开放许可证。
这些不会包含在您的游戏中（除非您复制），并且它们不会分布在已发布的 bevy 板条箱中。
有关这些文件的许可证的详细信息，请参阅 [CREDITS.md](CREDITS.md)。

### 你的贡献

除非您另有明确说明，
您有意提交以包含在作品中的任何贡献，
根据 Apache-2.0 许可证中的定义，
应获得上述双重许可，
没有任何附加条款或条件。

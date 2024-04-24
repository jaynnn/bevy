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

Before contributing or participating in discussions with the community, you should familiarize yourself with our [**Code of Conduct**](./CODE_OF_CONDUCT.md).

* **[Discord](https://discord.gg/bevy):** Bevy's official discord server.
* **[Reddit](https://reddit.com/r/bevy):** Bevy's official subreddit.
* **[GitHub Discussions](https://github.com/bevyengine/bevy/discussions):** The best place for questions about Bevy, answered right here!
* **[Bevy Assets](https://bevyengine.org/assets/):** A collection of awesome Bevy projects, tools, plugins and learning materials.

### Contributing

If you'd like to help build Bevy, check out the **[Contributor's Guide](https://github.com/bevyengine/bevy/blob/main/CONTRIBUTING.md)**.
For simple problems, feel free to [open an issue](https://github.com/bevyengine/bevy/issues) or
[PR](https://github.com/bevyengine/bevy/pulls) and tackle it yourself!

For more complex architecture decisions and experimental mad science, please open an [RFC](https://github.com/bevyengine/rfcs) (Request For Comments) so we can brainstorm together effectively!

## Getting Started

We recommend checking out the [Quick Start Guide](https://bevyengine.org/learn/quick-start/introduction) for a brief introduction.

Follow the [Setup guide](https://bevyengine.org/learn/quick-start/getting-started/setup) to ensure your development environment is set up correctly.
Once set up, you can quickly try out the [examples](https://github.com/bevyengine/bevy/tree/latest/examples) by cloning this repo and running the following commands:

```sh
# Switch to the correct version (latest release, default is main development branch)
git checkout latest
# Runs the "breakout" example
cargo run --example breakout
```

To draw a window with standard functionality enabled, use:

```rust
use bevy::prelude::*;

fn main(){
  App::new()
    .add_plugins(DefaultPlugins)
    .run();
}
```

### Fast Compiles

Bevy can be built just fine using default configuration on stable Rust. However for really fast iterative compiles, you should enable the "fast compiles" setup by [following the instructions here](https://bevyengine.org/learn/quick-start/getting-started/setup).

## [Bevy Cargo Features][cargo_features]

This [list][cargo_features] outlines the different cargo features supported by Bevy. These allow you to customize the Bevy feature set for your use-case.

[cargo_features]: docs/cargo_features.md

## Thanks

Bevy is the result of the hard work of many people. A huge thanks to all Bevy contributors, the many open source projects that have come before us, the [Rust gamedev ecosystem](https://arewegameyet.rs/), and the many libraries we build on.

A huge thanks to Bevy's [generous sponsors](https://bevyengine.org). Bevy will always be free and open source, but it isn't free to make. Please consider [sponsoring our work](https://bevyengine.org/donate/) if you like what we're building.

<!-- This next line need to stay exactly as is. It is required for BrowserStack sponsorship. -->
This project is tested with BrowserStack.

## License

Bevy is free, open source and permissively licensed!
Except where noted (below and/or in individual files), all code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.
This means you can select the license you prefer!
This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to include both.

Some of the engine's code carries additional copyright notices and license terms due to their external origins.
These are generally BSD-like, but exact details vary by crate:
If the README of a crate contains a 'License' header (or similar), the additional copyright notices and license terms applicable to that crate will be listed.
The above licensing requirement still applies to contributions to those crates, and sections of those crates will carry those license terms.
The [license](https://doc.rust-lang.org/cargo/reference/manifest.html#the-license-and-license-file-fields) field of each crate will also reflect this.
For example, [`bevy_mikktspace`](./crates/bevy_mikktspace/README.md#license-agreement) has code under the Zlib license (as well as a copyright notice when choosing the MIT license).

The [assets](assets) included in this repository (for our [examples](./examples/README.md)) typically fall under different open licenses.
These will not be included in your game (unless copied in by you), and they are not distributed in the published bevy crates.
See [CREDITS.md](CREDITS.md) for the details of the licenses of those files.

### Your contributions

Unless you explicitly state otherwise,
any contribution intentionally submitted for inclusion in the work by you,
as defined in the Apache-2.0 license,
shall be dual licensed as above,
without any additional terms or conditions.

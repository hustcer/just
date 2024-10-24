<div align=right>目录↗️</div>

<h1 align="center"><code>just</code></h1>

<div align="center">
  <a href="https://crates.io/crates/just">
    <img src="https://img.shields.io/crates/v/just.svg" alt="crates.io version">
  </a>
  <a href="https://github.com/casey/just/actions">
    <img src="https://github.com/casey/just/actions/workflows/ci.yaml/badge.svg" alt="build status">
  </a>
  <a href="https://github.com/casey/just/releases">
    <img src="https://img.shields.io/github/downloads/casey/just/total.svg" alt="downloads">
  </a>
  <a href="https://discord.gg/ezYScXR">
    <img src="https://img.shields.io/discord/695580069837406228?logo=discord" alt="chat on discord">
  </a>
  <a href="mailto:casey@rodarmor.com?subject=Thanks%20for%20Just!">
    <img src="https://img.shields.io/badge/Say%20Thanks-!-1EAEDB.svg" alt="say thanks">
  </a>
</div>
<br>

`just` 为您提供一种保存和运行项目特有命令的便捷方式。

本指南同时也可以以 [书](https://just.systems/man/zh/) 的形式提供在线阅读。

命令，在此也称为配方，存储在一个名为 `justfile` 的文件中，其语法受 `make` 启发：

![screenshot](https://raw.githubusercontent.com/casey/just/master/screenshot.png)

然后你可以用 `just RECIPE` 运行它们：

```sh
$ just test-all
cc *.c -o main
./test --all
Yay, all your tests passed!
```

`just` 有很多很棒的特性，而且相比 `make` 有很多改进：

- `just` 是一个命令运行器，而不是一个构建系统，所以它避免了许多 [`make` 的复杂性和特异性](#just-避免了-make-的哪些特异性)。不需要 `.PHONY` 配方!

- 支持 Linux、MacOS 和 Windows，而且无需额外的依赖。(尽管如果你的系统没有 `sh`，你需要 [选择一个不同的 Shell](#shell))。

- 错误具体且富有参考价值，语法错误将会与产生它们的上下文一起被报告。

- 配方可以接受 [命令行参数](#配方参数)。

- 错误会尽可能被静态地解决。未知的配方和循环依赖关系会在运行之前被报告。

- `just` 可以 [加载`.env`文件](#环境变量加载)，简化环境变量注入。

- 配方可以在 [命令行中列出](#列出可用的配方)。

- 命令行自动补全脚本 [支持大多数流行的 Shell](#shell-自动补全脚本)。

- 配方可以用 [任意语言](#用其他语言书写配方) 编写，如 Python 或 NodeJS。

- `just` 可以从任何子目录中调用，而不仅仅是包含 `justfile` 的目录。

- 不仅如此，还有 [更多](https://just.systems/man/zh/)！

如果你在使用 `just` 方面需要帮助，请随时创建一个 Issue 或在 [Discord](https://discord.gg/ezYScXR) 上与我联系。我们随时欢迎功能请求和错误报告！

安装
------------

### 预备知识

`just` 应该可以在任何有合适的 `sh` 的系统上运行，包括 Linux、MacOS 和 BSD。

在 Windows 上，`just` 可以使用 [Git for Windows](https://git-scm.com)、[GitHub Desktop](https://desktop.github.com) 或 [Cygwin](http://www.cygwin.com) 所提供的 `sh`。

如果你不愿意安装 `sh`，也可以使用 `shell` 设置来指定你要使用的 Shell。

比如 PowerShell：

```just
# 使用 PowerShell 替代 sh:
set shell := ["powershell.exe", "-c"]

hello:
  Write-Host "Hello, world!"
```

…或者 `cmd.exe`:

```just
# 使用 cmd.exe 替代 sh:
set shell := ["cmd.exe", "/c"]

list:
  dir
```

你也可以使用命令行参数来设置 Shell。例如，若要使用 PowerShell 也可以用 `--shell powershell.exe --shell-arg -c` 启动`just`。

(PowerShell 默认安装在 Windows 7 SP1 和 Windows Server 2008 R2 S1 及更高版本上，而 `cmd.exe` 相当麻烦，所以 PowerShell 被推荐给大多数 Windows 用户)

### 安装包

#### Cross-platform

<table>
  <thead>
    <tr>
      <th>包管理器</th>
      <th>安装包</th>
      <th>命令</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><a href=https://asdf-vm.com>asdf</a></td>
      <td><a href=https://github.com/olofvndrhr/asdf-just>just</a></td>
      <td>
        <code>asdf plugin add just</code><br>
        <code>asdf install just &lt;version&gt;</code>
      </td>
    </tr>
    <tr>
      <td><a href=https://www.rust-lang.org>Cargo</a></td>
      <td><a href=https://crates.io/crates/just>just</a></td>
      <td><code>cargo install just</code></td>
    </tr>
    <tr>
      <td><a href=https://docs.conda.io/projects/conda/en/latest/index.html>Conda</a></td>
      <td><a href=https://anaconda.org/conda-forge/just>just</a></td>
      <td><code>conda install -c conda-forge just</code></td>
    </tr>
    <tr>
      <td><a href=https://brew.sh>Homebrew</a></td>
      <td><a href=https://formulae.brew.sh/formula/just>just</a></td>
      <td><code>brew install just</code></td>
    </tr>
    <tr>
      <td><a href=https://nixos.org/nix/>Nix</a></td>
      <td><a href=https://github.com/NixOS/nixpkgs/blob/master/pkgs/development/tools/just/default.nix>just</a></td>
      <td><code>nix-env -iA nixpkgs.just</code></td>
    </tr>
    <tr>
      <td><a href=https://www.npmjs.com/>npm</a></td>
      <td><a href=https://www.npmjs.com/package/rust-just>rust-just</a></td>
      <td><code>npm install rust-just</code></td>
    </tr>
    <tr>
      <td><a href=https://pypi.org/>PyPI</a></td>
      <td><a href=https://pypi.org/project/rust-just/>rust-just</a></td>
      <td><code>pipx install rust-just</code></td>
    </tr>
  </tbody>
</table>

#### BSD

<table>
  <thead>
    <tr>
      <th>操作系统</th>
      <th>包管理器</th>
      <th>安装包</th>
      <th>命令</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><a href=https://www.freebsd.org>FreeBSD</a></td>
      <td><a href=https://www.freebsd.org/doc/handbook/pkgng-intro.html>pkg</a></td>
      <td><a href=https://www.freshports.org/deskutils/just/>just</a></td>
      <td><code>pkg install just</code></td>
    </tr>
  </tbody>
</table>

#### Linux

<table>
  <thead>
    <tr>
      <th>操作系统</th>
      <th>包管理器</th>
      <th>安装包</th>
      <th>命令</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><a href=https://alpinelinux.org>Alpine</a></td>
      <td><a href=https://wiki.alpinelinux.org/wiki/Alpine_Linux_package_management>apk-tools</a></td>
      <td><a href=https://pkgs.alpinelinux.org/package/edge/community/x86_64/just>just</a></td>
      <td><code>apk add just</code></td>
    </tr>
    <tr>
      <td><a href=https://www.archlinux.org>Arch</a></td>
      <td><a href=https://wiki.archlinux.org/title/Pacman>pacman</a></td>
      <td><a href=https://archlinux.org/packages/extra/x86_64/just/>just</a></td>
      <td><code>pacman -S just</code></td>
    </tr>
    <tr>
      <td>
        <a href=https://debian.org>Debian 13 (unreleased)</a> and
        <a href=https://ubuntu.com>Ubuntu 24.04</a> derivatives</td>
      <td><a href=https://en.wikipedia.org/wiki/APT_(software)>apt</a></td>
      <td><a href=https://packages.debian.org/trixie/just>just</a></td>
      <td><code>apt install just</code></td>
    </tr>
    <tr>
      <td><a href=https://debian.org>Debian</a> and <a href=https://ubuntu.com>Ubuntu</a> derivatives</td>
      <td><a href=https://mpr.makedeb.org>MPR</a></td>
      <td><a href=https://mpr.makedeb.org/packages/just>just</a></td>
      <td>
        <code>git clone https://mpr.makedeb.org/just</code><br>
        <code>cd just</code><br>
        <code>makedeb -si</code>
      </td>
    </tr>
    <tr>
      <td><a href=https://debian.org>Debian</a> and <a href=https://ubuntu.com>Ubuntu</a> derivatives</td>
      <td><a href=https://docs.makedeb.org/prebuilt-mpr>Prebuilt-MPR</a></td>
      <td><a href=https://mpr.makedeb.org/packages/just>just</a></td>
      <td>
        <sup><b>You must have the <a href=https://docs.makedeb.org/prebuilt-mpr/getting-started/#setting-up-the-repository>Prebuilt-MPR set up</a> on your system in order to run this command.</b></sup><br>
        <code>apt install just</code>
      </td>
    </tr>
    <tr>
      <td><a href=https://getfedora.org>Fedora</a></td>
      <td><a href=https://dnf.readthedocs.io/en/latest/>DNF</a></td>
      <td><a href=https://src.fedoraproject.org/rpms/rust-just>just</a></td>
      <td><code>dnf install just</code></td>
    </tr>
    <tr>
      <td><a href=https://www.gentoo.org>Gentoo</a></td>
      <td><a href=https://wiki.gentoo.org/wiki/Portage>Portage</a></td>
      <td><a href=https://github.com/gentoo-mirror/guru/tree/master/dev-build/just>guru/dev-build/just</a></td>
      <td>
        <code>eselect repository enable guru</code><br>
        <code>emerge --sync guru</code><br>
        <code>emerge dev-build/just</code>
      </td>
    </tr>
    <tr>
      <td><a href=https://nixos.org/nixos/>NixOS</a></td>
      <td><a href=https://nixos.org/nix/>Nix</a></td>
      <td><a href=https://github.com/NixOS/nixpkgs/blob/master/pkgs/development/tools/just/default.nix>just</a></td>
      <td><code>nix-env -iA nixos.just</code></td>
    </tr>
    <tr>
      <td><a href=https://opensuse.org>openSUSE</a></td>
      <td><a href=https://en.opensuse.org/Portal:Zypper>Zypper</a></td>
      <td><a href=https://build.opensuse.org/package/show/Base:System/just>just</a></td>
      <td><code>zypper in just</code></td>
    </tr>
    <tr>
      <td><a href=https://getsol.us>Solus</a></td>
      <td><a href=https://getsol.us/articles/package-management/basics/en>eopkg</a></td>
      <td><a href=https://dev.getsol.us/source/just/>just</a></td>
      <td><code>eopkg install just</code></td>
    </tr>
    <tr>
      <td><a href=https://voidlinux.org>Void</a></td>
      <td><a href=https://wiki.voidlinux.org/XBPS>XBPS</a></td>
      <td><a href=https://github.com/void-linux/void-packages/blob/master/srcpkgs/just/template>just</a></td>
      <td><code>xbps-install -S just</code></td>
    </tr>
  </tbody>
</table>

#### Windows

<table>
  <thead>
    <tr>
      <th>包管理器</th>
      <th>安装包</th>
      <th>命令</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><a href=https://chocolatey.org>Chocolatey</a></td>
      <td><a href=https://github.com/michidk/just-choco>just</a></td>
      <td><code>choco install just</code></td>
    </tr>
    <tr>
      <td><a href=https://scoop.sh>Scoop</a></td>
      <td><a href=https://github.com/ScoopInstaller/Main/blob/master/bucket/just.json>just</a></td>
      <td><code>scoop install just</code></td>
    </tr>
    <tr>
      <td><a href=https://learn.microsoft.com/en-us/windows/package-manager/>Windows Package Manager</a></td>
      <td><a href=https://github.com/microsoft/winget-pkgs/tree/master/manifests/c/Casey/Just>Casey/Just</a></td>
      <td><code>winget install --id Casey.Just --exact</code></td>
    </tr>
  </tbody>
</table>

#### macOS

<table>
  <thead>
    <tr>
      <th>包管理器</th>
      <th>安装包</th>
      <th>命令</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><a href=https://www.macports.org>MacPorts</a></td>
      <td><a href=https://ports.macports.org/port/just/summary>just</a></td>
      <td><code>port install just</code></td>
    </tr>
  </tbody>
</table>

![just 软件包版本表](https://repology.org/badge/vertical-allrepos/just.svg)

![rust:just 软件包版本表](https://repology.org/badge/vertical-allrepos/rust:just.svg)

### 预制二进制文件

Linux、MacOS 和 Windows 的预制二进制文件可以在 [发布页](https://github.com/casey/just/releases) 上找到。

你也可以在 Linux、MacOS 或 Windows 上使用下面的命令来下载最新的版本，只需将 `DEST` 替换为你想安装 `just` 的目录即可：

```sh
curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to DEST
```

例如，安装 `just` 到 `~/bin` 目录：

```sh
# 创建 ~/bin
mkdir -p ~/bin

# 下载并解压 just 到 ~/bin/just
curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to ~/bin

# 在 Shell 搜索可执行文件的路径中添加`~/bin`
# 这一行应该被添加到你的 Shell 初始化文件中，e.g. `~/.bashrc` 或者 `~/.zshrc`：
export PATH="$PATH:$HOME/bin"

# 现在 just 应该就可以执行了
just --help
```
请注意，`install.sh` 在 GitHub Actions 或其他许多机器共享 IP 地址的环境中可能会失败。`install.sh` 调用 GitHub API 来确定要安装的 `just` 的最新版本，而这些 API 调用是基于每个 IP 地址进行速率限制的。为了使这种情况下的 `install.sh` 更加可靠，可以通过 `--tag` 参数传递一个特定的标签来安装。

[发布版本](https://github.com/casey/just/releases) 包含一个 `SHA256SUM` 文件，可以用来验证预构建的二进制文件的完整性。

要验证一个发布版本，请下载预构建的二进制文件以及 `SHA256SUM` 文件，并运行：

```sh
shasum --algorithm 256 --ignore-missing --check SHA256SUMS
```

### GitHub Actions

`just` 可以通过几种方式在 GitHub Actions 上安装。

在 MacOS 上使用 GitHub Actions Runners 预安装的包管理器，通过 `brew install just` 进行安装，而在 Windows 上可以使用 `choco install just` 安装。

使用 [extractions/setup-just](https://github.com/extractions/setup-just):

```yaml
- uses: extractions/setup-just@v1
  with:
    just-version: 1.5.0 # 可选的语义化版本规范，否则为最新版本
```

使用 [taiki-e/install-action](https://github.com/taiki-e/install-action):

```yaml
- uses: taiki-e/install-action@just
```

### 发布 RSS 订阅

`just` 的发布 [RSS 订阅](https://en.wikipedia.org/wiki/RSS) 可以在 [这里](https://github.com/casey/just/releases.atom) 找到。

### Node.js 安装

[just-install](https://npmjs.com/package/just-install) 可用于在 Node.js 应用程序中自动安装 `just`。

`just` 是一个很赞的比 npm 脚本更强大的替代品。如果你想在 Node.js 应用程序的依赖中包含 `just`，可以通过 `just-install`，它将在本机安装一个针对特定平台的二进制文件作为 `npm install` 安装结果的一部分。这样就不需要每个开发者使用上述提到的步骤独立安装 `just`。安装后，`just` 命令将在 npm 脚本或 npx 中工作。这对那些想让项目的配置过程尽可能简单的团队来说是很有用的。

想了解更多信息, 请查看 [just-install 说明文件](https://github.com/brombal/just-install#readme)。

向后兼容性
-----------------------

随着 1.0 版本的发布，`just` 突出对向后兼容性和稳定性的强烈承诺。

未来的版本将不会引入向后不兼容的变化，不会使现有的 `justfile` 停止工作，或破坏命令行界面的正常调用。

然而，这并不妨碍修复明显的问题，即使这样做可能会破坏依赖于这些行为的 `justfiles`。

永远不会有一个 `just` 2.0。任何理想的向后不兼容的变更都是在每个 `justfile` 的基础上选择性加入的，所以用户可以在他们的闲暇时间进行迁移。

尚未准备好稳定化的特性被标记为不稳定，并且可能在任何时候被更改或移除。默认情况下，使用不稳定的特性会产生错误，但可以通过传递 `--unstable` 标志、设置 `set unstable` 或设置环境变量 `JUST_UNSTABLE` 为除 `false`、`0` 或空字符串之外的任何值来抑制这个错误。


编辑器支持
--------------

`justfile` 的语法与 `make` 非常接近，你可以让你的编辑器对 `just` 使用 `make` 语法高亮。

### Vim 和 Neovim

#### `vim-just`

[vim-just](https://github.com/NoahTheDuke/vim-just) 插件可以为 vim 提供 `justfile` 语法高亮显示。

你可以用你喜欢的软件包管理器安装它，如 [Plug](https://github.com/junegunn/vim-plug)：

```vim
call plug#begin()

Plug 'NoahTheDuke/vim-just'

call plug#end()
```

或者使用 Vim 的内置包支持：

```sh
mkdir -p ~/.vim/pack/vendor/start
cd ~/.vim/pack/vendor/start
git clone https://github.com/NoahTheDuke/vim-just.git
```

#### `tree-sitter-just`

[tree-sitter-just](https://github.com/IndianBoy42/tree-sitter-just) 是一个针对 Neovim 的 [Nvim Treesitter](https://github.com/nvim-treesitter/nvim-treesitter) 插件。

#### Makefile 语法高亮

Vim 内置的 makefile 语法高亮对 `justfile` 来说并不完美，但总比没有好。你可以把以下内容放在 `~/.vim/filetype.vim` 中：

```vimscript
if exists("did_load_filetypes")
  finish
endif

augroup filetypedetect
  au BufNewFile,BufRead justfile setf make
augroup END
```
或者在单个 `justfile` 中添加以下内容，以在每个文件的基础上启用 `make` 模式：

```text
# vim: set ft=make :
```

### Emacs

[just-mode](https://github.com/leon-barrett/just-mode.el) 可以为 `justfile` 提供语法高亮和自动缩进。它可以在 [MELPA](https://melpa.org/) 上通过 [just-mode](https://melpa.org/#/just-mode) 获得。

[justl](https://github.com/psibi/justl.el) 提供了执行和列出配方的命令。

你可以在一个单独的 `justfile` 中添加以下内容，以便对每个文件启用 `make` 模式：

```text
# Local Variables:
# mode: makefile
# End:
```

### Visual Studio Code

一个适用于 VS Code 的扩展 [可在此获得](https://github.com/nefrob/vscode-just)。

不再维护的 VS Code 扩展包括 [skellock/vscode-just](https://github.com/skellock/vscode-just) 和 [sclu1034/vscode-just](https://github.com/sclu1034/vscode-just)。

### JetBrains IDEs

由 [linux_china](https://github.com/linux-china) 为 JetBrains IDEs 提供的插件可 [由此获得](https://plugins.jetbrains.com/plugin/18658-just)。

### Kakoune

Kakoune 已经内置支持 `justfile` 语法高亮，这要感谢 TeddyDD。

### Helix

[Helix](https://helix-editor.com/) 从 23.05 版本开始内置支持 `justfile` 语法高亮。

### Sublime Text

由 [nk9](https://github.com/nk9) 提供的 [Just 包](https://github.com/nk9/just_sublime) 支持 `just` 语法高亮，同时还有其它工具，这些可以在 [PackageControl](https://packagecontrol.io/packages/Just) 上找到。

### Micro

[Micro](https://micro-editor.github.io/) 内置支持 Justfile 语法高亮，这要感谢 [tomodachi94](https://github.com/tomodachi94)。

### 其它编辑器

欢迎给我发送必要的命令，以便在你选择的编辑器中实现语法高亮，这样我就可以把它们放在这里。

快速开始
-----------

参见 [安装部分](#安装) 了解如何在你的电脑上安装 `just`。试着运行 `just --version` 以确保它被正确安装。

关于语法的概述，请查看这个 [速查表](https://cheatography.com/linux-china/cheat-sheets/justfile/)。

一旦 `just` 安装完毕并开始工作，在你的项目根目录创建一个名为 `justfile` 的文件，内容如下：

```just
recipe-name:
  echo 'This is a recipe!'

# 这是一行注释
another-recipe:
  @echo 'This is another recipe.'
```

当你调用 `just` 时，它会在当前目录和父目录寻找文件 `justfile`，所以你可以从你项目的任何子目录中调用它。

搜索 `justfile` 是不分大小写的，所以任何大小写，如 `Justfile`、`JUSTFILE` 或 `JuStFiLe` 都可以工作。`just` 也会寻找名字为 `.justfile` 的文件，以便你打算隐藏一个 `justfile`。

运行 `just` 时未传参数，则运行 `justfile` 中的第一个配方：

```console
$ just
echo 'This is a recipe!'
This is a recipe!
```

通过一个或多个参数指定要运行的配方：

```console
$ just another-recipe
This is another recipe.
```

`just` 在运行每条命令前都会将其打印到标准错误中，这就是为什么 `echo 'This is a recipe!'` 被打印出来。对于以 `@` 开头的行，这将被抑制，这就是为什么 `echo 'This is another recipe.'` 没有被打印。

如果一个命令失败，配方就会停止运行。这里 `cargo publish` 只有在 `cargo test` 成功后才会运行：

```just
publish:
  cargo test
  # 前面的测试通过才会执行 publish!
  cargo publish
```

配方可以依赖其他配方。在这里，`test` 配方依赖于 `build` 配方，所以 `build` 将在 `test` 之前运行：

```just
build:
  cc main.c foo.c bar.c -o main

test: build
  ./test

sloc:
  @echo "`wc -l *.c` lines of code"
```

```console
$ just test
cc main.c foo.c bar.c -o main
./test
testing… all tests passed!
```

没有依赖关系的配方将按照命令行上给出的顺序运行：

```console
$ just build sloc
cc main.c foo.c bar.c -o main
1337 lines of code
```

依赖项总是先运行，即使它们被放在依赖它们的配方之后：

```console
$ just test build
cc main.c foo.c bar.c -o main
./test
testing… all tests passed!
```

示例
--------

可以在 [示例目录](https://github.com/casey/just/tree/master/examples) 和 [GitHub](https://github.com/search?q=path%3A**%2Fjustfile&type=code) 上找到多个 `justfile` 示例。


特性介绍
--------

### 默认配方

当 `just` 被调用而没有传入任何配方时，它会运行 `justfile` 中的第一个配方。这个配方可能是项目中最常运行的命令，比如运行测试：

```just
test:
  cargo test
```

你也可以使用依赖关系来默认运行多个配方：

```just
default: lint build test

build:
  echo Building…

test:
  echo Testing…

lint:
  echo Linting…
```

在没有合适配方作为默认配方的情况下，你也可以在 `justfile` 的开头添加一个配方，用于列出可用的配方：

```just
default:
  just --list
```

### 列出可用的配方

可以用 `just --list` 按字母顺序列出配方：

```sh
$ just --list
Available recipes:
    build
    test
    deploy
    lint
```

`just --summary` 以更简洁的形式列出配方：

```sh
$ just --summary
build test deploy lint
```

传入 `--unsorted` 选项可以按照它们在 `justfile` 中出现的顺序打印配方：

```just
test:
  echo 'Testing!'

build:
  echo 'Building!'
```

```sh
$ just --list --unsorted
Available recipes:
    test
    build
```

```sh
$ just --summary --unsorted
test build
```

如果你想让 `just` 默认列出 `justfile` 中的配方，你可以使用这个作为默认配方：

```just
default:
  @just --list
```

请注意，你可能需要在上面这一行中添加 `--justfile {{justfile()}}`。没有它，如果你执行 `just -f /some/distant/justfile -d .` 或 `just -f ./non-standard-justfile` 配方中的普通 `just --list` 就不一定会使用你提供的文件，它将试图在你的当前路径中找到一个 `justfile`，甚至可能导致 `No justfile found` 的错误。

标题文本可以用 `--list-heading` 来定制：

```sh
$ just --list --list-heading $'Cool stuff…\n'
Cool stuff…
    test
    build
```

而缩进可以用 `--list-prefix` 来定制：

```sh
$ just --list --list-prefix ····
Available recipes:
····test
····build
```

### 配方参数

配方可以有参数。这里 `build` 有一个参数叫 `target`：

```just
build target:
    @echo 'Building {{target}}…'
    cd {{target}} && make
```

```sh
$ just build my-awesome-project
Building my-awesome-project…
cd my-awesome-project && make
```

参数可以有默认值：

```just
commit message='默认提交信息':
    git commit -m "{{message}}"
```

带默认值的参数可以在调用时省略：

```sh
$ just commit
git commit -m "默认提交信息"
$ just commit "更好的提交信息"
git commit -m "更好的提交信息"
```

参数也可以是可变参数，它们会收集所有剩余的参数：

```just
backup +FILES:
    scp {{FILES}} me@server.com:
```

```sh
$ just backup foo.txt bar.txt baz.txt
scp foo.txt bar.txt baz.txt me@server.com:
```

可变参数也可以有默认值：

```just
test +FLAGS='-q':
    cargo test {{FLAGS}}
```

### 环境变量加载

`just` 会在运行之前自动加载 `.env` 文件中的环境变量。

这些变量可以在配方中使用：

```just
run:
    echo "当前用户是 ${USER}"
```

也可以在配方参数的默认值中使用：

```just
greet name=$USER:
    echo "你好，{{name}}！"
```

### 用其他语言书写配方

配方可以用任何语言编写，只需在配方的第一行添加一个 shebang：

```just
python-hello:
    #!/usr/bin/env python3
    print('Hello from python!')

node-hello:
    #!/usr/bin/env node
    console.log('Hello from node!')

perl-hello:
    #!/usr/bin/env perl
    print "Hello from perl!\n";
```

```sh
$ just python-hello
Hello from python!
$ just node-hello
Hello from node!
$ just perl-hello
Hello from perl!
```

注意：Shebang 行分割在不同操作系统上并不一致。上述例子只在 macOS 上测试过。在 Linux 上，你可能需要给 `env` 传递 `-S` 标志：

```just
#!/usr/bin/env -S just --justfile

default:
  echo foo
```

### 格式化和导出 `justfile`

每个 `justfile` 在空白和换行方面都有一个规范格式。

你可以使用当前不稳定的 `--fmt` 标志用规范格式化的版本覆盖当前的 justfile：

```sh
$ cat justfile
# 很多空行





some-recipe:
  echo "foo"
$ just --fmt --unstable
$ cat justfile
# 很多空行

some-recipe:
    echo "foo"
```

调用 `just --fmt --check --unstable` 会在检查模式下运行 `--fmt`。不是覆盖 `justfile`，`just` 会在格式正确时以退出代码 0 退出，如果格式不正确，则以退出代码 1 退出并打印差异。

你可以使用 `--dump` 命令将格式化后的 `justfile` 版本输出到标准输出：

```sh
$ just --dump > formatted-justfile
```

`--dump` 命令可以与 `--dump-format json` 一起使用，以打印 `justfile` 的 JSON 表示。

### 回退到父级 `justfile`

如果在一个 `justfile` 中找不到配方，并且设置了 `fallback`，`just` 将在父目录中寻找 `justfile`，直到到达根目录。`just` 会在到达一个 `fallback` 设置为 `false` 或未设置的 `justfile` 后停止。

例如，假设当前目录包含这个 `justfile`：

```just
set fallback
foo:
  echo foo
```

而父目录包含这个 `justfile`：

```just
bar:
  echo bar
```

```sh
$ just bar
Trying ../justfile
echo bar
bar
```

### 避免参数分割

给定这个 `justfile`：

```just
foo argument:
  touch {{argument}}
```

以下命令将创建两个文件，`some` 和 `argument.txt`：

```sh
$ just foo "some argument.txt"
```

用户的 shell 会将 `"some argument.txt"` 解析为一个参数，但当 `just` 将 `touch {{argument}}` 替换为 `touch some argument.txt` 时，引号并没有被保留，`touch` 将收到两个参数。

有几种方法可以避免这种情况：引用、位置参数和导出参数。

#### 引用

可以在 `{{argument}}` 插值周围添加引号：

```just
foo argument:
  touch '{{argument}}'
```

这保留了 `just` 在运行前捕获变量名拼写错误的能力，例如如果你写成了 `{{argument}}`，但如果 `argument` 的值包含单引号，这种方法就不会如你所愿。

#### 位置参数

`positional-arguments` 设置会导致所有参数作为位置参数传递，允许使用 `$1`、`$2`、... 和 `$@` 访问它们，然后可以用双引号引起来以避免 shell 进一步分割：

```just
set positional-arguments

foo argument:
  touch "$1"
```

这会使 `just` 无法捕获拼写错误，例如如果你输入 `$2` 而不是 `$1`，但对于所有可能的 `argument` 值都有效，包括那些包含双引号的值。

#### 导出参数

当设置了 `export` 设置时，所有参数都会被导出：

```just
set export

foo argument:
  touch "$argument"
```

或者可以通过在参数前加上 `$` 来导出单个参数：

```just
foo $argument:
  touch "$argument"
```

这会使 `just` 无法捕获拼写错误，例如如果你输入 `$argument`，但对于所有可能的 `argument` 值都有效，包括那些包含双引号的值。

### 配置 Shell

有多种方法可以为逐行配方配置 shell，这是在配方不以 `#!` shebang 开头时的默认设置。它们的优先级从高到低是：

1. `--shell` 和 `--shell-arg` 命令行选项。传递其中任何一个都会导致 `just` 忽略当前 justfile 中的任何设置。
2. `set windows-shell := [...]`
3. `set windows-powershell`（已弃用）
4. `set shell := [...]`

由于 `set windows-shell` 的优先级高于 `set shell`，你可以使用 `set windows-shell` 为 Windows 选择一个 shell，而使用 `set shell` 为所有其他平台选择一个 shell。

### 时间戳

`just` 可以在每个配方命令之前打印时间戳：

```just
recipe:
  echo one
  sleep 2
  echo two
```

```sh
$ just --timestamp recipe
[07:28:46] echo one
one
[07:28:46] sleep 2
[07:28:48] echo two
two
```

默认情况下，时间戳的格式为 `HH:MM:SS`。可以使用 `--timestamp-format` 更改格式：

```sh
$ just --timestamp recipe --timestamp-format '%H:%M:%S%.3f %Z'
[07:32:11:.349 UTC] echo one
one
[07:32:11:.350 UTC] sleep 2
[07:32:13:.352 UTC] echo two
two
```

`--timestamp-format` 的参数是一个 `strftime` 风格的格式字符串，详见 [`chrono` 库文档](https://docs.rs/chrono/latest/chrono/format/strftime/index.html)。

更新日志
---------

最新版本的更新日志可以在 [CHANGELOG.md](https://raw.githubusercontent.com/casey/just/master/CHANGELOG.md) 中找到。以前版本的更新日志可以在 [发布页面](https://github.com/casey/releases) 上找到。`just --changelog` 也可以用来让 `just` 二进制文件打印其更新日志。

杂项
-----------

### 当文件更改时重新运行配方

[`watchexec`](https://github.com/mattgreen/watchexec) 可以在文件更改时重新运行任何命令。

要在任何文件更改时重新运行 `foo` 配方：

```sh
watchexec just foo
```

查看 `watchexec --help` 了解更多信息，包括如何指定应该监视哪些文件的更改。

### 并行运行任务

GNU parallel 可以用来并发运行任务：

```just
parallel:
  #!/usr/bin/env -S parallel --shebang --ungroup --jobs {{ num_cpus() }}
  echo task 1 start; sleep 3; echo task 1 done
  echo task 2 start; sleep 3; echo task 2 done
  echo task 3 start; sleep 3; echo task 3 done
  echo task 4 start; sleep 3; echo task 4 done
```

### Shell 别名

为了快速运行命令，可以在你的 shell 配置文件中添加 `alias j=just`。

在 `bash` 中，别名命令可能无法保持下一节描述的 shell 补全功能。在你的 `.bashrc` 中添加以下行，为你的别名命令使用与 `just` 相同的补全函数：

```sh
complete -F _just -o bashdefault -o default j
```

### Shell 补全脚本

Bash、Elvish、Fish、Nushell、PowerShell 和 Zsh 的 Shell 补全脚本可以在 [发布归档](https://github.com/casey/just/releases) 中找到。

`just` 二进制文件也可以在运行时使用 `just --completions SHELL` 生成相同的补全脚本：

```sh
$ just --completions zsh > just.zsh
```

请参考你的 shell 文档了解如何安装它们。

*macOS 注意事项：* 最新版本的 macOS 使用 zsh 作为默认 shell。如果你使用 Homebrew 安装 `just`，它会自动在 Homebrew zsh 目录中安装最新版本的 zsh 补全脚本，但内置版本的 zsh 默认不知道这个目录。最好使用这个脚本的副本（如果可能的话），因为它会在你通过 Homebrew 更新 `just` 时更新。此外，许多其他 Homebrew 包也在相同的位置使用补全脚本，内置的 zsh 也不知道这些。要在这种情况下在 zsh 中利用 `just` 补全，你可以在调用 `compinit` 之前将 `fpath` 设置为 Homebrew 位置。还要注意，Oh My Zsh 默认运行 `compinit`。所以你的 `.zshrc` 文件可能看起来像这样：

```zsh
# 初始化 Homebrew，它会添加环境变量
eval "$(brew shellenv)"

fpath=($HOMEBREW_PREFIX/share/zsh/site-functions $fpath)

# 然后选择以下选项之一：
# 1. 如果你使用 Oh My Zsh，你可以在这里初始化它
# source $ZSH/oh-my-zsh.sh

# 2. 否则，自己运行 compinit
# autoload -U compinit
# compinit
```

### 手册页

`just` 可以用 `just --man` 打印自己的手册页。手册页是用 [`roff`](https://en.wikipedia.org/wiki/Roff_%28software%29) 编写的，这是一种古老的标记语言，也是 Unix 最早的实际应用之一。如果你安装了 [`groff`](https://www.gnu.org/software/groff/)，你可以用 `just --man | groff -mandoc -Tascii | less` 查看手册页。

### 语法

`justfile` 的非规范语法可以在 [GRAMMAR.md](https://github.com/casey/just/blob/master/GRAMMAR.md) 中找到。

### just.sh

在 `just` 成为一个花哨的 Rust 程序之前，它是一个调用 `make` 的小型 shell 脚本。你可以在 [contrib/just.sh](https://github.com/casey/just/blob/master/contrib/just.sh) 中找到旧版本。

### 全局和用户 `justfile`

如果你想让一些配方在任何地方都可用，你有几个选择。

#### 全局 Justfile

`just --global-justfile`，或简写为 `just -g`，会按顺序搜索以下路径的 justfile：

- `$XDG_CONFIG_HOME/just/justfile`
- `$HOME/.config/just/justfile`
- `$HOME/justfile`
- `$HOME/.justfile`

你可以把在许多项目中使用的配方放在全局 justfile 中，以便从任何目录轻松调用它们。

#### 用户 justfile 技巧

你也可以采用以下一些工作流程。这些技巧假设你在 `~/.user.justfile` 创建了一个 `justfile`，但你可以把这个 `justfile` 放在系统上任何方便的路径。

##### 配方别名

如果你想按名称调用 `~/.user.justfile` 中的配方，并且不介意为每个配方创建一个别名，可以在你的 shell 初始化脚本中添加以下内容：

```sh
for recipe in `just --justfile ~/.user.justfile --summary`; do
  alias $recipe="just --justfile ~/.user.justfile --working-directory . $recipe"
done
```

现在，如果你在 `~/.user.justfile` 中有一个叫做 `foo` 的配方，你只需在命令行输入 `foo` 就可以运行它。

我花了太长时间才意识到你可以这样创建配方别名。尽管我来得晚，但我很高兴为你带来这个 `justfile` 技术的重大进步。

##### 转发别名

如果你不想为每个配方创建别名，你可以创建一个单一的别名：

```sh
alias .j='just --justfile ~/.user.justfile --working-directory .'
```

现在，如果你在 `~/.user.justfile` 中有一个叫做 `foo` 的配方，你只需在命令行输入 `.j foo` 就可以运行它。

我很确定没有人真正使用这个功能，但它就在那里。

¯\\\_(ツ)\_/¯

##### 自定义

你可以用额外的选项自定义上述别名。例如，如果你更喜欢让 `justfile` 中的配方在你的主目录而不是当前目录中运行：

```sh
alias .j='just --justfile ~/.user.justfile --working-directory ~'
```

### Node.js `package.json` 脚本兼容性

以下导出语句使 `just` 配方可以访问本地 Node 模块二进制文件，并使 `just` 配方命令的行为更像 Node.js `package.json` 文件中的 `script` 条目：

```just
export PATH := "./node_modules/.bin:" + env_var('PATH')
```

### Windows 上的路径

在 Windows 上，返回路径的函数将返回 `\` 分隔的路径。当不使用 PowerShell 或 `cmd.exe` 时，这些路径应该被引用以防止 `\` 被解释为字符转义：

```just
ls:
    echo '{{absolute_path(".")}}'
```

### 远程 Justfiles

如果你希望在许多 `justfile` 中包含一个 `mod` 或 `import` 源文件而不需要复制它，你可以使用可选的 `mod` 或 `import`，以及一个获取模块源的配方：

```just
import? 'foo.just'

fetch:
  curl https://raw.githubusercontent.com/casey/just/master/justfile > foo.just
```

给定上面的 `justfile`，运行 `just fetch` 后，`foo.just` 中的配方将可用。

### 替代品和先前的工作

命令运行器并不缺乏！一些或多或少类似于 `just` 的替代品包括：

- [make](https://en.wikipedia.org/wiki/Make_(software))：启发 `just` 的 Unix 构建工具。原始 `make` 有几个现代的后代，包括 [FreeBSD Make](https://www.freebsd.org/cgi/man.cgi?make(1)) 和 [GNU Make](https://www.gnu.org/software/make/)。
- [task](https://github.com/go-task/task)：用 Go 编写的基于 YAML 的命令运行器。
- [maid](https://github.com/egoist/maid)：用 JavaScript 编写的基于 Markdown 的命令运行器。
- [microsoft/just](https://github.com/microsoft/just)：用 JavaScript 编写的基于 JavaScript 的命令运行器。
- [cargo-make](https://github.com/sagiegurari/cargo-make)：Rust 项目的命令运行器。
- [mmake](https://github.com/tj/mmake)：`make` 的包装器，有许多改进，包括远程包含。
- [robo](https://github.com/tj/robo)：用 Go 编写的基于 YAML 的命令运行器。
- [mask](https://github.com/jakedeichert/mask)：用 Rust 编写的基于 Markdown 的命令运行器。
- [makesure](https://github.com/xonixx/makesure)：用 AWK 和 shell 编写的简单且可移植的命令运行器。
- [haku](https://github.com/VladimirMarkelov/haku)：用 Rust 编写的类似 make 的命令运行器。

贡献
------------

`just` 欢迎你的贡献！`just` 在最大程度上宽松的 [CC0](https://creativecommons.org/publicdomain/zero/1.0/legalcode.txt) 公共领域贡献和后备许可下发布，所以你的更改也必须在此许可下发布。

### 入门

`just` 是用 Rust 编写的。使用 [rustup](https://www.rust-lang.org/tools/install) 安装 Rust 工具链。

`just` 经过广泛测试。所有新功能都必须由单元测试或集成测试覆盖。单元测试位于 [src](https://github.com/casey/just/blob/master/src) 下，与被测试的代码放在一起，并单独测试代码。集成测试在 [tests 目录](https://github.com/casey/just/blob/master/tests) 中，通过在给定的 `justfile` 和命令行参数集上调用 `just` 并检查输出来从外部测试 `just` 二进制文件。

你应该编写对你的功能最容易编写的测试类型，同时仍提供良好的测试覆盖率。

单元测试对于测试内部使用的新 Rust 函数和作为开发辅助很有用。一个很好的例子是覆盖 [`unindent()` 函数](https://github.com/casey/just/blob/master/src/unindent.rs) 的单元测试，该函数用于取消三引号字符串和反引号的缩进。`unindent()` 有很多棘手的边缘情况，这些情况很容易用直接调用 `unindent()` 的单元测试来测试。

集成测试对于确保 `just` 二进制文件的最终行为是正确的很有用。`unindent()` 也被集成测试覆盖，这些测试确保评估三引号字符串产生正确的未缩进值。然而，并非所有可能的情况都有集成测试。这些由更快、更简洁的直接调用 `unindent()` 的单元测试覆盖。

现有的集成测试有两种形式，使用 `test!` 宏的和直接使用 `Test` 结构的。`test!` 宏虽然通常简洁，但灵活性较差且难以理解，所以新测试应该使用 `Test` 结构。`Test` 结构是一个构建器，允许轻松地用给定的 `justfile`、参数和环境变量调用 `just`，并检查程序的标准输出、标准错误和退出代码。

### 贡献工作流程

1. 确保该功能是需要的。应该有一个关于该功能的开放 issue，其中有来自 [@casey](https://github.com/casey) 的评论说这是一个好主意或看起来合理。如果没有，打开一个新的 issue 并寻求反馈。

   有很多好功能无法合并，要么是因为它们不向后兼容，要么是因为它们的实现会使代码库过于复杂，或者违背了 `just` 的设计理念。

2. 确定功能的设计。如果该功能有多种可能的实现或语法，请确保在 issue 中确定细节。

3. 克隆 `just` 并开始编码。最好的工作流程是在一个编辑器中打开你正在处理的代码，同时运行一个在文件更改时重新运行测试的作业。你可以通过安装 [cargo-watch](https://github.com/watchexec/cargo-watch)（使用 `cargo install cargo-watch`）并运行 `just watch test` 来运行这样一个作业。

4. 为你的功能添加一个失败的测试。大多数情况下，这将是一个端到端测试该功能的集成测试。在 [tests](https://github.com/casey/just/blob/master/tests) 中寻找一个适当的文件来放置测试，或者在 [tests](https://github.com/casey/just/blob/master/tests) 中添加一个新文件，并在 [tests/lib.rs](https://github.com/casey/just/blob/master/tests/lib.rs) 中添加一个 `mod` 语句导入该文件。

5. 实现该功能。

6. 运行 `just ci` 以确保所有测试、lint 和检查都通过。

7. 打开一个可由维护者编辑的包含新代码的 PR。PR 经常需要重新基础和小的调整。如果 PR 不能由维护者编辑，每次重新基础和调整都需要一轮代码审查。如果你的 PR 不能由维护者编辑，它可能会被直接关闭。

8. 纳入反馈。

9. 享受你的 PR 被合并的甜蜜感觉！

随时可以打开一个草稿 PR 进行讨论和反馈。

### 提示

这里有一些提示可以帮助你开始特定类型的新功能，你可以在上述贡献工作流程之外使用这些提示。

#### 添加新属性

1. 在 [tests/attributes.rs](https://github.com/casey/just/blob/master/tests/attributes.rs) 中编写一个新的集成测试。

2. 在 [`Attribute`](https://github.com/casey/just/blob/master/src/attribute.rs) 枚举中添加一个新的变体。

3. 实现新属性的功能。

4. 运行 `just ci` 以确保所有测试都通过。

### Janus

[Janus](https://github.com/casey/janus) 是一个用于检查对 `just` 的更改是否会破坏或改变现有 `justfile` 解释的工具。它收集并分析 GitHub 上的公共 `justfile`。

在合并特别大或复杂的更改之前，应该运行 Janus 以确保没有任何东西被破坏。不用担心自己运行 Janus，Casey 会很乐意为需要的更改运行它。

### 最低支持的 Rust 版本

最低支持的 Rust 版本，或 MSRV，是当前稳定的 Rust。它可能在旧版本的 Rust 上构建，但这不能保证。

### 新版本

`just` 的新版本经常发布，以便用户能够快速获得新功能。

发布提交消息使用以下模板：

```
Release x.y.z

- 版本升级：x.y.z → x.y.z
- 更新更新日志
- 更新更新日志贡献者致谢
- 更新依赖
- 更新 readme 中的版本引用
```

常见问题
--------------------------

### Just 避免了 Make 的哪些特异性？

`make` 有一些行为令人困惑、复杂或使其不适合用作通用命令运行器。

一个例子是在某些情况下，`make` 实际上不会运行配方中的命令。例如，如果你有一个名为 `test` 的文件和以下 makefile：

```just
test:
  ./test
```

`make` 将拒绝运行你的测试：

```sh
$ make test
make: `test' is up to date.
```

`make` 假设 `test` 配方会产生一个名为 `test` 的文件。由于这个文件已经存在，并且配方没有其他依赖项，`make` 认为它没有什么可做的，就退出了。

公平地说，当使用 `make` 作为构建系统时，这种行为是可取的，但当将其用作命令运行器时则不然。你可以使用 `make` 内置的 [`.PHONY` 目标名称](https://www.gnu.org/software/make/manual/html_node/Phony-Targets.html) 为特定目标禁用此行为，但语法冗长且难以记住。显式的假目标列表，与配方定义分开写，也引入了意外定义新的非假目标的风险。在 `just` 中，所有配方都被视为假目标。

`make` 特异性的其他例子包括赋值中 `=` 和 `:=` 的区别，如果你搞砸了 makefile 会产生令人困惑的错误消息，在配方中需要 `$$` 来使用环境变量，以及不同版本的 `make` 之间的不兼容性。

### Just 和 Cargo 构建脚本之间有什么关系？

[`cargo` 构建脚本](http://doc.crates.io/build-script.html)有一个相当具体的用途，就是控制 `cargo` 如何构建你的 Rust 项目。这可能包括向 `rustc` 调用添加标志、构建外部依赖项，或运行某种代码生成步骤。

另一方面，`just` 用于开发过程中可能运行的所有其他杂项命令。比如在不同配置下运行测试、检查代码、将构建产物推送到服务器、删除临时文件等。

此外，尽管 `just` 是用 Rust 编写的，但它可以用于任何语言或构建系统的项目。

进一步的思考
-----------------

我个人发现，为几乎每个项目（无论大小）编写一个 `justfile` 都非常有用。

对于有多个贡献者的大型项目，将所有处理项目所需的命令放在一个随手可得的文件中非常有用。

可能有不同的命令用于测试、构建、检查、部署等，将它们都放在一个地方很有用，可以减少你告诉人们运行哪些命令以及如何输入这些命令的时间。

而且，有了一个放置命令的简单位置，你可能会想出其他有用的东西，这些东西是项目集体智慧的一部分，但没有写在任何地方，比如某些版本控制工作流程所需的神秘命令，安装项目的所有依赖项，或者可能需要传递给构建系统的所有随机标志。

一些配方的想法：

- 部署/发布项目

- 在发布模式与调试模式下构建

- 在调试模式下运行或启用日志记录

- 复杂的 git 工作流程

- 更新依赖项

- 运行不同的测试集，例如快速测试与慢速测试，或者使用详细输出运行它们

- 任何你真的应该写下来的复杂命令集，哪怕只是为了能够记住它们

即使对于小型的个人项目，能够通过名称记住命令而不是反向搜索你的 shell 历史也很好，而且能够进入一个用随机语言编写的旧项目，带有神秘的构建系统，并知道你需要做任何事情的所有命令都在 `justfile` 中，这是一个巨大的福音，如果你输入 `just`，可能会发生一些有用（或至少有趣！）的事情。

关于配方的想法，可以查看[这个项目的 `justfile`](https://github.com/casey/just/blob/master/justfile)，或者一些[野外的 `justfile`](https://github.com/search?q=path%3A**%2Fjustfile&type=code)。

总之，我想这就是这个极其冗长的 README 的全部内容了。

我希望你喜欢使用 `just`，并在所有计算endeavors中取得巨大成功和满足感！

😸

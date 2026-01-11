<div align=right>目录↗️</div>

<h1 align=center><code>just</code></h1>

<div align=center>
  <a href=https://crates.io/crates/just>
    <img src=https://img.shields.io/crates/v/just.svg alt="crates.io version">
  </a>
  <a href=https://github.com/casey/just/actions/workflows/ci.yaml>
    <img src=https://github.com/casey/just/actions/workflows/ci.yaml/badge.svg alt="build status">
  </a>
  <a href=https://github.com/casey/just/releases>
    <img src=https://img.shields.io/github/downloads/casey/just/total.svg alt=downloads>
  </a>
  <a href=https://discord.gg/ezYScXR>
    <img src=https://img.shields.io/discord/695580069837406228?logo=discord alt="chat on discord">
  </a>
  <a href=mailto:casey@rodarmor.com?subject=Thanks%20for%20Just!>
    <img src=https://img.shields.io/badge/Say%20Thanks-!-1EAEDB.svg alt="say thanks">
  </a>
</div>
<br>

`just` 为您提供一种保存和运行项目特有命令的便捷方式。

本文档也可以作为[在线手册](https://just.systems/man/en/)阅读。在线手册反映的是最新发布版本，而 [GitHub 上的 README](https://github.com/casey/just/blob/master/README.md) 反映的是最新的 master 分支。

(English version is available [here](https://github.com/casey/just/blob/master/README.md), check it out!)

命令（称为配方）存储在名为 `justfile` 的文件中，其语法受 `make` 启发：

![screenshot](https://raw.githubusercontent.com/casey/just/master/screenshot.png)

然后你可以用 `just RECIPE` 来运行它们：

```console
$ just test-all
cc *.c -o main
./test --all
Yay, all your tests passed!
```

`just` 有大量实用的功能，相比 `make` 有很多改进：

- `just` 是一个命令运行器，而不是构建系统，因此它避免了 [`make` 的许多复杂性和怪异行为](#just-避免了-make-的哪些特性)。无需 `.PHONY` 配方！

- 支持 Linux、MacOS、Windows 及其他类 Unix 系统，无需额外依赖。（不过如果你的系统没有 `sh`，则需要[选择其他 shell](#shell)。）

- 错误输出具体且信息丰富，语法错误会连同其源代码上下文一起报告。

- 配方可以接受[命令行参数](#配方参数)。

- 错误尽可能在静态阶段就被检测出来。未知配方和循环依赖会在实际运行前就被报告。

- `just` 可[加载 `.env` 文件](#dotenv-设置)，方便配置环境变量。

- 可以从[命令行列出](#列出可用配方)配方。

- 命令行补全脚本[适用于大多数流行的 shell](#shell-补全脚本)。

- 配方可以用[任意语言](#shebang-配方)编写，如 Python 或 NodeJS。

- `just` 可以从任何子目录调用，而不仅仅是包含 `justfile` 的目录。

- 还有[更多功能](https://just.systems/man/en/)！

如果你需要 `just` 相关帮助，请随时创建一个 issue 或在 [Discord](https://discord.gg/ezYScXR) 上联系我。功能请求和错误报告随时欢迎！

安装
----

### 前提条件

`just` 可以在任何具备标准 `sh` 的系统上运行，包括 Linux、macOS 和 BSD 系列。

#### Windows

在 Windows 上，`just` 可以与 [Git for Windows](https://git-scm.com)、[GitHub Desktop](https://desktop.github.com) 或 [Cygwin](http://www.cygwin.com) 提供的 `sh` 一起工作。安装后，`sh` 必须在你想要调用 `just` 的 shell 的 `PATH` 中可用。

如果不想安装 `sh`，可以通过 `shell` 设置指定所用的 shell。

比如 PowerShell：

```just
# use PowerShell instead of sh:
set shell := ["powershell.exe", "-c"]

hello:
  Write-Host "Hello, world!"
```

…或者 `cmd.exe`：

```just
# use cmd.exe instead of sh:
set shell := ["cmd.exe", "/c"]

list:
  dir
```

你也可以使用命令行参数设置 shell。例如，要使用 PowerShell，用 `--shell powershell.exe --shell-arg -c` 启动 `just`。

（PowerShell 默认安装在 Windows 7 SP1 和 Windows Server 2008 R2 S1 及更高版本上，而 `cmd.exe` 相当繁琐，所以对于大多数 Windows 用户推荐使用 PowerShell。）

### 软件包

#### 跨平台

<table>
  <thead>
    <tr>
      <th>包管理器</th>
      <th>软件包</th>
      <th>命令</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td><a href=https://github.com/alexellis/arkade>arkade</a></td>
      <td>just</td>
      <td><code>arkade get just</code></td>
    </tr>
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
      <td><a href=https://github.com/NixOS/nixpkgs/blob/master/pkgs/by-name/ju/just/package.nix>just</a></td>
      <td><code>nix-env -iA nixpkgs.just</code></td>
    </tr>
    <tr>
      <td><a href=https://www.npmjs.com/>npm</a></td>
      <td><a href=https://www.npmjs.com/package/rust-just>rust-just</a></td>
      <td><code>npm install -g rust-just</code></td>
    </tr>
    <tr>
      <td><a href=https://pipx.pypa.io/stable/>pipx</a></td>
      <td><a href=https://pypi.org/project/rust-just/>rust-just</a></td>
      <td><code>pipx install rust-just</code></td>
    </tr>
    <tr>
      <td><a href=https://snapcraft.io>Snap</a></td>
      <td><a href=https://snapcraft.io/just>just</a></td>
      <td><code>snap install --edge --classic just</code></td>
    </tr>
  </tbody>
</table>

#### BSD

<table>
  <thead>
    <tr>
      <th>操作系统</th>
      <th>包管理器</th>
      <th>软件包</th>
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
    <tr>
      <td><a href=https://www.openbsd.org>OpenBSD</a></td>
      <td><a href=https://www.openbsd.org/faq/faq15.html>pkg_*</a></td>
      <td><a href=https://cvsweb.openbsd.org/cgi-bin/cvsweb/ports/sysutils/just>just</a></td>
      <td><code>pkg_add just</code></td>
    </tr>
  </tbody>
</table>

#### Linux

<table>
  <thead>
    <tr>
      <th>操作系统</th>
      <th>包管理器</th>
      <th>软件包</th>
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
        <a href=https://debian.org>Debian 13</a> 和
        <a href=https://ubuntu.com>Ubuntu 24.04</a> 衍生版</td>
      <td><a href=https://en.wikipedia.org/wiki/APT_(software)>apt</a></td>
      <td><a href=https://packages.debian.org/trixie/just>just</a></td>
      <td><code>apt install just</code></td>
    </tr>
    <tr>
      <td><a href=https://debian.org>Debian</a> 和 <a href=https://ubuntu.com>Ubuntu</a> 衍生版</td>
      <td><a href=https://mpr.makedeb.org>MPR</a></td>
      <td><a href=https://mpr.makedeb.org/packages/just>just</a></td>
      <td>
        <code>git clone https://mpr.makedeb.org/just</code><br>
        <code>cd just</code><br>
        <code>makedeb -si</code>
      </td>
    </tr>
    <tr>
      <td><a href=https://debian.org>Debian</a> 和 <a href=https://ubuntu.com>Ubuntu</a> 衍生版</td>
      <td><a href=https://docs.makedeb.org/prebuilt-mpr>Prebuilt-MPR</a></td>
      <td><a href=https://mpr.makedeb.org/packages/just>just</a></td>
      <td>
        <sup><b>你必须先在系统上<a href=https://docs.makedeb.org/prebuilt-mpr/getting-started/#setting-up-the-repository>设置 Prebuilt-MPR</a> 才能运行此命令。</b></sup><br>
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
      <td><a href=https://github.com/NixOS/nixpkgs/blob/master/pkgs/by-name/ju/just/package.nix>just</a></td>
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
      <th>软件包</th>
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
      <th>软件包</th>
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

![just package version table](https://repology.org/badge/vertical-allrepos/just.svg)

### 预编译二进制文件

适用于 Linux、macOS 和 Windows 的预编译二进制文件可在[发布页面](https://github.com/casey/just/releases)下载。

你可以使用以下命令在 Linux、macOS 或 Windows 上下载最新版本，只需将 `DEST` 替换为你想要安装 `just` 的目录：

```console
curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to DEST
```

例如，要将 `just` 安装到 `~/bin`：

```console
# create ~/bin
mkdir -p ~/bin

# download and extract just to ~/bin/just
curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to ~/bin

# add `~/bin` to the paths that your shell searches for executables
# this line should be added to your shells initialization file,
# e.g. `~/.bashrc` or `~/.zshrc`
export PATH="$PATH:$HOME/bin"

# just should now be executable
just --help
```

请注意，`install.sh` 可能在 GitHub Actions 或其他许多机器共享 IP 地址的环境中失败。`install.sh` 调用 GitHub API 来确定要安装的 `just` 最新版本，这些 API 调用按 IP 地址进行速率限制。为了在这种情况下使 `install.sh` 更可靠，可以使用 `--tag` 传递特定的标签进行安装。

另一种避免速率限制的方法是将 GitHub 身份验证令牌作为名为 `GITHUB_TOKEN` 的环境变量传递给 `install.sh`，允许它对其请求进行身份验证。

[发布版本](https://github.com/casey/just/releases)包含一个 `SHA256SUM` 文件，可用于验证预编译二进制归档文件的完整性。

要验证发布版本，请下载预编译二进制归档文件以及 `SHA256SUM` 文件并运行：

```sh
shasum --algorithm 256 --ignore-missing --check SHA256SUMS
```

### GitHub Actions

`just` 可以通过几种方式在 GitHub Actions 上安装。

在 macOS 上使用 GitHub Actions 运行器预装的包管理器 `brew install just`，在 Windows 上使用 `choco install just`。

使用 [extractions/setup-just](https://github.com/extractions/setup-just)：

```yaml
- uses: extractions/setup-just@v3
  with:
    just-version: 1.5.0  # optional semver specification, otherwise latest
```

或者使用 [taiki-e/install-action](https://github.com/taiki-e/install-action)：

```yaml
- uses: taiki-e/install-action@just
```

### 发布 RSS 订阅

`just` 发布的 [RSS 订阅](https://en.wikipedia.org/wiki/RSS)可以在[这里](https://github.com/casey/just/releases.atom)获取。

### Node.js 安装

[just-install](https://npmjs.com/package/just-install) 可用于在 Node.js 应用程序中自动安装 `just`。

`just` 是一款出色且更稳定的 npm 脚本替代工具。如果你想在 Node.js 项目的依赖中包含 `just`，使用 `just-install` 会在执行 `npm install` 时自动安装适配当前平台的二进制版本。这样就无需每位开发者单独安装 `just`。安装后，`just` 命令可在 npm 脚本中或通过 npx 直接运行。对于希望简化项目搭建流程的团队来说，这非常实用。

更多信息，请参阅 [just-install README 文件](https://github.com/brombal/just-install#readme)。

向后兼容性
----------

随着 1.0 版本的发布，`just` 对向后兼容性和稳定性做出了强有力的承诺。

未来的版本不会引入使现有 `justfile` 停止工作或破坏命令行界面正常调用的向后不兼容更改。

然而，这并不排除修复明显的错误，即使这样做可能会破坏依赖于这些行为的 `justfile`。

永远不会有 `just` 2.0。任何需要的向后不兼容更改都将基于每个 `justfile` 选择性启用，因此用户可以在方便时进行迁移。

尚未准备好稳定化的功能被标记为不稳定，可能随时更改或删除。默认情况下，使用不稳定功能会产生错误，可以通过传递 `--unstable` 标志、`set unstable` 或将环境变量 `JUST_UNSTABLE` 设置为 `false`、`0` 或空字符串以外的任何值来抑制。

编辑器支持
----------

`justfile` 语法与 `make` 足够接近，你可以告诉你的编辑器对 `just` 使用 `make` 语法高亮。

### Vim 和 Neovim

Vim 9.1.1042 或更高版本以及 Neovim 0.11 或更高版本开箱即用地支持 Justfile 语法高亮，感谢 [pbnj](https://github.com/pbnj)。

#### `vim-just`

[vim-just](https://github.com/NoahTheDuke/vim-just) 插件为 `justfile` 提供语法高亮。

使用你喜欢的包管理器安装它，比如 [Plug](https://github.com/junegunn/vim-plug)：

```vim
call plug#begin()

Plug 'NoahTheDuke/vim-just'

call plug#end()
```

或者使用 Vim 的内置包支持：

```console
mkdir -p ~/.vim/pack/vendor/start
cd ~/.vim/pack/vendor/start
git clone https://github.com/NoahTheDuke/vim-just.git
```

#### `tree-sitter-just`

[tree-sitter-just](https://github.com/IndianBoy42/tree-sitter-just) 是一个用于 Neovim 的 [Nvim Treesitter](https://github.com/nvim-treesitter/nvim-treesitter) 插件。

#### Makefile 语法高亮

Vim 内置的 makefile 语法高亮对于 `justfile` 来说并不完美，但总比没有好。你可以将以下内容放入 `~/.vim/filetype.vim`：

```vimscript
if exists("did_load_filetypes")
  finish
endif

augroup filetypedetect
  au BufNewFile,BufRead justfile setf make
augroup END
```

或者将以下内容添加到单个 `justfile` 中，以便按文件启用 `make` 模式：

```text
# vim: set ft=make :
```

### Emacs

[just-mode](https://github.com/leon-barrett/just-mode.el) 为 `justfile` 提供语法高亮和自动缩进。它在 [MELPA](https://melpa.org/) 上以 [just-mode](https://melpa.org/#/just-mode) 的形式提供。

[justl](https://github.com/psibi/justl.el) 提供执行和列出配方的命令。

你可以将以下内容添加到单个 `justfile` 中，以便按文件启用 `make` 模式：

```text
# Local Variables:
# mode: makefile
# End:
```

### Visual Studio Code

VS Code 的扩展[可在这里获取](https://github.com/nefrob/vscode-just)。

不再维护的 VS Code 扩展包括 [skellock/vscode-just](https://github.com/skellock/vscode-just) 和 [sclu1034/vscode-just](https://github.com/sclu1034/vscode-just)。

### JetBrains IDEs

[linux_china](https://github.com/linux-china) 为 JetBrains IDEs 开发的插件[可在这里获取](https://plugins.jetbrains.com/plugin/18658-just)。

### Kakoune

Kakoune 原生支持 `justfile` 语法高亮，感谢 TeddyDD。

### Helix

[Helix](https://helix-editor.com/) 从 23.05 版本起原生支持 `justfile` 语法高亮。

### Sublime Text

由 [nk9](https://github.com/nk9) 开发的 [Just package](https://github.com/nk9/just_sublime) 包含 `just` 语法和一些其他工具，可在 [PackageControl](https://packagecontrol.io/packages/Just) 上获取。

### Micro

[Micro](https://micro-editor.github.io/) 原生支持 Justfile 语法高亮，感谢 [tomodachi94](https://github.com/tomodachi94)。

### Zed

由 [jackTabsCode](https://github.com/jackTabsCode) 开发的 [zed-just](https://github.com/jackTabsCode/zed-just/) 扩展可在 [Zed 扩展页面](https://zed.dev/extensions?query=just)获取。

### 其他编辑器

欢迎发送在你选择的编辑器中启用语法高亮所需的命令给我，以便我可以将它们包含在这里。

### 语言服务器协议

[just-lsp](https://github.com/terror/just-lsp) 提供了[语言服务器协议](https://en.wikipedia.org/wiki/Language_Server_Protocol)实现，启用了跳转到定义、内联诊断和代码补全等功能。

### 模型上下文协议

[just-mcp](http://github.com/promptexecution/just-mcp) 提供了[模型上下文协议](https://en.wikipedia.org/wiki/Model_Context_Protocol)适配器，允许 LLM 查询 `justfile` 的内容并运行配方。

快速开始
--------

参阅[安装](#安装)部分了解如何在你的计算机上安装 `just`。运行 `just --version` 确保安装正确。

有关语法概述，请查看[这个速查表](https://cheatography.com/linux-china/cheat-sheets/justfile/)。

一旦 `just` 安装并正常工作，在你的项目根目录创建一个名为 `justfile` 的文件，内容如下：

```just
recipe-name:
  echo 'This is a recipe!'

# this is a comment
another-recipe:
  @echo 'This is another recipe.'
```

当你调用 `just` 时，它会在当前目录及其父目录中查找 `justfile` 文件，因此你可以从项目的任何子目录调用它。

`justfile` 的搜索不区分大小写，所以任何大小写形式，如 `Justfile`、`JUSTFILE` 或 `JuStFiLe`，都可以工作。`just` 还会查找名为 `.justfile` 的文件，以便你可以隐藏 `justfile`。

不带参数运行 `just` 会运行 `justfile` 中的第一个配方：

```console
$ just
echo 'This is a recipe!'
This is a recipe!
```

一个或多个参数指定要运行的配方：

```console
$ just another-recipe
This is another recipe.
```

`just` 在执行每条命令前会将其输出到标准错误，所以 `echo 'This is a recipe!'` 会被打印出来。以 `@` 开头的行会抑制这个输出，所以 `echo 'This is another recipe.'` 不会被打印。

如果命令失败，配方会停止运行。这里 `cargo publish` 只有在 `cargo test` 成功时才会运行：

```just
publish:
  cargo test
  # tests passed, time to publish!
  cargo publish
```

配方可以依赖其他配方。这里 `test` 配方依赖于 `build` 配方，所以 `build` 会在 `test` 之前运行：

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

没有依赖的配方将按照在命令行上给出的顺序运行：

```console
$ just build sloc
cc main.c foo.c bar.c -o main
1337 lines of code
```

依赖项总是先运行，即使它们在依赖它们的配方之后传递：

```console
$ just test build
cc main.c foo.c bar.c -o main
./test
testing… all tests passed!
```

配方可以依赖子模块中的配方：

```justfile
mod foo

baz: foo::bar
```

示例
----

各种 `justfile` 可以在[示例目录](https://github.com/casey/just/tree/master/examples)和 [GitHub](https://github.com/search?q=path%3A**%2Fjustfile&type=code) 上找到。

功能特性
--------

### 默认配方

当不带配方名称调用 `just` 时，它会运行带有 `[default]` 属性的配方，如果没有配方具有 `[default]` 属性，则运行 `justfile` 中的第一个配方。

这个配方可能是项目中最常运行的命令，比如运行测试：

```just
test:
  cargo test
```

你也可以使用依赖来默认运行多个配方：

```just
default: lint build test

build:
  echo Building…

test:
  echo Testing…

lint:
  echo Linting…
```

如果没有适合作为默认配方的配方，你可以在 `justfile` 开头添加一个列出可用配方的配方：

```just
default:
  just --list
```

### 列出可用配方

可以使用 `just --list` 按字母顺序列出配方：

```console
$ just --list
Available recipes:
    build
    test
    deploy
    lint
```

[子模块](#模块)中的配方可以使用 `just --list PATH` 列出，其中 `PATH` 是以空格或 `::` 分隔的模块路径：

```
$ cat justfile
mod foo
$ cat foo.just
mod bar
$ cat bar.just
baz:
$ just --list foo bar
Available recipes:
    baz
$ just --list foo::bar
Available recipes:
    baz
```

`just --summary` 更加简洁：

```console
$ just --summary
build test deploy lint
```

传递 `--unsorted` 可以按照配方在 `justfile` 中出现的顺序打印：

```just
test:
  echo 'Testing!'

build:
  echo 'Building!'
```

```console
$ just --list --unsorted
Available recipes:
    test
    build
```

```console
$ just --summary --unsorted
test build
```

如果你希望 `just` 默认列出 `justfile` 中的配方，可以使用以下作为默认配方：

```just
default:
  @just --list
```

请注意，你可能需要在上面这行添加 `--justfile {{justfile()}}`。否则，如果你执行 `just -f /some/distant/justfile -d .` 或 `just -f ./non-standard-justfile`，配方内部的 `just --list` 不一定会使用你提供的文件。它会尝试在当前路径查找 justfile，甚至可能导致 `No justfile found` 错误。

标题文本可以使用 `--list-heading` 自定义：

```console
$ just --list --list-heading $'Cool stuff…\n'
Cool stuff…
    test
    build
```

缩进可以使用 `--list-prefix` 自定义：

```console
$ just --list --list-prefix ····
Available recipes:
····test
····build
```

`--list-heading` 的参数会替换标题和其后的换行符，因此如果非空则应包含换行符。这样设计是为了让你可以通过传递空字符串来完全隐藏标题行：

```console
$ just --list --list-heading ''
    test
    build
```

### 调用多个配方

可以在命令行上同时调用多个配方：

```just
build:
  make web

serve:
  python3 -m http.server -d out 8000
```

```console
$ just build serve
make web
python3 -m http.server -d out 8000
```

注意，带参数的配方会"吞掉"后续参数，即使这些参数与其他配方的名称相同：

```just
build project:
  make {{project}}

serve:
  python3 -m http.server -d out 8000
```

```console
$ just build serve
make: *** No rule to make target `serve'.  Stop.
```

`--one` 标志可用于将命令行调用限制为单个配方：

```console
$ just --one build serve
error: Expected 1 command-line recipe invocation but found 2.
```

### 工作目录

默认情况下，配方在包含 `justfile` 的目录中运行。

`[no-cd]` 属性可用于使配方在调用 `just` 的目录中运行。

```just
@foo:
  pwd

[no-cd]
@bar:
  pwd
```

```console
$ cd subdir
$ just foo
/
$ just bar
/subdir
```

你可以使用 `set working-directory := '…'` 为所有配方覆盖工作目录：

```just
set working-directory := 'bar'

@foo:
  pwd
```

```console
$ pwd
/home/bob
$ just foo
/home/bob/bar
```

你可以使用 `working-directory` 属性<sup>1.38.0</sup>为特定配方覆盖工作目录：

```just
[working-directory: 'bar']
@foo:
  pwd
```

```console
$ pwd
/home/bob
$ just foo
/home/bob/bar
```

`working-directory` 设置或 `working-directory` 属性的参数可以是绝对路径或相对路径。如果是相对路径，则相对于默认工作目录进行解释。

### 别名

别名允许在命令行上使用替代名称调用配方：

```just
alias b := build

build:
  echo 'Building!'
```

```console
$ just b
echo 'Building!'
Building!
```

别名的目标可以是子模块中的配方：

```justfile
mod foo

alias baz := foo::bar
```

### 设置

设置控制解释和执行。每个设置最多只能在 `justfile` 中的任何位置指定一次。

例如：

```just
set shell := ["zsh", "-cu"]

foo:
  # this line will be run as `zsh -cu 'ls **/*.txt'`
  ls **/*.txt
```

#### 设置表

| 名称 | 值 | 默认值 | 描述 |
|------|-------|---------|-------------|
| `allow-duplicate-recipes` | 布尔值 | `false` | 允许 `justfile` 中后出现的配方覆盖同名的早期配方。 |
| `allow-duplicate-variables` | 布尔值 | `false` | 允许 `justfile` 中后出现的变量覆盖同名的早期变量。 |
| `dotenv-filename` | 字符串 | - | 如果存在，加载自定义名称的 `.env` 文件。 |
| `dotenv-load` | 布尔值 | `false` | 如果存在，加载 `.env` 文件。 |
| `dotenv-override` | 布尔值 | `false` | 使用 `.env` 文件中的值覆盖现有环境变量。 |
| `dotenv-path` | 字符串 | - | 从自定义路径加载 `.env` 文件，如果不存在则报错。覆盖 `dotenv-filename`。 |
| `dotenv-required` | 布尔值 | `false` | 如果找不到 `.env` 文件则报错。 |
| `export` | 布尔值 | `false` | 将所有变量导出为环境变量。 |
| `fallback` | 布尔值 | `false` | 如果命令行上的第一个配方未找到，则在父目录中搜索 `justfile`。 |
| `ignore-comments` | 布尔值 | `false` | 忽略以 `#` 开头的配方行。 |
| `positional-arguments` | 布尔值 | `false` | 传递位置参数。 |
| `quiet` | 布尔值 | `false` | 禁用执行前回显配方行。 |
| `script-interpreter`<sup>1.33.0</sup> | `[COMMAND, ARGS…]` | `['sh', '-eu']` | 设置用于调用带有空 `[script]` 属性的配方的命令。 |
| `shell` | `[COMMAND, ARGS…]` | - | 设置用于调用配方和求值反引号的命令。 |
| `tempdir` | 字符串 | - | 在 `tempdir` 中创建临时目录，而不是系统默认临时目录。 |
| `unstable`<sup>1.31.0</sup> | 布尔值 | `false` | 启用不稳定功能。 |
| `windows-powershell` | 布尔值 | `false` | 在 Windows 上使用 PowerShell 作为默认 shell。（已弃用。请改用 `windows-shell`。） |
| `windows-shell` | `[COMMAND, ARGS…]` | - | 设置用于调用配方和求值反引号的命令。 |
| `working-directory`<sup>1.33.0</sup> | 字符串 | - | 设置配方和反引号的工作目录，相对于默认工作目录。 |

布尔设置可以写成：

```justfile
set NAME
```

等同于：

```justfile
set NAME := true
```

非布尔设置可以设置为字符串和表达式。<sup>1.46.0</sup>

然而，由于设置会影响反引号和许多函数的行为，这些表达式不能直接或通过引用间接包含反引号或函数调用。

#### 允许重复配方

如果 `allow-duplicate-recipes` 设置为 `true`，定义多个同名配方不会报错，将使用最后一个定义。默认为 `false`。

```just
set allow-duplicate-recipes

@foo:
  echo foo

@foo:
  echo bar
```

```console
$ just foo
bar
```

#### 允许重复变量

如果 `allow-duplicate-variables` 设置为 `true`，定义多个同名变量不会报错，将使用最后一个定义。默认为 `false`。

```just
set allow-duplicate-variables

a := "foo"
a := "bar"

@foo:
  echo {{a}}
```

```console
$ just foo
bar
```

#### Dotenv 设置

如果设置了 `dotenv-load`、`dotenv-filename`、`dotenv-override`、`dotenv-path` 或 `dotenv-required` 中的任何一个，`just` 将尝试从文件加载环境变量。

如果设置了 `dotenv-path`，`just` 将在给定路径查找文件，该路径可以是绝对路径或相对于工作目录的相对路径。

命令行选项 `--dotenv-path`（简写形式 `-E`）可用于在运行时设置或覆盖 `dotenv-path`。

如果设置了 `dotenv-filename`，`just` 将在给定路径（相对于工作目录及其每个祖先目录）查找文件。

如果未设置 `dotenv-filename`，但设置了 `dotenv-load` 或 `dotenv-required`，just 将查找名为 `.env` 的文件（相对于工作目录及其每个祖先目录）。

`dotenv-filename` 和 `dotenv-path` 类似，但 `dotenv-path` 仅相对于工作目录检查，而 `dotenv-filename` 相对于工作目录及其每个祖先目录检查。

如果未找到环境文件，不会报错，除非设置了 `dotenv-required`。

加载的变量是环境变量，而不是 `just` 变量，因此必须在配方和反引号中使用 `$VARIABLE_NAME` 访问。

如果设置了 `dotenv-override`，来自环境文件的变量将覆盖现有的环境变量。

例如，如果你的 `.env` 文件包含：

```console
# a comment, will be ignored
DATABASE_ADDRESS=localhost:6379
SERVER_PORT=1337
```

而你的 `justfile` 包含：

```just
set dotenv-load

serve:
  @echo "Starting server with database $DATABASE_ADDRESS on port $SERVER_PORT…"
  ./server --database $DATABASE_ADDRESS --port $SERVER_PORT
```

`just serve` 将输出：

```console
$ just serve
Starting server with database localhost:6379 on port 1337…
./server --database $DATABASE_ADDRESS --port $SERVER_PORT
```

#### 导出

`export` 设置会将所有 `just` 变量导出为环境变量。默认为 `false`。

```just
set export

a := "hello"

@foo b:
  echo $a
  echo $b
```

```console
$ just foo goodbye
hello
goodbye
```

#### 位置参数

如果 `positional-arguments` 为 `true`，配方参数将作为位置参数传递给命令。对于逐行配方，参数 `$0` 将是配方的名称。

例如，运行这个配方：

```just
set positional-arguments

@foo bar:
  echo $0
  echo $1
```

将产生以下输出：

```console
$ just foo hello
foo
hello
```

使用与 `sh` 兼容的 shell（如 `bash` 或 `zsh`）时，`$@` 会展开为从 1 开始传递给配方的位置参数。在双引号内使用 `"$@"` 时，包含空格的参数会像被双引号包围一样传递。也就是说，`"$@"` 等同于 `"$1" "$2"`……当没有位置参数时，`"$@"` 和 `$@` 展开为空（即被移除）。

这个示例配方将逐行打印参数：

```just
set positional-arguments

@test *args='':
  bash -c 'while (( "$#" )); do echo - $1; shift; done' -- "$@"
```

使用 _两个_ 参数运行：

```console
$ just test foo "bar baz"
- foo
- bar baz
```

也可以使用 `[positional-arguments]` 属性<sup>1.29.0</sup>为单个配方开启位置参数：

```just
[positional-arguments]
@foo bar:
  echo $0
  echo $1
```

请注意，PowerShell 处理位置参数的方式与其他 shell 不同，因此开启位置参数可能会破坏使用 PowerShell 的配方。

如果使用 PowerShell 7.4 或更高版本，`-CommandWithArgs` 标志将使位置参数按预期工作：

```just
set shell := ['pwsh.exe', '-CommandWithArgs']
set positional-arguments

print-args a b c:
  Write-Output @($args[1..($args.Count - 1)])
```

#### Shell

`shell` 设置控制用于调用配方行和反引号的命令。Shebang 配方不受影响。默认 shell 是 `sh -cu`。

```just
# use python3 to execute recipe lines and backticks
set shell := ["python3", "-c"]

# use print to capture result of evaluation
foos := `print("foo" * 4)`

foo:
  print("Snake snake snake snake.")
  print("{{foos}}")
```

`just` 将要执行的命令作为参数传递。许多 shell 需要一个额外的标志（通常是 `-c`）来使它们求值第一个参数。

##### Windows Shell

`just` 在 Windows 上默认使用 `sh`。要在 Windows 上使用不同的 shell，请使用 `windows-shell`：

```just
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

hello:
  Write-Host "Hello, world!"
```

参见 [powershell.just](https://github.com/casey/just/blob/master/examples/powershell.just) 了解在所有平台上使用 PowerShell 的 justfile。

##### Windows PowerShell

*`set windows-powershell` 使用旧版 `powershell.exe` 二进制文件，不再推荐使用。请参阅上面的 `windows-shell` 设置了解在 Windows 上控制使用哪个 shell 的更灵活方式。*

`just` 在 Windows 上默认使用 `sh`。要改用 `powershell.exe`，请将 `windows-powershell` 设置为 true。

```just
set windows-powershell := true

hello:
  Write-Host "Hello, world!"
```

##### Python 3

```just
set shell := ["python3", "-c"]
```

##### Bash

```just
set shell := ["bash", "-uc"]
```

##### Z Shell

```just
set shell := ["zsh", "-uc"]
```

##### Fish

```just
set shell := ["fish", "-c"]
```

##### Nushell

```just
set shell := ["nu", "-c"]
```

如果你想将默认表格模式更改为 `light`：

```just
set shell := ['nu', '-m', 'light', '-c']
```

*[Nushell](https://github.com/nushell/nushell) 是用 Rust 编写的，**支持 Windows / macOS 和 Linux 跨平台**。*

### 文档注释

紧接在配方前面的注释将出现在 `just --list` 中：

```just
# build stuff
build:
  ./bin/build

# test stuff
test:
  ./bin/test
```

```console
$ just --list
Available recipes:
    build # build stuff
    test # test stuff
```

`[doc]` 属性可用于设置或隐藏配方的文档注释：

```just
# This comment won't appear
[doc('Build stuff')]
build:
  ./bin/build

# This one won't either
[doc]
test:
  ./bin/test
```

```console
$ just --list
Available recipes:
    build # Build stuff
    test
```

### 表达式和替换

表达式支持各种运算符和函数调用，可用于赋值、默认配方参数以及配方体内的 `{{…}}` 替换。

```just
tmpdir  := `mktemp -d`
version := "0.2.7"
tardir  := tmpdir / "awesomesauce-" + version
tarball := tardir + ".tar.gz"
config  := quote(config_dir() / ".project-config")

publish:
  rm -f {{tarball}}
  mkdir {{tardir}}
  cp README.md *.c {{ config }} {{tardir}}
  tar zcvf {{tarball}} {{tardir}}
  scp {{tarball}} me@server.com:release/
  rm -rf {{tarball}} {{tardir}}
```

#### 连接

`+` 运算符返回左侧参数与右侧参数的连接结果：

```just
foobar := 'foo' + 'bar'
```

#### 逻辑运算符

逻辑运算符 `&&` 和 `||` 可用于合并字符串值<sup>1.37.0</sup>，类似于 Python 的 `and` 和 `or`。这些运算符将空字符串 `''` 视为假，所有其他字符串视为真。

这些运算符目前是不稳定的。

`&&` 运算符在左侧参数为空字符串时返回空字符串，否则返回右侧参数：

```justfile
foo := '' && 'goodbye'      # ''
bar := 'hello' && 'goodbye' # 'goodbye'
```

`||` 运算符在左侧参数非空时返回左侧参数，否则返回右侧参数：

```justfile
foo := '' || 'goodbye'      # 'goodbye'
bar := 'hello' || 'goodbye' # 'hello'
```

#### 路径连接

`/` 运算符可用于用斜杠连接两个字符串：

```just
foo := "a" / "b"
```

```
$ just --evaluate foo
a/b
```

请注意，即使已经存在斜杠也会添加 `/`：

```just
foo := "a/"
bar := foo / "b"
```

```
$ just --evaluate bar
a//b
```

也可以构建绝对路径<sup>1.5.0</sup>：

```just
foo := / "b"
```

```
$ just --evaluate foo
/b
```

`/` 运算符使用 `/` 字符，即使在 Windows 上也是如此。因此，应避免将 `/` 运算符用于使用通用命名约定（UNC）的路径，即以 `\?` 开头的路径，因为 UNC 路径不支持正斜杠。

#### 转义 `{{`

要编写包含 `{{` 的配方，请使用 `{{{{`：

```just
braces:
  echo 'I {{{{LOVE}} curly braces!'
```

（不匹配的 `}}` 会被忽略，因此不需要转义。）

另一种选择是将所有要转义的文本放在插值内：
```just
braces:
  echo '{{'I {{LOVE}} curly braces!'}}'
```

另一个选择是使用 `{{ "{{" }}`：

```just
braces:
  echo 'I {{ "{{" }}LOVE}} curly braces!'
```

### 字符串

支持 `'single'`（单引号）、`"double"`（双引号）和 `'''triple'''`（三引号）字符串字面量。与配方主体不同，字符串内部不支持 `{{…}}` 插值。

双引号字符串支持转义序列：

```just
carriage-return   := "\r"
double-quote      := "\""
newline           := "\n"
no-newline        := "\
"
slash             := "\\"
tab               := "\t"
unicode-codepoint := "\u{1F916}"
```

```console
$ just --evaluate
"arriage-return   := "
double-quote      := """
newline           := "
"
no-newline        := ""
slash             := "\"
tab               := "     "
unicode-codepoint := "🤖"
```

Unicode 字符转义序列 `\u{…}`<sup>1.36.0</sup> 最多接受六位十六进制数字。

字符串可以包含换行符：

```just
single := '
hello
'

double := "
goodbye
"
```

单引号字符串不识别转义序列：

```just
escapes := '\t\n\r\"\\'
```

```console
$ just --evaluate
escapes := "\t\n\r\"\\"
```

支持单引号和双引号字符串的缩进版本，使用三个单引号或三个双引号作为分隔符。缩进字符串会去除开头的换行符，以及所有非空行共有的前导空白：

```just
# 这个字符串的值为 `foo\nbar\n`
x := '''
  foo
  bar
'''

# 这个字符串的值为 `abc\n  wuv\nxyz\n`
y := """
  abc
    wuv
  xyz
"""
```

与非缩进字符串类似，缩进的双引号字符串会处理转义序列，而缩进的单引号字符串则忽略转义序列。转义序列处理在去除缩进之后进行。去缩进算法不会考虑由转义序列产生的空白或换行。

#### Shell 扩展字符串

以 `x` 为前缀的字符串会进行 shell 扩展<sup>1.27.0</sup>：

```justfile
foobar := x'~/$FOO/${BAR}'
```

| 值 | 替换结果 |
|------|-------------|
| `$VAR` | 环境变量 `VAR` 的值 |
| `${VAR}` | 环境变量 `VAR` 的值 |
| `${VAR:-DEFAULT}` | 环境变量 `VAR` 的值，如果 `VAR` 未设置则使用 `DEFAULT` |
| 开头的 `~` | 当前用户主目录的路径 |
| 开头的 `~USER` | `USER` 用户主目录的路径 |

此扩展在编译时执行，因此不能使用 `.env` 文件中的变量和导出的 `just` 变量。不过，这允许在设置和导入路径等位置使用 shell 扩展字符串，这些位置不能依赖 `just` 变量和 `.env` 文件。

#### 格式化字符串

以 `f` 为前缀的字符串是格式化字符串<sup>1.44.0</sup>：

```justfile
name := "world"
message := f'Hello, {{name}}!'
```

格式化字符串可以包含用 `{{…}}` 分隔的插值，其中包含表达式。格式化字符串的值是连接后的字符串片段和计算后的表达式。

使用 `{{{{` 在格式化字符串中包含字面量 `{{`：

```justfile
foo := f'I {{{{LOVE} curly braces!'
```

### 忽略错误

通常，如果命令返回非零退出状态，执行将停止。要在命令失败后继续执行，请在命令前加上 `-`：

```just
foo:
  -cat foo
  echo 'Done!'
```

```console
$ just foo
cat foo
cat: foo: No such file or directory
echo 'Done!'
Done!
```

### 函数

`just` 提供了许多内置函数，可用于表达式，包括配方主体的 `{{…}}` 替换、赋值和默认参数值。

所有以 `_directory` 结尾的函数都可以缩写为 `_dir`。因此 `home_directory()` 也可以写成 `home_dir()`。此外，`invocation_directory_native()` 可以缩写为 `invocation_dir_native()`。

#### 系统信息

- `arch()` — 指令集架构。可能的值有：`"aarch64"`、`"arm"`、`"asmjs"`、`"hexagon"`、`"mips"`、`"msp430"`、`"powerpc"`、`"powerpc64"`、`"s390x"`、`"sparc"`、`"wasm32"`、`"x86"`、`"x86_64"` 和 `"xcore"`。
- `num_cpus()`<sup>1.15.0</sup> - 逻辑 CPU 数量。
- `os()` — 操作系统。可能的值有：`"android"`、`"bitrig"`、`"dragonfly"`、`"emscripten"`、`"freebsd"`、`"haiku"`、`"ios"`、`"linux"`、`"macos"`、`"netbsd"`、`"openbsd"`、`"solaris"` 和 `"windows"`。
- `os_family()` — 操作系统家族；可能的值有：`"unix"` 和 `"windows"`。

例如：

```just
system-info:
  @echo "This is an {{arch()}} machine".
```

```console
$ just system-info
This is an x86_64 machine
```

`os_family()` 函数可用于创建跨平台的 `justfile`，使其能在各种操作系统上工作。有关示例，请参阅 [cross-platform.just](https://github.com/casey/just/blob/master/examples/cross-platform.just) 文件。

#### 外部命令

- `shell(command, args...)`<sup>1.27.0</sup> 返回 shell 脚本 `command` 的标准输出，可带有零个或多个位置参数 `args`。用于解释 `command` 的 shell 与用于执行配方行的 shell 相同，可以通过 `set shell := […]` 更改。

  `command` 作为第一个参数传递，因此如果命令是 `'echo $@'`，使用默认 shell 命令 `sh -cu` 以及 `args` 为 `'foo'` 和 `'bar'` 时，完整的命令行将是：

  ```
  'sh' '-cu' 'echo $@' 'echo $@' 'foo' 'bar'
  ```

  这样 `$@` 就能按预期工作，而 `$1` 指向第一个参数。`$@` 不包含第一个位置参数，该参数预期是正在运行的程序的名称。

```just
# 参数可以是变量或表达式
file := '/sys/class/power_supply/BAT0/status'
bat0stat := shell('cat $1', file)

# 命令可以是变量或表达式
command := 'wc -l'
output := shell(command + ' "$1"', 'main.c')

# shell 命令引用的参数必须被使用
empty := shell('echo', 'foo')
full := shell('echo $1', 'foo')
error := shell('echo $1')
```

```just
# 使用 python 作为 shell。由于 `python -c` 将 `sys.argv[0]` 设置为 `'-c'`，
# 第一个"真正的"位置参数将是 `sys.argv[2]`。
set shell := ["python3", "-c"]
olleh := shell('import sys; print(sys.argv[2][::-1])', 'hello')
```

#### 环境变量

- `env(key)`<sup>1.15.0</sup> — 获取名为 `key` 的环境变量，如果不存在则中止执行。

```just
home_dir := env('HOME')

test:
  echo "{{home_dir}}"
```

```console
$ just
/home/user1
```

- `env(key, default)`<sup>1.15.0</sup> — 获取名为 `key` 的环境变量，如果不存在则返回 `default`。
- `env_var(key)` — `env(key)` 的已弃用别名。
- `env_var_or_default(key, default)` — `env(key, default)` 的已弃用别名。

可以使用 `||` 运算符为空环境变量值替换默认值，此功能目前不稳定：

```just
set unstable

foo := env('FOO', '') || 'DEFAULT_VALUE'
```

#### 可执行文件

- `require(name)`<sup>1.39.0</sup> — 在 `PATH` 环境变量的目录中搜索可执行文件 `name` 并返回其完整路径，如果不存在名为 `name` 的可执行文件则报错停止。

  ```just
  bash := require("bash")

  @test:
      echo "bash: '{{bash}}'"
  ```

  ```console
  $ just
  bash: '/bin/bash'
  ```

- `which(name)`<sup>1.39.0</sup> — 在 `PATH` 环境变量的目录中搜索可执行文件 `name` 并返回其完整路径，如果不存在名为 `name` 的可执行文件则返回空字符串。此功能目前不稳定。

  ```just
  set unstable

  bosh := which("bosh")

  @test:
      echo "bosh: '{{bosh}}'"
  ```

  ```console
  $ just
  bosh: ''
  ```

#### 调用信息

- `is_dependency()` - 如果当前配方作为另一个配方的依赖运行（而不是直接运行），则返回字符串 `true`，否则返回字符串 `false`。

#### 调用目录

- `invocation_directory()` - 获取调用 `just` 时当前目录的绝对路径，即 `just` 在执行命令之前更改目录（chdir）前的路径。在 Windows 上，`invocation_directory()` 使用 `cygpath` 将调用目录转换为兼容 Cygwin 的 `/` 分隔路径。使用 `invocation_directory_native()` 可在所有平台上返回原始调用目录。

例如，要对"当前目录"（从用户/调用者的角度）下的文件调用 `rustfmt`，请使用以下规则：

```just
rustfmt:
  find {{invocation_directory()}} -name \*.rs -exec rustfmt {} \;
```

或者，如果您的命令需要从当前目录运行，可以使用（例如）：

```just
build:
  cd {{invocation_directory()}}; ./some_script_that_needs_to_be_run_from_here
```

- `invocation_directory_native()` - 获取调用 `just` 时当前目录的绝对路径，即 `just` 在执行命令之前更改目录（chdir）前的路径。

#### justfile 和 justfile 目录

- `justfile()` - 获取当前 `justfile` 的路径。

- `justfile_directory()` - 获取当前 `justfile` 父目录的路径。

例如，要运行相对于当前 `justfile` 位置的命令：

```just
script:
  {{justfile_directory()}}/scripts/some_script
```

#### 源文件和源目录

- `source_file()`<sup>1.27.0</sup> - 获取当前源文件的路径。

- `source_directory()`<sup>1.27.0</sup> - 获取当前源文件父目录的路径。

在根 `justfile` 中，`source_file()` 和 `source_directory()` 的行为与 `justfile()` 和 `justfile_directory()` 相同，但在 import 或子模块中调用时，它们将分别返回当前 `import` 或 `mod` 源文件的路径和目录。

#### Just 可执行文件

- `just_executable()` - `just` 可执行文件的绝对路径。

例如：

```just
executable:
  @echo The executable is at: {{just_executable()}}
```

```console
$ just
The executable is at: /bin/just
```

#### Just 进程 ID

- `just_pid()` - `just` 可执行文件的进程 ID。

例如：

```just
pid:
  @echo The process ID is: {{ just_pid() }}
```

```console
$ just
The process ID is: 420
```

#### 字符串操作

- `append(suffix, s)`<sup>1.27.0</sup> 将 `suffix` 追加到 `s` 中以空白分隔的字符串后。`append('/src', 'foo bar baz')` → `'foo/src bar/src baz/src'`
- `prepend(prefix, s)`<sup>1.27.0</sup> 将 `prefix` 添加到 `s` 中以空白分隔的字符串前。`prepend('src/', 'foo bar baz')` → `'src/foo src/bar src/baz'`
- `encode_uri_component(s)`<sup>1.27.0</sup> - 对 `s` 中除 `[A-Za-z0-9_.!~*'()-]` 外的字符进行百分号编码，行为与 [JavaScript 的 `encodeURIComponent` 函数](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/encodeURIComponent) 一致。
- `quote(s)` - 将所有单引号替换为 `'\''` 并在 `s` 的开头和结尾添加单引号。这足以为许多 shell（包括大多数 Bourne shell 衍生版本）转义特殊字符。
- `replace(s, from, to)` - 将 `s` 中所有的 `from` 替换为 `to`。
- `replace_regex(s, regex, replacement)` - 将 `s` 中所有匹配 `regex` 的内容替换为 `replacement`。正则表达式由 [Rust `regex` crate](https://docs.rs/regex/latest/regex/) 提供。有关用法示例，请参阅[语法文档](https://docs.rs/regex/latest/regex/#syntax)。支持捕获组。`replacement` 字符串使用[替换字符串语法](https://docs.rs/regex/latest/regex/struct.Regex.html#replacement-string-syntax)。
- `trim(s)` - 移除 `s` 开头和结尾的空白。
- `trim_end(s)` - 移除 `s` 结尾的空白。
- `trim_end_match(s, substring)` - 移除 `s` 结尾匹配 `substring` 的后缀。
- `trim_end_matches(s, substring)` - 重复移除 `s` 结尾匹配 `substring` 的后缀。
- `trim_start(s)` - 移除 `s` 开头的空白。
- `trim_start_match(s, substring)` - 移除 `s` 开头匹配 `substring` 的前缀。
- `trim_start_matches(s, substring)` - 重复移除 `s` 开头匹配 `substring` 的前缀。

#### 大小写转换

- `capitalize(s)`<sup>1.7.0</sup> - 将 `s` 的首字符转为大写，其余转为小写。
- `kebabcase(s)`<sup>1.7.0</sup> - 将 `s` 转换为 `kebab-case`。
- `lowercamelcase(s)`<sup>1.7.0</sup> - 将 `s` 转换为 `lowerCamelCase`。
- `lowercase(s)` - 将 `s` 转换为小写。
- `shoutykebabcase(s)`<sup>1.7.0</sup> - 将 `s` 转换为 `SHOUTY-KEBAB-CASE`。
- `shoutysnakecase(s)`<sup>1.7.0</sup> - 将 `s` 转换为 `SHOUTY_SNAKE_CASE`。
- `snakecase(s)`<sup>1.7.0</sup> - 将 `s` 转换为 `snake_case`。
- `titlecase(s)`<sup>1.7.0</sup> - 将 `s` 转换为 `Title Case`。
- `uppercamelcase(s)`<sup>1.7.0</sup> - 将 `s` 转换为 `UpperCamelCase`。
- `uppercase(s)` - 将 `s` 转换为大写。

#### 路径操作

##### 可能失败的函数

- `absolute_path(path)` - 获取工作目录中相对路径 `path` 的绝对路径。在 `/foo` 目录中 `absolute_path("./bar.txt")` 的结果是 `/foo/bar.txt`。
- `canonicalize(path)`<sup>1.24.0</sup> - 通过解析符号链接并尽可能移除 `.`、`..` 和多余的 `/` 来规范化 `path`。
- `extension(path)` - `path` 的扩展名。`extension("/foo/bar.txt")` 的结果是 `txt`。
- `file_name(path)` - `path` 的文件名，移除所有前导目录组件。`file_name("/foo/bar.txt")` 的结果是 `bar.txt`。
- `file_stem(path)` - `path` 的文件名（不含扩展名）。`file_stem("/foo/bar.txt")` 的结果是 `bar`。
- `parent_directory(path)` - `path` 的父目录。`parent_directory("/foo/bar.txt")` 的结果是 `/foo`。
- `without_extension(path)` - 不含扩展名的 `path`。`without_extension("/foo/bar.txt")` 的结果是 `/foo/bar`。

这些函数可能会失败，例如当路径没有扩展名时，这将导致执行停止。

##### 不会失败的函数

- `clean(path)` - 通过移除多余的路径分隔符、中间的 `.` 组件以及尽可能的 `..` 来简化 `path`。`clean("foo//bar")` 的结果是 `foo/bar`，`clean("foo/..")` 的结果是 `.`，`clean("foo/./bar")` 的结果是 `foo/bar`。
- `join(a, b…)` - *此函数在 Unix 上使用 `/`，在 Windows 上使用 `\`，这可能导致不期望的行为。`/` 运算符（例如 `a / b`）始终使用 `/`，除非特别需要在 Windows 上使用 `\`，否则应考虑使用它作为替代。* 将路径 `a` 与路径 `b` 连接。`join("foo/bar", "baz")` 的结果是 `foo/bar/baz`。接受两个或更多参数。

#### 文件系统访问

- `path_exists(path)` - 如果路径指向现有实体则返回 `true`，否则返回 `false`。会遍历符号链接，如果路径不可访问或指向损坏的符号链接则返回 `false`。
- `read(path)`<sup>1.39.0</sup> - 以字符串形式返回 `path` 处文件的内容。

##### 错误报告

- `error(message)` - 中止执行并向用户报告错误 `message`。

#### UUID 和哈希生成

- `blake3(string)`<sup>1.25.0</sup> - 以十六进制字符串形式返回 `string` 的 [BLAKE3] 哈希值。
- `blake3_file(path)`<sup>1.25.0</sup> - 以十六进制字符串形式返回 `path` 处文件的 [BLAKE3] 哈希值。
- `sha256(string)` - 以十六进制字符串形式返回 `string` 的 SHA-256 哈希值。
- `sha256_file(path)` - 以十六进制字符串形式返回 `path` 处文件的 SHA-256 哈希值。
- `uuid()` - 生成随机的版本 4 UUID。

[BLAKE3]: https://github.com/BLAKE3-team/BLAKE3/

#### 随机

- `choose(n, alphabet)`<sup>1.27.0</sup> - 从 `alphabet` 中随机选择 `n` 个字符生成字符串，`alphabet` 不能包含重复字符。例如，`choose('64', HEX)` 将生成一个随机的 64 字符小写十六进制字符串。

#### 日期时间

- `datetime(format)`<sup>1.30.0</sup> - 以 `format` 格式返回本地时间。
- `datetime_utc(format)`<sup>1.30.0</sup> - 以 `format` 格式返回 UTC 时间。

`datetime` 和 `datetime_utc` 的参数是 `strftime` 风格的格式字符串，详情请参阅 [`chrono` 库文档](https://docs.rs/chrono/latest/chrono/format/strftime/index.html)。

#### 语义版本

- `semver_matches(version, requirement)`<sup>1.16.0</sup> - 检查[语义版本 `version`](https://semver.org)（例如 `"0.1.0"`）是否匹配 `requirement`（例如 `">=0.1.0"`），如果匹配则返回 `"true"`，否则返回 `"false"`。

#### 样式

- `style(name)`<sup>1.37.0</sup> - 返回 `just` 使用的命名终端显示属性转义序列。与包含标准颜色和样式的终端显示属性转义序列常量不同，`style(name)` 返回 `just` 本身使用的转义序列，可用于使配方输出与 `just` 自己的输出风格一致。

  `name` 可识别的值有 `'command'`（用于回显的配方行）、`error` 和 `warning`。

  例如，为错误消息设置样式：

  ```just
  scary:
    @echo '{{ style("error") }}OH NO{{ NORMAL }}'
  ```

##### 用户目录<sup>1.23.0</sup>

这些函数返回用户特定目录的路径，如配置、数据、缓存、可执行文件目录和用户主目录。

在 Unix 上，这些函数遵循 [XDG 基础目录规范](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html)。

在 MacOS 和 Windows 上，这些函数返回系统指定的用户特定目录。例如，`cache_directory()` 在 MacOS 上返回 `~/Library/Caches`，在 Windows 上返回 `{FOLDERID_LocalAppData}`。

有关更多详情，请参阅 [`dirs`](https://docs.rs/dirs/latest/dirs/index.html) crate。

- `cache_directory()` - 用户特定的缓存目录。
- `config_directory()` - 用户特定的配置目录。
- `config_local_directory()` - 本地用户特定的配置目录。
- `data_directory()` - 用户特定的数据目录。
- `data_local_directory()` - 本地用户特定的数据目录。
- `executable_directory()` - 用户特定的可执行文件目录。
- `home_directory()` - 用户主目录。

如果您想在所有平台上使用 XDG 基础目录，可以使用 `env(…)` 函数配合适当的环境变量和回退值，但请注意 XDG 规范要求忽略非绝对路径，因此为了与符合规范的应用程序完全兼容，您需要这样做：

```just
xdg_config_dir := if env('XDG_CONFIG_HOME', '') =~ '^/' {
  env('XDG_CONFIG_HOME')
} else {
  home_directory() / '.config'
}
```

### 常量

预定义了许多常量：

| 名称 | 值 | Windows 上的值 |
|---|---|---|
| `HEX`<sup>1.27.0</sup> | `"0123456789abcdef"` |  |
| `HEXLOWER`<sup>1.27.0</sup> | `"0123456789abcdef"` |  |
| `HEXUPPER`<sup>1.27.0</sup> | `"0123456789ABCDEF"` |  |
| `PATH_SEP`<sup>1.41.0</sup> | `"/"` | `"\"` |
| `PATH_VAR_SEP`<sup>1.41.0</sup> | `":"` | `";"` |
| `CLEAR`<sup>1.37.0</sup> | `"\ec"` |  |
| `NORMAL`<sup>1.37.0</sup> | `"\e[0m"` |  |
| `BOLD`<sup>1.37.0</sup> | `"\e[1m"` |  |
| `ITALIC`<sup>1.37.0</sup> | `"\e[3m"` |  |
| `UNDERLINE`<sup>1.37.0</sup> | `"\e[4m"` |  |
| `INVERT`<sup>1.37.0</sup> | `"\e[7m"` |  |
| `HIDE`<sup>1.37.0</sup> | `"\e[8m"` |  |
| `STRIKETHROUGH`<sup>1.37.0</sup> | `"\e[9m"` |  |
| `BLACK`<sup>1.37.0</sup> | `"\e[30m"` |  |
| `RED`<sup>1.37.0</sup> | `"\e[31m"` |  |
| `GREEN`<sup>1.37.0</sup> | `"\e[32m"` |  |
| `YELLOW`<sup>1.37.0</sup> | `"\e[33m"` |  |
| `BLUE`<sup>1.37.0</sup> | `"\e[34m"` |  |
| `MAGENTA`<sup>1.37.0</sup> | `"\e[35m"` |  |
| `CYAN`<sup>1.37.0</sup> | `"\e[36m"` |  |
| `WHITE`<sup>1.37.0</sup> | `"\e[37m"` |  |
| `BG_BLACK`<sup>1.37.0</sup> | `"\e[40m"` |  |
| `BG_RED`<sup>1.37.0</sup> | `"\e[41m"` |  |
| `BG_GREEN`<sup>1.37.0</sup> | `"\e[42m"` |  |
| `BG_YELLOW`<sup>1.37.0</sup> | `"\e[43m"` |  |
| `BG_BLUE`<sup>1.37.0</sup> | `"\e[44m"` |  |
| `BG_MAGENTA`<sup>1.37.0</sup> | `"\e[45m"` |  |
| `BG_CYAN`<sup>1.37.0</sup> | `"\e[46m"` |  |
| `BG_WHITE`<sup>1.37.0</sup> | `"\e[47m"` |  |

```just
@foo:
  echo {{HEX}}
```

```console
$ just foo
0123456789abcdef
```

以 `\e` 开头的常量是 [ANSI 转义序列](https://en.wikipedia.org/wiki/ANSI_escape_code)。

`CLEAR` 清除屏幕，类似于 `clear` 命令。其余常量的形式为 `\e[Nm`，其中 `N` 是整数，用于设置终端显示属性。

终端显示属性转义序列可以组合使用，例如文本粗细 `BOLD`、文本样式 `STRIKETHROUGH`、前景色 `CYAN` 和背景色 `BG_BLUE`。它们后面应该跟着 `NORMAL`，以将终端恢复正常。

转义序列应该加引号，因为某些 shell 会将 `[` 视为特殊字符。

```just
@foo:
  echo '{{BOLD + STRIKETHROUGH + CYAN + BG_BLUE}}Hi!{{NORMAL}}'
```

### 属性

配方、`mod` 语句和别名可以使用属性进行注解，以改变它们的行为。

| 名称 | 类型 | 描述 |
|------|------|-------------|
| `[arg(ARG, help="HELP")]`<sup>1.46.0</sup> | 配方 | 在使用说明中为 `ARG` 打印帮助字符串 `HELP`。 |
| `[arg(ARG, long="LONG")]`<sup>1.46.0</sup> | 配方 | 要求参数 `ARG` 的值以 `--LONG` 选项形式传递。 |
| `[arg(ARG, short="S")]`<sup>1.46.0</sup> | 配方 | 要求参数 `ARG` 的值以短选项 `-S` 形式传递。 |
| `[arg(ARG, value="VALUE")]`<sup>1.46.0</sup> | 配方 | 使选项 `ARG` 成为不接受值的标志。 |
| `[arg(ARG, pattern="PATTERN")]`<sup>1.45.0</sup> | 配方 | 要求参数 `ARG` 的值匹配正则表达式 `PATTERN`。 |
| `[confirm]`<sup>1.17.0</sup> | 配方 | 执行配方前需要确认。 |
| `[confirm(PROMPT)]`<sup>1.23.0</sup> | 配方 | 执行配方前需要确认，使用自定义提示语。 |
| `[default]`<sup>1.43.0</sup> | 配方 | 将配方用作模块的默认配方。 |
| `[doc(DOC)]`<sup>1.27.0</sup> | 模块、配方 | 将配方或模块的[文档注释](#文档注释)设置为 `DOC`。 |
| `[extension(EXT)]`<sup>1.32.0</sup> | 配方 | 将 shebang 配方脚本的文件扩展名设置为 `EXT`。如果需要句点，`EXT` 应包含句点。 |
| `[group(NAME)]`<sup>1.27.0</sup> | 模块、配方 | 将配方或模块放入[分组](#分组) `NAME` 中。 |
| `[linux]`<sup>1.8.0</sup> | 配方 | 在 Linux 上启用配方。 |
| `[macos]`<sup>1.8.0</sup> | 配方 | 在 MacOS 上启用配方。 |
| `[metadata(METADATA)]`<sup>1.42.0</sup> | 配方 | 将 `METADATA` 附加到配方。 |
| `[no-cd]`<sup>1.9.0</sup> | 配方 | 执行配方前不切换目录。 |
| `[no-exit-message]`<sup>1.7.0</sup> | 配方 | 配方失败时不打印错误消息。 |
| `[no-quiet]`<sup>1.23.0</sup> | 配方 | 覆盖全局静默配方设置，始终回显配方输出。 |
| `[openbsd]`<sup>1.38.0</sup> | 配方 | 在 OpenBSD 上启用配方。 |
| `[parallel]`<sup>1.42.0</sup> | 配方 | 并行运行此配方的依赖。 |
| `[positional-arguments]`<sup>1.29.0</sup> | 配方 | 为此配方启用[位置参数](#位置参数)。 |
| `[private]`<sup>1.10.0</sup> | 别名、配方 | 将配方、别名或变量设为私有。参见[私有配方](#私有配方)。 |
| `[script]`<sup>1.33.0</sup> | 配方 | 将配方作为脚本执行。更多详情请参阅[脚本配方](#脚本配方)。 |
| `[script(COMMAND)]`<sup>1.32.0</sup> | 配方 | 将配方作为由 `COMMAND` 解释的脚本执行。更多详情请参阅[脚本配方](#脚本配方)。 |
| `[unix]`<sup>1.8.0</sup> | 配方 | 在 Unix 系统上启用配方（包括 MacOS）。 |
| `[windows]`<sup>1.8.0</sup> | 配方 | 在 Windows 上启用配方。 |
| `[working-directory(PATH)]`<sup>1.38.0</sup> | 配方 | 设置配方的工作目录。`PATH` 可以是相对路径或绝对路径。如果是相对路径，则相对于默认工作目录解释。 |

一个配方可以有多个属性，可以写在多行上：

```just
[no-cd]
[private]
foo:
    echo "foo"
```

或者用逗号分隔写在一行上<sup>1.14.0</sup>：

```just
[no-cd, private]
foo:
    echo "foo"
```

带单个参数的属性可以使用冒号形式书写：

```just
[group: 'bar']
foo:
```

#### 启用和禁用配方<sup>1.8.0</sup>

`[linux]`、`[macos]`、`[unix]` 和 `[windows]` 属性是配置属性。默认情况下，配方始终启用。带有一个或多个配置属性的配方仅在其中一个或多个配置处于活动状态时才会启用。

这可用于编写根据运行的操作系统表现不同的 `justfile`。此 `justfile` 中的 `run` 配方将编译并运行 `main.c`，根据操作系统使用不同的 C 编译器和正确的输出二进制文件名：

```just
[unix]
run:
  cc main.c
  ./a.out

[windows]
run:
  cl main.c
  main.exe
```

#### 禁用目录切换<sup>1.9.0</sup>

`just` 通常会将当前目录设置为包含 `justfile` 的目录来执行配方。可以使用 `[no-cd]` 属性禁用此行为。这可用于创建使用相对于调用目录的路径的配方，或在当前目录上操作的配方。

例如，这个 `commit` 配方：

```just
[no-cd]
commit file:
  git add {{file}}
  git commit
```

可以使用相对于当前目录的路径，因为 `[no-cd]` 阻止 `just` 在执行 `commit` 时更改当前目录。

#### 配方需要确认<sup>1.17.0</sup>

`just` 通常会执行所有配方，除非出现错误。`[confirm]` 属性允许配方在运行前需要在终端中确认。可以通过向 `just` 传递 `--yes` 来覆盖此行为，这将自动确认所有标记了此属性的配方。

依赖于需要确认的配方的其他配方，如果所依赖的配方未被确认，则不会运行，同样，在需要确认的配方之后传递的配方也不会运行。

```just
[confirm]
delete-all:
  rm -rf *
```

#### 自定义确认提示<sup>1.23.0</sup>

可以使用 `[confirm(PROMPT)]` 覆盖默认确认提示：

```just
[confirm("Are you sure you want to delete everything?")]
delete-everything:
  rm -rf *
```

### 分组

配方和模块可以用一个或多个分组名称进行注解：

```just
[group('lint')]
js-lint:
    echo 'Running JS linter…'

[group('rust recipes')]
[group('lint')]
rust-lint:
    echo 'Running Rust linter…'

[group('lint')]
cpp-lint:
  echo 'Running C++ linter…'

# 不属于任何分组
email-everyone:
    echo 'Sending mass email…'
```

配方按分组列出：
```
$ just --list
Available recipes:
    email-everyone # not in any group

    [lint]
    cpp-lint
    js-lint
    rust-lint

    [rust recipes]
    rust-lint
```

`just --list --unsorted` 按照 justfile 中的原始顺序打印每个分组内的配方：

```
$ just --list --unsorted
Available recipes:
    (no group)
    email-everyone # not in any group

    [lint]
    js-lint
    rust-lint
    cpp-lint

    [rust recipes]
    rust-lint
```

可以使用 `--groups` 列出所有分组：

```
$ just --groups
Recipe groups:
  lint
  rust recipes
```

使用 `just --groups --unsorted` 可以按照 justfile 中的原始顺序打印分组。

### 使用反引号执行命令

反引号可用于存储命令的执行结果：

```just
localhost := `dumpinterfaces | cut -d: -f2 | sed 's/\/.*//' | sed 's/ //g'`

serve:
  ./serve {{localhost}} 8080
```

缩进的反引号由三个反引号分隔，其去缩进方式与缩进字符串相同：

````just
# 这个反引号执行命令 `echo foo\necho bar\n`，产生值 `foo\nbar\n`。
stuff := ```
    echo foo
    echo bar
  ```
````

有关去缩进的详细信息，请参阅[字符串](#字符串)部分。

反引号不能以 `#!` 开头。此语法保留供将来升级使用。

[`shell(…)` 函数](#外部命令)提供了更通用的机制来调用外部命令，包括执行变量内容作为命令的能力，以及向命令传递参数的能力。

### 条件表达式

`if`/`else` 表达式根据两个表达式的值是否相同来执行不同的分支：

```just
foo := if "2" == "2" { "Good!" } else { "1984" }

bar:
  @echo "{{foo}}"
```

```console
$ just bar
Good!
```

也可以测试不等性：

```just
foo := if "hello" != "goodbye" { "xyz" } else { "abc" }

bar:
  @echo {{foo}}
```

```console
$ just bar
xyz
```

还可以使用正则表达式进行匹配：

```just
foo := if "hello" =~ 'hel+o' { "match" } else { "mismatch" }

bar:
  @echo {{foo}}
```

```console
$ just bar
match
```

正则表达式由 [regex crate](https://github.com/rust-lang/regex) 提供，其语法文档位于 [docs.rs](https://docs.rs/regex/1.5.4/regex/#syntax)。由于正则表达式通常使用反斜杠转义序列，建议使用单引号字符串字面量，这样可以将反斜杠原样传递给正则表达式解析器。

条件表达式具有短路特性，这意味着它们只会执行其中一个分支。这可以用来确保反引号表达式在不应该运行时不会执行。

```just
foo := if env_var("RELEASE") == "true" { `get-something-from-release-database` } else { "dummy-value" }
```

条件表达式可以在配方内部使用：

```just
bar foo:
  echo {{ if foo == "bar" { "hello" } else { "goodbye" } }}
```

可以链接多个条件表达式：

```just
foo := if "hello" == "goodbye" {
  "xyz"
} else if "a" == "a" {
  "abc"
} else {
  "123"
}

bar:
  @echo {{foo}}
```

```console
$ just bar
abc
```

### 使用错误停止执行

可以使用 `error` 函数停止执行。例如：

```just
foo := if "hello" == "goodbye" {
  "xyz"
} else if "a" == "b" {
  "abc"
} else {
  error("123")
}
```

运行时将产生以下错误：

```
error: Call to function `error` failed: 123
   |
16 |   error("123")
```

### 从命令行设置变量

可以从命令行覆盖变量。

```just
os := "linux"

test: build
  ./test --test {{os}}

build:
  ./build {{os}}
```

```console
$ just
./build linux
./test --test linux
```

可以在配方名称之前传递任意数量的 `NAME=VALUE` 格式的参数：

```console
$ just os=plan9
./build plan9
./test --test plan9
```

或者使用 `--set` 标志：

```console
$ just --set os bsd
./build bsd
./test --test bsd
```

### 获取和设置环境变量

#### 导出 `just` 变量

以 `export` 关键字为前缀的赋值将作为环境变量导出到配方中：

```just
export RUST_BACKTRACE := "1"

test:
  # will print a stack trace if it crashes
  cargo test
```

以 `$` 为前缀的参数将作为环境变量导出：

```just
test $RUST_BACKTRACE="1":
  # will print a stack trace if it crashes
  cargo test
```

导出的变量和参数不会导出到同一作用域内的反引号中。

```just
export WORLD := "world"
# This backtick will fail with "WORLD: unbound variable"
BAR := `echo hello $WORLD`
```

```just
# Running `just a foo` will fail with "A: unbound variable"
a $A $B=`echo $A`:
  echo $A $B
```

当 [export](#导出) 设置被启用时，所有 `just` 变量都会作为环境变量导出。

#### 取消导出环境变量<sup>1.29.0</sup>

可以使用 `unexport` 关键字取消导出环境变量：

```just
unexport FOO

@foo:
  echo $FOO
```

```
$ export FOO=bar
$ just foo
sh: FOO: unbound variable
```

#### 从环境中获取环境变量

来自环境的环境变量会自动传递给配方。

```just
print_home_folder:
  echo "HOME is: '${HOME}'"
```

```console
$ just
HOME is '/home/myuser'
```

#### 从环境变量设置 `just` 变量

可以使用 `env()` 函数将环境变量传播到 `just` 变量中。参见[环境变量](#环境变量)。

### 配方参数

配方可以有参数。这里配方 `build` 有一个名为 `target` 的参数：

```just
build target:
  @echo 'Building {{target}}…'
  cd {{target}} && make
```

要在命令行上传递参数，请将它们放在配方名称之后：

```console
$ just build my-awesome-project
Building my-awesome-project…
cd my-awesome-project && make
```

要向依赖传递参数，请将依赖与参数一起放在括号中：

```just
default: (build "main")

build target:
  @echo 'Building {{target}}…'
  cd {{target}} && make
```

变量也可以作为参数传递给依赖：

```just
target := "main"

_build version:
  @echo 'Building {{version}}…'
  cd {{version}} && make

build: (_build target)
```

命令的参数可以通过将依赖与参数一起放在括号中来传递给依赖：

```just
build target:
  @echo "Building {{target}}…"

push target: (build target)
  @echo 'Pushing {{target}}…'
```

参数可以有默认值：

```just
default := 'all'

test target tests=default:
  @echo 'Testing {{target}}:{{tests}}…'
  ./test --tests {{tests}} {{target}}
```

有默认值的参数可以省略：

```console
$ just test server
Testing server:all…
./test --tests all server
```

或者提供：

```console
$ just test server unit
Testing server:unit…
./test --tests unit server
```

默认值可以是任意表达式，但包含 `+`、`&&`、`||` 或 `/` 运算符的表达式必须用括号括起来：

```just
arch := "wasm"

test triple=(arch + "-unknown-unknown") input=(arch / "input.dat"):
  ./test {{triple}}
```

配方的最后一个参数可以是可变参数，通过在参数名前加 `+` 或 `*` 来表示：

```just
backup +FILES:
  scp {{FILES}} me@server.com:
```

以 `+` 为前缀的可变参数接受*一个或多个*参数，并展开为由空格分隔的包含这些参数的字符串：

```console
$ just backup FAQ.md GRAMMAR.md
scp FAQ.md GRAMMAR.md me@server.com:
FAQ.md                  100% 1831     1.8KB/s   00:00
GRAMMAR.md              100% 1666     1.6KB/s   00:00
```

以 `*` 为前缀的可变参数接受*零个或多个*参数，并展开为由空格分隔的包含这些参数的字符串，如果没有参数则为空字符串：

```just
commit MESSAGE *FLAGS:
  git commit {{FLAGS}} -m "{{MESSAGE}}"
```

可变参数可以指定默认值。这些默认值会被命令行传递的参数覆盖：

```just
test +FLAGS='-q':
  cargo test {{FLAGS}}
```

如果 `{{…}}` 替换包含空格，可能需要加引号。例如，如果你有以下配方：

```just
search QUERY:
  lynx https://www.google.com/?q={{QUERY}}
```

然后输入：

```console
$ just search "cat toupee"
```

`just` 将运行命令 `lynx https://www.google.com/?q=cat toupee`，这会被 `sh` 解析为 `lynx`、`https://www.google.com/?q=cat` 和 `toupee`，而不是预期的 `lynx` 和 `https://www.google.com/?q=cat toupee`。

你可以通过添加引号来修复这个问题：

```just
search QUERY:
  lynx 'https://www.google.com/?q={{QUERY}}'
```

以 `$` 为前缀的参数将作为环境变量导出：

```just
foo $bar:
  echo $bar
```

参数可以使用 `[arg("name", pattern="pattern")]` 属性<sup>1.45.0</sup>约束为匹配正则表达式模式：

```just
[arg('n', pattern='\d+')]
double n:
  echo $(({{n}} * 2))
```

模式会自动添加前导 `^` 和尾随 `$`，因此它必须匹配整个参数值。

你可以使用 `|` 运算符将模式约束为多个备选项之一：

```just
[arg('flag', pattern='--help|--version')]
info flag:
  just {{flag}}
```

正则表达式由 [Rust `regex` crate](https://docs.rs/regex/latest/regex/) 提供。有关使用示例，请参阅[语法文档](https://docs.rs/regex/latest/regex/#syntax)。

可以使用 `--usage` 子命令<sup>1.46.0</sup>打印配方的使用信息：

```console
$ just --usage foo
Usage: just foo [OPTIONS] bar

Arguments:
  bar
```

可以使用 `[arg(ARG, help=HELP)]` 属性为参数添加帮助字符串：

```just
[arg("bar", help="hello")]
foo bar:
```

```console
$ just --usage foo
Usage: just foo bar

Arguments:
  bar hello
```

#### 配方标志和选项

配方参数默认是位置参数。

在这个 justfile 中：

```just
@foo bar:
  echo bar={{bar}}
```

参数 `bar` 是位置参数：

```console
$ just foo hello
bar=hello
```

`[arg(ARG, long=OPTION)]`<sup>1.46.0</sup> 属性可用于将参数设为长选项。

在这个 justfile 中：

```just
[arg("bar", long="bar")]
foo bar:
```

参数 `bar` 通过 `--bar` 选项传递：

```console
$ just foo --bar hello
bar=hello
```

选项也可以使用 `--name=value` 语法传递：

```console
$ just foo --bar=hello
bar=hello
```

`long` 的值可以省略，在这种情况下选项默认为参数的名称：

```just
[arg("bar", long)]
foo bar:
```

`[arg(ARG, short=OPTION)]`<sup>1.46.0</sup> 属性可用于将参数设为短选项。

在这个 justfile 中：

```just
[arg("bar", short="b")]
foo bar:
```

参数 `bar` 通过 `-b` 选项传递：

```console
$ just foo -b hello
bar=hello
```

如果参数同时具有长选项和短选项，则可以使用任一方式传递。

可变参数 `+` 和 `?` 不能作为选项。

`[arg(ARG, value=VALUE, …)]`<sup>1.46.0</sup> 属性可与 `long` 或 `short` 一起使用，将参数设为不接受值的标志。

在这个 justfile 中：

```just
[arg("bar", long="bar", value="hello")]
foo bar:
```

参数 `bar` 通过 `--bar` 选项传递，但不接受值，而是使用 `[arg]` 属性中给定的值：

```console
$ just foo --bar
bar=hello
```

这对于在危险命令上无条件要求 `--force` 之类的标志很有用。

如果参数有默认值，则标志是可选的：

```just
[arg("bar", long="bar", value="hello")]
foo bar="goodbye":
```

当调用时未传递该标志时，它会接收默认值：

```console
$ just foo
bar=goodbye
```

### 依赖

依赖在依赖它们的配方之前运行：

```just
a: b
  @echo A

b:
  @echo B
```

```
$ just a
B
A
```

在 `just` 的给定调用中，具有相同参数的配方只会运行一次，无论它在命令行调用中出现多少次，或者作为依赖出现多少次：

```just
a:
  @echo A

b: a
  @echo B

c: a
  @echo C
```

```
$ just a a a a a
A
$ just b c
A
B
C
```

多个配方可以依赖于执行某种设置的配方，当这些配方运行时，该设置只会执行一次：

```just
build:
  cc main.c

test-foo: build
  ./a.out --test foo

test-bar: build
  ./a.out --test bar
```

```
$ just test-foo test-bar
cc main.c
./a.out --test foo
./a.out --test bar
```

在给定的运行中，只有当配方接收相同的参数时才会被跳过：

```just
build:
  cc main.c

test TEST: build
  ./a.out --test {{TEST}}
```

```
$ just test foo test bar
cc main.c
./a.out --test foo
./a.out --test bar
```

#### 在配方末尾运行配方

配方的普通依赖总是在配方开始之前运行。也就是说，被依赖者总是在依赖者之前运行。这些依赖被称为"前置依赖"。

配方也可以有后置依赖，它们在配方之后立即运行，用 `&&` 引入：

```just
a:
  echo 'A!'

b: a && c d
  echo 'B!'

c:
  echo 'C!'

d:
  echo 'D!'
```

...运行 _b_ 打印：

```console
$ just b
echo 'A!'
A!
echo 'B!'
B!
echo 'C!'
C!
echo 'D!'
D!
```

#### 在配方中间运行配方

`just` 不支持在另一个配方的中间运行配方，但你可以在配方中间递归调用 `just`。给定以下 justfile：

```just
a:
  echo 'A!'

b: a
  echo 'B start!'
  just c
  echo 'B end!'

c:
  echo 'C!'
```

...运行 _b_ 打印：

```console
$ just b
echo 'A!'
A!
echo 'B start!'
B start!
echo 'C!'
C!
echo 'B end!'
B end!
```

这有一些限制，因为配方 `c` 是用全新的 `just` 调用运行的：赋值会被重新计算，依赖可能会运行两次，命令行参数不会传播到子 `just` 进程。

### Shebang 配方

以 `#!` 开头的配方称为 shebang 配方，它们通过将配方体保存到文件并运行该文件来执行。这让你可以用不同的语言编写配方：

```just
polyglot: python js perl sh ruby nu

python:
  #!/usr/bin/env python3
  print('Hello from python!')

js:
  #!/usr/bin/env node
  console.log('Greetings from JavaScript!')

perl:
  #!/usr/bin/env perl
  print "Larry Wall says Hi!\n";

sh:
  #!/usr/bin/env sh
  hello='Yo'
  echo "$hello from a shell script!"

nu:
  #!/usr/bin/env nu
  let hello = 'Hola'
  echo $"($hello) from a nushell script!"

ruby:
  #!/usr/bin/env ruby
  puts "Hello from ruby!"
```

```console
```console
$ just polyglot
Hello from python!
Greetings from JavaScript!
Larry Wall says Hi!
Yo from a shell script!
Hola from a nushell script!
Hello from ruby!
```

在类 Unix 操作系统上,包括 Linux 和 MacOS,shebang 配方通过将配方主体保存到临时目录中的文件、将文件标记为可执行,然后执行它来运行。操作系统随后解析 shebang 行为命令行并调用它,包括文件路径。例如,如果配方以 `#!/usr/bin/env bash` 开头,操作系统运行的最终命令将类似于 `/usr/bin/env bash /tmp/PATH_TO_SAVED_RECIPE_BODY`。

Shebang 行的分割是依赖于操作系统的。当传递带有参数的命令时,你可能需要使用 `-S` 标志告诉 `env` 显式分割它们:

```just
run:
  #!/usr/bin/env -S bash -x
  ls
```

Windows 不支持 shebang 行。在 Windows 上,`just` 将 shebang 行分割为命令和参数,将配方主体保存到文件,并调用分割后的命令和参数,将保存的配方主体路径作为最后一个参数添加。例如,在 Windows 上,如果配方以 `#! py` 开头,操作系统运行的最终命令将类似于 `py C:\Temp\PATH_TO_SAVED_RECIPE_BODY`。

### 脚本配方

带有 `[script(COMMAND)]`<sup>1.32.0</sup> 属性的配方作为由 `COMMAND` 解释的脚本运行。这避免了 shebang 配方的一些问题,例如在 Windows 上使用 `cygpath`、需要使用 `/usr/bin/env`、不同 Unix 操作系统之间 shebang 行分割的不一致性,以及需要一个可以执行文件的临时目录。

带有空 `[script]` 属性的配方使用 `set script-interpreter := […]`<sup>1.33.0</sup> 的值执行,默认为 `sh -eu`,而*不是* `set shell` 的值。

配方主体被求值后写入临时目录中的磁盘,并通过将其路径作为参数传递给 `COMMAND` 来运行。

### 脚本和 Shebang 配方临时文件

脚本配方和 shebang 配方都将配方主体写入临时文件以供执行。脚本配方通过将文件传递给命令来执行,而 shebang 配方直接执行该文件。如果包含临时文件的文件系统以 `noexec` 挂载或以其他方式不可执行,shebang 配方执行将失败。

`just` 写入临时文件的目录可以通过多种方式配置,按优先级从高到低:

- 使用 `--tempdir` 命令行选项或 `JUST_TEMPDIR` 环境变量<sup>1.41.0</sup>全局设置。

- 使用 `tempdir` 设置按模块设置。

- 在 Linux 上使用 `XDG_RUNTIME_DIR` 环境变量全局设置。

- 回退到 [std::env::temp_dir](https://doc.rust-lang.org/std/env/fn.temp_dir.html) 返回的目录。

### 使用 `uv` 运行 Python 配方

[`uv`](https://github.com/astral-sh/uv) 是一个出色的跨平台 Python 项目管理器,用 Rust 编写。

使用 `[script]` 属性和 `script-interpreter` 设置,`just` 可以轻松配置为使用 `uv` 运行 Python 配方:

```just
set unstable

set script-interpreter := ['uv', 'run', '--script']

[script]
hello:
  print("Hello from Python!")

[script]
goodbye:
  # /// script
  # requires-python = ">=3.11"
  # dependencies=["sh"]
  # ///
  import sh
  print(sh.echo("Goodbye from Python!"), end='')
```

当然,shebang 也可以工作:

```just
hello:
  #!/usr/bin/env -S uv run --script
  print("Hello from Python!")
```


### 更安全的 Bash Shebang 配方

如果你正在编写 `bash` shebang 配方,考虑添加 `set -euxo pipefail`:

```just
foo:
  #!/usr/bin/env bash
  set -euxo pipefail
  hello='Yo'
  echo "$hello from Bash!"
```

这不是严格必需的,但 `set -euxo pipefail` 启用了一些有用的特性,使 `bash` shebang 配方的行为更像普通的逐行 `just` 配方:

- `set -e` 使 `bash` 在命令失败时退出。

- `set -u` 使 `bash` 在变量未定义时退出。

- `set -x` 使 `bash` 在运行每行脚本前打印它。

- `set -o pipefail` 使 `bash` 在管道中的命令失败时退出。这是 `bash` 特有的,所以在普通的逐行 `just` 配方中不会启用。

这些选项组合在一起,可以避免很多 shell 脚本的陷阱。

#### Windows 上的 Shebang 配方执行

在 Windows 上,包含 `/` 的 shebang 解释器路径会使用 `cygpath` 从 Unix 风格路径转换为 Windows 风格路径,`cygpath` 是随 [Cygwin](http://www.cygwin.com) 一起提供的实用程序。

例如,要在 Windows 上执行这个配方:

```just
echo:
  #!/bin/sh
  echo "Hello!"
```

解释器路径 `/bin/sh` 将在执行前使用 `cygpath` 转换为 Windows 风格路径。

如果解释器路径不包含 `/`,它将直接执行而不进行转换。如果 `cygpath` 不可用,或者你希望向解释器传递 Windows 风格路径,这会很有用。

### 在配方中设置变量

配方行由 shell 解释,而不是 `just`,所以不可能在配方中间设置 `just` 变量:

```justfile
foo:
  x := "hello" # 这不起作用!
  echo {{x}}
```

可以使用 shell 变量,但还有另一个问题。每个配方行都由新的 shell 实例运行,所以在一行中设置的变量在下一行中不会被设置:

```just
foo:
  x=hello && echo $x # 这有效!
  y=bye
  echo $y            # 这不行,`y` 在这里未定义!
```

解决这个问题的最佳方法是使用 shebang 配方。shebang 配方主体被提取并作为脚本运行,所以单个 shell 实例将运行整个内容:

```just
foo:
  #!/usr/bin/env bash
  set -euxo pipefail
  x=hello
  echo $x
```

### 在配方之间共享环境变量

每个配方的每一行都由新的 shell 执行,因此不可能在配方之间共享环境变量。

#### 使用 Python 虚拟环境

一些工具,如 [Python 的 venv](https://docs.python.org/3/library/venv.html),需要加载环境变量才能工作,这使得它们难以与 `just` 一起使用。作为解决方法,你可以直接执行虚拟环境二进制文件:

```just
venv:
  [ -d foo ] || python3 -m venv foo

run: venv
  ./foo/bin/python3 main.py
```

### 在配方中更改工作目录

每个配方行都由新的 shell 执行,所以如果你在一行中更改工作目录,它不会影响后面的行:

```just
foo:
  pwd    # 这个 `pwd` 将打印相同的目录...
  cd bar
  pwd    # ...和这个 `pwd`!
```

有几种方法可以解决这个问题。一种是在同一行中调用 `cd` 和你想运行的命令:

```just
foo:
  cd bar && pwd
```

另一种是使用 shebang 配方。shebang 配方主体被提取并作为脚本运行,所以单个 shell 实例将运行整个内容,因此一行中的 `cd` 将影响后面的行,就像 shell 脚本一样:

```just
foo:
  #!/usr/bin/env bash
  set -euxo pipefail
  cd bar
  pwd
```

### 缩进

配方行可以用空格或制表符缩进,但不能混合使用。一个配方的所有行必须具有相同类型的缩进,但同一个 `justfile` 中的不同配方可以使用不同的缩进。

每个配方必须至少比 `recipe-name` 缩进一级,但之后可以进一步缩进。

这是一个配方用空格缩进(用 `·` 表示)和制表符缩进(用 `→` 表示)的 justfile。

```justfile
set windows-shell := ["pwsh", "-NoLogo", "-NoProfileLoadTime", "-Command"]

set ignore-comments

list-space directory:
··#!pwsh
··foreach ($item in $(Get-ChildItem {{directory}} )) {
····echo $item.Name
··}
··echo ""

# 缩进嵌套即使在换行符被转义时也有效
list-tab directory:
→ @foreach ($item in $(Get-ChildItem {{directory}} )) { \
→ → echo $item.Name \
→ }
→ @echo ""
```

```pwsh
PS > just list-space ~
Desktop
Documents
Downloads

PS > just list-tab ~
Desktop
Documents
Downloads
```

### 多行结构

没有初始 shebang 的配方逐行求值和运行,这意味着多行结构可能不会按你期望的方式工作。

例如,使用以下 `justfile`:

```justfile
conditional:
  if true; then
    echo 'True!'
  fi
```

`conditional` 配方第二行前面的额外前导空白将产生解析错误:

```console
$ just conditional
error: Recipe line has extra leading whitespace
  |
3 |         echo 'True!'
  |     ^^^^^^^^^^^^^^^^
```

要解决这个问题,你可以将条件语句写在一行上,用斜杠转义换行符,或者为配方添加 shebang。以下提供了一些多行结构的示例供参考。

#### `if` 语句

```just
conditional:
  if true; then echo 'True!'; fi
```

```just
conditional:
  if true; then \
    echo 'True!'; \
  fi
```

```just
conditional:
  #!/usr/bin/env sh
  if true; then
    echo 'True!'
  fi
```

#### `for` 循环

```just
for:
  for file in `ls .`; do echo $file; done
```

```just
for:
  for file in `ls .`; do \
    echo $file; \
  done
```

```just
for:
  #!/usr/bin/env sh
  for file in `ls .`; do
    echo $file
  done
```

#### `while` 循环

```just
while:
  while `server-is-dead`; do ping -c 1 server; done
```

```just
while:
  while `server-is-dead`; do \
    ping -c 1 server; \
  done
```

```just
while:
  #!/usr/bin/env sh
  while `server-is-dead`; do
    ping -c 1 server
  done
```

#### 配方主体之外

括号表达式可以跨越多行:

```just
abc := ('a' +
        'b'
         + 'c')

abc2 := (
  'a' +
  'b' +
  'c'
)

foo param=('foo'
      + 'bar'
    ):
  echo {{param}}

bar: (foo
        'Foo'
     )
  echo 'Bar!'
```

以反斜杠结尾的行延续到下一行,就像这些行被空白连接一样<sup>1.15.0</sup>:

```just
a := 'foo' + \
     'bar'

foo param1 \
  param2='foo' \
  *varparam='': dep1 \
                (dep2 'foo')
  echo {{param1}} {{param2}} {{varparam}}

dep1: \
    # 这个注释不是配方主体的一部分
  echo 'dep1'

dep2 \
  param:
    echo 'Dependency with parameter {{param}}'
```

反斜杠行延续也可以用在插值中。反斜杠后面的行必须缩进。

```just
recipe:
  echo '{{ \
  "This interpolation " + \
    "has a lot of text." \
  }}'
  echo 'back to recipe body'
```

### 命令行选项

`just` 支持许多有用的命令行选项,用于列出、转储和调试配方和变量:

```console
$ just --list
Available recipes:
  js
  perl
  polyglot
  python
  ruby
$ just --show perl
perl:
  #!/usr/bin/env perl
  print "Larry Wall says Hi!\n";
$ just --show polyglot
polyglot: python js perl sh ruby
```

#### 使用环境变量设置命令行选项

一些命令行选项可以用环境变量设置

例如,可以使用 `--unstable` 标志启用不稳定特性:

```console
$ just --unstable
```

或者通过设置 `JUST_UNSTABLE` 环境变量:

```console
$ export JUST_UNSTABLE=1
$ just
```

由于环境变量被子进程继承,用环境变量设置的命令行选项会被 `just` 的递归调用继承,而用参数设置的命令行选项则不会。

查阅 `just --help` 了解哪些选项可以用环境变量设置。

### 私有配方

名称以 `_` 开头的配方和别名在 `just --list` 中被省略:

```just
test: _test-helper
  ./bin/test

_test-helper:
  ./bin/super-secret-test-helper-stuff
```

```console
$ just --list
Available recipes:
    test
```

以及在 `just --summary` 中:

```console
$ just --summary
test
```

`[private]` 属性<sup>1.10.0</sup>也可用于隐藏配方或别名,而无需更改名称:

```just
[private]
foo:

[private]
alias b := bar

bar:
```

```console
$ just --list
Available recipes:
    bar
```

这对于仅用作其他配方依赖的辅助配方很有用。

### 静默配方

配方名称可以用 `@` 前缀来反转每行前面 `@` 的含义:

```just
@quiet:
  echo hello
  echo goodbye
  @# all done!
```

现在只有以 `@` 开头的行会被回显:

```console
$ just quiet
hello
goodbye
# all done!
```

justfile 中的所有配方都可以通过 `set quiet` 设为静默:

```just
set quiet

foo:
  echo "This is quiet"

@foo2:
  echo "This is also quiet"
```

`[no-quiet]` 属性可以覆盖这个设置:

```just
set quiet

foo:
  echo "This is quiet"

[no-quiet]
foo2:
  echo "This is not quiet"
```

Shebang 配方默认是静默的:

```just
foo:
  #!/usr/bin/env bash
  echo 'Foo!'
```

```console
$ just foo
Foo!
```

在 shebang 配方名称前添加 `@` 会使 `just` 在执行前打印配方:

```just
@bar:
  #!/usr/bin/env bash
  echo 'Bar!'
```

```console
$ just bar
#!/usr/bin/env bash
echo 'Bar!'
Bar!
```

`just` 通常在配方行失败时打印错误消息。这些错误消息可以使用 `[no-exit-message]`<sup>1.7.0</sup> 属性抑制。当配方包装工具时,你可能会发现这特别有用:

```just
git *args:
    @git {{args}}
```

```console
$ just git status
fatal: not a git repository (or any of the parent directories): .git
error: Recipe `git` failed on line 2 with exit code 128
```

添加该属性以在工具以非零代码退出时抑制退出错误消息:

```just
[no-exit-message]
git *args:
    @git {{args}}
```

```console
$ just git status
fatal: not a git repository (or any of the parent directories): .git
```

### 使用交互式选择器选择要运行的配方

`--choose` 子命令使 `just` 调用选择器来选择要运行的配方。选择器应从标准输入读取包含配方名称的行,并将一个或多个以空格分隔的名称打印到标准输出。

由于目前没有办法用 `--choose` 运行需要参数的配方,这类配方不会提供给选择器。私有配方和别名也会被跳过。

可以使用 `--chooser` 标志覆盖选择器。如果没有给出 `--chooser`,`just` 首先检查是否设置了 `$JUST_CHOOSER`。如果没有,选择器默认为 `fzf`,一个流行的模糊查找器。

参数可以包含在选择器中,例如 `fzf --exact`。

选择器的调用方式与配方行相同。例如,如果选择器是 `fzf`,它将通过 `sh -cu 'fzf'` 调用,如果 shell 或 shell 参数被覆盖,选择器调用将遵循这些覆盖。

如果你希望 `just` 默认使用选择器选择配方,你可以将此作为默认配方:

```just
default:
  @just --choose
```

### 在其他目录中调用 `justfile`

如果传递给 `just` 的第一个参数包含 `/`,则会发生以下情况:

1.  参数在最后一个 `/` 处分割。

2.  最后一个 `/` 之前的部分被视为目录。`just` 将从那里开始搜索 `justfile`,而不是从当前目录开始。

3.  最后一个斜杠之后的部分被视为普通参数,如果为空则被忽略。

这可能看起来有点奇怪,但如果你想运行子目录中 `justfile` 中的命令,它很有用。

例如,如果你在一个包含名为 `foo` 的子目录的目录中,该子目录包含一个带有 `build` 配方的 `justfile`,并且 `build` 也是默认配方,以下命令都是等效的:

```console
$ (cd foo && just build)
$ just foo/build
$ just foo/
```

第一个之后的其他配方在同一个 `justfile` 中查找。例如,以下两个命令都是等效的:

```console
$ just foo/a b
$ (cd foo && just a b)
```

并且都会调用 `foo/justfile` 中的配方 `a` 和 `b`。

### 导入

一个 `justfile` 可以使用 `import` 语句包含另一个文件的内容。

如果你有以下 `justfile`:

```justfile
import 'foo/bar.just'

a: b
  @echo A
```

以及 `foo/bar.just` 中的以下文本:

```just
b:
  @echo B
```

`foo/bar.just` 将被包含在 `justfile` 中,配方 `b` 将被定义:

```console
$ just b
B
$ just a
B
A
```

`import` 路径可以是绝对路径或相对于包含它的 justfile 位置的相对路径。导入路径中的前导 `~/` 会被替换为当前用户的主目录。

justfile 对顺序不敏感,所以被包含的文件可以引用 `import` 语句之后定义的变量和配方。

被导入的文件本身可以包含 `import`,这些会被递归处理。

`allow-duplicate-recipes` 和 `allow-duplicate-variables` 分别允许重复的配方和变量相互覆盖,而不是产生错误。

在模块内,后面的定义覆盖前面的定义:

```just
set allow-duplicate-recipes

foo:

foo:
  echo 'yes'
```

当涉及 `import` 时,情况不幸变得更加复杂且难以解释。

较浅的定义总是覆盖较深的定义,所以顶层的配方将覆盖导入中的配方,导入中的配方将覆盖本身导入这些配方的导入中的配方。

当两个重复的定义被导入且处于相同深度时,来自较早导入的定义将覆盖来自较晚导入的定义。

这是因为 `just` 在处理导入时使用栈,按源代码顺序将导入推入栈中,并始终处理栈顶,所以较早的导入实际上被编译器稍后处理。

这确实是一个 bug,但由于 `just` 有非常强的向后兼容性保证,我们非常努力地不破坏任何人的 `justfile`,我们创建了 issue #2540 来讨论是否真的可以修复它。

可以通过在 `import` 关键字后放置 `?` 使导入变为可选:

```just
import? 'foo/bar.just'
```

多次导入同一个源文件不是错误<sup>1.37.0</sup>。这允许导入多个 justfile,例如 `foo.just` 和 `bar.just`,它们都导入包含共享配方的第三个 justfile,例如 `baz.just`,而 `baz.just` 的重复导入不会是错误:

```justfile
# justfile
import 'foo.just'
import 'bar.just'
```

```justfile
# foo.just
import 'baz.just'
foo: baz
```

```justfile
# bar.just
import 'baz.just'
bar: baz
```
import 'baz.just'
bar: baz
```

```just
# baz
baz:
```

### 模块<sup>1.19.0</sup>

`justfile` 可以使用 `mod` 语句声明模块。

`mod` 语句在 `just`<sup>1.31.0</sup> 中已稳定。在早期版本中，您需要使用 `--unstable` 标志、`set unstable` 或设置 `JUST_UNSTABLE` 环境变量来使用它们。

如果您有以下 `justfile`：

```justfile
mod bar

a:
  @echo A
```

以及 `bar.just` 中的以下内容：

```just
b:
  @echo B
```

`bar.just` 将作为子模块包含在 `justfile` 中。在一个子模块中定义的配方、别名和变量不能在另一个子模块中使用，并且每个模块使用自己的设置。

子模块中的配方可以作为子命令调用：

```console
$ just bar b
B
```

或使用路径语法：

```console
$ just bar::b
B
```

如果模块名为 `foo`，just 将在 `foo.just`、`foo/mod.just`、`foo/justfile` 和 `foo/.justfile` 中搜索模块文件。在后两种情况下，模块文件可以使用任何大小写形式。

模块语句可以是以下形式：

```justfile
mod foo 'PATH'
```

这将从 `PATH` 加载模块的源文件，而不是从通常的位置。`PATH` 中的前导 `~/` 将被替换为当前用户的主目录。`PATH` 可以指向模块源文件本身，或指向包含名为 `mod.just`、`justfile` 或 `.justfile` 的模块源文件的目录。在后两种情况下，模块文件可以使用任何大小写形式。

环境文件仅为根 justfile 加载，加载的环境变量在子模块中可用。子模块中影响环境文件加载的设置将被忽略。

子模块中没有 `[no-cd]` 属性的配方将在包含子模块源文件的目录中运行。

`justfile()` 和 `justfile_directory()` 始终返回根 justfile 的路径及其所在目录，即使从子模块配方中调用也是如此。

可以通过在 `mod` 关键字后加 `?` 使模块变为可选：

```just
mod? foo
```

可选模块缺少源文件不会产生错误。

没有源文件的可选模块不会冲突，因此您可以有多个具有相同名称但不同源文件路径的 mod 语句，只要最多存在一个源文件：

```just
mod? foo 'bar.just'
mod? foo 'baz.just'
```

模块可以添加文档注释，这些注释会出现在 `--list` 输出中<sup>1.30.0</sup>：

```justfile
# foo is a great module!
mod foo
```

```console
$ just --list
Available recipes:
    foo ... # foo is a great module!
```

模块仍然缺少很多功能，例如引用其他模块中变量的能力。有关更多信息，请参阅[模块改进跟踪 issue](https://github.com/casey/just/issues/2252)。

### 隐藏 `justfile`

`just` 会查找名为 `justfile` 和 `.justfile` 的 justfile，后者可用于隐藏 `justfile`。

### Just 脚本

通过在 `justfile` 顶部添加 shebang 行并使其可执行，`just` 可以用作脚本的解释器：

```console
$ cat > script <<EOF
#!/usr/bin/env just --justfile

foo:
  echo foo
EOF
$ chmod +x script
$ ./script foo
echo foo
foo
```

当执行带有 shebang 的脚本时，系统会将脚本的路径作为参数提供给 shebang 中的命令。因此，使用 `#!/usr/bin/env just --justfile` shebang 时，命令将是 `/usr/bin/env just --justfile PATH_TO_SCRIPT`。

使用上述 shebang，`just` 会将其工作目录更改为脚本所在位置。如果您希望保持工作目录不变，请使用 `#!/usr/bin/env just --working-directory . --justfile`。

注意：Shebang 行分割在不同操作系统之间不一致。前面的示例仅在 macOS 上测试过。在 Linux 上，您可能需要向 `env` 传递 `-S` 标志：

```just
#!/usr/bin/env -S just --justfile

default:
  echo foo
```

### 格式化和导出 `justfile`

每个 `justfile` 都有关于空白和换行的规范格式。

您可以使用当前不稳定的 `--fmt` 标志用规范格式化的版本覆盖当前 justfile：

```console
$ cat justfile
# A lot of blank lines




some-recipe:
  echo "foo"
$ just --fmt --unstable
$ cat justfile
# A lot of blank lines

some-recipe:
    echo "foo"
```

调用 `just --fmt --check --unstable` 以检查模式运行 `--fmt`。`just` 不会覆盖 `justfile`，而是在格式正确时以退出码 0 退出，如果格式不正确则以退出码 1 退出并打印差异。

您可以使用 `--dump` 命令将格式化版本的 `justfile` 输出到标准输出：

```console
$ just --dump > formatted-justfile
```

`--dump` 命令可以与 `--dump-format json` 一起使用，以打印 `justfile` 的 JSON 表示。

### 回退到父 `justfile`

如果在 `justfile` 中找不到配方且设置了 `fallback` 设置，`just` 将在父目录及更上层目录中查找 `justfile`，直到到达根目录。`just` 将在到达 `fallback` 设置为 `false` 或未设置的 `justfile` 后停止。

例如，假设当前目录包含以下 `justfile`：

```just
set fallback
foo:
  echo foo
```

父目录包含以下 `justfile`：

```just
bar:
  echo bar
```

```console
$ just bar
Trying ../justfile
echo bar
bar
```

### 避免参数分割

给定以下 `justfile`：

```just
foo argument:
  touch {{argument}}
```

以下命令将创建两个文件，`some` 和 `argument.txt`：

```console
$ just foo "some argument.txt"
```

用户的 shell 会将 `"some argument.txt"` 解析为单个参数，但当 `just` 将 `touch {{argument}}` 替换为 `touch some argument.txt` 时，引号不会保留，`touch` 将收到两个参数。

有几种方法可以避免这种情况：引号、位置参数和导出参数。

#### 引号

可以在 `{{argument}}` 插值周围添加引号：

```just
foo argument:
  touch '{{argument}}'
```

这保留了 `just` 在运行前捕获变量名拼写错误的能力，例如如果您写成 `{{argument}}`，但如果 `argument` 的值包含单引号，则不会按预期工作。

#### 位置参数

`positional-arguments` 设置会使所有参数作为位置参数传递，允许使用 `$1`、`$2`、... 和 `$@` 访问它们，然后可以用双引号括起来以避免 shell 进一步分割：

```just
set positional-arguments

foo argument:
  touch "$1"
```

这会使 `just` 无法捕获拼写错误，例如如果您输入 `$2` 而不是 `$1`，但适用于 `argument` 的所有可能值，包括那些带有双引号的值。

#### 导出参数

当设置 `export` 设置时，所有参数都会被导出：

```just
set export

foo argument:
  touch "$argument"
```

或者可以通过在参数前加 `$` 前缀来导出单个参数：

```just
foo $argument:
  touch "$argument"
```

这会使 `just` 无法捕获拼写错误，例如如果您输入 `$argument`，但适用于 `argument` 的所有可能值，包括那些带有双引号的值。

### 配置 Shell

有多种方法可以为逐行配方配置 shell，这是配方不以 `#!` shebang 开头时的默认行为。它们的优先级从高到低为：

1. `--shell` 和 `--shell-arg` 命令行选项。传递其中任何一个都会导致 `just` 忽略当前 justfile 中的任何设置。
2. `set windows-shell := [...]`
3. `set windows-powershell`（已弃用）
4. `set shell := [...]`

由于 `set windows-shell` 的优先级高于 `set shell`，您可以使用 `set windows-shell` 在 Windows 上选择 shell，使用 `set shell` 为所有其他平台选择 shell。

### 时间戳

`just` 可以在每个配方命令之前打印时间戳：

```just
recipe:
  echo one
  sleep 2
  echo two
```

```
$ just --timestamp recipe
[07:28:46] echo one
one
[07:28:46] sleep 2
[07:28:48] echo two
two
```

默认情况下，时间戳格式为 `HH:MM:SS`。可以使用 `--timestamp-format` 更改格式：

```
$ just --timestamp recipe --timestamp-format '%H:%M:%S%.3f %Z'
[07:32:11:.349 UTC] echo one
one
[07:32:11:.350 UTC] sleep 2
[07:32:13:.352 UTC] echo two
two
```

`--timestamp-format` 的参数是 `strftime` 风格的格式字符串，详情请参阅 [`chrono` 库文档](https://docs.rs/chrono/latest/chrono/format/strftime/index.html)。

### 信号处理

[信号](https://en.wikipedia.org/wiki/Signal_(IPC))是发送给运行中程序以触发特定行为的消息。例如，当按下 `CTRL-C` 时，`SIGINT` 会发送到终端前台进程组中的所有进程。

`just` 尝试在收到信号请求时退出，但它也尽量避免留下正在运行的子进程，这两个目标有些冲突。

如果 `just` 退出时留下了子进程，用户将不得不使用 `ps aux | grep` 查找子进程并手动 `kill` 它们，这是一项繁琐的工作。

#### 致命信号

`SIGHUP`、`SIGINT` 和 `SIGQUIT` 分别在用户关闭终端、输入 `ctrl-c` 或输入 `ctrl-\` 时生成，并发送到前台进程组中的所有进程。

`SIGTERM` 是 `kill` 命令发送的默认信号，仅发送给其目标进程。

当没有子进程运行时，`just` 收到上述任何信号后会立即退出。

当子进程*正在*运行时，`just` 会等待其终止，以避免留下它。

此外，收到 `SIGTERM` 时，`just` 会将 `SIGTERM` 转发给任何正在运行的子进程<sup>1.41.0</sup>，因为与其他致命信号不同，`SIGTERM` 可能仅发送给 `just` 本身。

无论 `just` 收到致命信号后子进程是否成功终止，`just` 都会停止执行。

#### `SIGINFO`

`SIGINFO` 在用户在 [BSD](https://en.wikipedia.org/wiki/Berkeley_Software_Distribution) 衍生的操作系统（包括 MacOS，但不包括 Linux）上输入 `ctrl-t` 时发送到前台进程组中的所有进程。

`just` 通过打印所有子进程 ID 和命令列表来响应<sup>1.41.0</sup>。

#### Windows

在 Windows 上，当用户输入 `ctrl-c` 时，`just` 的行为就像收到了 `SIGINT` 一样。其他信号不受支持。
更新日志
-------

最新版本的更新日志可在 [CHANGELOG.md](https://raw.githubusercontent.com/casey/just/master/CHANGELOG.md) 中查看。
先前版本的更新日志可在 [发布页面](https://github.com/casey/just/releases) 查看。`just --changelog` 也可以用来让 `just` 二进制文件打印其更新日志。
杂项
-----------

### 文件变更时重新运行配方

[`watchexec`](https://github.com/mattgreen/watchexec) 可以在文件变更时重新运行任何命令。

要在任何文件变更时重新运行配方 `foo`：

```console
watchexec just foo
```

查看 `watchexec --help` 获取更多信息，包括如何指定应该监视哪些文件的变更。

### 并行执行

使用 `[parallel]` 属性可以并行运行依赖。

在这个 justfile 中，当运行 `main` 时，`foo`、`bar` 和 `baz` 将并行执行：

```just
[parallel]
main: foo bar baz

foo:
  sleep 1

bar:
  sleep 1

baz:
  sleep 1
```

GNU `parallel` 可用于并发运行配方行：

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

在 `bash` 中，别名命令可能不会保留下一节描述的 shell 补全功能。将以下行添加到你的 `.bashrc` 中，以便为别名命令使用与 `just` 相同的补全函数：

```console
complete -F _just -o bashdefault -o default j
```

### Shell 补全脚本

Bash、Elvish、Fish、Nushell、PowerShell 和 Zsh 的 shell 补全脚本可在[发布存档](https://github.com/casey/just/releases)中获取。

`just` 二进制文件也可以使用 `just --completions SHELL` 在运行时生成相同的补全脚本：

```console
$ just --completions zsh > just.zsh
```

请参阅你的 shell 文档了解如何安装它们。

*macOS 注意事项：* 最新版本的 macOS 使用 zsh 作为默认 shell。如果你使用 Homebrew 安装 `just`，它会自动在 Homebrew zsh 目录中安装最新版本的 zsh 补全脚本，而内置版本的 zsh 默认不知道这个目录。如果可能的话，最好使用这个版本的脚本，因为每次通过 Homebrew 更新 `just` 时它都会更新。此外，许多其他 Homebrew 包也在同一位置放置补全脚本，内置的 zsh 也不知道这些。要在这种情况下在 zsh 中使用 `just` 补全，你可以在调用 `compinit` 之前将 `fpath` 设置为 Homebrew 位置。还要注意 Oh My Zsh 默认会运行 `compinit`。所以你的 `.zshrc` 文件可能看起来像这样：

```zsh
# 初始化 Homebrew，这会添加环境变量
eval "$(brew shellenv)"

fpath=($HOMEBREW_PREFIX/share/zsh/site-functions $fpath)

# 然后选择以下选项之一：
# 1. 如果你使用 Oh My Zsh，可以在这里初始化它
# source $ZSH/oh-my-zsh.sh

# 2. 否则，自己运行 compinit
# autoload -U compinit
# compinit
```

### Man 手册页

`just` 可以使用 `just --man` 打印自己的 man 手册页。Man 手册页使用 [`roff`](https://en.wikipedia.org/wiki/Roff_%28software%29) 编写，这是一种古老的标记语言，也是 Unix 最早的实际应用之一。如果你安装了 [`groff`](https://www.gnu.org/software/groff/)，可以使用 `just --man | groff -mandoc -Tascii | less` 查看 man 手册页。

### 语法

justfile 的非规范语法可以在 [GRAMMAR.md](https://github.com/casey/just/blob/master/GRAMMAR.md) 中找到。

### just.sh

在 `just` 成为一个精致的 Rust 程序之前，它是一个调用 `make` 的小型 shell 脚本。你可以在 [contrib/just.sh](https://github.com/casey/just/blob/master/contrib/just.sh) 中找到旧版本。

### 全局和用户 justfile

如果你希望某些配方在任何地方都可用，你有几个选择。

#### 全局 Justfile

`just --global-justfile`，或简写为 `just -g`，按顺序在以下路径搜索 justfile：

- `$XDG_CONFIG_HOME/just/justfile`
- `$HOME/.config/just/justfile`
- `$HOME/justfile`
- `$HOME/.justfile`

你可以将跨多个项目使用的配方放在全局 justfile 中，以便从任何目录轻松调用它们。

#### 用户 justfile 技巧

你也可以采用以下一些工作流程。这些技巧假设你已经在 `~/.user.justfile` 创建了一个 justfile，但你可以将这个 justfile 放在系统上任何方便的路径。

##### 配方别名

如果你想通过名称调用 `~/.user.justfile` 中的配方，并且不介意为每个配方创建别名，请将以下内容添加到你的 shell 初始化脚本中：

```console
for recipe in `just --justfile ~/.user.justfile --summary`; do
  alias $recipe="just --justfile ~/.user.justfile --working-directory . $recipe"
done
```

现在，如果你在 `~/.user.justfile` 中有一个名为 `foo` 的配方，你只需在命令行中输入 `foo` 即可运行它。

我花了很长时间才意识到可以像这样创建配方别名。尽管我反应迟钝，但我很高兴能为你带来这项 justfile 技术的重大进步。

##### 转发别名

如果你不想为每个配方创建别名，可以创建一个单一的别名：

```console
alias .j='just --justfile ~/.user.justfile --working-directory .'
```

现在，如果你在 `~/.user.justfile` 中有一个名为 `foo` 的配方，你只需在命令行中输入 `.j foo` 即可运行它。

我很确定没有人真正使用这个功能，但它确实存在。

¯\\\_(ツ)\_/¯

##### 自定义

你可以使用其他选项自定义上述别名。例如，如果你希望 justfile 中的配方在你的主目录而不是当前目录中运行：

```console
alias .j='just --justfile ~/.user.justfile --working-directory ~'
```

### Node.js `package.json` 脚本兼容性

以下导出语句使 `just` 配方可以访问本地 Node 模块二进制文件，并使 `just` 配方命令的行为更像 Node.js `package.json` 文件中的 `script` 条目：

```just
export PATH := "./node_modules/.bin:" + env_var('PATH')
```

### Windows 上的路径

在 Windows 上，除了 `invocation_directory()` 之外，所有返回路径的函数都将返回以 `\` 分隔的路径。当不使用 PowerShell 或 `cmd.exe` 时，这些路径应该用引号括起来，以防止 `\` 被解释为字符转义：

```just
ls:
    echo '{{absolute_path(".")}}'
```

`cygpath.exe` 是一些 Windows Unix 用户空间发行版中包含的可执行文件，包括 [Cygwin](https://www.cygwin.com/) 和 Windows 版 [Git](https://git-scm.com/downloads)。

`just` 在两个地方使用 `cygpath.exe`：

为了向后兼容，`invocation_directory()` 使用 `cygpath.exe` 将调用目录转换为 Unix 风格的 `/` 分隔路径。使用 `invocation_directory_native()` 获取原生的 Windows 风格路径。在 Unix 上，`invocation_directory()` 和 `invocation_directory_native()` 都返回相同的 Unix 风格路径。

`cygpath.exe` 还用于将 Unix 风格的 shebang 行转换为 Windows 路径。作为替代方案，可以使用 `[script]` 属性（目前不稳定），它不依赖于 `cygpath.exe`。

如果 `cygpath.exe` 可用，你可以使用它在路径风格之间转换：

```just
foo_unix := '/hello/world'
foo_windows := shell('cygpath --windows $1', foo_unix)

bar_windows := 'C:\hello\world'
bar_unix := shell('cygpath --unix $1', bar_windows)
```

### 远程 Justfile

如果你希望在多个 justfile 中包含一个 `mod` 或 `import` 源文件而无需复制它，你可以使用可选的 `mod` 或 `import`，以及一个获取模块源文件的配方：

```just
import? 'foo.just'

fetch:
  curl https://raw.githubusercontent.com/casey/just/master/justfile > foo.just
```

给定上面的 justfile，在运行 `just fetch` 之后，`foo.just` 中的配方将可用。

### 打印复杂字符串

`echo` 可用于打印字符串，但因为它会处理转义序列（如 `\n`），而且不同的 `echo` 实现识别不同的转义序列，所以使用 `printf` 通常是更好的选择。

`printf` 接受一个 C 风格的格式字符串和任意数量的参数，这些参数会被插入到格式字符串中。

这可以与缩进的三引号字符串结合使用，以模拟 shell heredoc。

使用 `{…}` 将复杂字符串替换到配方体中也可能导致问题，因为它可能会根据空白和引号的存在而被 shell 分割成多个参数。将复杂字符串导出为环境变量并用 `"$NAME"`（注意双引号）引用它们也会有帮助。

综合以上所有内容，要将字符串原样打印到标准输出，保持其各种转义序列和引号不变：

```just
export FOO := '''
  a complicated string with
  some dis\tur\bi\ng escape sequences
  and "quotes" of 'different' kinds
'''

bar:
  printf %s "$FOO"
```

### 替代方案和先驱技术

命令运行器可不少！一些与 `just` 或多或少相似的替代方案包括：

- [make](https://en.wikipedia.org/wiki/Make_(software))：启发了 `just` 的 Unix 构建工具。原始 `make` 有几个不同的现代后代，包括 [FreeBSD Make](https://www.freebsd.org/cgi/man.cgi?make(1)) 和 [GNU Make](https://www.gnu.org/software/make/)。
- [task](https://github.com/go-task/task)：一个用 Go 编写的基于 YAML 的命令运行器。
- [maid](https://github.com/egoist/maid)：一个用 JavaScript 编写的基于 Markdown 的命令运行器。
- [microsoft/just](https://github.com/microsoft/just)：一个用 JavaScript 编写的基于 JavaScript 的命令运行器。
- [cargo-make](https://github.com/sagiegurari/cargo-make)：一个用于 Rust 项目的命令运行器。
- [mmake](https://github.com/tj/mmake)：一个 `make` 的包装器，具有许多改进，包括远程包含。
- [robo](https://github.com/tj/robo)：一个用 Go 编写的基于 YAML 的命令运行器。
- [mask](https://github.com/jakedeichert/mask)：一个用 Rust 编写的基于 Markdown 的命令运行器。
- [makesure](https://github.com/xonixx/makesure)：一个用 AWK 和 shell 编写的简单且可移植的命令运行器。
- [haku](https://github.com/VladimirMarkelov/haku)：一个用 Rust 编写的类 make 命令运行器。
- [mise](https://mise.jdx.dev/)：一个用 Rust 编写的开发环境工具管理器，支持 TOML 文件中的任务和独立脚本。
贡献
------------

`just` 欢迎你的贡献！`just` 采用最宽松的
[CC0](https://creativecommons.org/publicdomain/zero/1.0/legalcode.txt) 公共领域
贡献和备用许可证发布，因此你的更改也必须在此许可证下发布。

### 入门指南

`just` 使用 Rust 编写。使用
[rustup](https://www.rust-lang.org/tools/install) 安装 Rust 工具链。

`just` 经过广泛测试。所有新功能都必须有单元测试或集成测试覆盖。单元测试位于
[src](https://github.com/casey/just/blob/master/src) 目录下，与被测试的代码放在
一起，用于隔离测试代码。集成测试位于
[tests 目录](https://github.com/casey/just/blob/master/tests)，通过在给定的
`justfile` 和命令行参数上调用 `just` 并检查输出，从外部测试 `just` 二进制文件。

你应该编写最容易为你的功能编写的测试类型，同时仍然提供良好的测试覆盖率。

单元测试适用于测试内部使用的新 Rust 函数，并有助于开发。一个很好的例子是覆盖
[`unindent()` 函数](https://github.com/casey/just/blob/master/src/unindent.rs)
的单元测试，该函数用于取消三引号字符串和反引号的缩进。`unindent()` 有许多棘手的
边缘情况，通过直接调用 `unindent()` 的单元测试可以轻松地进行测试。

集成测试适用于确保 `just` 二进制文件的最终行为正确。`unindent()` 也被集成测试
覆盖，这些测试确保评估三引号字符串会产生正确的未缩进值。但是，并非所有可能的
情况都有集成测试。这些情况由更快、更简洁的单元测试覆盖，这些测试直接调用
`unindent()`。

集成测试使用 `Test` 结构体，这是一个构建器，允许轻松地使用给定的 `justfile`、
参数和环境变量调用 `just`，并检查程序的标准输出、标准错误和退出代码。

### 贡献工作流程

1. 确保该功能是需要的。应该有一个关于该功能的开放 issue，并且有来自
   [@casey](https://github.com/casey) 的评论说这是个好主意或看起来合理。如果
   没有，请开一个新 issue 并征求反馈。

   有很多好的功能无法合并，要么因为它们不向后兼容，要么其实现会使代码库过于
   复杂，要么违背了 `just` 的设计理念。

2. 确定功能的设计。如果功能有多种可能的实现方式或语法，请确保在 issue 中敲定
   细节。

3. 克隆 `just` 并开始开发。最佳工作流程是在编辑器中打开你正在处理的代码，同时
   运行一个在文件更改时重新运行测试的任务。你可以通过使用 `cargo install
   cargo-watch` 安装 [cargo-watch](https://github.com/watchexec/cargo-watch)
   并运行 `just watch test` 来运行这样的任务。

4. 为你的功能添加一个失败的测试。大多数时候这将是一个端到端测试该功能的集成
   测试。在 [tests](https://github.com/casey/just/blob/master/tests) 中查找
   合适的文件来放置测试，或者在
   [tests](https://github.com/casey/just/blob/master/tests) 中添加新文件，并在
   [tests/lib.rs](https://github.com/casey/just/blob/master/tests/lib.rs) 中
   添加导入该文件的 `mod` 语句。

5. 实现该功能。

6. 运行 `just ci` 以确保所有测试、lint 检查和其他检查都通过。需要安装
   [mdBook](https://github.com/rust-lang/mdBook) 和
   [mdbook-linkcheck](https://github.com/Michael-F-Bryan/mdbook-linkcheck)。

7. 打开一个维护者可编辑的 PR 并提交新代码。PR 通常需要变基和小幅调整。如果 PR
   不允许维护者编辑，每次变基和调整都需要一轮代码审查往返。如果你的 PR 不允许
   维护者编辑，可能会被直接关闭。

8. 整合反馈。

9. 享受你的 PR 被合并的美妙感觉！

欢迎随时打开草稿 PR 进行讨论和获取反馈。

### 提示

以下是一些帮助你开始特定类型新功能的提示，你可以将其与上述贡献工作流程结合使用。

#### 添加新属性

1. 在
   [tests/attributes.rs](https://github.com/casey/just/blob/master/tests/attributes.rs)
   中编写新的集成测试。

2. 在
   [`Attribute`](https://github.com/casey/just/blob/master/src/attribute.rs)
   枚举中添加新的变体。

3. 实现新属性的功能。

4. 运行 `just ci` 以确保所有测试都通过。

### Janus

[Janus](https://github.com/casey/janus) 是一个用于检查对 `just` 的更改是否会破坏
或改变现有 `justfile` 解释的工具。它收集并分析 GitHub 上的公开 `justfile`。

在合并特别大或复杂的更改之前，应该运行 Janus 以确保没有任何问题。不用担心自己
运行 Janus，Casey 会很乐意在需要时为你的更改运行它。

### 最低支持的 Rust 版本

最低支持的 Rust 版本（MSRV）是当前的稳定版 Rust。它可能在旧版本的 Rust 上构建，
但这不能保证。

### 新版本发布

`just` 频繁发布新版本，以便用户能够快速获得新功能。

发布提交消息使用以下模板：

```
Release x.y.z

- Bump version: x.y.z → x.y.z
- Update changelog
- Update changelog contributor credits
- Update dependencies
- Update version references in readme
```
常见问题
--------

### Just 避免了 Make 的哪些特性？

`make` 有一些令人困惑、复杂或使其不适合作为通用命令运行器的行为。

一个例子是在某些情况下，`make` 实际上不会运行配方中的命令。例如，如果你有一个名为 `test` 的文件和以下 makefile：

```just
test:
  ./test
```

`make` 将拒绝运行你的测试：

```console
$ make test
make: `test' is up to date.
```

`make` 假设 `test` 配方生成了一个名为 `test` 的文件。由于该文件已经存在且配方没有其他依赖项，`make` 认为它没有任何工作要做并退出。

公平地说，当将 `make` 用作构建系统时，这种行为是理想的，但当将其用作命令运行器时则不然。你可以使用 `make` 内置的 [`.PHONY` 目标名称](https://www.gnu.org/software/make/manual/html_node/Phony-Targets.html) 为特定目标禁用此行为，但语法冗长且难以记住。显式的伪目标（phony targets）列表与配方定义分开编写，还引入了意外定义新的非伪目标的风险。在 `just` 中，所有配方都被视为伪目标。

`make` 特性的其他例子包括赋值中 `=` 和 `:=` 之间的区别、如果你搞砸了 makefile 会产生的令人困惑的错误消息、在配方中使用环境变量需要 `$$`，以及不同版本的 `make` 之间的不兼容性。

### Just 与 Cargo 构建脚本之间的关系是什么？

[`cargo` 构建脚本](http://doc.crates.io/build-script.html) 有一个非常具体的用途，即控制 `cargo` 如何构建你的 Rust 项目。这可能包括向 `rustc` 调用添加标志、构建外部依赖项或运行某种代码生成步骤。

另一方面，`just` 用于你可能在开发过程中运行的所有其他杂项命令。例如在不同配置下运行测试、检查代码规范（linting）、将构建产物推送到服务器、删除临时文件等等。

此外，尽管 `just` 是用 Rust 编写的，但无论你的项目使用什么语言或构建系统，都可以使用它。

更多闲谈
-------

我个人发现为几乎每个项目（无论大小）编写一个 `justfile` 都非常有用。

在一个有多个贡献者的大型项目中，有一个包含所有项目所需命令的文件在手边是非常有用的。

通常会有用于测试、构建、代码检查（lint）、部署等工作的不同命令，将它们全部集中在一个地方非常有用，可以减少你告诉别人该运行哪些命令以及如何输入它们的时间。

而且，有了存放命令的便捷位置，你很可能会想到其他有用的东西，这些东西是项目集体智慧的一部分，但以前没有在任何地方记录下来，比如版本控制工作流中某些部分所需的晦涩命令，安装项目所有依赖项所需的命令，或者你可能需要传递给构建系统的各种随机标志。

一些配方的想法：

- 部署/发布项目

- 在发布（release）模式与调试（debug）模式下构建

- 在调试模式下运行或启用日志记录

- 复杂的 Git 工作流

- 更新依赖项

- 运行不同的测试集，例如快速测试与慢速测试，或者以详细输出模式运行它们

- 任何你确实应该记录在某处以便能够记住它们的复杂命令集

即使对于小型的个人项目，能够通过名称而不是通过 ^R 反向搜索 Shell 历史记录来记住命令也是很棒的。而且，当你进入一个使用某种随机语言编写且具有神秘构建系统的旧项目时，知道你所需要执行的所有命令都在 `justfile` 中，并且如果你输入 `just`，一些有用的（或者至少是有趣的！）事情可能会发生，这真是一个巨大的福音。

有关配方的想法，请查看 [该项目的 justfile](https://github.com/casey/just/blob/master/justfile)，或者一些 [在实际应用中](https://github.com/search?q=path%3A**%2Fjustfile&type=code) 的 `justfile`。

总之，我想这就是这篇极其啰嗦的 README 的全部内容了。

希望你喜欢使用 `just`，并在你所有的计算工作中取得巨大的成功和满足感！

😸

[🔼 回到顶部！](#just)

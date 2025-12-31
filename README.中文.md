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

`just` 是一个保存和运行专案特有命令的便捷方式。

本 README 也可以作为 [book](https://just.systems/man/en/) 阅读。book 反映了最新的 release 版本，而
[GitHub 上的 readme](https://github.com/casey/just/blob/master/README.md)
反映了最新的 master 分支。

(中文文档在 [这里](https://github.com/casey/just/blob/master/README.中文.md),
快看过来!)

命令，被称为配方（recipes），存储在一个名为 `justfile` 的文件中，其语法受 `make` 启发：

![screenshot](https://raw.githubusercontent.com/casey/just/master/screenshot.png)

然后你可以用 `just RECIPE` 运行它们：

```console
$ just test-all
cc *.c -o main
./test --all
Yay, all your tests passed!
```

`just` 有很多有用的功能，并且比 `make` 有许多改进：

- `just` 是一个命令运行器，而不是构建系统，因此它避免了许多
  [`make` 的复杂性和特异性](#what-are-the-idiosyncrasies-of-make-that-just-avoids)。
  不需要 `.PHONY` 配方！

- Linux、MacOS、Windows 和其他合理的 Unix 系统都支持，无需
  额外的依赖。（虽然如果你的系统没有 `sh`，
  你需要 [选择一个不同的 shell](#shell)。）

- 错误具体且信息丰富，语法错误会连同其
  源上下文一起报告。

- 配方可以接受 [命令行参数](#recipe-parameters)。

- 尽可能静态地解决错误。未知的配方和
  循环依赖会在任何运行之前报告。

- `just` [加载 `.env` 文件](#dotenv-settings)，这使得填充
  环境变量变得容易。

- 配方可以 [从命令行列出](#listing-available-recipes)。

- 命令行补全脚本
  [可用于大多数流行的 shell](#shell-completion-scripts)。

- 配方可以用
  [任意语言](#shebang-recipes) 编写，比如 Python or NodeJS。

- `just` 可以从任何子目录调用，而不仅仅是
  包含 `justfile` 的目录。

- 还有 [更多](https://just.systems/man/en/)！

如果你需要关于 `just` 的帮助，请随意开启一个 issue 或在
[Discord](https://discord.gg/ezYScXR) 上找我。功能请求和 bug 报告
总是受欢迎的！

## 安装

### 先决条件

`just` 应该可以在任何具有合理 `sh` 的系统上运行，包括 Linux、MacOS
和 BSD。

#### Windows

在 Windows 上，`just` 可以与 [Git for Windows](https://git-scm.com)、
[GitHub Desktop](https://desktop.github.com) 或
[Cygwin](http://www.cygwin.com) 提供的 `sh` 一起使用。安装后，`sh` 必须
在你想从中调用 `just` 的 shell 的 `PATH` 中可用。

如果你不想安装 `sh`，你可以使用 `shell` 设置来使用
你选择的 shell。

像是 PowerShell：

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

你也可以使用命令行参数设置 shell。例如，要使用
PowerShell，用 `--shell powershell.exe --shell-arg -c` 启动 `just`。

（PowerShell 默认安装在 Windows 7 SP1 和 Windows Server 2008 R2
S1 及更高版本上，而 `cmd.exe` 非常难用，因此推荐大多数 Windows 用户使用 PowerShell。）

### 安装包

#### 跨平台

<table>
  <thead>
    <tr>
      <th>包管理器</th>
      <th>包</th>
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
      <th>包</th>
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
      <th>包</th>
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
        <sup><b>你必须在系统上 <a href=https://docs.makedeb.org/prebuilt-mpr/getting-started/#setting-up-the-repository>设置 Prebuilt-MPR</a> 才能运行此命令。</b></sup><br>
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
      <th>包</th>
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
      <th>包</th>
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

### 预构建二进制文件

Linux、MacOS 和 Windows 的预构建二进制文件可以在
[发布页面](https://github.com/casey/just/releases) 找到。

你可以在 Linux、MacOS 或 Windows 上使用以下命令下载
最新版本，只需将 `DEST` 替换为你想要放置 `just` 的目录：

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

请注意，`install.sh` 可能会在 GitHub Actions 或其他许多机器共享 IP 地址的环境中
失败。`install.sh` 调用 GitHub API 以
确定要安装的 `just` 的最新版本，而这些 API 调用是
基于 IP 进行速率限制的。为了在这种情况下使 `install.sh` 更可靠，
请使用 `--tag` 传递特定的 tag 来安装。

另一种避免速率限制的方法是将 GitHub 身份验证令牌作为
名为 `GITHUB_TOKEN` 的环境变量传递给 `install.sh`，允许其
对请求进行身份验证。

[Releases](https://github.com/casey/just/releases) 包含一个 `SHA256SUM` 文件，
可用于验证预构建二进制档案的完整性。

要验证发布版本，请下载预构建的二进制档案以及
`SHA256SUM` 文件并运行：

```sh
shasum --algorithm 256 --ignore-missing --check SHA256SUMS
```

### GitHub Actions

`just` 可以通过几种方式安装在 GitHub Actions 上。

使用在 MacOS 上预装在 GitHub Actions 运行器上的包管理器
`brew install just`，以及在 Windows 上使用 `choco install just`。

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

### 发布 RSS Feed

`just` 发布的 [RSS feed](https://en.wikipedia.org/wiki/RSS) 可在 [这里](https://github.com/casey/just/releases.atom) 获得。

### Node.js 安装

[just-install](https://npmjs.com/package/just-install) 可用于在 Node.js 应用程序中自动
安装 `just`。

`just` 是 npm 脚本的一个很好的、更强大的替代品。如果你想将
`just` 包含在 Node.js 应用程序的依赖项中，`just-install`
将在 `npm install` 命令的一部分中安装本地的、特定于平台的二进制文件。
这消除了每个开发人员使用上述步骤之一
独立安装 `just` 的需要。安装后，
`just` 命令将在 npm 脚本或使用 npx 时工作。这对于那些
希望让项目的设置过程尽可能简单的团队来说非常棒。

有关更多信息，请参阅
[just-install README 文件](https://github.com/brombal/just-install#readme)。

## 向后兼容性

随着 1.0 版本的发布，`just` 致力于
向后兼容性和稳定性。

未来的版本将不会引入使
现有 `justfile` 停止工作，或破坏命令行界面
工作调用的向后不兼容更改。

然而，这并不排除修复彻头彻尾的 bug，即使这样做可能会
破坏依赖其行为的 `justfiles`。

永远不会有 `just` 2.0。任何理想的向后不兼容更改
都将在每个 `justfile` 的基础上选择加入，因此用户可以
从容迁移。

尚未准备好稳定的功能被标记为不稳定，并且可能
随时更改或删除。使用不稳定功能默认会产生错误，
可以通过传递 `--unstable` 标志、
`set unstable` 或设置环境变量 `JUST_UNSTABLE` 为
除 `false`、`0` 或空字符串以外的任何值来抑制。

## 编辑器支持

`justfile` 语法与 `make` 足够接近，你可以告诉你的
编辑器为 `just` 使用 `make` 语法高亮。

### Vim 和 Neovim

Vim 9.1.1042 或更高版本以及 Neovim 0.11 或更高版本开箱即用支持
Justfile 语法高亮，感谢
[pbnj](https://github.com/pbnj)。

#### `vim-just`

[vim-just](https://github.com/NoahTheDuke/vim-just) 插件提供
针对 `justfile` 的语法高亮。

使用你最喜欢的包管理器安装它，例如
[Plug](https://github.com/junegunn/vim-plug)：

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

[tree-sitter-just](https://github.com/IndianBoy42/tree-sitter-just) 是一个
用于 Neovim 的 [Nvim Treesitter](https://github.com/nvim-treesitter/nvim-treesitter) 插件。

#### Makefile 语法高亮

Vim 的内置 makefile 语法高亮对于 `justfile` 来说并不完美，但
总比没有好。你可以将以下内容放入 `~/.vim/filetype.vim`：

```vimscript
if exists("did_load_filetypes")
  finish
endif

augroup filetypedetect
  au BufNewFile,BufRead justfile setf make
augroup END
```

或者将以下内容添加到单个 `justfile` 中，以便在
每个文件的基础上启用 `make` 模式：

```text
# vim: set ft=make :
```

### Emacs

[just-mode](https://github.com/leon-barrett/just-mode.el) 提供
`justfile` 的语法高亮和自动缩进。它在
[MELPA](https://melpa.org/) 上作为 [just-mode](https://melpa.org/#/just-mode) 提供。

[justl](https://github.com/psibi/justl.el) 提供用于执行和
列出配方的命令。

你可以将以下内容添加到单个 `justfile` 中，以便在
每个文件的基础上启用 `make` 模式：

```text
# Local Variables:
# mode: makefile
# End:
```

### Visual Studio Code

VS Code 的扩展 [在此处可用](https://github.com/nefrob/vscode-just)。

未维护的 VS Code 扩展包括
[skellock/vscode-just](https://github.com/skellock/vscode-just) 和
[sclu1034/vscode-just](https://github.com/sclu1034/vscode-just)。

### JetBrains IDEs

[linux_china](https://github.com/linux-china) 开发的 JetBrains IDEs 插件
[在此处可用](https://plugins.jetbrains.com/plugin/18658-just)。

### Kakoune

Kakoune 开箱即用支持 `justfile` 语法高亮，感谢
TeddyDD。

### Helix

[Helix](https://helix-editor.com/) 自 23.05 版本起
开箱即用支持 `justfile` 语法高亮。

### Sublime Text

由 [nk9](https://github.com/nk9) 开发的 [Just package](https://github.com/nk9/just_sublime)
具有 `just` 语法和其他一些工具，
可在 [PackageControl](https://packagecontrol.io/packages/Just) 上找到。

### Micro

[Micro](https://micro-editor.github.io/) 开箱即用支持 Justfile 语法高亮，
感谢 [tomodachi94](https://github.com/tomodachi94)。

### Zed

由 [jackTabsCode](https://github.com/jackTabsCode) 开发的 [zed-just](https://github.com/jackTabsCode/zed-just/) 扩展
可在 [Zed 扩展页面](https://zed.dev/extensions?query=just) 上找到。

### 其他编辑器

为了让我可以将它们包含在这里，如果是你选择的编辑器，
请随时向我发送让语法高亮工作所需的命令。

### 语言服务器协议 (LSP)

[just-lsp](https://github.com/terror/just-lsp) 提供了一个 [语言服务器
协议](https://en.wikipedia.org/wiki/Language_Server_Protocol)
实现，启用诸如跳转到定义、内联诊断
和代码补全等功能。

### 模型上下文协议 (MCP)

[just-mcp](http://github.com/promptexecution/just-mcp) 提供了一个
[模型上下文协议](https://en.wikipedia.org/wiki/Model_Context_Protocol)
适配器，允许 LLM 查询 `justfiles` 的内容并运行配方。

## 快速开始

请参阅安装部分了解如何在你的计算机上安装 `just`。尝试
运行 `just --version` 以确保它已正确安装。

有关语法的概述，请查看
[此备忘单](https://cheatography.com/linux-china/cheat-sheets/justfile/)。

一旦安装好 `just` 并开始工作，请在
项目的根目录中创建一个名为 `justfile` 的文件，内容如下：

```just
recipe-name:
  echo 'This is a recipe!'

# this is a comment
another-recipe:
  @echo 'This is another recipe.'
```

当你调用 `just` 时，它会在当前目录
及向上查找文件 `justfile`，因此你可以从项目的任何子目录调用它。

搜索 `justfile` 不区分大小写，因此任何形式，如 `Justfile`、
`JUSTFILE` 或 `JuStFiLe` 都可以。`just` 还会查找名为 `.justfile` 的文件，
以防你想隐藏 `justfile`。

运行不带参数的 `just` 会运行 `justfile` 中的第一个配方：

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

`just` 在运行每个命令之前会将其打印到标准错误，这就是为什么
打印了 `echo 'This is a recipe!'`。对于以 `@` 开头的行，这将通过抑制，
这就是为什么没有打印 `echo 'This is another recipe.'`。

如果命令失败，配方将停止运行。这里 `cargo publish` 仅在
`cargo test` 成功时运行：

```just
publish:
  cargo test
  # tests passed, time to publish!
  cargo publish
```

配方可以依赖于其他配方。这里 `test` 配方依赖于
`build` 配方，因此 `build` 将在 `test` 之前运行：

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

没有依赖关系的配方将按照它们在命令行上给出的顺序运行：

```console
$ just build sloc
cc main.c foo.c bar.c -o main
1337 lines of code
```

依赖项将始终首先运行，即使它们是在依赖它们的配方之后传递的：

```console
$ just test build
cc main.c foo.c bar.c -o main
./test
testing… all tests passed!
```

配方可能依赖于子模块中的配方：

```justfile
mod foo

baz: foo::bar
```

## 示例

可以在 [示例目录](https://github.com/casey/just/tree/master/examples) 和
[GitHub](https://github.com/search?q=path%3A**%2Fjustfile&type=code) 上找到各种 `justfile`。

## 特性

### 默认配方

当调用 `just` 而没有配方时，它运行带有
`[default]` 属性的配方，或者如果没有任何配方具有 `[default]` 属性，
则运行 `justfile` 中的第一个配方。

此配方可能是项目中运行最频繁的命令，例如
运行测试：

```just
test:
  cargo test
```

你还可以使用依赖项默认运行多个配方：

```just
default: lint build test

build:
  echo Building…

test:
  echo Testing…

lint:
  echo Linting…
```

如果没有配方作为默认配方有意义，你可以将一个配方添加到
你的 `justfile` 的开头，列出可用的配方：

```just
default:
  just --list
```

### 列出可用的配方

配方可以使用 `just --list` 按字母顺序列出：

```console
$ just --list
Available recipes:
    build
    test
    deploy
    lint
```

[子模块](#modules1190) 中的配方可以使用 `just --list PATH` 列出，
其中 `PATH` 是空格或 `::` 分隔的模块路径：

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

`just --summary` 更简洁：

```console
$ just --summary
build test deploy lint
```

传递 `--unsorted` 以按照它们在 `justfile` 中出现的顺序打印配方：

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

如果你希望 `just` 默认列出 `justfile` 中的配方，你
可以使用此作为你的默认配方：

```just
default:
  @just --list
```

请注意，你可能需要在上面的行中添加 `--justfile {{justfile()}}`。
没有它，如果你执行 `just -f /some/distant/justfile -d .` 或
`just -f ./non-standard-justfile`，配方中的普通 `just --list`
不一定会使用你提供的文件。它会尝试在你的当前路径中找到一个
justfile，甚至可能导致 `No justfile found` 错误。

标题文本可以使用 `--list-heading` 自定义：

```console
$ just --list --list-heading $'Cool stuff…\n'
Cool stuff…
    test
    build
```

并且缩进可以使用 `--list-prefix` 自定义：

```console
$ just --list --list-prefix ····
Available recipes:
····test
····build
```

`--list-heading` 的参数替换标题及其后面的换行符，
因此如果非空，它应该包含一个换行符。这样工作是为了
你可以通过传递空字符串来完全抑制标题行：

```console
$ just --list --list-heading ''
    test
    build
```

### 调用多个配方

可以在命令行上一次调用多个配方：

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

请记住，带有参数的配方将吞噬参数，即使它们
与其他配方的名称匹配：

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

`--one` 标志可用于限制命令行调用为单个
配方：

```console
$ just --one build serve
error: Expected 1 command-line recipe invocation but found 2.
```

### 工作目录

默认情况下，配方运行时的工作目录设置为
包含 `justfile` 的目录。

`[no-cd]` 属性可用于使配方运行时的工作
目录设置为调用 `just` 的目录。

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

你可以使用 `set working-directory := '…'`
覆盖所有配方的工作目录：

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

你可以使用 `working-directory` 属性<sup>1.38.0</sup>
覆盖特定配方的工作目录：

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

`working-directory` 设置或 `working-directory` 属性的参数
可以是绝对的或相对的。如果是相对的，它将解释为
相对于默认工作目录。

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

设置控制解释和执行。每个设置最多可以指定
一次，可以出现在 `justfile` 的任何位置。

例如：

```just
set shell := ["zsh", "-cu"]

foo:
  # this line will be run as `zsh -cu 'ls **/*.txt'`
  ls **/*.txt
```

#### 设置表

| 名称                                  | 值                 | 默认值          | 描述                                                                            |
| ------------------------------------- | ------------------ | --------------- | ------------------------------------------------------------------------------- |
| `allow-duplicate-recipes`             | boolean            | `false`         | 允许 `justfile` 中稍后出现的配方覆盖同名的早期配方。                            |
| `allow-duplicate-variables`           | boolean            | `false`         | 允许 `justfile` 中稍后出现的变量覆盖同名的早期变量。                            |
| `dotenv-filename`                     | string             | -               | 如果存在，加载自定义名称的 `.env` 文件。                                        |
| `dotenv-load`                         | boolean            | `false`         | 如果存在，加载 `.env` 文件。                                                    |
| `dotenv-override`                     | boolean            | `false`         | 使用 `.env` 文件中的值覆盖现有的环境变量。                                      |
| `dotenv-path`                         | string             | -               | 从自定义路径加载 `.env` 文件，如果不存在则报错。覆盖 `dotenv-filename`。        |
| `dotenv-required`                     | boolean            | `false`         | 如果找不到 `.env` 文件则报错。                                                  |
| `export`                              | boolean            | `false`         | 将所有变量导出为环境变量。                                                      |
| `fallback`                            | boolean            | `false`         | 如果找不到命令行上的第一个配方，则在父目录中搜索 `justfile`。                   |
| `ignore-comments`                     | boolean            | `false`         | 忽略以 `#` 开头的配方行。                                                       |
| `positional-arguments`                | boolean            | `false`         | 传递位置参数。                                                                  |
| `quiet`                               | boolean            | `false`         | 在执行前禁用回显配方行。                                                        |
| `script-interpreter`<sup>1.33.0</sup> | `[COMMAND, ARGS…]` | `['sh', '-eu']` | 设置用于调用带有空 `[script]` 属性的配方的命令。                                |
| `shell`                               | `[COMMAND, ARGS…]` | -               | 设置用于调用配方和评估反引号的命令。                                            |
| `tempdir`                             | string             | -               | 在 `tempdir` 中而不是系统默认的临时目录中创建临时目录。                         |
| `unstable`<sup>1.31.0</sup>           | boolean            | `false`         | 启用不稳定功能。                                                                |
| `windows-powershell`                  | boolean            | `false`         | 在 Windows 上使用 PowerShell 作为默认 shell。（已弃用。请改用 `windows-shell`。 |
| `windows-shell`                       | `[COMMAND, ARGS…]` | -               | 设置用于调用配方和评估反引号的命令。                                            |
| `working-directory`<sup>1.33.0</sup>  | string             | -               | 设置配方和反引号的工作目录，相对于默认工作目录。                                |

Boolean 设置可以写成：

```justfile
set NAME
```

这等价于：

```justfile
set NAME := true
```

非 boolean 设置可以设置为字符串和
表达式。<sup>master</sup>

但是，由于设置会影响反引号和许多函数的行为，
因此这些表达式不能包含反引号或函数调用，无论直接
或通过引用传递。

#### 允许重复配方

如果 `allow-duplicate-recipes` 设置为 `true`，定义多个同名
配方不是错误，将使用最后的定义。默认为
`false`。

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

如果 `allow-duplicate-variables` 设置为 `true`，定义多个同名
变量不是错误，将使用最后的定义。默认为
`false`。

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

如果设置了 `dotenv-load`、`dotenv-filename`、`dotenv-override`、`dotenv-path`
或 `dotenv-required` 中的任何一个，`just` 将尝试从文件中加载
环境变量。

如果设置了 `dotenv-path`，`just` 将在给定的路径查找文件，这
可以是绝对路径，或相对于工作目录的路径。

命令行选项 `--dotenv-path`，缩写形式 `-E`，可用于在运行时设置或
覆盖 `dotenv-path`。

如果设置了 `dotenv-filename`，`just` 将在给定的路径查找文件，
相对于工作目录及其每个父目录。

如果没有设置 `dotenv-filename`，但设置了 `dotenv-load` 或 `dotenv-required`，
just 将查找名为 `.env` 的文件，相对于工作目录
及其每个父目录。

`dotenv-filename` 和 `dotenv-path` 类似，但 `dotenv-path` 仅
相对于工作目录进行检查，而 `dotenv-filename` 则是相对于
工作目录及其每个父目录进行检查。

如果没有找到环境文件并不是错误，除非
设置了 `dotenv-required`。

加载的变量是环境变量，而不是 `just` 变量，因此
必须在配方和反引号中使用 `$VARIABLE_NAME` 访问。

如果设置了 `dotenv-override`，来自环境文件的变量将覆盖
现有的环境变量。

例如，如果你的 `.env` 文件包含：

```console
# a comment, will be ignored
DATABASE_ADDRESS=localhost:6379
SERVER_PORT=1337
```

并且你的 `justfile` 包含：

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

`export` 设置导致所有 `just` 变量导出为环境
变量。默认为 `false`。

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

如果 `positional-arguments` 为 `true`，配方参数将作为
位置参数传递给命令。对于逐行配方，参数 `$0` 将是
配方的名称。

例如，运行此配方：

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

当使用兼容 `sh` 的 shell 时，例如 `bash` 或 `zsh`，`$@` 扩展为
给予配方的位置参数，从一开始。当在
双引号内作为 `"$@"` 使用时，包括空格的参数将按
原样传递，就好像它们被双引号括起来一样。也就是说，`"$@"` 等价于 `"$1" "$2"`…
如果没有位置参数，`"$@"` 和 `$@` 扩展为空
（即，它们被移除）。

这个示例配方将在单独的行上逐个打印参数：

```just
set positional-arguments

@test *args='':
  bash -c 'while (( "$#" )); do echo - $1; shift; done' -- "$@"
```

用 _两个_ 参数运行它：

```console
$ just test foo "bar baz"
- foo
- bar baz
```

位置参数也可以在每个配方的基础上使用
`[positional-arguments]` 属性打开<sup>1.29.0</sup>：

```just
[positional-arguments]
@foo bar:
  echo $0
  echo $1
```

请注意，PowerShell 处理位置参数的方式与
其他 shell 不同，因此打开位置参数可能会破坏使用
PowerShell 的配方。

如果使用 PowerShell 7.4 或更高版本，`-CommandWithArgs` 标志将使
位置参数按预期工作：

```just
set shell := ['pwsh.exe', '-CommandWithArgs']
set positional-arguments

print-args a b c:
  Write-Output @($args[1..($args.Count - 1)])
```

#### Shell

`shell` 设置控制用于调用配方行和
反引号的命令。Shebang 配方不受影响。默认 shell 是 `sh -cu`。

```just
# use python3 to execute recipe lines and backticks
set shell := ["python3", "-c"]

# use print to capture result of evaluation
foos := `print("foo" * 4)`

foo:
  print("Snake snake snake snake.")
  print("{{foos}}")
```

`just` 将要执行的命令作为参数传递。许多 shell 需要
一个额外的标志，通常是 `-c`，以使它们评估第一个参数。

##### Windows Shell

`just` 默认在 Windows 上使用 `sh`。要在 Windows 上使用不同的 shell，
请使用 `windows-shell`：

```just
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

hello:
  Write-Host "Hello, world!"
```

请参阅
[powershell.just](https://github.com/casey/just/blob/master/examples/powershell.just)
了解在所有平台上使用 PowerShell 的 justfile。

##### Windows PowerShell

_`set windows-powershell` 使用传统的 `powershell.exe` 二进制文件，且不再
推荐。请参阅上面的 `windows-shell` 设置以了解更灵活的
控制 Windows 上使用的 shell 的方法。_

`just` 默认在 Windows 上使用 `sh`。要改用 `powershell.exe`，请将
`windows-powershell` 设置为 true。

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

_[Nushell](https://github.com/nushell/nushell) 使用 Rust 编写，且 **具有
针对 Windows / macOS 和 Linux 的跨平台支持**。_

### 文档注释

配方之前的注释将出现在 `just --list` 中：

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

`[doc]` 属性可用于设置或抑制配方的文档注释：

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

表达式支持各种运算符和函数调用，可以
用于赋值、默认配方参数和配方体 `{{…}}`
替换中。

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

#### 拼接

`+` 运算符返回左侧参数与
右侧参数拼接的结果：

```just
foobar := 'foo' + 'bar'
```

#### 逻辑运算符

逻辑运算符 `&&` and `||` 可用于合并字符串
值<sup>1.37.0</sup>，类似于 Python 的 `and` 和 `or`。这些运算符
将空字符串 `''` 视为 false，所有其他字符串视为 true。

这些运算符目前是不稳定的。

`&&` 运算符如果左侧参数是
空字符串，则返回空字符串，否则返回右侧参数：

```justfile
foo := '' && 'goodbye'      # ''
bar := 'hello' && 'goodbye' # 'goodbye'
```

`||` 运算符如果左侧参数是非空的，则返回它，否则
返回右侧参数：

```justfile
foo := '' || 'goodbye'      # 'goodbye'
bar := 'hello' || 'goodbye' # 'hello'
```

#### 连接路径

`/` 运算符可用于用斜杠连接两个字符串：

```just
foo := "a" / "b"
```

```
$ just --evaluate foo
a/b
```

请注意，即使已经存在 `/`，也会添加 `/`：

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

`/` 运算符即使在 Windows 上也使用 `/` 字符。因此，应避免
在通过使用通用命名约定 (UNC) 的路径（即以 `\\?` 开头的路径）上使用 `/` 运算符，
因为 UNC 路径不支持正斜杠。

#### 转义 `{{`

要编写包含 `{{` 的配方，请使用 `{{{{`：

```just
braces:
  echo 'I {{{{LOVE}} curly braces!'
```

（不匹配的 `}}` 被忽略，因此不需要转义。）

另一种选择是将所有你要转义的文本放在
插值内：

```just
braces:
  echo '{{'I {{LOVE}} curly braces!'}}'
```

还有一个选择是使用 `{{ "{{" }}`：

```just
braces:
  echo 'I {{ "{{" }}LOVE}} curly braces!'
```

### 字符串

支持 `'single'`、`"double"` 和 `'''triple'''` 引用的字符串字面量。
与配方体不同，字符串内不支持 `{{…}}` 插值。

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

unicode 字符转义序列 `\u{…}`<sup>1.36.0</sup> 接受最多
六个十六进制数字。

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

支持缩进的单引号和双引号字符串（由三个单引号或双引号分隔）。
缩进的字符串行会剥离前导换行符，以及所有
非空行共有的前导空格：

```just
# this string will evaluate to `foo\nbar\n`
x := '''
  foo
  bar
'''

# this string will evaluate to `abc\n  wuv\nxyz\n`
y := """
  abc
    wuv
  xyz
"""
```

与非缩进字符串类似，缩进的双引号字符串处理转义
序列，缩进的单引号字符串忽略转义序列。转义
序列处理发生在取消缩进之后。取消缩进
算法不考虑转义序列产生的空格或换行符。

#### Shell 扩展字符串

以 `x` 为前缀的字符串是 shell 扩展的<sup>1.27.0</sup>：

```justfile
foobar := x'~/$FOO/${BAR}'
```

| 值                | 替换                                                 |
| ----------------- | ---------------------------------------------------- |
| `$VAR`            | 环境变量 `VAR` 的值                                  |
| `${VAR}`          | 环境变量 `VAR` 的值                                  |
| `${VAR:-DEFAULT}` | 环境变量 `VAR` 的值，如果 `VAR` 未设置则为 `DEFAULT` |
| Leading `~`       | 当前用户主目录的路径                                 |
| Leading `~USER`   | `USER` 的主目录路径                                  |

此扩展在编译时执行，因此不能使用 `.env` 文件中的变量和
导出的 `just` 变量。但是，这允许 shell 扩展
字符串用于设置和导入路径等不能
依赖 `just` 变量和 `.env` 文件的地方。

#### 格式化字符串

以 `f` 为前缀的字符串是格式化字符串<sup>1.44.0</sup>：

```justfile
name := "world"
message := f'Hello, {{name}}!'
```

格式字符串可以包含用 `{{…}}` 分隔的插值，其中包含
表达式。格式字符串计算为拼接的字符串片段和
计算后的表达式。

使用 `{{{{` 在格式字符串中包含字面量 `{{`：

```justfile
foo := f'I {{{{LOVE} curly braces!'
```

### 忽略错误

通常，如果命令返回非零退出状态，执行将停止。要
导致命令失败时继续执行，请在命令前加上
`-`：

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

`just` 提供了通过表达式使用的许多内置函数，包括
在配方体 `{{…}}` 替换、赋值和默认参数值中使用。

所有以 `_directory` 结尾的函数都可以缩写为 `_dir`。所以
`home_directory()` 也可以写作 `home_dir()`。此外，
`invocation_directory_native()` 可以缩写为
`invocation_dir_native()`。

#### 系统信息

- `arch()` — 指令集架构。可能的值为：`"aarch64"`、
  `"arm"`、`"asmjs"`、`"hexagon"`、`"mips"`、`"msp430"`、`"powerpc"`、
  `"powerpc64"`、`"s390x"`、`"sparc"`、`"wasm32"`、`"x86"`、`"x86_64"` 和
  `"xcore"`。
- `num_cpus()`<sup>1.15.0</sup> - 逻辑 CPU 数量。
- `os()` — 操作系统。可能的值为：`"android"`、`"bitrig"`、
  `"dragonfly"`、`"emscripten"`、`"freebsd"`、`"haiku"`、`"ios"`、`"linux"`、
  `"macos"`、`"netbsd"`、`"openbsd"`、`"solaris"` 和 `"windows"`。
- `os_family()` — 操作系统家族；可能的值为：`"unix"` 和
  `"windows"`。

例如：

```just
system-info:
  @echo "This is an {{arch()}} machine".
```

```console
$ just system-info
This is an x86_64 machine
```

`os_family()` 函数可用于创建跨平台 `justfile`，
可在各种操作系统上运行。有关示例，请参阅
[cross-platform.just](https://github.com/casey/just/blob/master/examples/cross-platform.just)
文件。

#### 外部命令

- `shell(command, args...)`<sup>1.27.0</sup> 返回 shell 脚本
  `command` 的标准输出，带有零个或多个位置参数 `args`。用于
  解释 `command` 的 shell 与用于评估配方行的 shell 相同，
  并且可以用 `set shell := […]` 更改。

  `command` 作为第一个参数传递，因此如果命令是 `'echo $@'`，
  带有默认 shell 命令 `sh -cu` 和 `args`
  `'foo'` 和 `'bar'` 的完整命令行将是：

  ```
  'sh' '-cu' 'echo $@' 'echo $@' 'foo' 'bar'
  ```

  这样 `$@` 如预期工作，`$1` 引用第一个
  参数。`$@` 不包括第一个位置参数，它
  预期是正在运行的程序的名称。

```just
# arguments can be variables or expressions
file := '/sys/class/power_supply/BAT0/status'
bat0stat := shell('cat $1', file)

# commands can be variables or expressions
command := 'wc -l'
output := shell(command + ' "$1"', 'main.c')

# arguments referenced by the shell command must be used
empty := shell('echo', 'foo')
full := shell('echo $1', 'foo')
error := shell('echo $1')
```

```just
# Using python as the shell. Since `python -c` sets `sys.argv[0]` to `'-c'`,
# the first "real" positional argument will be `sys.argv[2]`.
set shell := ["python3", "-c"]
olleh := shell('import sys; print(sys.argv[2][::-1])', 'hello')
```

#### 环境变量

- `env(key)`<sup>1.15.0</sup> — 检索名为 `key` 的环境变量，
  如果不存在则中止。

```just
home_dir := env('HOME')

test:
  echo "{{home_dir}}"
```

```console
$ just
/home/user1
```

- `env(key, default)`<sup>1.15.0</sup> — 检索名为 `key` 的环境
  变量，如果不存在则返回 `default`。
- `env_var(key)` — `env(key)` 的已弃用别名。
- `env_var_or_default(key, default)` — `env(key, default)` 的已弃用别名。

可以使用 `||` 运算符（当前不稳定）为空环境变量值替换默认值：

```just
set unstable

foo := env('FOO', '') || 'DEFAULT_VALUE'
```

#### 可执行文件

- `require(name)`<sup>1.39.0</sup> — 在 `PATH` 环境变量的目录中搜索
  可执行文件 `name` 并返回其完整路径，或者
  如果不存在名为 `name` 的可执行文件，则停止并报错。

  ```just
  bash := require("bash")

  @test:
      echo "bash: '{{bash}}'"
  ```

  ```console
  $ just
  bash: '/bin/bash'
  ```

- `which(name)`<sup>1.39.0</sup> — 在 `PATH` 环境变量的目录中搜索
  可执行文件 `name` 并返回其完整路径，或者
  如果不存在名为 `name` 的可执行文件，则返回空字符串。当前不稳定。

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

- `is_dependency()` - 如果当前配方作为另一个配方的
  依赖项运行，而不是直接运行，则返回字符串 `true`，
  否则返回字符串 `false`。

#### 调用目录

- `invocation_directory()` - 检索调用 `just` 时的当前
  目录的绝对路径，在 `just` 更改它（chdir'd）之前，即
  执行命令之前。在 Windows 上，`invocation_directory()` 使用 `cygpath`
  将调用目录转换为兼容 Cygwin 的 `/` 分隔路径。
  使用 `invocation_directory_native()` 在所有平台上返回逐字的调用
  目录。

例如，要在用户/调用者视角下的“当前目录”下的文件
上调用 `rustfmt`，请使用以下规则：

```just
rustfmt:
  find {{invocation_directory()}} -name \*.rs -exec rustfmt {} \;
```

或者，如果你的命令需要从当前目录运行，你
可以使用（例如）：

```just
build:
  cd {{invocation_directory()}}; ./some_script_that_needs_to_be_run_from_here
```

- `invocation_directory_native()` - 检索调用 `just` 时的当前
  目录的绝对路径，在 `just` 更改它（chdir'd）之前，即
  执行命令之前。

#### Justfile 和 Justfile 目录

- `justfile()` - 检索当前 `justfile` 的路径。

- `justfile_directory()` - 检索当前 `justfile` 的
  父目录的路径。

例如，要运行相对于当前 `justfile` 位置的命令：

```just
script:
  {{justfile_directory()}}/scripts/some_script
```

#### 源文件和源代码目录

- `source_file()`<sup>1.27.0</sup> - 检索当前源文件的路径。

- `source_directory()`<sup>1.27.0</sup> - 检索当前源文件的
  父目录的路径。

在根 `justfile` 中，`source_file()` 和 `source_directory()` 的行为与 `justfile()` 和
`justfile_directory()` 相同，但当从导入或子模块内调用时，
将分别返回当前 `import` 或 `mod` 源文件的路径和目录。

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

- `append(suffix, s)`<sup>1.27.0</sup> 将 `suffix` 附加到 `s` 中的
  空格分隔字符串。 `append('/src', 'foo bar baz')` → `'foo/src bar/src baz/src'`
- `prepend(prefix, s)`<sup>1.27.0</sup> 将 `prefix` 放在 `s` 中的
  空格分隔字符串之前。 `prepend('src/', 'foo bar baz')` →
  `'src/foo src/bar src/baz'`
- `encode_uri_component(s)`<sup>1.27.0</sup> - 对 `s` 中的字符进行百分比编码，
  除了 `[A-Za-z0-9_.!~*'()-]`，匹配
  [JavaScript `encodeURIComponent` 函数](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/encodeURIComponent) 的行为。
- `quote(s)` - 将所有单引号替换为 `'\''`，并在 `s` 的前后加上
  单引号。这足以转义特殊字符，以供
  许多 shell 使用，包括大多数 Bourne shell 后代。
- `replace(s, from, to)` - 将 `s` 中所有出现的 `from` 替换为 `to`。
- `replace_regex(s, regex, replacement)` - 将 `s` 中所有出现的 `regex`
  替换为 `replacement`。正则表达式由
  [Rust `regex` crate](https://docs.rs/regex/latest/regex/) 提供。有关用法
  示例，请参阅 [语法文档](https://docs.rs/regex/latest/regex/#syntax)。
  支持捕获组。`replacement` 字符串使用
  [替换字符串语法](https://docs.rs/regex/latest/regex/struct.Regex.html#replacement-string-syntax)。
- `trim(s)` - 从 `s` 中移除前导和尾随空格。
- `trim_end(s)` - 从 `s` 中移除尾随空格。
- `trim_end_match(s, substring)` - 移除 `s` 中匹配 `substring` 的后缀。
- `trim_end_matches(s, substring)` - 重复移除 `s` 中匹配 `substring` 的后缀。
- `trim_start(s)` - 从 `s` 中移除前导空格。
- `trim_start_match(s, substring)` - 移除 `s` 中匹配 `substring` 的前缀。
- `trim_start_matches(s, substring)` - 重复移除 `s` 中
  匹配 `substring` 的前缀。

#### 大小写转换

- `capitalize(s)`<sup>1.7.0</sup> - 将 `s` 的第一个字符转换为大写，
  其余字符转换为小写。
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

##### 可能失败的操作

- `absolute_path(path)` - 工作目录中相对 `path` 的绝对路径。
  目录 `/foo` 中的 `absolute_path("./bar.txt")` 是 `/foo/bar.txt`。
- `canonicalize(path)`<sup>1.24.0</sup> - 通过解析符号链接并尽可能
  移除 `.`、`..` 和多余的 `/` 来规范化 `path`。
- `extension(path)` - `path` 的扩展名。`extension("/foo/bar.txt")` 是 `txt`。
- `file_name(path)` - 移除任何前导目录组件的 `path` 的文件名。
  `file_name("/foo/bar.txt")` 是 `bar.txt`。
- `file_stem(path)` - 无扩展名的 `path` 文件名。
  `file_stem("/foo/bar.txt")` 是 `bar`。
- `parent_directory(path)` - `path` 的父目录。
  `parent_directory("/foo/bar.txt")` 是 `/foo`。
- `without_extension(path)` - 无扩展名的 `path`。
  `without_extension("/foo/bar.txt")` 是 `/foo/bar`。

这些函数可能会失败，例如如果路径没有扩展名，
这将停止执行。

##### 不会失败的操作

- `clean(path)` - 通过移除多余的路径分隔符、
  中间的 `.` 组件以及可能的 `..` 来简化 `path`。`clean("foo//bar")` 是
  `foo/bar`，`clean("foo/..")` 是 `.`，`clean("foo/./bar")` 是 `foo/bar`。
- `join(a, b…)` - _此函数在 Unix 上使用 `/`，在 Windows 上使用 `\`，这可能
  会导致不想要的行为。`/` 运算符，例如 `a / b`，始终
  使用 `/`，应被视为替代方案，除非在 Windows 上明确需要 `\`。_
  将路径 `a` 与路径 `b` 连接。`join("foo/bar", "baz")` 是 `foo/bar/baz`。接受两个或更多参数。

#### 文件系统访问

- `path_exists(path)` - 如果路径指向存在的实体，则返回 `true`，
  否则返回 `false`。遍历符号链接，如果
  路径不可访问或指向损坏的符号链接，则返回 `false`。
- `read(path)`<sup>1.39.0</sup> - 以字符串形式返回 `path` 处的文件内容。

##### 错误报告

- `error(message)` - 中止执行并向用户报告错误 `message`。

#### UUID 和哈希生成

- `blake3(string)`<sup>1.25.0</sup> - 以十六进制字符串返回 `string` 的 [BLAKE3] 哈希。
- `blake3_file(path)`<sup>1.25.0</sup> - 以十六进制字符串返回 `path` 处文件的 [BLAKE3]
  哈希。
- `sha256(string)` - 以十六进制字符串返回 `string` 的 SHA-256 哈希。
- `sha256_file(path)` - 以十六进制字符串返回 `path` 处文件的 SHA-256
  哈希。
- `uuid()` - 生成随机版本 4 UUID。

[BLAKE3]: https://github.com/BLAKE3-team/BLAKE3/

#### 随机数

- `choose(n, alphabet)`<sup>1.27.0</sup> - 从 `alphabet` 中生成一个由 `n` 个随机
  选择的字符组成的字符串，该字符串不能包含重复的
  字符。例如，`choose('64', HEX)` 将生成一个随机的
  64 字符的小写十六进制字符串。

#### 日期时间

- `datetime(format)`<sup>1.30.0</sup> - 返回带有 `format` 的本地时间。
- `datetime_utc(format)`<sup>1.30.0</sup> - 返回带有 `format` 的 UTC 时间。

`datetime` 和 `datetime_utc` 的参数是 `strftime` 风格的格式
字符串，详见
[`chrono` 库文档](https://docs.rs/chrono/latest/chrono/format/strftime/index.html)。

#### 语义化版本

- `semver_matches(version, requirement)`<sup>1.16.0</sup> - 检查
  [语义化 `version`](https://semver.org)，例如 `"0.1.0"` 是否匹配
  `requirement`，例如 `">=0.1.0"`，如果是则返回 `"true"`，否则返回 `"false"`。

#### 样式

- `style(name)`<sup>1.37.0</sup> - 返回 `just` 使用的命名终端显示属性
  转义序列。与包含标准颜色和样式的终端显示属性转义
  序列常量不同，`style(name)`
  返回 `just` 本身使用的转义序列，可用于使
  配方输出与 `just` 自身的输出相匹配。

  `name` 的通过值有 `'command'`（用于回显的配方行），
  `error`，和 `warning`。

  例如，要设置错误消息的样式：

  ```just
  scary:
    @echo '{{ style("error") }}OH NO{{ NORMAL }}'
  ```

##### 用户目录<sup>1.23.0</sup>

这些函数返回用户特定目录的路径，用于存放
配置、数据、缓存、可执行文件和用户的主目录等。

在 Unix 上，这些函数遵循
[XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html)。

在 MacOS 和 Windows 上，这些函数返回系统指定的用户特定
目录。例如，`cache_directory()` 在 MacOS 上返回
`~/Library/Caches`，在 Windows 上返回 `{FOLDERID_LocalAppData}`。

有关更多详细信息，请参阅 [`dirs`](https://docs.rs/dirs/latest/dirs/index.html) crate。

- `cache_directory()` - 用户特定的缓存目录。
- `config_directory()` - 用户特定的配置目录。
- `config_local_directory()` - 本地用户特定的配置目录。
- `data_directory()` - 用户特定的数据目录。
- `data_local_directory()` - 本地用户特定的数据目录。
- `executable_directory()` - 用户特定的可执行文件目录。
- `home_directory()` - 用户的主目录。

如果你想在所有平台上使用 XDG 基本目录，你可以使用
带有适当环境变量和回退的 `env(…)` 函数，
虽然要注意 XDG 规范要求忽略非绝对路径，
因此为了与符合规范的应用程序完全兼容，你需要这样做：

```just
xdg_config_dir := if env('XDG_CONFIG_HOME', '') =~ '^/' {
  env('XDG_CONFIG_HOME')
} else {
  home_directory() / '.config'
}
```

### 常量

预定义了许多常量：

| 名称                             | 值                   | Windows 上的值 |
| -------------------------------- | -------------------- | -------------- |
| `HEX`<sup>1.27.0</sup>           | `"0123456789abcdef"` |                |
| `HEXLOWER`<sup>1.27.0</sup>      | `"0123456789abcdef"` |                |
| `HEXUPPER`<sup>1.27.0</sup>      | `"0123456789ABCDEF"` |                |
| `PATH_SEP`<sup>1.41.0</sup>      | `"/"`                | `"\"`          |
| `PATH_VAR_SEP`<sup>1.41.0</sup>  | `":"`                | `";"`          |
| `CLEAR`<sup>1.37.0</sup>         | `"\ec"`              |                |
| `NORMAL`<sup>1.37.0</sup>        | `"\e[0m"`            |                |
| `BOLD`<sup>1.37.0</sup>          | `"\e[1m"`            |                |
| `ITALIC`<sup>1.37.0</sup>        | `"\e[3m"`            |                |
| `UNDERLINE`<sup>1.37.0</sup>     | `"\e[4m"`            |                |
| `INVERT`<sup>1.37.0</sup>        | `"\e[7m"`            |                |
| `HIDE`<sup>1.37.0</sup>          | `"\e[8m"`            |                |
| `STRIKETHROUGH`<sup>1.37.0</sup> | `"\e[9m"`            |                |
| `BLACK`<sup>1.37.0</sup>         | `"\e[30m"`           |                |
| `RED`<sup>1.37.0</sup>           | `"\e[31m"`           |                |
| `GREEN`<sup>1.37.0</sup>         | `"\e[32m"`           |                |
| `YELLOW`<sup>1.37.0</sup>        | `"\e[33m"`           |                |
| `BLUE`<sup>1.37.0</sup>          | `"\e[34m"`           |                |
| `MAGENTA`<sup>1.37.0</sup>       | `"\e[35m"`           |                |
| `CYAN`<sup>1.37.0</sup>          | `"\e[36m"`           |                |
| `WHITE`<sup>1.37.0</sup>         | `"\e[37m"`           |                |
| `BG_BLACK`<sup>1.37.0</sup>      | `"\e[40m"`           |                |
| `BG_RED`<sup>1.37.0</sup>        | `"\e[41m"`           |                |
| `BG_GREEN`<sup>1.37.0</sup>      | `"\e[42m"`           |                |
| `BG_YELLOW`<sup>1.37.0</sup>     | `"\e[43m"`           |                |
| `BG_BLUE`<sup>1.37.0</sup>       | `"\e[44m"`           |                |
| `BG_MAGENTA`<sup>1.37.0</sup>    | `"\e[45m"`           |                |
| `BG_CYAN`<sup>1.37.0</sup>       | `"\e[46m"`           |                |
| `BG_WHITE`<sup>1.37.0</sup>      | `"\e[47m"`           |                |

```just
@foo:
  echo {{HEX}}
```

```console
$ just foo
0123456789abcdef
```

以 `\e` 开头的常量是
[ANSI 转义序列](https://en.wikipedia.org/wiki/ANSI_escape_code)。

`CLEAR` 清除屏幕，类似于 `clear` 命令。其余的是
`\e[Nm` 的形式，其中 `N` 是一个整数，并设置终端显示属性。

终端显示属性转义序列可以组合，例如文本
粗细 `BOLD`，文本样式 `STRIKETHROUGH`，前景色 `CYAN`，和
背景色 `BG_BLUE`。它们后面应该跟有 `NORMAL`，以将
终端重置为正常。

转义序列应该被引用，因为 `[` 在某些 shell 中
被视为特殊字符。

```just
@foo:
  echo '{{BOLD + STRIKETHROUGH + CYAN + BG_BLUE}}Hi!{{NORMAL}}'
```

### 属性

配方、`mod` 语句和别名可以用更改其
行为的属性进行注释。

| 名称                                             | 类型           | 描述                                                                                        |
| ------------------------------------------------ | -------------- | ------------------------------------------------------------------------------------------- |
| `[arg(ARG, help="HELP")]`<sup>master</sup>       | recipe         | 在用法消息中打印 `ARG` 的帮助字符串 `HELP`。                                                |
| `[arg(ARG, long="LONG")]`<sup>master</sup>       | recipe         | 要求 `ARG` 的值作为 `--LONG` 选项传递。                                                     |
| `[arg(ARG, short="S")]`<sup>master</sup>         | recipe         | 要求 `ARG` 的值作为短 `-S` 选项传递。                                                       |
| `[arg(ARG, value="VALUE")]`<sup>master</sup>     | recipe         | 使选项 `ARG` 成为不带值的标志。                                                             |
| `[arg(ARG, pattern="PATTERN")]`<sup>1.45.0</sup> | recipe         | 要求 `ARG` 的值匹配正则表达式 `PATTERN`。                                                   |
| `[confirm]`<sup>1.17.0</sup>                     | recipe         | 在执行配方之前要求确认。                                                                    |
| `[confirm(PROMPT)]`<sup>1.23.0</sup>             | recipe         | 在执行配方之前要求带自定义提示的确认。                                                      |
| `[default]`<sup>1.43.0</sup>                     | recipe         | 使用配方作为模块的默认配方。                                                                |
| `[doc(DOC)]`<sup>1.27.0</sup>                    | module, recipe | 将配方或模块的 [文档注释](#documentation-comments) 设置为 `DOC`。                           |
| `[extension(EXT)]`<sup>1.32.0</sup>              | recipe         | 将 shebang 配方脚本的文件扩展名设置为 `EXT`。如果是需要的，`EXT` 应包含句点。               |
| `[group(NAME)]`<sup>1.27.0</sup>                 | module, recipe | 将配方或模块放入 [组](#groups) `NAME` 中。                                                  |
| `[linux]`<sup>1.8.0</sup>                        | recipe         | 在 Linux 上启用配方。                                                                       |
| `[macos]`<sup>1.8.0</sup>                        | recipe         | 在 MacOS 上启用配方。                                                                       |
| `[metadata(METADATA)]`<sup>1.42.0</sup>          | recipe         | 将 `METADATA` 附加到配方。                                                                  |
| `[no-cd]`<sup>1.9.0</sup>                        | recipe         | 执行配方前不更改目录。                                                                      |
| `[no-exit-message]`<sup>1.7.0</sup>              | recipe         | 如果配方失败，不打印错误消息。                                                              |
| `[no-quiet]`<sup>1.23.0</sup>                    | recipe         | 覆盖全局安静的配方，并始终回显配方。                                                        |
| `[openbsd]`<sup>1.38.0</sup>                     | recipe         | 在 OpenBSD 上启用配方。                                                                     |
| `[parallel]`<sup>1.42.0</sup>                    | recipe         | 并行运行此配方的依赖项。                                                                    |
| `[positional-arguments]`<sup>1.29.0</sup>        | recipe         | 为此配方打开 [位置参数](#positional-arguments)。                                            |
| `[private]`<sup>1.10.0</sup>                     | alias, recipe  | 将配方、别名或变量设为私有。请参阅 [私有配方](#private-recipes)。                           |
| `[script]`<sup>1.33.0</sup>                      | recipe         | 作为脚本执行配方。有关系列信息，请参阅 [脚本配方](#script-recipes)。                        |
| `[script(COMMAND)]`<sup>1.32.0</sup>             | recipe         | 作为由 `COMMAND` 解释的脚本执行配方。有关更多详细信息，请参阅 [脚本配方](#script-recipes)。 |
| `[unix]`<sup>1.8.0</sup>                         | recipe         | 在 Unix 上启用配方。（包括 MacOS）。                                                        |
| `[windows]`<sup>1.8.0</sup>                      | recipe         | 在 Windows 上启用配方。                                                                     |
| `[working-directory(PATH)]`<sup>1.38.0</sup>     | recipe         | 设置配方工作目录。`PATH` 可以是相对的或绝对的。如果是相对的，它将解释为相对于默认工作目录。 |

配方可以有多个属性，或者在多行上：

```just
[no-cd]
[private]
foo:
    echo "foo"
```

或者在单行上用逗号分隔<sup>1.14.0</sup>：

```just
[no-cd, private]
foo:
    echo "foo"
```

具有单个参数的属性可以用冒号编写：

```just
[group: 'bar']
foo:
```

#### 启用和禁用配方<sup>1.8.0</sup>

`[linux]`、`[macos]`、`[unix]` 和 `[windows]` 属性是
配置属性。默认情况下，配方始终处于启用状态。具有
一个或多个配置属性的配方仅在
其中一个或多个配置处于活动状态时才会启用。

这可用于编写根据
运行的操作系统表现不同的 `justfile`。此 `justfile` 中的 `run` 配方将
编译并运行 `main.c`，使用不同的 C 编译器并使用
正确的输出二进制名称，具体取决于操作系统：

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

#### 禁用更改目录<sup>1.9.0</sup>

`just` 通常在执行配方时将当前目录设置为
包含 `justfile` 的目录。可以使用
`[no-cd]` 属性禁用此功能。这可用于创建使用
相对于调用目录路径的配方，或操作当前
目录的配方。

例如，此 `commit` 配方：

```just
[no-cd]
commit file:
  git add {{file}}
  git commit
```

可以与相对于当前目录的路径一起使用，因为
`[no-cd]` 阻止 `just` 在执行 `commit` 时更改当前目录。

#### 要求配方确认<sup>1.17.0</sup>

`just` 通常执行所有配方，除非发生错误。`[confirm]`
属性允许配方在运行之前要求在终端中确认。
可以通过传递 `--yes` 给 `just` 来覆盖此行为，这将自动
确认任何标记为此属性的配方。

如果所依赖的配方未被确认，则依赖于要求确认的配方的配方将不会运行，
以及在任何要求确认的配方之后传递的配方。

```just
[confirm]
delete-all:
  rm -rf *
```

#### 自定义确认提示<sup>1.23.0</sup>

默认确认提示可以使用 `[confirm(PROMPT)]` 覆盖：

```just
[confirm("Are you sure you want to delete everything?")]
delete-everything:
  rm -rf *
```

### 组

配方和模块可以用一个或多个组名进行注释：

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

# not in any group
email-everyone:
    echo 'Sending mass email…'
```

配方按组列出：

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

`just --list --unsorted` 在每个组内按 justfile 顺序列出配方：

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

组也可以用 `--groups` 列出：

```
$ just --groups
Recipe groups:
  lint
  rust recipes
```

使用 `just --groups --unsorted` 按 justfile 顺序列出组。

### 使用反引号的命令评估

反引号可用于存储命令的结果：

```just
localhost := `dumpinterfaces | cut -d: -f2 | sed 's/\/.*//' | sed 's/ //g'`

serve:
  ./serve {{localhost}} 8080
```

缩进的反引号，由三个反引号分隔，以与缩进字符串相同的
方式取消缩进：

````just
# This backtick evaluates the command `echo foo\necho bar\n`, which produces the value `foo\nbar\n`.
stuff := ```
    echo foo
    echo bar
  ```
````

有关取消缩进的详细信息，请参阅 [字符串](#strings) 部分。

反引号不能以 `#!` 开头。此语法保留用于将来的
升级。

[`shell(…)` 函数](#external-commands) 提供了一种更通用的
调用外部命令的机制，包括将
变量的内容作为命令执行，以及向命令传递参数的能力。

### 条件表达式

`if`/`else` 表达式根据两个
表达式计算结果是否相同来计算不同的分支：

```just
foo := if "2" == "2" { "Good!" } else { "1984" }

bar:
  @echo "{{foo}}"
```

```console
$ just bar
Good!
```

也可以测试不等式：

```just
foo := if "hello" != "goodbye" { "xyz" } else { "abc" }

bar:
  @echo {{foo}}
```

```console
$ just bar
xyz
```

并匹配正则表达式：

```just
foo := if "hello" =~ 'hel+o' { "match" } else { "mismatch" }

bar:
  @echo {{foo}}
```

```console
$ just bar
match
```

正则表达式由
[regex crate](https://github.com/rust-lang/regex) 提供，其语法记录在
[docs.rs](https://docs.rs/regex/1.5.4/regex/#syntax) 上。由于正则表达式
通常使用反斜杠转义序列，请考虑使用单引号字符串
字面量，这将把斜杠原封不动地传递给正则解析器。

条件表达式短路，这意味着它们只计算
其中一个分支。这可用于确保反引号表达式在
不应该运行时不运行。

```just
foo := if env_var("RELEASE") == "true" { `get-something-from-release-database` } else { "dummy-value" }
```

条件语句可以在配方内部使用：

```just
bar foo:
  echo {{ if foo == "bar" { "hello" } else { "goodbye" } }}
```

多个条件可以链接：

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

### 停止执行并报错

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

运行时产生以下错误：

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

任意数量的 `NAME=VALUE` 形式的参数可以在配方之前传递：

```console
$ just os=plan9
./build plan9
./test --test plan9
```

或者你可以使用 `--set` 标志：

```console
$ just --set os bsd
./build bsd
./test --test bsd
```

### 获取和设置环境变量

#### 导出 `just` 变量

以 `export` 关键字为前缀的赋值将作为
环境变量导出到配方：

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

导出的变量和参数不会导出到同一范围内的反引号。

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

当设置了 [export](#export) 时，所有 `just` 变量都将作为环境
变量导出。

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

来自环境的环境变量会自动传递给
配方。

```just
print_home_folder:
  echo "HOME is: '${HOME}'"
```

```console
$ just
HOME is '/home/myuser'
```

#### 从环境变量设置 `just` 变量

环境变更可以传播到 `just` 变量使用 `env()` 函数。
参见
[environment-variables](#environment-variables)。

### 配方参数

配方可能有参数。这里配方 `build` 有一个名为
`target` 的参数：

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

要将参数传递给依赖项，请将依赖项放在括号中以及
参数：

```just
default: (build "main")

build target:
  @echo 'Building {{target}}…'
  cd {{target}} && make
```

变量也可以作为参数传递给依赖项：

```just
target := "main"

_build version:
  @echo 'Building {{version}}…'
  cd {{version}} && make

build: (_build target)
```

命令的参数可以通过将依赖项放在
括号中以及参数来传递给依赖项：

```just
build target:
  @echo "Building {{target}}…"

push target: (build target)
  @echo 'Pushing {{target}}…'
```

参数可能有默认值：

```just
default := 'all'

test target tests=default:
  @echo 'Testing {{target}}:{{tests}}…'
  ./test --tests {{tests}} {{target}}
```

具有默认值的参数可以省略：

```console
$ just test server
Testing server:all…
./test --tests all server
```

或提供：

```console
$ just test server unit
Testing server:unit…
./test --tests unit server
```

默认值可以是任意表达式，但包含
`+`、`&&`、`||` 或 `/` 运算符的表达式必须加括号：

```just
arch := "wasm"

test triple=(arch + "-unknown-unknown") input=(arch / "input.dat"):
  ./test {{triple}}
```

配方的最后一个参数可以是可变的，在参数名称前用 `+` 或 `*`
表示：

```just
backup +FILES:
  scp {{FILES}} me@server.com:
```

以 `+` 为前缀的可变参数接受 _一个或多个_ 参数并扩展
为包含那些参数的字符串，参数之间用空格分隔：

```console
$ just backup FAQ.md GRAMMAR.md
scp FAQ.md GRAMMAR.md me@server.com:
FAQ.md                  100% 1831     1.8KB/s   00:00
GRAMMAR.md              100% 1666     1.6KB/s   00:00
```

以 `*` 为前缀的可变参数接受 _零个或多个_ 参数并
扩展为包含那些参数的字符串，参数之间用空格分隔，或者如果
不存在参数则为空字符串：

```just
commit MESSAGE *FLAGS:
  git commit {{FLAGS}} -m "{{MESSAGE}}"
```

可变参数可以分配默认值。这些被
命令行传递的参数覆盖：

```just
test +FLAGS='-q':
  cargo test {{FLAGS}}
```

`{{…}}` 替换如果包含空格，可能需要用引号引起来。
例如，如果你有以下配方：

```just
search QUERY:
  lynx https://www.google.com/?q={{QUERY}}
```

而你输入：

```console
$ just search "cat toupee"
```

`just` 将运行命令 `lynx https://www.google.com/?q=cat toupee`，这
将被 `sh` 解析为 `lynx`、`https://www.google.com/?q=cat` 和
`toupee`，而不是预期的 `lynx` 和 `https://www.google.com/?q=cat toupee`。

你可以通过添加引号来修复此问题：

```just
search QUERY:
  lynx 'https://www.google.com/?q={{QUERY}}'
```

以 `$` 为前缀的参数将作为环境变量导出：

```just
foo $bar:
  echo $bar
```

参数可以限制为使用
`[arg("name", pattern="pattern")]` 属性<sup>1.45.0</sup> 匹配正则表达式模式：

```just
[arg('n', pattern='\d+')]
double n:
  echo $(({{n}} * 2))
```

模式前后添加了 `^` 和 `$`，因此必须匹配
整个参数值。

你可以使用 `|` 运算符将模式限制为多个
备选项：

```just
[arg('flag', pattern='--help|--version')]
info flag:
  just {{flag}}
```

正则表达式由
[Rust `regex` crate](https://docs.rs/regex/latest/regex/) 提供。有关用法
示例，请参阅 [语法文档](https://docs.rs/regex/latest/regex/#syntax)。

配方的用法信息可以使用 `--usage` 子命令<sup>master</sup>
打印：

```console
$ just --usage foo
Usage: just foo [OPTIONS] bar

Arguments:
  bar
```

可以使用 `[arg(ARG, help=HELP)]` 属性将帮助字符串添加到参数：

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

在这个 `justfile` 中：

```just
@foo bar:
  echo bar={{bar}}
```

参数 `bar` 是位置参数：

```console
$ just foo hello
bar=hello
```

`[arg(ARG, long=OPTION)]`<sup>master</sup> 属性可用于使
参数成为长选项。

在这个 `justfile` 中：

```just
[arg("bar", long="bar")]
foo bar:
```

参数 `bar` 通过 `--bar` 选项给出：

```console
$ just foo --bar hello
bar=hello
```

选项也可以使用 `--name=value` 语法传递：

```console
$ just foo --bar=hello
bar=hello
```

`[arg(ARG, short=OPTION)]`<sup>master</sup> 属性可用于使
参数成为短选项。

在这个 `justfile` 中：

```just
[arg("bar", short="b")]
foo bar:
```

参数 `bar` 通过 `-b` 选项给出：

```console
$ just foo -b hello
bar=hello
```

如果参数同时具有长选项和短选项，这可以使用任一选项传递。

可变 `+` 和 `?` 参数不能是选项。

`[arg(ARG, value=VALUE, …)]`<sup>master</sup> 属性可以与
`long` 或 `short` 一起使用以使参数成为不接受值的标志。

在这个 `justfile` 中：

```just
[arg("bar", long="bar", value="hello")]
foo bar:
```

参数 `bar` 通过 `--bar` 选项给出，但不接受
值，而是接受 `[arg]` 属性中给出的值：

```console
$ just foo --bar
bar=hello
```

这对于无条件要求像 `--force` 这样的标志在危险
命令上很有用。

如果其参数具有默认值，则标志是可选的：

```just
[arg("bar", long="bar", value="hello")]
foo bar="goodbye":
```

导致未在调用中传递时接收默认值：

```console
$ just foo
bar=goodbye
```

### 依赖

依赖项在依赖它们的配方之前运行：

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

在给定的 `just` 调用中，具有相同参数的配方将仅运行
一次，无论它在命令行调用中出现多少次，
或者它作为依赖项出现多少次：

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

多个配方可能依赖于执行某种设置的配方，
当这些配方运行时，该设置仅执行一次：

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

给定运行中的配方只有在接收到相同参数时才会被跳过：

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

配方的正常依赖项总是在配方开始之前运行。也就是
说，被依赖者总是在依赖者之前运行。这些依赖项
称为“前置依赖项”。

配方也可以有后续依赖项，它们在配方之后之后立即运行
并用 `&&` 引入：

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

…运行 _b_ 打印：

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

`just` 不支持在另一个配方中间运行配方，但你
可以在配方中间递归调用 `just`。给定以下
`justfile`：

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

…运行 _b_ 打印：

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

这有局限性，因为配方 `c` 是用完全新的 `just` 调用运行的
：赋值将被重新计算，依赖项可能会运行两次，并且
命令行参数不会传播到子 `just` 进程。

### Shebang 配方

以 `#!` 开头的配方称为 shebang 配方，并通过
将配方体保存到文件并运行它来执行。这允许你用
不同的语言编写配方：

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
$ just polyglot
Hello from python!
Greetings from JavaScript!
Larry Wall says Hi!
Yo from a shell script!
Hola from a nushell script!
Hello from ruby!
```

在包括 Linux 和 MacOS 的类 Unix 操作系统上，shebang 配方通过
将配方体保存到临时目录中的文件，将
文件标记为可执行文件并执行它来执行。然后 OS 将 shebang 行解析
为命令行并调用它，包括文件的路径。例如，
如果配方以 `#!/usr/bin/env bash` 开头，OS 运行的最终命令
将类似于 `/usr/bin/env bash
/tmp/PATH_TO_SAVED_RECIPE_BODY`。

Shebang 行拆分取决于操作系统。当传递
带有参数的命令时，你可能需要告诉 `env` 显式拆分它们，使用
`-S` 标志：

```just
run:
  #!/usr/bin/env -S bash -x
  ls
```

Windows 不支持 shebang 行。在 Windows 上，`just` 将 shebang 行
拆分为命令和参数，将配方体保存到文件，并调用
拆分的命令和参数，将保存的配方体的路径作为
最后一个参数添加。例如，在 Windows 上，如果配方以 `#! py` 开头，
OS 运行的最终命令将类似于
`py C:\Temp\PATH_TO_SAVED_RECIPE_BODY`。

### 脚本配方

带有 `[script(COMMAND)]`<sup>1.32.0</sup> 属性的配方作为
由 `COMMAND` 解释的脚本运行。这避免了 shebang 配方的一些问题
，例如 Windows 上 `cygpath` 的使用，需要使用
`/usr/bin/env`，Unix OS 之间 shebang 行拆分的不一致，以及
需要可以从中执行文件的临时目录。

带有空 `[script]` 属性的配方使用 `set script-interpreter := […]`<sup>1.33.0</sup> 的值执行，
默认为 `sh -eu`，而 _不是_ `set shell` 的值。

配方的主体被评估，写入临时目录中的磁盘，
并作为参数传递给 `COMMAND` 运行。

### 脚本和 Shebang 配方临时文件

脚本和 shebang 配方都将配方体写入临时文件以
执行。脚本配方通过将其传递给命令来执行该文件，而
shebang 配方直接执行该文件。如果
包含临时文件的文件系统挂载为 `noexec` 或
其他原因不可执行，Shebang 配方执行将失败。

`just` 写入临时文件的目录可以配置为
多种方式，从优先级最高到最低：

- 全局使用 `--tempdir` 命令行选项或 `JUST_TEMPDIR`
  环境变量<sup>1.41.0</sup>。

- 在每个模块的基础上使用 `tempdir` 设置。

- 在 Linux 上全局使用 `XDG_RUNTIME_DIR` 环境变量。

- 回退到
  [std::env::temp_dir](https://doc.rust-lang.org/std/env/fn.temp_dir.html) 返回的目录。

### 使用 `uv` 的 Python 配方

[`uv`](https://github.com/astral-sh/uv) 是一个优秀的跨平台 python
项目管理器，使用 Rust 编写。

使用 `[script]` 属性和 `script-interpreter` 设置，`just` 可以
轻松配置为使用 `uv` 运行 Python 配方：

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

当然，shebang 也可以工作：

```just
hello:
  #!/usr/bin/env -S uv run --script
  print("Hello from Python!")
```

### 更安全的 Bash Shebang 配方

如果你正在编写 `bash` shebang 配方，请考虑添加 `set -euxo pipefail`：

```just
foo:
  #!/usr/bin/env bash
  set -euxo pipefail
  hello='Yo'
  echo "$hello from Bash!"
```

这并非严格必要，但 `set -euxo pipefail` 开启了一些有用的
功能，使 `bash` shebang 配方的行为更像普通的、逐行的
`just` 配方：

- `set -e` 使 `bash` 在命令失败时退出。

- `set -u` 使 `bash` 在变量未定义时退出。

- `set -x` 使 `bash` 在运行之前打印每个脚本行。

- `set -o pipefail` 使 `bash` 在管道中的命令失败时退出。这是
  `bash` 特有的，因此在普通的逐行 `just` 配方中未开启。

总的来说，这些避免了很多 shell 脚本陷阱。

#### Windows 上的 Shebang 配方执行

在 Windows 上，包含 `/` 的 shebang 解释器路径会使用
随 [Cygwin](http://www.cygwin.com) 附带的 `cygpath` 工具从
Unix 样式路径转换为 Windows 样式路径。

例如，要在 Windows 上执行此配方：

```just
echo:
  #!/bin/sh
  echo "Hello!"
```

解释器路径 `/bin/sh` 将在执行前使用 `cygpath` 转换为 Windows 样式路径。

如果解释器路径不包含 `/`，它将在不进行
转换的情况下执行。如果你没有 `cygpath`，或者你希望
将 Windows 样式路径传递给解释器，这很有用。

### 在配方中设置变量

配方行由 shell 解释，而不是 `just`，因此不可能
在配方中间设置 `just` 变量：

```justfile
foo:
  x := "hello" # This doesn't work!
  echo {{x}}
```

可以使用 shell 变量，但还有另一个问题。每个
配方行都由一个新的 shell 实例运行，因此在一行中设置的变量不会
在下一行中设置：

```just
foo:
  x=hello && echo $x # This works!
  y=bye
  echo $y            # This doesn't, `y` is undefined here!
```

解决此问题的最佳方法是使用 shebang 配方。Shebang 配方
主体被提取并作为脚本运行，因此单个 shell 实例将运行
整个过程：

```just
foo:
  #!/usr/bin/env bash
  set -euxo pipefail
  x=hello
  echo $x
```

### 在配方之间共享环境变量

每个配方的每一行都由一个新的 shell 执行，因此不可能
在配方之间共享环境变量。

#### 使用 Python 虚拟环境

一些工具，如 [Python 的 venv](https://docs.python.org/3/library/venv.html)，
需要加载环境变量才能工作，这使得它们很难
与 `just` 一起使用。作为一种解决方法，你可以直接执行虚拟环境
二进制文件：

```just
venv:
  [ -d foo ] || python3 -m venv foo

run: venv
  ./foo/bin/python3 main.py
```

### 在配方中更改工作目录

每个配方行都由一个新的 shell 执行，因此如果你在
一行上更改工作目录，它不会对后续行产生影响：

```just
foo:
  pwd    # This `pwd` will print the same directory…
  cd bar
  pwd    # …as this `pwd`!
```

有几种方法可以解决这个问题。一种是在同一行上调用 `cd` 以及
你要运行的命令：

```just
foo:
  cd bar && pwd
```

另一种是使用 shebang 配方。Shebang 配方主体被提取并
作为脚本运行，因此单个 shell 实例将运行整个过程，因此
一行上的 `cd` 将影响后续行，就像 shell 脚本一样：

```just
foo:
  #!/usr/bin/env bash
  set -euxo pipefail
  cd bar
  pwd
```

### 缩进

配方行可以用空格或制表符缩进，但不能混合使用。
配方的所有行必须具有相同类型的缩进，但
同一 `justfile` 中的不同配方可以使用不同的缩进。

每个配方必须从 `recipe-name` 至少缩进一级，但
在此之后可以进一步缩进。

这是一个用空格（表示为 `·`）和
制表符（表示为 `→`）缩进配方的 justfile。

```justfile
set windows-shell := ["pwsh", "-NoLogo", "-NoProfileLoadTime", "-Command"]

set ignore-comments

list-space directory:
··#!pwsh
··foreach ($item in $(Get-ChildItem {{directory}} )) {
····echo $item.Name
··}
··echo ""

# indentation nesting works even when newlines are escaped
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

### 多行构造

没有初始 shebang 的配方是逐行评估和运行的，这
意味着多行构造可能不会按你的意愿执行。

例如，使用以下 `justfile`：

```justfile
conditional:
  if true; then
    echo 'True!'
  fi
```

`conditional` 配方第二行之前的额外前导空格
将产生解析错误：

```console
$ just conditional
error: Recipe line has extra leading whitespace
  |
3 |         echo 'True!'
  |     ^^^^^^^^^^^^^^^^
```

为了解决这个问题，你可以将条件语句写在一行上，用反斜杠转义
换行符，或者在你的配方中添加 shebang。为参考提供了一些多行
构造的示例。

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

#### 配方体之外

括号表达式可以跨越多行：

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

以反斜杠结尾的行继续到下一行，就像这些行由
空格连接一样<sup>1.15.0</sup>：

```just
a := 'foo' + \
     'bar'

foo param1 \
  param2='foo' \
  *varparam='': dep1 \
                (dep2 'foo')
  echo {{param1}} {{param2}} {{varparam}}

dep1: \
    # this comment is not part of the recipe body
  echo 'dep1'

dep2 \
  param:
    echo 'Dependency with parameter {{param}}'
```

反斜杠行续行也可以在插值中使用。
反斜杠后面的行必须缩进。

```just
recipe:
  echo '{{ \
  "This interpolation " + \
    "has a lot of text." \
  }}'
  echo 'back to recipe body'
```

### 命令行选项

`just` 支持许多用于列出、转储
和调试配方和变量的有用的命令行选项：

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

一些命令行选项可以使用环境变量设置

例如，可以使用 `--unstable` 标志启用不稳定功能：

```console
$ just --unstable
```

或者通过设置 `JUST_UNSTABLE` 环境变量：

```console
$ export JUST_UNSTABLE=1
$ just
```

由于环境变量由子进程继承，因此使用环境变量
设置的命令行选项由 `just` 的递归调用继承，
而使用参数设置的命令行选项则不会。

请参阅 `just --help` 了解可以使用环境变量设置哪些选项。

### 私有配方

名称以 `_` 开头的配方和别名将从 `just --list` 中省略：

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

以及从 `just --summary` 中省略：

```console
$ just --summary
test
```

`[private]` 属性<sup>1.10.0</sup> 也可用于隐藏配方或
别名，而无需更改名称：

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

这对仅用作其他配方的依赖项的辅助配方
非常有用。

### 安静配方

配方名称可以以 `@` 为前缀，以反转每行之前 `@` 的含义：

```just
@quiet:
  echo hello
  echo goodbye
  @# all done!
```

现在只有以 `@` 开头的行会被回显：

```console
$ just quiet
hello
goodbye
# all done!
```

可以使用 `set quiet` 使 Justfile 中的所有配方都变安静：

```just
set quiet

foo:
  echo "This is quiet"

@foo2:
  echo "This is also quiet"
```

`[no-quiet]` 属性覆盖此设置：

```just
set quiet

foo:
  echo "This is quiet"

[no-quiet]
foo2:
  echo "This is not quiet"
```

Shebang 配方默认是安静的：

```just
foo:
  #!/usr/bin/env bash
  echo 'Foo!'
```

```console
$ just foo
Foo!
```

将 `@` 添加到 shebang 配方名称会使 `just` 在
执行之前打印配方：

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

当配方行失败时，`just` 通常会打印错误消息。这些错误
消息可以使用 `[no-exit-message]`<sup>1.7.0</sup>
属性来抑制。你可能会发现这对包装工具的配方特别有用：

```just
git *args:
    @git {{args}}
```

```console
$ just git status
fatal: not a git repository (or any of the parent directories): .git
error: Recipe `git` failed on line 2 with exit code 128
```

添加属性以在工具以
非零代码退出时抑制退出错误消息：

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

`--choose` 子命令使 `just` 调用选择器来选择要
运行的配方。选择器应从标准输入读取包含配方名称的行，
并将一个或多个空格分隔的名称打印到标准输出。

由于目前无法使用 `--choose` 运行需要参数的
配方，因此此类配方不会提供给选择器。私有配方和
别名也会被跳过。

可以使用 `--chooser` 标志覆盖选择器。如果没有
给出 `--chooser`，则 `just` 首先检查是否设置了 `$JUST_CHOOSER`。如果没有，
则选择器默认为 `fzf`，一个流行的模糊查找器。

参数可以包含在选择器中，即 `fzf --exact`。

选择器的调用方式与配方行相同。例如，如果
选择器是 `fzf`，它将通过 `sh -cu 'fzf'` 调用，如果 shell 或
shell 参数被覆盖，选择器调用将遵守这些
覆盖。

如果你希望 `just` 默认使用选择器选择配方，你
可以使用此作为你的默认配方：

```just
default:
  @just --choose
```

### 在其他目录中调用 `justfile`

如果传递给 `just` 的第一个参数包含 `/`，则
发生以下情况：

1.  参数在最后一个 `/` 处拆分。

2.  最后一个 `/` 之前的部分被视为目录。`just` 将在那里
    开始搜索 `justfile`，而不是在当前目录中。

3.  最后一个斜杠之后的部分被视为普通参数，或者如果
    为空则忽略。

这可能看起来有点奇怪，但如果你想在
位于子目录中的 `justfile` 中运行命令，这很有用。

例如，如果你在一个包含名为
`foo` 的子目录的目录中，该子目录包含带有配方 `build` 的 `justfile`，这也是
默认配方，则以下所有内容都是等效的：

```console
$ (cd foo && just build)
$ just foo/build
$ just foo/
```

在同一个 `justfile` 中寻找第一个之后的其他配方。
例如，以下两者都是等效的：

```console
$ just foo/a b
$ (cd foo && just a b)
```

并且都会调用 `foo/justfile` 中的配方 `a` 和 `b`。

### 导入

一个 `justfile` 可以使用 `import` 语句包含另一个的内容。

如果你有以下 `justfile`：

```justfile
import 'foo/bar.just'

a: b
  @echo A
```

以及 `foo/bar.just` 中的以下文本：

```just
b:
  @echo B
```

`foo/bar.just` 将包含在 `justfile` 中，并且配方 `b` 将被定义：

```console
$ just b
B
$ just a
B
A
```

`import` 路径可以是绝对的或相对于包含它的 justfile 的位置。
导入路径前导的 `~/` 替换为当前
用户的主目录。

Justfiles 对顺序不敏感，因此包含的文件可以引用
在 `import` 语句之后定义的变量和配方。

导入的文件本身可以包含 `import`，这些将被
递归处理。

`allow-duplicate-recipes` 和 `allow-duplicate-variables` 分别
允许重复的配方和变量相互覆盖，而不是
产生错误。

在模块内，后面的定义覆盖前面的定义：

```just
set allow-duplicate-recipes

foo:

foo:
  echo 'yes'
```

当涉及 `import` 时，事情不幸地变得更加复杂且
难以解释。

较浅的定义总是覆盖较深的定义，因此顶层的配方
将覆盖导入中的配方，导入中的配方将覆盖
本身导入这些配方的导入中的配方。

当两个重复的定义被导入并且处于相同的深度时，来自
较早导入的那个将覆盖来自较晚导入的那个。

这是因为 `just` 在处理导入时使用堆栈，将导入
按源顺序推入堆栈，并且始终
下一个处理堆栈顶部，因此较早的导入实际上由编译器稍后处理。

这绝对是一个 bug，但由于 `just` 有非常强的向后
兼容性保证，如果不打破任何人的 `justfile` 我们会非常痛苦，
我们创建了 issue #2540 来讨论我们是否可以
实际上修复它。

通过在 `import` 关键字后放置 `?`，可以使导入可选：

```just
import? 'foo/bar.just'
```

多次导入同一源文件不是错误<sup>1.37.0</sup>。
这允许导入多个 justfiles，例如 `foo.just` 和
`bar.just`，它们都导入包含共享配方的第三个 justfile，
例如 `baz.just`，而重复导入 `baz.just` 不会报错：

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

```just
# baz
baz:
```

### 模块<sup>1.19.0</sup>

`justfile` 可以使用 `mod` 语句声明模块。

`mod` 语句在 `just`<sup>1.31.0</sup> 中已稳定。在早期
版本中，你可以使用 `--unstable` 标志、`set unstable` 或设置
`JUST_UNSTABLE` 环境变量来使用它们。

如果你有以下 `justfile`：

```justfile
mod bar

a:
  @echo A
```

以及 `bar.just` 中的以下文本：

```just
b:
  @echo B
```

`bar.just` 将作为子模块包含在 `justfile` 中。在一个子模块中定义的配方、别名和
变量不能在另一个中使用，并且每个模块
使用其自己的设置。

子模块中的配方可以作为子命令调用：

```console
$ just bar b
B
```

或者使用路径语法：

```console
$ just bar::b
B
```

如果模块名为 `foo`，just 将在 `foo.just`、
`foo/mod.just`、`foo/justfile` 和 `foo/.justfile` 中搜索模块文件。在后两种情况下，
模块文件可以采用任何大小写。

模块语句可以是以下形式：

```justfile
mod foo 'PATH'
```

它从 `PATH` 加载模块的源文件，而不是从通常的
位置。`PATH` 中前导的 `~/` 替换为当前用户的主
目录。`PATH` 可能指向模块源文件本身，或者指向
包含名为 `mod.just`、`justfile` 或
`.justfile` 的模块源文件的目录。在后两种情况下，模块文件可以采用任何
大小写。

环境文件仅为根 justfile 加载，加载的环境
变量在子模块中可用。子模块中影响
环境文件加载的设置将被忽略。

没有 `[no-cd]` 属性的子模块中的配方使用设置为
包含子模块源文件的目录的工作目录运行。

`justfile()` 和 `justfile_directory()` 始终返回根
justfile 的路径和包含它的目录，即使从子模块
配方中调用也是如此。

可以使模块可选，只需在 `mod` 关键字后放置 `?`：

```just
mod? foo
```

可选模块缺少源文件不会产生错误。

没有源文件的可选模块不冲突，因此你可以有多个
同名的 mod 语句，但具有不同的源文件路径，
只要最多存在一个源文件：

```just
mod? foo 'bar.just'
mod? foo 'baz.just'
```

模块可以给出文档注释，这些注释出现在 `--list`
输出中<sup>1.30.0</sup>：

```justfile
# foo is a great module!
mod foo
```

```console
$ just --list
Available recipes:
    foo ... # foo is a great module!
```

模块仍然缺少许多功能，例如，引用
其他模块中变量的能力。请参阅 [模块改进跟踪
issue](https://github.com/casey/just/issues/2252) 了解更多信息。

### 隐藏 Justfiles

如果是点文件，`just` 也可以找到 `justfile`。

```
$ mv justfile .justfile
$ just
```

这对于如果你不想让 `justfile` 弄乱你的项目根目录很有用。

### Just 脚本

如果在脚本的第一行使用 `#!/usr/bin/env -S just --justfile`，
`just` 可以用作就像 `sh` 或 `python` 一样的脚本解释器：

```just
#!/usr/bin/env -S just --justfile

print:
  echo 'Hello, world!'
```

这使得脚本可以使用 `just` 的功能，例如
依赖性解决和命令行解析。

### 格式化 Justfiles

`--fmt` 子命令格式化并覆盖 `justfile`，在适当的地方
添加空格和缩进。

```justfile
# a messy justfile
foo:
  echo   bar
```

```console
$ just --fmt
$ cat justfile
# a messy justfile
foo:
    echo bar
```

如果传递了 `--check`，`just` 将检查 `justfile` 格式是否正确，
如果是则退出并返回 0，如果不正确则返回 1：

```console
$ just --check --fmt
error: Justfile is not effectively formatted.
```

要将格式化的 `justfile` 打印到标准输出，请使用 `--stdout`。

`--fmt` 仍然是不稳定的。如果你在使用它时遇到任何错误，
请开启一个 issue！

`--fmt` 保留注释，但不能保留它们相对于
重写代码的位置。注释与配方、变量和
其他主要构造相关联。

### 转储 Justfiles

`--dump` 子命令将打印整个 `justfile` 到标准输出：

```console
$ just --dump
a := "A"

b:
    echo {{a}}
```

这对于调试 `import` 很有用，因为它会打印
带有导入的 `justfile`。

`--dump-format json` 标志以 JSON 格式转储 `justfile`
<sup>1.25.0</sup>。
JSON 格式目前是不稳定的，可能会在没有弃用周期的
情况下更改。

### 回退到父目录的 Justfiles

如果从子目录调用 `just` 并且它找不到 `justfile`，
它将向上搜索直到找到一个。这允许你在
子目录中运行命令：

```console
$ cd my-project
$ ls
justfile  main.c
$ mkdir subdir
$ cd subdir
$ just build
cc ../main.c
```

这不适用于 `fallback` 设置，如果找不到命令，
它将尝试在父目录中运行它。

这将使用调用 `just` 的子目录作为工作目录运行配方。
要改为在定义 `justfile` 的目录中运行配方，
请使用 `set working-directory := justfile_directory()` 设置。

### 避免参数拆分

如果你想将命令行参数传递给命令而不被
shell 拆分，你可以转义它们或将它们放在引号中。

然而，如果参数来自变量，这很难做到。

`just` 提供了一个特殊的双括号语法 `{{( ... )}}`<sup>1.34.0</sup>，它
避免了这个问题。在 `{{(` 和 `)}}` 内，不仅允许变量替换，还允许
其它的 `just` 表达式。

每个表达式都被评估并在不拆分的情况下作为单个参数传递给命令：

```just
test +args:
  # pass args to cargo test without splitting
  cargo test {{ (args) }}
```

```console
$ just test "a b"
cargo test "a b"
```

这仅适用于 `sh` 风格的 shell（即 `sh`、`bash`、`zsh` 等）。
对于其他 shell，表达式在作为参数传递之前被计算
并连接成一个字符串。

### 配置 shell

`just` 默认使用 `sh` 运行配方。大多数系统都预装了它。
如果你的系统没有 `sh`，你可以使用 `set shell := [...]`
更改 shell。

例如，要使用 Python：

```just
set shell := ["python3", "-c"]

default:
  print("Hello, world!")
```

传递给 `shell` 设置的列表中的第一个字符串是是要运行的命令，
所有后续字符串都是参数。配方作为最后一个
参数传递。

### 时间戳

可以通过设置 `JUST_TIMESTAMP=1` 或传递 `--timestamp`
在输出中启用时间戳：

```console
$ just --timestamp foo
[0.000s] echo foo
foo
```

### 信号处理

`just` 接收到的 `SIGINT`、`SIGTERM` 和 `SIGQUIT` 信号
（在 Windows 上是 CTRL-C 和 CTRL-BREAK）会传播到
正在运行的配方子进程。

一旦子进程退出，`just` 将以相同的信号退出。

### 更新日志

更新日志可以在 [这里](https://github.com/casey/just/blob/master/CHANGELOG.md) 找到。

### 常见问题解答

#### `just` 避免了 `make` 的哪些特性？

`make` 的许多特性对于构建 C 代码很有用，但对于运行
项目特定的命令却没有用处，甚至很烦人。

`just` 避免了 Make 的以下怪癖：

- `make` 必须以非标准方式使用才能像命令运行器一样工作。
  如果要定义的命令与文件同名，则需要 `.PHONY`
  规则，而这通常是命令运行器的常态。
- `make` 不能将参数传递给配方。
- `make` 大多对功能（如条件或字符串操作）有一套
  神秘的语法。
- `make` 有许多默认配方和变量，这在作为
  命令运行器使用时几乎不需要。
- `make` 对空格敏感。
- `make` 默认回显配方。

#### `just` 和 Cargo 构建脚本有什么关系？

Cargo 在编译 crate 之前运行
[构建脚本](https://doc.rust-lang.org/cargo/reference/build-scripts.html)。
它们用于确定库的位置、生成代码或
配置构建。

`just` 并不打算成为 cargo 构建脚本的替代品。

`just` 是一个命令运行器。它用于运行不属于
cargo 构建流程的命令，例如安装依赖项、
运行部署脚本或运行测试。

### 更多随笔

`just` 是保存和运行专案特有命令的便捷方式。

如果你经常运行，但又不足以让你记住它，
这是一个很好的候选者：

```bash
# connect to the production database console
psql -h production.example.com -U user production_db
```

你可以将其放入 `.bashrc` 或 `.zshrc` 中的别名中，但这使得它
特定于特定用户，并且如果你有多个项目，这很难
管理。

或者，如果你有一组用于项目的命令，你可以在项目的根目录中
创建一个 `justfile`，并逐步将它们添加进去：

```just
# connect to the database
db:
  psql -h production.example.com -U user production_db

# run the server
serve:
  ./serve

# run the tests
test:
  ./test
```

现在你把它们都列在了一个地方，任何为该项目工作的人
都可以看到和运行它们。

### 贡献

功能请求和错误报告总是受欢迎的！
请尽管在 GitHub 上开启 issue。

Just 由社区维护。请参阅 [MAINTAINERS.md](MAINTAINERS.md)
以获取当前的维护者列表。请注意，虽然维护者
列表可能与提交历史没有太大重叠，但该列表
包含致力于保持项目运行和响应
问题的人员。

### 故障排除

如果你遇到意外行为，请尝试使用 `--verbose`。

如果你使用的是 VS Code 和 Just 扩展，请注意
许多扩展尚不支持所有语法<sup>1.36.0</sup>，
并可能产生看起来像 `just` 错误的解析错误。

### 手册页

手册页托管在 [just.systems/man/](https://just.systems/man/)。

### Shell 补全脚本

Shell 补全脚本是使用 [clap](https://github.com/kbknapp/clap-rs)
生成的。

生成的 Shell 补全脚本可以在
[completions](https://github.com/casey/just/tree/master/completions) 目录中找到。

#### Bash / Zsh

将 `completions/just.bash` 添加到你的 `.bashrc`：

```bash
source path/to/just.bash
```

#### Fish

将 `completions/just.fish` 放在你的 fish completions 目录中：

```bash
cp completions/just.fish ~/.config/fish/completions
```

#### PowerShell

将 `completions/just.ps1` 放在你的 PowerShell completions 目录中，在 PowerShell 中：

```powershell
mkdir -p $HOME\Documents\WindowsPowerShell\Modules\just
cp completions\just.ps1 $HOME\Documents\WindowsPowerShell\Modules\just\just.psm1
Import-Module just
```

### 食谱

`just` 用户的技巧和窍门集合可以在
[wiki](https://github.com/casey/just/wiki) 上找到。

### 类似工具

`just` 受到 `make` 的启发，还有许多其他工具也同样如此。

- [cargo-make](https://github.com/sagiegurari/cargo-make) 和
  [cargo-run-script](https://github.com/JoshMcguigan/cargo-run-script)
  是 cargo 插件。
- [mmake](https://github.com/tj/mmake) 对 `make` 进行了现代化改造，
  并提供了有用的反馈。
- [myke](https://github.com/go-task/task) 类似于 `just`，具有
  YAML 语法。
- [ThoNy](https://github.com/Koka-Kiwa/ThoNy) 是另一个任务运行器，
  具有 YAML 语法，支持 Python 和 Bucket。
- [mask](https://github.com/jakode/mask) 具有 markdown 语法。
- [run](https://github.com/TekWizely/run) 的灵感来自 `make` 和 `easy-make`。
- [asdf](https://github.com/asdf-vm/asdf) 是一个多版本管理器。
- [robo](https://github.com/tj/robo) 是一个带有 YAML 语法的简单
  任务运行器。
- [maid](https://github.com/maidjs/maid) 是一个基于 markdown 的任务
  运行器。

### 许可证

`just` 采用 [CC0 1.0 Universal](LICENSE) 许可证。

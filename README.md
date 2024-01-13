# A WebRTC IP Leak Vulnerability Demonstration

This is a WebRTC IP leak vulnerability demonstration written in the Rust programming language and JavaScript.

## Table of Contents

- [Disclaimer](#disclaimer)
- [License](#license)
- [Changelog](#changelog)
- [Prerequisites](#prerequisites)
  - [Prerequisites installation on FreeBSD](#prerequisites-installation-on-freebsd)
  - [Prerequisites installation on Gentoo Linux](#prerequisites-installation-on-gentoo-linux)
  - [Prerequisites installation on AlmaLinux](#prerequisites-installation-on-almalinux)
  - [Prerequisites installation on CentOS](#prerequisites-installation-on-centos)
  - [Prerequisites installation on Debian GNU/Linux](#prerequisites-installation-on-debian-gnulinux)
  - [Prerequisites installation on Rocky Linux](#prerequisites-installation-on-rocky-linux)
  - [Prerequisites installation on Ubuntu](#prerequisites-installation-on-ubuntu)
  - [GNU Make Installation on Microsoft Windows](#gnu-make-installation-on-microsoft-windows)
- [Checkout](#checkout)
- [Building](#building)
  - [Debug Builds](#debug-builds)
- [Running](#running)
- [Usage](#usage)
- [Obtaining a STUN server](#obtaining-a-stun-server)

## Disclaimer

```
THE CONTENT IS PROVIDED FOR EDUCATIONAL AND INFORMATIONAL PURPOSES ONLY WITHOUT
ANY WARRANTIES, GUARANTEES, OR CONDITIONS, OF ANY KIND, AND MAY NOT BE ACCURATE,
UP-TO-DATE, OR COMPLETE. ANY USE OR RELIANCE ON ANY CONTENT OR MATERIALS
PUBLISHED, MENTIONED, OR LINKED HERE IS AT YOUR OWN RISK AND THE AUTHOR/AUTHORS
ACCEPT NO LIABILITY OR RESPONSIBILITY FOR.

THE CODE PROVIDED IN THIS REPOSITORY IS FOR ACADEMIC AND EDUCATIONAL PURPOSES
ONLY AND ANY MALICIOUS USE OF THIS SOFTWARE IS YOUR LEGAL LIABILITY AND ETHICAL
RESPONSIBILITY.

FOR THE USAGE LICENSE SEE BELOW.
```

```
تمامی محتویات ارائه شده صرفا جنبه آموزشی و اطلاعاتی داشته و فاقد هرگونه ضمانت،
تعهد یا شرایطی از هر نوع می باشد. بایستی توجه نمود که اطلاعات عرضه شده حتی ممکن
است دقیق و یا بروز نباشد. هرگونه اطمینان به و یا استفاده از محتویات یا منابع
منتشر شده در این مخزن با مسئولیت مخاطب بوده و نگارنده یا نگارندگان هیچ گونه
مسئولیتی در مورد عواقب آن را نخواهند پذیرفت.

کد ارائه شده در این مخزن تنها جهت مقاصد تحصیلی و آموزشی می‌باشد و هر گونه
استفاده مخرب از این نرم افزار مسئولیت قانونی و  اخلاقی شما را به همراه خواهد
داشت.

جهت مشاهده پروانه استفاده به انتها مراجعه نمایید.
```

## License

Everything in this repository is licensed under the terms of [the MIT License](LICENSE.md).

## Changelog

[A comprehensive changelog](CHANGELOG.md) is being kept for this project. Prior to upgrading to any newer releases, kindly refer to the changelog to review the modifications that have been made.

## Prerequisites

- Git is required for checking out, and also building the source code as the version extraction relies on Git.
- A stable or nightly Rust toolchain. For installation instructions [see here](https://rustup.rs/).
- GNU Make (optional). For installation instructions on Windows [see here](#gnu-make-installation-on-microsoft-windows).
- There might be other dependencies per platform or distro that you might be required to install as they might not come pre-installed with your operating system; Please refer to the distribution below in order to get a sense of what might be required.

### Prerequisites installation on FreeBSD

On FreeBSD, the easiest way to install the dependencies would be to use the binary packages via pkgng:

```sh
$ pkg update
$ pkg install curl git gmake
```

Or, alternatively building the packages from source via the Ports system:

```sh
$ cd /usr/ports/ftp/curl
$ make config-recursive
$ make install

$ cd /usr/ports/devel/git
$ make config-recursive
$ make install

$ cd /usr/ports/devel/gmake
$ make config-recursive
$ make install
```

_Note__: <code>ftp/curl</code> is only required for Rust installation using the <code>rustup</code> method, as FreeBSD does not ship with cURL installed into the base system.

### Prerequisites installation on Gentoo Linux

```
$ emerge --sync
$ emerge -atuv dev-vcs/git
```

### Prerequisites installation on AlmaLinux

```
$ dnf check-update
$ dnf install git make pkgconf gcc openssl-devel
```

### Prerequisites installation on CentOS

```
$ dnf check-update
$ dnf install git make pkgconf gcc openssl-devel
```

### Prerequisites installation on Debian GNU/Linux

```
$ apt-get update
$ apt install git make pkg-config build-essential libssl-dev
```

### Prerequisites installation on Rocky Linux

```
$ dnf check-update
$ dnf install git make pkgconf gcc openssl-devel
```

### Prerequisites installation on Ubuntu

```
$ apt-get update
$ apt install git make pkg-config build-essential libssl-dev
```

### GNU Make Installation on Microsoft Windows

1. Download and install [Git for Windows].
2. Obtain [a without-guile binary build of GNU Make for Windows from ezwinports](https://sourceforge.net/projects/ezwinports/files/), e.g. make-4.4.1-without-guile-w32-bin.zip
3. <code>make-*-without-guile-w32-bin.zip/*</code> contents should be extracted and copied to your <code>Git\\mingw64\\</code> directory, e.g. <code>C:\\Program Files\\Git\\mingw64\\</code>, merging the folders, WITHOUT overwriting/replacing any existing files.
4. Reboot
5. Open a Git Bash terminal by right-clicking inside any (preferably empty folder) and issue the <code>make</code> command; after pressing the <code>Enter/Return</code> key you probably see a message like the following which means the installation has been done properly:

```sh
> make

make: *** No targets specified and no makefile found.  Stop.
```

## Checkout

For HTTPS checkouts from GitHub:

```sh
$ git clone https://github.com/NuLL3rr0r/webrtc-leak-demo.git
```

For HTTPS checkouts from GitLab:

```sh
$ git clone https://gitlab.com/NuLL3rr0r/webrtc-leak-demo.git
```

For SSH checkouts from GitHub:

```sh
$ git clone git@github.com:NuLL3rr0r/webrtc-leak-demo.git
```

For SSH checkouts from GitLab:

```sh
$ git clone git@gitlab.com:NuLL3rr0r/webrtc-leak-demo.git
```

## Building

If you have installed GNU Make and would like to build using it, just open a GitBash terminal on Windows, or your favorite terminal in either FreeBSD or Linux, and inside the root of the cloned repository, issue the following command:

```sh
$ make
```

_Note_: The Makefile only supports 64-bit installations of FreeBSD, Linux, and Microsoft Windows as these are the only platforms that I have tested this on.

Regardless of using GNU Make or building on an unsupported platform, you could always invoke the Cargo build tool directly to build the demo:

```sh
$ cargo build --release
```

### Debug Builds

By default invoking the <code>make</code> command triggers the release builds. Nonetheless, one could always specify the value of the <code>WEBRTC_LEAK_DEMO_BUILD_TYPE</code> variable on the command line as either <code>debug</code> or <code>release</code> for the desired build type:

```sh
$ make WEBRTC_LEAK_DEMO_BUILD_TYPE=debug
```

On the contrary, invoking the <code>cargo build</code> command without the <code>--release</code> flag triggers the debug build:

```sh
$ cargo build
```

## Running

To run the server from your terminal window, in case you used the <code>make</code> command:

```sh
$ ./stage/webrtc-leak-demo
```

Otherwise, if you invoked the <code>cargo build --release</code> command directly:

```sh
$ ./target/release/webrtc-leak-demo
```

And, accordingly for the <code>cargo build</code> command without the <code>--release</code> flag:

```sh
$ ./target/debug/webrtc-leak-demo
```

## Usage

To see the usage and the default values:

```sh
$ ./stage/webrtc-leak-demo --help

A WebRTC IP Leak Vulnerability Demonstration

Usage: webrtc-leak-demo [OPTIONS]

Options:
      --host <HOST>                [default: 0.0.0.0]
      --port <PORT>                [default: 9999]
      --stun-server <STUN_SERVER>  [default: stun:stun.l.google.com:19302]
      --geoip2-url <GEOIP2_URL>    [default: https://github.com/P3TERX/GeoLite.mmdb/releases/latest/download/GeoLite2-City.mmdb]
  -h, --help                       Print help
  -V, --version                    Print version
```

## Obtaining a STUN server

By default, we use <code>stun:stun.l.google.com:19302</code> as our STUN server. In case for any reason it happens to be down, you could always obtain an alternative STUN server from the [pradt2/always-online-stun](https://github.com/pradt2/always-online-stun) project and replace it inside the <code>src/main.rs</code> file.
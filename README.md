# A WebRTC IP Leak Vulnerability Demonstration

This is a WebRTC IP leak vulnerability demonstration written in the Rust programming language and JavaScript.

## Table of Contents

- [Legal Disclaimer](#legal-disclaimer)
- [License](#license)
- [Changelog](#changelog)
- [YouTube Tutorials](#youtube-tutorials)
- [UpCloud Promo Code](#upcloud-promo-code)
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

## Legal Disclaimer

```
THE CONTENT HEREIN IS PRESENTED SOLELY FOR EDUCATIONAL AND INFORMATIONAL
PURPOSES, DEVOID OF ANY WARRANTIES, GUARANTEES, OR CONDITIONS. IT MAY NOT BE
ACCURATE, UP-TO-DATE, OR COMPREHENSIVE. ANY UTILIZATION OR RELIANCE ON THE
CONTENT OR MATERIALS PROVIDED, MENTIONED, OR LINKED HERE IS UNDERTAKEN AT YOUR
OWN RISK, AND THE AUTHOR(S) DISCLAIM ANY LIABILITY OR RESPONSIBILITY.

THE CODE WITHIN THIS REPOSITORY IS INTENDED EXCLUSIVELY FOR ACADEMIC AND
EDUCATIONAL PURPOSES. ANY MALICIOUS USE OF THIS SOFTWARE IS THE LEGAL LIABILITY
AND ETHICAL RESPONSIBILITY OF THE END USER. UTILIZING THIS TOOL FOR
UNAUTHORIZED ATTACKS ON TARGETS IS ILLEGAL, AND THE END USER IS REQUIRED TO
ADHERE TO ALL APPLICABLE LOCAL, STATE, AND FEDERAL LAWS. THE AUTHOR(S) BEAR NO
LIABILITY AND DISCLAIM RESPONSIBILITY FOR ANY MISUSE OR DAMAGE RESULTING FROM
THE UTILIZATION OF THIS PROGRAM OR THE PROVIDED CONTENT.

FOR THE DETAILED USAGE LICENSE, KINDLY CONSULT THE ACCOMPANYING LICENSE.
```

```
تمامی محتویات ارائه شده صرفا جنبه آموزشی و اطلاعاتی داشته و فاقد هرگونه ضمانت،
تعهد یا شرایطی از هر نوع می باشد. بایستی توجه نمود که اطلاعات عرضه شده حتی ممکن
است دقیق و یا بروز نباشد. هرگونه اطمینان به و یا استفاده از محتویات یا منابع
منتشر شده در این مخزن با مسئولیت مخاطب بوده و نگارنده یا نگارندگان هیچ گونه
مسئولیتی در مورد عواقب آن را نخواهند پذیرفت.

کد ارائه شده در این مخزن تنها جهت مقاصد تحصیلی و آموزشی می‌باشد و هر گونه
استفاده مخرب از این نرم افزار مسئولیت قانونی و  اخلاقی کاربر استفاده کننده را به
همراه خواهد داشت. استفاده از این ابزار جهت حملات غیرمجاز یا هداف گرفتن سایرین
غیرقانونی است، و کاربر نهایی ملزم به رعایت تمامی قوانین محلی، ایالتی و فدرال
مربوطه است. نگارنده یا نگارندگان هیچ‌گونه مسئولیتی را بر عهده نمی‌گیرند و از
قبول مسئولیت در قبال هرگونه سوءاستفاده یا صدمه ناشی از استفاده از این برنامه
یا محتوای ارائه شده معذور خواهند بود.

جهت اطلاعات بیشتر در مورد مجوز استفاده، لطفا مجوز همراه را مطالعه نمایید.
```

## License

Everything in this repository is licensed under the terms of [the MIT License](LICENSE.md).

## Changelog

[A comprehensive changelog](CHANGELOG.md) is being kept for this project. Prior to upgrading to any newer releases, kindly refer to the changelog to review the modifications that have been made.

## YouTube Tutorials

It's on my TODO list and will be coming soon :)

## UpCloud Promo Code

I use UpCloud as my main cloud VPS provider in my development, testing, and production cycles. You'll receive a bonus worth of [€25 credits on UpCloud by signing up through the promo code A5X7BK](https://upcloud.com/signup/?promo=A5X7BK) which enables you to evaluate and utilize their infrastructure for free for a few months. This also supports my work in a small way, by allowing me to earn some extra credits on UpCloud.

## Prerequisites

- Git is required for checking out, and also building the source code as the version extraction relies on Git.
- A stable or nightly Rust toolchain. For installation instructions on your desired platform follow the instructions on either [rustup.rs](https://rustup.rs/) or [rust-lang.org](https://www.rust-lang.org/tools/install).
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

And, finally install the Rust toolchain:

```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Also, depending on the shell that you're utilizing, after all the installation steps and prior to proceeding further, you might be required to run the `rehash` command in order to update the command search path used by the shell:

```
$ rehash
```

### Prerequisites installation on Gentoo Linux

```
$ emerge --sync
$ emerge -atuv dev-vcs/git
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Prerequisites installation on AlmaLinux

```
$ dnf check-update
$ dnf install git make pkgconf gcc openssl-devel
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Prerequisites installation on CentOS

```
$ dnf check-update
$ dnf install git make pkgconf gcc openssl-devel
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Prerequisites installation on Debian GNU/Linux

```
$ apt-get update
$ apt install git make pkg-config build-essential libssl-dev
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Prerequisites installation on Rocky Linux

```
$ dnf check-update
$ dnf install git make pkgconf gcc openssl-devel
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Prerequisites installation on Ubuntu

```
$ apt-get update
$ apt install git make pkg-config build-essential libssl-dev
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### GNU Make Installation on Microsoft Windows

1. Download and install the Rust toolchain and its dependencies according to the instructions from either the [rustup.rs](https://rustup.rs/) or [rust-lang.org](https://www.rust-lang.org/tools/install) websites.
2. Download and install [Git for Windows](https://gitforwindows.org/).
3. Obtain [a without-guile binary build of GNU Make for Windows from ezwinports](https://sourceforge.net/projects/ezwinports/files/), e.g. make-4.4.1-without-guile-w32-bin.zip
4. `make-*-without-guile-w32-bin.zip/*` contents should be extracted and copied to your `Git\\mingw64\\` directory, e.g. `C:\\Program Files\\Git\\mingw64\\`, merging the folders, WITHOUT overwriting/replacing any existing files.
5. Reboot
6. Open a Git Bash terminal by right-clicking inside any (preferably empty folder) and issue the `make` command; after pressing the `Enter/Return` key you probably see a message like the following which means the installation has been done properly:

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

By default invoking the `make` command triggers the release builds. Nonetheless, one could always specify the value of the `WEBRTC_LEAK_DEMO_BUILD_TYPE` variable on the command line as either `debug` or `release` for the desired build type:

```sh
$ make WEBRTC_LEAK_DEMO_BUILD_TYPE=debug
```

On the contrary, invoking the `cargo build` command without the `--release` flag triggers the debug build:

```sh
$ cargo build
```

## Running

To run the server from your terminal window, in case you used the `make` command:

```sh
$ ./stage/webrtc-leak-demo
```

Otherwise, if you invoked the `cargo build --release` command directly:

```sh
$ ./target/release/webrtc-leak-demo
```

And, accordingly for the `cargo build` command without the `--release` flag:

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

By default, we use `stun:stun.l.google.com:19302` as our STUN server. In case for any reason it happens to be down, you could always obtain an alternative STUN server from the [pradt2/always-online-stun](https://github.com/pradt2/always-online-stun) project and replace it inside the `src/main.rs` file.

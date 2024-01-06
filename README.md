# A WebRTC IP Leak Vulnerability Demonstration

This is a WebRTC IP leak vulnerability demonstration written in the Rust programming language and JavaScript.

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

## Prerequisites

- A stable or nightly Rust toolchain. For installation instructions [see here](https://rustup.rs/).
- GNU Make (optional). For installation instructions on Windows [see here](#gnu-make-installation-on-microsoft-windows) and for FreeBSD [see here](#gnu-make-installation-on-freebsd). 

### GNU Make Installation on FreeBSD

GNU Make on FreeBSD could be install either from source via the Ports system:

```sh
$ /usr/ports/devel/gmake/
$ make config-recursive
$ make install
```

Or as a binary package via pkgng:

```sh
$ pkg install gmake
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

## Debug Builds

By default invoking the <code>make</code> command triggers the release builds. Nonetheless, one could always specify the value of the <code>WEBRTC_LEAK_DEMO_BUILD_TYPE</code> variable on the command line as either <code>debug</code> or <code>release</code> for the desired build type:

```sh
$ make WEBRTC_LEAK_DEMO_BUILD_TYPE=debug
```

On the contrary, invoking the <code>cargo build</code> command without the <code>--release</code> flag triggers the debug builds

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

## License

Everything in this repository is licensed under the terms of the MIT License. Below is the MIT License:

```
(The MIT License)

Copyright (c) 2024 Mamadou Babaei

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
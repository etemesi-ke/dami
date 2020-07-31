# dami 
<table>
 <tr>
  <td>Lines of code</td>
  <td>
   <img src="https://tokei.rs/b1/github/etemesi-ke/dami/"/>
  </td>
  </tr>
 <tr>
   <td>Build Status</td>
  <td>
    <a href="https://travis-ci.com/etemesi-ke/dami">
    <img src="https://travis-ci.com/etemesi-ke/dami.svg?branch=master" />
    </a>
  </td>
<tr>
 <tr>
  <td>Speed</td>
  <td>
  <img src="https://img.shields.io/badge/SUPER-FAST-BLUE.svg"/>
  </td>
 </tr>
</table>
 
## Data Manipulations in Rust

## Building Documentation locally
You need to set `RUSTDOCSFLAGS` EXPLICITLY
```bash
RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --no-deps
```
This only works for `--no-deps` because `katex-header.html` doesn't exist fot dependent crates.

If you with to set `RUSTDOC` flags automatically in this crate you can put this in your  [`.cargo/config.toml`](https://doc.rust-lang.org/cargo/reference/config.html) file

```toml
[build]
rustdocflags = ["--html-in-header", "katex-header.html"]
```

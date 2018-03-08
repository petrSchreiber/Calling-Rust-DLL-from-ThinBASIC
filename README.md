# Rust and ThinBASIC DLL interop
Basic (pun intended) example of interop between ThinBASIC and Rust via DLL to enable the community to create safe, high-performance extensions for our precious interpreter.

The main goal is to provide examples of passing of various ThinBASIC data types to and back from  Rust.

## Let's get started
You will need Rust, to compile the DLL and ThinBASIC to call it.

### Installing Rust
* Download and install the [Build Tools for Visual Studio](https://www.visualstudio.com/cs/downloads/?q=Build+Tools+for+Visual+Studio).
* [Install Rust](https://www.rust-lang.org/en-US/install.html) via Rustup. **Please** customize the installation to ensure 32 bit pipeline currently needed by ThinBASIC:
  * Run the rustup-init.exe
  * Press *2* to alter the default settings
  * For *Default host triple?* enter *i686-pc-windows-msvc*
  * For *Default toolchain?* enter *stable*
  * For *Modify PATH variable?* enter *y*
  * Proceed with installation with *1*

### Installing ThinBASIC
A bit more _basic_ indeed. Just [download the installer](http://www.thinbasic.com/index.php/download/thinbasic-beta-1-10-4-0/) of the bleeding edge version.

### How to make the DLL?
* Checkout this repo and GOTO root directory (where this README is).
* Fire up a console and cast the spell `cargo rustc --release`.
* You will hear a wolf howl and the DLL will appear in `target/release`

### How to test the DLL?
Run the script _test/test_lib.tbasic_.

## IDE recommendations
I strongly recommend [Sublime Text 3](https://www.sublimetext.com/3) for Rust editing. It is very elegant. Just boost it with [rust-enhanced](https://github.com/rust-lang/rust-enhanced) plugin.

For ThinBASIC, grab ThinAIR, which is part of the default installation. Made with love.

<div align="center"><h1>Blokator - System-wide CLI adblocker</h1></div>

[![asciicast](https://asciinema.org/a/649867.svg)](https://asciinema.org/a/649867)

## How it works
Blokator is hosts-based. The hosts file is a way to map hostnames to IP addresses, which acts like local DNS server.

## Features
- Apply
- Revert
- Create a backup of current hosts file
- Restore backup
- Sync
- Add custom repos
- Delete custom repos
- Access custom repos with TOR
- Apply for Android device (experimental, root required)

## Compatibility
Blokator can run basically everywhere. These are the tested platforms:
- Linux (just works)
- BSDs (just works)
- Windows (works, but for normal Windows user it is not user-friendly)

## Supported init systems
- SystemD
- Runit
- OpenRC
- s6

If you have another init system or networking service, you can restart it manually or even reboot the computer.

## Known issues
- Colors don't work on Windows
- OpenRC NetworkManager restart can return exit with status `1` so Blokator thinks that the restart of NetworkManager failed

## Usage
After you installed Blokator, you need to sync the hosts files. To sync the hosts files, run Blokator with **--sync** argument:

```
blokator sync
```

Then you can enable Blokator with **--apply** argument:

```
blokator apply
```

To add your own hosts file, you can add your repo with **--add-repo** argument:

```
blokator add-repo <URL>
```

To delete repo you added, you can run Blokator with **--del-repo** argument:

```
blokator del-repo <URL>
```

To display the help page with all available commands, you can run Blokator with **--help** argument:

```
blokator --help
```

**You will find more usage examples in man page**

## Installation
### Manual / Compilation
Before you begin, make sure you have installed these things:
- Rust
- Cargo
- Make
- OpenSSL
- pkg-config

#### With Cargo
You can install **Blokator** easily with Cargo package manager:
```
cargo install blokator
```

**MAKE SURE THAT `~/.cargo/bin` IS IN YOUR `$PATH`**

#### With Git and Make
First, you need to clone the [git repo](https://codeberg.org/Tomkoid/blokator) to your local machine:

```
git clone https://codeberg.org/tomkoid/blokator.git
```

Now go to the new cloned directory:

```
cd blokator
```

In that directory you can build **Blokator** using **make**:

```
make build
```

And install it to the system:

```
sudo make install
```

**You're done now!**

### From package manager
At this moment, Blokator is available only in the [AUR](https://aur.archlinux.org) for **[Arch Linux](https://archlinux.org)**.

To download Blokator from AUR it's recommended to use [AUR helper](https://wiki.archlinux.org/title/AUR_helpers) to install Blokator.

## License

Copyright 2024 Tomkoid 

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

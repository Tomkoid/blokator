# Blokator
**Simple cross-platform and system-wide CLI adblocker**

## How it works
First, Blokator fetches data from [StevenBlack hosts](https://github.com/StevenBlack/hosts) and stores it locally. After blokator fetched the data, it overwrites the system **/etc/hosts** file. The hosts file redirects every ad URL / website to **0.0.0.0**, which should be empty.

## Installation
### Manual / Compilation
Before we begin, make sure you have installed these things:
- Rust
- Cargo

First, you need to clone the [git repo](https://gitlab.com/Tomkoid/blokator) to your local machine:

```
git clone https://gitlab.com/Tomkoid/blokator.git
```

Now CD into that directory:

```
cd blokator
```

In that directory we can compile Blokator:

```
cargo build --release
```

And install it to the system:

```
sudo cp target/release/blokator /usr/bin/blokator
```

**You're done now!**

### From package manager
At this moment, Blokator is available only in [tomkoid-repo](https://gitlab.com/Tomkoid/tomkoid-repo) for **[Arch Linux](https://archlinux.org)**.

Installation guide for [tomkoid-repo](https://gitlab.com/Tomkoid/tomkoid-repo) is in README.

You can install **Blokator** with pacman:
```
sudo pacman -Sy blokator
```

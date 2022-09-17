# Blokator
**Simple cross-platform and system-wide CLI adblocker**

[![asciicast](https://asciinema.org/a/520298.svg)](https://asciinema.org/a/520298)

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
- BSDs (not that much tested as Linux)
- Windows (not tested much)
- Redox (basically working)

## Supported init systems
- SystemD
- Runit
- OpenRC (restarting NetworkManager issue with OpenRC is in Known Issues category below)

If you have another init system or networking service, you can restart it manually or even reboot the computer.

## Known issues
- Colors don't work in Windows

## Usage
After you installed Blokator, you need to sync the hosts files. To sync the hosts files, run Blokator with **--sync** argument:

```
blokator --sync
```

Then you can enable Blokator with **--apply** argument:

```
blokator --apply
```

To add your own hosts file, you can add your repo with **--add-repo** argument:

```
blokator --add-repo <URL>
```

To delete repo you added, you can run Blokator with **--del-repo** argument:

```
blokator --del-repo <URL>
```

To display the help page with all available commands, you can run Blokator with **--help** argument:

```
blokator --help
```

## Installation
### Manual / Compilation
Before you begin, make sure you have installed these things:
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

In that directory we can compile **Blokator**:

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

Installation guide for [tomkoid-repo](https://gitlab.com/Tomkoid/tomkoid-repo) is in the **README**.

You can install **Blokator** with pacman:
```
sudo pacman -Sy blokator
```

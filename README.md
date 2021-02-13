# Rust KX - II

This session is all about [Unikernels][1] and [rusty-hermit][2].

## Intro

> containers where yesterday, today we'll package our web service as OS and run it directly on a hypervisor

[![arch](https://rust-osdev.com/showcase/rusty-hermit/libos.png)][2]

maybe slides?

## Setup

### 0. Git Submodules!

make sure you have them all updated.

```sh
git submodules init
git submodules update
```

### 0.1 Qemu + deps

We going to use Qemu as hypervisor, let's install it.

> Windows users [might look](https://github.com/hermitcore/rusty-loader/blob/master/.github/workflows/build.yml#L52) up their dependencies, sorry I have not tested it.

#### Linux

```sh
sudo apt-get update --fix-missing 
sudo apt-get install qemu-system-x86 nasm
```

#### MacOS

```sh
brew install qemu nasm
```

### 1. Rust nightly

```sh
rustup default nightly
cargo install cargo-download
rustup component add rust-src llvm-tools-preview
```

### 2. Build the loader

```sh
cd rusty-loader
release=1 make
```

### Linux

### MacOS

## Demo

### Hello World

```sh
cd hello-hermit
cargo run --release
```

What we should get is something like this:

```plain
[LOADER] Loader: [0x100000 - 0x31a018]
[LOADER] Found Multiboot information at 0x9500
[LOADER] Found module: [0x31c000 - 0x5113d0]
[LOADER] Module length: 0x1f53d0
[LOADER] Found an ELF module at 0x31c000
[LOADER] Map 228 pages at 0x31c000 (page size 4 KByte)
[LOADER] Map 1 pages at 0x400000 (page size 2048 KByte)
....
[0][INFO] ===================== MULTIPROCESSOR INFORMATION =====================
[0][INFO] APIC in use:             xAPIC
[0][INFO] Initialized CPUs:        1
[0][INFO] ======================================================================
[0][INFO]
[0][INFO] Compiled with PCI support
[0][INFO] Compiled with ACPI support
[0][INFO] Compiled with SMP support
[0][INFO] HermitCore is running on common system!
Hello World!
[0][INFO] Number of interrupts
[0][INFO] [0][7]: 1
[0][INFO] Shutting down system
```

🚀 Congrats! The first application as OS has been launched

> NOTE: the demo project has a `.cargo/config` file that describes the target architecture, features and the runner, that is `qemu` in this case

### Echo TCP Server

#### Preparing the network on Linux

Setting up the tun network device:

```sh
sudo ip tuntap add tap10 mode tap
sudo ip addr add 10.0.5.1/24 broadcast 10.0.5.255 dev tap10
sudo ip link set dev tap10 up
sudo bash -c 'echo 1 > /proc/sys/net/ipv4/conf/tap10/proxy_arp'
# sudo route add -net 10.0.5.0/24 gw 10.0.5.1
```

> Note: when running qemu must provide the right network device with those options
`qemu-system-x86_64` options:
- `-netdev tap,id=net0,ifname=tap10,script=no,downscript=no,vhost=on`
- `-device virtio-net-pci,netdev=net0,disable-legacy=on`
- `-m 512M` memory needs to be at least `512M`

```sh
cd echo-hermit
cargo run --release
```

> Note: you can control the network properties via env variables
> `HERMIT_VERBOSE=1 HERMIT_IP="10.0.5.3" HERMIT_GATEWAY="10.0.5.2" HERMIT_MASK="255.255.255.0" HERMIT_NETIF=bridge102 cargo run --release`

#### testing

```sh
telnet localhost 8080
hello hermit
hello hermit
```

## Bonus Track AWS

Let's go to the cloud

[1]: http://unikernel.org/
[2]: https://rust-osdev.com/showcase/rusty-hermit/

## References

- [github/hermitcore](https://github.com/hermitcore)
- [hermit/playground](https://raw.githubusercontent.com/hermitcore/hermit-playground/master/README.md)
- [macos/tuntaposx/faq](http://tuntaposx.sourceforge.net/faq.xhtml)
- [macos/qemu-bridging](https://www.dzombak.com/files/qemu-bridging-mavericks.pdf)
- [qemu/networking](https://wiki.qemu.org/Documentation/Networking#Network_Backends)

## Appendix MacOS networking Cheat Sheet

- remove a network bridge interface: `sudo ifconfig <bridge01> destroy`
- create a tun interface: `brew install tuntap && sudo ifconfig tun0 create`
- check route to host: `traceroute 10.0.5.3`
- add a route to one (first) host via (second) gateway: `sudo route add 10.0.5.3 10.0.5.1`
- qemu supported models: `qemu-system-x86_64 -nic model=help`
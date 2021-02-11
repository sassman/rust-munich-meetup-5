## Networking
To enable an Ethernet device for `uhyve`, we have to setup a tap device on the
host system. 

### On Linux
For instance, the following command establishes the tap device
`tap100` on Linux:

```bash
$ sudo ip tuntap add tap100 mode tap
$ sudo ip addr add 10.0.5.1/24 broadcast 10.0.5.255 dev tap100
$ sudo ip link set dev tap100 up
$ sudo bash -c 'echo 1 > /proc/sys/net/ipv4/conf/tap100/proxy_arp'
```

### Uhyve configuration

By default, `uhyve`'s network interface uses `10.0.5.2`as IP address, `10.0.5.1`
for the gateway and `255.255.255.0` as network mask.
The default configuration can be overwritten by the environment variables
`HERMIT_IP`, `HERMIT_GATEWAY` and `HERMIT_MASk`.
To enable the device, `HERMIT_NETIF` must be set to the name of the tap device.
For instance, the following command starts an HermitCore application within `uhyve`
and enables the network support:

```bash
$ HERMIT_IP="10.0.5.3" HERMIT_GATEWAY="10.0.5.1" HERMIT_MASK="255.255.255.0" HERMIT_NETIF=tap100 bin/uhyve x86_64-hermit/extra/tests/hello
```

## References
- [hermit/playground](https://raw.githubusercontent.com/hermitcore/hermit-playground/master/README.md)
- [tuntaposx/faq](http://tuntaposx.sourceforge.net/faq.xhtml)
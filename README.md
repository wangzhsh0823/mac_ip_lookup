# mac_ip_lookup

A simple Rust utility to lookup MAC addresses from IPs and vice‑versa using the local ARP table.

## Features

- Display the entire ARP table (MAC ↔ IP mappings)
- Lookup the IP address for a given MAC address
- Lookup the MAC address for a given IP address
- Small binary size (~358 KB) thanks to size‑optimized Rust compilation
- No external dependencies – pure Rust, uses only the standard library

## Installation

### From source (requires Rust toolchain)

```bash
# Clone the repository (if you haven't already)
git clone https://github.com/wangzhsh0823/mac_ip_lookup.git
cd mac_ip_lookup

# Build with size optimizations
rustc mac_ip_lookup.rs -C opt-level=z -C strip=symbols -o mac_ip_lookup
```

### Using the pre‑built binary

If you prefer not to compile yourself, download the released binary from the **Releases** page and make it executable:

```bash
chmod +x mac_ip_lookup
```

## Usage

Run the binary without arguments to see the full ARP table:

```bash
$ ./mac_ip_lookup
IP address       MAC address
-----------------------------------------
192.168.2.1      00:1c:42:xx:xx:xx
192.168.2.69     00:0c:29:aa:02:43
...
```

### Lookup IP from MAC

```bash
$ ./mac_ip_lookup 00:0c:29:aa:02:43
MAC: 00:0c:29:aa:02:43 -> IP: 192.168.2.69
```

### Lookup MAC from IP

```bash
$ ./mac_ip_lookup 192.168.2.69
IP: 192.168.2.69 -> MAC: 00:0c:29:aa:02:43
```

If the requested address is not present in the ARP table, the program will report “Not found”.

## How it works

The tool reads the system's ARP cache (via `/proc/net/arp` on Linux) and builds a hash map for quick lookups. On other Unix‑like systems you can adapt the source to read the appropriate ARP source (e.g., using `sysctl net.link.ether.inet.arp` on BSD/macOS).

## License

This project is licensed under the MIT License – see the [LICENSE](LICENSE) file for details.


use std::env;
use std::fs;
use std::io;
use std::process;

const MAP_FILE: &str = "/tmp/host_map.txt";
const ARP_FILE: &str = "/proc/net/arp";

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            // No arguments: show map file content
            if let Ok(content) = fs::read_to_string(MAP_FILE) {
                // Ensure a newline at end so next prompt appears on new line
                if content.ends_with('\n') {
                    print!("{}", content);
                } else {
                    print!("{}\n", content);
                }
            } else {
                // File missing: build then show
                if let Err(e) = build_mapping() {
                    eprintln!("构建映射失败: {}", e);
                    process::exit(1);
                }
                if let Ok(content) = fs::read_to_string(MAP_FILE) {
                    if content.ends_with('\n') {
                        print!("{}", content);
                    } else {
                        print!("{}\n", content);
                    }
                } else {
                    eprintln!("映射文件仍为空");
                    process::exit(1);
                }
            }
        }
        2 => {
            let query = &args[1];
            if is_ipv4(query) {
                // Query is IP -> get MAC
                match lookup_mac(query) {
                    Some(mac) => {
                        println!("{}", mac);
                    }
                    None => {
                        // Not found in map; try to get from ARP and update map
                        if let Some(mac) = get_mac_from_arp(query) {
                            // Update map with this entry
                            let _ = add_to_map(query, &mac);
                            println!("{}", mac);
                        } else {
                            eprintln!("未找到对应 MAC");
                            process::exit(1);
                        }
                    }
                }
            } else {
                // Query is MAC -> get IP
                match lookup_ip(query) {
                    Some(ip) => {
                        println!("{}", ip);
                    }
                    None => {
                        // Not found in map; try to get from ARP and update map
                        if let Some(ip) = get_ip_from_arp(query) {
                            let _ = add_to_map(&ip, query);
                            println!("{}", ip);
                        } else {
                            eprintln!("未找到对应 IP");
                            process::exit(1);
                        }
                    }
                }
            }
        }
        _ => {
            eprintln!(
                "用法:\n  {}           – 显示 /tmp/host_map.txt\n  {} <MAC>     – 查询 MAC 对应的 IP\n  {} <IP>      – 查询 IP 对应的 MAC",
                args[0], args[0], args[0]
            );
            process::exit(1);
        }
    }
}

/// Build/refresh the whole map from /proc/net/arp
fn build_mapping() -> io::Result<()> {
    let content = fs::read_to_string(ARP_FILE)?;
    let mut lines = Vec::new();
    for line in content.lines().skip(1) {
        let mut parts = line.split_whitespace();
        let ip_str = parts.next().unwrap_or("");
        let _hwtype = parts.next();
        let _flags = parts.next();
        let mac_str = parts.next().unwrap_or("");
        let _mask = parts.next();
        let _dev = parts.next();

        if !ip_str.is_empty()
            && !mac_str.is_empty()
            && mac_str != "00:00:00:00:00:00"
        {
            lines.push(format!(
                "{} {}",
                ip_str,
                mac_str.to_ascii_lowercase()
            ));
        }
    }
    // Ensure trailing newline
    let out = lines.join("\n") + "\n";
    fs::write(MAP_FILE, out)
}

/// Lookup IP by MAC in the map file
fn lookup_ip(target_mac: &str) -> Option<String> {
    if let Ok(content) = fs::read_to_string(MAP_FILE) {
        for line in content.lines() {
            let mut parts = line.split_whitespace();
            let ip = parts.next()?;
            let mac = parts.next()?;
            if mac.eq_ignore_ascii_case(target_mac) {
                return Some(ip.to_string());
            }
        }
    }
    None
}

/// Lookup MAC by IP in the map file
fn lookup_mac(target_ip: &str) -> Option<String> {
    if let Ok(content) = fs::read_to_string(MAP_FILE) {
        for line in content.lines() {
            let mut parts = line.split_whitespace();
            let ip = parts.next()?;
            let mac = parts.next()?;
            if ip.eq_ignore_ascii_case(target_ip) {
                return Some(mac.to_string());
            }
        }
    }
    None
}

/// Get MAC for a given IP by reading ARP directly
fn get_mac_from_arp(target_ip: &str) -> Option<String> {
    if let Ok(content) = fs::read_to_string(ARP_FILE) {
        for line in content.lines().skip(1) {
            let mut parts = line.split_whitespace();
            let ip_str = parts.next()?;
            let _hwtype = parts.next();
            let _flags = parts.next();
            let mac_str = parts.next()?;
            let _mask = parts.next();
            let _dev = parts.next();
            if ip_str.eq_ignore_ascii_case(target_ip) && mac_str != "00:00:00:00:00:00" {
                return Some(mac_str.to_string());
            }
        }
    }
    None
}

/// Get IP for a given MAC by reading ARP directly
fn get_ip_from_arp(target_mac: &str) -> Option<String> {
    if let Ok(content) = fs::read_to_string(ARP_FILE) {
        for line in content.lines().skip(1) {
            let mut parts = line.split_whitespace();
            let ip_str = parts.next()?;
            let _hwtype = parts.next();
            let _flags = parts.next();
            let mac_str = parts.next()?;
            let _mask = parts.next();
            let _dev = parts.next();
            if mac_str.eq_ignore_ascii_case(target_mac) && mac_str != "00:00:00:00:00:00" {
                return Some(ip_str.to_string());
            }
        }
    }
    None
}

/// Add a single IP-MAC entry to the map file (rebuild map for simplicity)
fn add_to_map(_ip: &str, _mac: &str) -> io::Result<()> {
    // Simpler: just rebuild the whole map (fast enough for small arp table)
    let _ = build_mapping();
    Ok(())
}

/// Simple IPv4 check
fn is_ipv4(s: &str) -> bool {
    let mut octets = s.split('.');
    octets.clone().count() == 4 && octets.all(|o| o.parse::<u8>().is_ok())
}

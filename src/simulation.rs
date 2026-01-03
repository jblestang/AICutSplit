use crate::packet::{FiveTuple, PROTO_IGMP, PROTO_TCP, PROTO_UDP};
use crate::rule::{Action, Range, Rule};
use alloc::vec::Vec;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;

pub struct Simulation {
    rng: Pcg32,
}

impl Simulation {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: Pcg32::seed_from_u64(seed),
        }
    }

    pub fn generate_rules(&mut self, n_rules: usize) -> Vec<Rule> {
        let mut rules = Vec::with_capacity(n_rules);

        // Simulate LAN (192.168.0.0/16) and WAN (Random)
        // Rule Types:
        // 1. LAN -> WAN (TCP/UDP) - Allow specific services
        // 2. WAN -> LAN (TCP/UDP) - Allow specific servers
        // 3. IGMP Multicast

        for i in 0..n_rules {
            let priority = i as u32;
            let action = if self.rng.gen_bool(0.8) {
                Action::Permit
            } else {
                Action::Deny
            }; // Mostly allow specific flows, default deny at end

            let rule = match self.rng.gen_range(0..10) {
                0..=5 => self.gen_lan_to_wan_rule(priority, action),
                6..=8 => self.gen_wan_to_lan_rule(priority, action),
                _ => self.gen_igmp_rule(priority, action),
            };
            rules.push(rule);
        }

        // Add default deny (though linear classifier handles implicit default, convenient for tree)
        rules.push(Rule {
            id: n_rules as u32,
            priority: n_rules as u32,
            src_ip: Range::any(0, u32::MAX),
            dst_ip: Range::any(0, u32::MAX),
            src_port: Range::any(0, 65535),
            dst_port: Range::any(0, 65535),
            proto: Range::any(0, 255),
            action: Action::Deny,
        });

        rules
    }

    fn gen_lan_to_wan_rule(&mut self, id: u32, action: Action) -> Rule {
        // Source: 192.168.x.x
        let src_ip_base = 0xC0A80000; // 192.168.0.0
        let src_ip_mask = self.rng.gen_range(16..32);
        let src_ip_suffix = self.rng.gen::<u32>() & ((1 << (32 - src_ip_mask)) - 1);
        let src_start = src_ip_base | src_ip_suffix;
        let src_end = src_start + self.rng.gen_range(0..255); // Small range

        // Dst: Random generic
        let dst_ip = self.rng.gen::<u32>();

        Rule {
            id,
            priority: id,
            src_ip: Range::new(src_start, src_end),
            dst_ip: Range::new(dst_ip, dst_ip + 100),
            src_port: Range::any(1024, 65535),
            dst_port: Range::exact(self.gen_service_port()),
            proto: Range::exact(if self.rng.gen() { PROTO_TCP } else { PROTO_UDP }),
            action,
        }
    }

    fn gen_wan_to_lan_rule(&mut self, id: u32, action: Action) -> Rule {
        let src_ip = self.rng.gen::<u32>();
        let dst_ip_base = 0xC0A80000;
        let dst_addr = dst_ip_base | (self.rng.gen::<u32>() & 0xFFFF);

        Rule {
            id,
            priority: id,
            src_ip: Range::new(src_ip, src_ip + 50),
            dst_ip: Range::exact(dst_addr),
            src_port: Range::any(0, 65535),
            dst_port: Range::exact(80), // Web server in LAN
            proto: Range::exact(PROTO_TCP),
            action,
        }
    }

    fn gen_igmp_rule(&mut self, id: u32, action: Action) -> Rule {
        Rule {
            id,
            priority: id,
            src_ip: Range::any(0, u32::MAX),
            dst_ip: Range::new(0xE0000000, 0xEFFFFFFF), // Multicast range 224.0.0.0/4
            src_port: Range::any(0, 65535),
            dst_port: Range::any(0, 65535),
            proto: Range::exact(PROTO_IGMP),
            action,
        }
    }

    fn gen_service_port(&mut self) -> u16 {
        match self.rng.gen_range(0..4) {
            0 => 80,
            1 => 443,
            2 => 53,
            _ => 8080,
        }
    }

    pub fn generate_packets(&mut self, n_packets: usize) -> Vec<FiveTuple> {
        let mut packets = Vec::with_capacity(n_packets);
        for _ in 0..n_packets {
            // Skew towards matching something (LAN or WAN IPs)
            let src_ip = if self.rng.gen_bool(0.5) {
                0xC0A80000 | (self.rng.gen::<u32>() & 0xFFFF)
            } else {
                self.rng.gen()
            };
            let dst_ip = if self.rng.gen_bool(0.5) {
                0xC0A80000 | (self.rng.gen::<u32>() & 0xFFFF)
            } else {
                self.rng.gen()
            };

            packets.push(FiveTuple {
                src_ip,
                dst_ip,
                src_port: self.rng.gen(),
                dst_port: self.rng.gen(),
                proto: if self.rng.gen_bool(0.1) {
                    PROTO_IGMP
                } else if self.rng.gen() {
                    PROTO_TCP
                } else {
                    PROTO_UDP
                },
            });
        }
        packets
    }
}

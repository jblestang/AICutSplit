

/// 5-tuple representation for classification.
///
/// This structure holds the key fields used for packet classification:
/// - Source IP Address
/// - Destination IP Address
/// - Source Port (L4)
/// - Destination Port (L4)
/// - IP Protocol (TCP, UDP, IGMP, etc.)
///
/// It is derived from the headers of the parsed packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FiveTuple {
    /// Source IP address (big-endian/network byte order usually, but here u32 host order assumed for sim)
    pub src_ip: u32,
    /// Destination IP address
    pub dst_ip: u32,
    /// Source L4 Port (0 if not applicable)
    pub src_port: u16,
    /// Destination L4 Port (0 if not applicable)
    pub dst_port: u16,
    /// IP Protocol Number (e.g. 6 for TCP, 17 for UDP)
    pub proto: u8,
}

/// IPv4 Header structure (simplified for simulation).
///
/// Contains the basic IP fields. In a real no_std environment,
/// this would be parsed from raw bytes.
#[derive(Debug, Clone, Copy, Default)]
pub struct Ipv4Header {
    /// Source IP Address
    pub src: u32,
    /// Destination IP Address
    pub dst: u32,
    /// Protocol Number (defines the L4 header type)
    pub proto: u8,
    // Add other fields if necessary for "completeness" simulation
    /// IP Version (implied 4)
    pub version: u8,
    /// Internet Header Length (IHL)
    pub ihl: u8,
    /// Time To Live (TTL)
    pub ttl: u8,
}

/// TCP Header
#[derive(Debug, Clone, Copy, Default)]
pub struct TcpHeader {
    pub src_port: u16,
    pub dst_port: u16,
    pub sequence: u32,
    pub ack: u32,
    pub flags: u8,
}

/// UDP Header.
///
/// User Datagram Protocol header. Simple and stateless.
#[derive(Debug, Clone, Copy, Default)]
pub struct UdpHeader {
    /// Source Port
    pub src_port: u16,
    /// Destination Port
    pub dst_port: u16,
    /// Length of the packet
    pub length: u16,
}

/// IGMP Header.
///
/// Internet Group Management Protocol, used for multicast correctness.
/// This implementation simulates basic IGMP fields.
#[derive(Debug, Clone, Copy, Default)]
pub struct IgmpHeader {
    /// IGMP Type (Query, Report, Leave)
    pub igmp_type: u8,
    /// Max Response Time (for queries)
    pub max_resp_time: u8,
    /// Header Checksum
    pub checksum: u16,
    /// Multicast Group Address
    pub group_addr: u32,
}

/// Abstract Packet wrapper.
///
/// Represents a fully parsed packet with IP and Layer 4 headers.
/// In a real system, this might be a pointer to a buffer, but here
/// we store the extracted headers for simulation.
#[derive(Debug, Clone)]
pub struct Packet {
    /// IPv4 Header
    pub ip: Ipv4Header,
    /// Layer 4 Header (TCP, UDP, IGMP, or Unknown)
    pub l4: L4Header,
}

#[derive(Debug, Clone, Copy)]
pub enum L4Header {
    Tcp(TcpHeader),
    Udp(UdpHeader),
    Igmp(IgmpHeader),
    Unknown,
}

impl Default for L4Header {
    fn default() -> Self {
        L4Header::Unknown
    }
}

impl Packet {
    /// Extract the 5-tuple from the packet
    pub fn to_5tuple(&self) -> FiveTuple {
        let (src_port, dst_port) = match self.l4 {
            L4Header::Tcp(h) => (h.src_port, h.dst_port),
            L4Header::Udp(h) => (h.src_port, h.dst_port),
            _ => (0, 0),
        };
        
        FiveTuple {
            src_ip: self.ip.src,
            dst_ip: self.ip.dst,
            proto: self.ip.proto,
            src_port,
            dst_port,
        }
    }
}

pub const PROTO_TCP: u8 = 6;
pub const PROTO_UDP: u8 = 17;
pub const PROTO_IGMP: u8 = 2;
pub const PROTO_ICMP: u8 = 1;

#![no_std]

use core::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut, Index, IndexMut},
};

use scroll::{
    ctx::{SizeWith, TryFromCtx, TryIntoCtx},
    Pread, Pwrite,
};

/// The broadcast address.
pub const BROADCAST: MACAddress = MACAddress::new([0xff; 6]);
/// An empty address.
pub const ZERO: MACAddress = MACAddress::new([0x00; 6]);

#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
/// A MAC address.
pub struct MACAddress(pub [u8; 6]);
impl MACAddress {
    /// Checks the unicast/multicast bit of the first octet.
    pub const fn is_multicast(&self) -> bool {
        self.0[0] & 0x1 != 0x0
    }
    /// Checks the global/local bit of the first octet.
    pub const fn is_local(&self) -> bool {
        self.0[0] & 0x2 != 0x0
    }
    pub const fn new(address: [u8; 6]) -> Self {
        Self(address)
    }
}
impl From<[u8; 6]> for MACAddress {
    fn from(value: [u8; 6]) -> Self {
        Self(value)
    }
}
impl From<MACAddress> for [u8; 6] {
    fn from(value: MACAddress) -> Self {
        *value.deref()
    }
}
impl SizeWith for MACAddress {
    fn size_with(_ctx: &()) -> usize {
        6
    }
}
impl TryFromCtx<'_> for MACAddress {
    type Error = scroll::Error;
    #[inline]
    fn try_from_ctx(from: &'_ [u8], _ctx: ()) -> Result<(Self, usize), Self::Error> {
        Ok((MACAddress::new(from.pread(0)?), 6))
    }
}
impl TryIntoCtx for MACAddress {
    type Error = scroll::Error;
    #[inline]
    fn try_into_ctx(self, buf: &mut [u8], _ctx: ()) -> Result<usize, Self::Error> {
        buf.pwrite(self.as_slice(), 0)
    }
}
impl Deref for MACAddress {
    type Target = [u8; 6];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for MACAddress {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Index<usize> for MACAddress {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl IndexMut<usize> for MACAddress {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
impl Debug for MACAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as core::fmt::Display>::fmt(self, f)
    }
}
impl Display for MACAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self[0], self[1], self[2], self[3], self[4], self[5]
        ))
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MACAddress {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self[0],
            self[1],
            self[2],
            self[3],
            self[4],
            self[5]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_MAC: MACAddress = MACAddress::new([0x11, 0x22, 0x33, 0x44, 0x55, 0x66]);

    #[test]
    fn test_read() {
        assert_eq!(TEST_MAC, TEST_MAC.0.pread(0).unwrap());
    }
    #[test]
    fn test_write() {
        let mut data = [0u8; 6];
        data.pwrite(TEST_MAC, 0).unwrap();
        assert_eq!(TEST_MAC.0, data);
    }

    #[test]
    fn test_display() {
        use core::fmt::Write;

        let mut s = heapless::String::<17>::new();
        write!(&mut s, "{TEST_MAC}").unwrap();
        assert_eq!(s, "11:22:33:44:55:66");
    }
}

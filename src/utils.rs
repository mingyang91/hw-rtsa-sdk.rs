
pub trait ToFixedBytes {
    fn to_fixed_bytes<const N: usize>(self) -> Option<[u8; N]>;
}

impl ToFixedBytes for String {
    fn to_fixed_bytes<const N: usize>(self) -> Option<[u8; N]> {
        if self.len() > N {
            return None;
        }
        // Determine the number of bytes to copy
        let copy_len = self.len().min(N);

        let mut target = [0u8; N];
        // Copy bytes from the source string to the target array
        target[..copy_len].copy_from_slice(&self.as_bytes()[..copy_len]);
        Some(target)
    }
}

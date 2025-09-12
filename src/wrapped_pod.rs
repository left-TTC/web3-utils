


pub trait WrappedPod<'a>: Sized {
    fn export(&self, buffer: &mut Vec<u8>);
    fn size(&self) -> usize;
    fn from_bytes(buffer: &'a [u8]) -> Self;
    #[allow(unused_variables)]
    fn try_from_bytes(buffer: &'a [u8]) -> Result<Self, std::io::Error> {
        // Default implementation for backward comp
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "Not implemented yet",
        ))
    }
}

pub trait WrappedPodMut<'a>: Sized {
    fn export(&self, buffer: &mut Vec<u8>);
    fn size(&self) -> usize;
    fn from_bytes(buffer: &'a mut [u8]) -> Self;
    #[allow(unused_variables)]
    fn try_from_bytes(buffer: &'a mut [u8]) -> Result<Self, std::io::Error> {
        // Default implementation for backward comp
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "Not implemented yet",
        ))
    }
}
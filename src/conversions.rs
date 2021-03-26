pub trait Truncate<T> {
    fn truncate(&self) -> T;
}

impl Truncate<u8> for usize {
    fn truncate(&self) -> u8 {
        *self as u8
    }
}

impl Truncate<u16> for usize {
    fn truncate(&self) -> u16 {
        *self as u16
    }
}

impl Truncate<u32> for usize {
    fn truncate(&self) -> u32 {
        *self as u32
    }
}

impl Truncate<u8> for u32 {
    fn truncate(&self) -> u8 {
        *self as u8
    }
}

impl Truncate<u16> for u32 {
    fn truncate(&self) -> u16 {
        *self as u16
    }
}

impl Truncate<u32> for u32 {
    fn truncate(&self) -> u32 {
        *self
    }
}

impl Truncate<u8> for u16 {
    fn truncate(&self) -> u8 {
        *self as u8
    }
}

impl Truncate<u16> for u16 {
    fn truncate(&self) -> u16 {
        *self as u16
    }
}

impl Truncate<u32> for u16 {
    fn truncate(&self) -> u32 {
        *self as u32
    }
}

impl Truncate<u8> for u8 {
    fn truncate(&self) -> u8 {
        *self
    }
}

impl Truncate<u16> for u8 {
    fn truncate(&self) -> u16 {
        *self as u16
    }
}

impl Truncate<u32> for u8 {
    fn truncate(&self) -> u32 {
        *self as u32
    }
}

impl Truncate<u8> for i32 {
    fn truncate(&self) -> u8 {
        *self as u8
    }
}

impl Truncate<u16> for i32 {
    fn truncate(&self) -> u16 {
        *self as u16
    }
}

impl Truncate<u32> for i32 {
    fn truncate(&self) -> u32 {
        *self as u32
    }
}

impl Truncate<u8> for i16 {
    fn truncate(&self) -> u8 {
        *self as u8
    }
}

impl Truncate<u16> for i16 {
    fn truncate(&self) -> u16 {
        *self as u16
    }
}

impl Truncate<u32> for i16 {
    fn truncate(&self) -> u32 {
        *self as u32
    }
}

impl Truncate<u8> for i8 {
    fn truncate(&self) -> u8 {
        *self as u8
    }
}

impl Truncate<u16> for i8 {
    fn truncate(&self) -> u16 {
        *self as u16
    }
}

impl Truncate<u32> for i8 {
    fn truncate(&self) -> u32 {
        *self as u32
    }
}

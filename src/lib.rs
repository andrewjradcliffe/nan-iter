use std::mem;

pub struct QuietNanIter {
    state: u64,
}
impl QuietNanIter {
    pub fn new() -> Self {
        QuietNanIter {
            state: 0x7ff8000000000000_u64,
        }
    }
}
impl Iterator for QuietNanIter {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.state <= 0x7fffffffffffffff {
            let current = unsafe { mem::transmute::<u64, f64>(self.state) };
            self.state += 1;
            Some(current)
        } else {
            None
        }
    }
}

pub struct SignalingNanIter {
    state: u64,
}
impl SignalingNanIter {
    pub fn new() -> Self {
        SignalingNanIter {
            state: 0x7ff0000000000001_u64,
        }
    }
}
impl Iterator for SignalingNanIter {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.state <= 0x7ff7ffffffffffff {
            let current = unsafe { mem::transmute::<u64, f64>(self.state) };
            self.state += 1;
            Some(current)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quiet_works() {
        let mut q = QuietNanIter::new();

        assert_eq!(
            unsafe { mem::transmute::<f64, u64>(q.nth(10).unwrap()) },
            0x7ff800000000000a
        );

        let mut q = QuietNanIter::new();

        assert_eq!(
            unsafe { mem::transmute::<f64, u64>(q.nth(0).unwrap()) },
            0x7ff8000000000000
        );
        assert_eq!(
            unsafe { mem::transmute::<f64, u64>(q.nth(10).unwrap()) },
            0x7ff800000000000b
        );
    }

    #[test]
    fn signaling_works() {
        let mut s = SignalingNanIter::new();

        assert_eq!(
            unsafe { mem::transmute::<f64, u64>(s.nth(10).unwrap()) },
            0x7ff000000000000b
        );

        let mut s = SignalingNanIter::new();
        assert_eq!(
            unsafe { mem::transmute::<f64, u64>(s.nth(0).unwrap()) },
            0x7ff0000000000001
        );
        assert_eq!(
            unsafe { mem::transmute::<f64, u64>(s.nth(10).unwrap()) },
            0x7ff000000000000c
        );
    }
}

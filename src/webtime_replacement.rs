#![allow(missing_docs, dead_code)]
#[cfg(all(target_family = "wasm", target_os = "unknown"))]
mod time {
    mod instant {
        use std::ops::{Add, AddAssign, Sub, SubAssign};
        use std::time::Duration;
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct Instant(Duration);
        impl Instant {
            pub fn now() -> Self {
                Self(Duration::from_secs_f64(miniquad::date::now()))
            }
            pub fn duration_since(&self, earlier: Self) -> Duration {
                self.checked_duration_since(earlier).unwrap_or_default()
            }
            pub fn checked_duration_since(&self, earlier: Self) -> Option<Duration> {
                self.0.checked_sub(earlier.0)
            }
            pub fn saturating_duration_since(&self, earlier: Self) -> Duration {
                self.checked_duration_since(earlier).unwrap_or_default()
            }
            pub fn elapsed(&self) -> Duration {
                Self::now() - *self
            }
            pub fn checked_add(&self, duration: Duration) -> Option<Self> {
                self.0.checked_add(duration).map(Instant)
            }
            pub fn checked_sub(&self, duration: Duration) -> Option<Self> {
                self.0.checked_sub(duration).map(Instant)
            }
        }
        impl Add<Duration> for Instant {
            type Output = Self;
            fn add(self, other: Duration) -> Self {
                self.checked_add(other)
                    .expect("overflow when adding duration to instant")
            }
        }
        impl AddAssign<Duration> for Instant {
            fn add_assign(&mut self, other: Duration) {
                *self = *self + other;
            }
        }
        impl Sub<Duration> for Instant {
            type Output = Self;
            fn sub(self, other: Duration) -> Self {
                self.checked_sub(other)
                    .expect("overflow when subtracting duration from instant")
            }
        }
        impl Sub<Self> for Instant {
            type Output = Duration;
            fn sub(self, other: Self) -> Duration {
                self.duration_since(other)
            }
        }
        impl SubAssign<Duration> for Instant {
            fn sub_assign(&mut self, other: Duration) {
                *self = *self - other;
            }
        }
    }
    mod system_time {
        use std::error::Error;
        use std::fmt::{self, Display, Formatter};
        use std::ops::{Add, AddAssign, Sub, SubAssign};
        use std::time::Duration;
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct SystemTime(pub(crate) Duration);
        impl SystemTime {
            pub const UNIX_EPOCH: Self = Self(Duration::ZERO);
            pub fn now() -> Self {
                Self(Duration::from_secs_f64(miniquad::date::now()))
            }
            pub fn duration_since(&self, earlier: Self) -> Result<Duration, SystemTimeError> {
                if self.0 < earlier.0 {
                    Err(SystemTimeError(earlier.0 - self.0))
                } else {
                    Ok(self.0 - earlier.0)
                }
            }
            pub fn elapsed(&self) -> Result<Duration, SystemTimeError> {
                Self::now().duration_since(*self)
            }
            pub fn checked_add(&self, duration: Duration) -> Option<Self> {
                self.0.checked_add(duration).map(SystemTime)
            }
            pub fn checked_sub(&self, duration: Duration) -> Option<Self> {
                self.0.checked_sub(duration).map(SystemTime)
            }
        }
        impl Add<Duration> for SystemTime {
            type Output = Self;
            fn add(self, dur: Duration) -> Self {
                self.checked_add(dur)
                    .expect("overflow when adding duration to instant")
            }
        }
        impl AddAssign<Duration> for SystemTime {
            fn add_assign(&mut self, other: Duration) {
                *self = *self + other;
            }
        }
        impl Sub<Duration> for SystemTime {
            type Output = Self;
            fn sub(self, dur: Duration) -> Self {
                self.checked_sub(dur)
                    .expect("overflow when subtracting duration from instant")
            }
        }
        impl SubAssign<Duration> for SystemTime {
            fn sub_assign(&mut self, other: Duration) {
                *self = *self - other;
            }
        }
        #[derive(Clone, Debug)]
        pub struct SystemTimeError(Duration);
        impl SystemTimeError {
            pub fn duration(&self) -> Duration {
                self.0
            }
        }
        impl Display for SystemTimeError {
            fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
                write!(formatter, "second time provided was later than self")
            }
        }
        impl Error for SystemTimeError {}
    }
    pub use std::time::*;

    pub use self::instant::Instant;
    pub use self::system_time::{SystemTime, SystemTimeError};
    pub const UNIX_EPOCH: SystemTime = SystemTime::UNIX_EPOCH;
}

#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
pub use std::time::*;

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
pub use self::time::*;

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
pub mod web {
    use super::SystemTime;
    use std::time::SystemTime as StdSystemTime;
    pub trait SystemTimeExt {
        fn to_std(self) -> std::time::SystemTime;
        fn from_std(time: std::time::SystemTime) -> SystemTime;
    }
    impl SystemTimeExt for SystemTime {
        fn to_std(self) -> std::time::SystemTime {
            StdSystemTime::UNIX_EPOCH + self.0
        }
        fn from_std(time: std::time::SystemTime) -> SystemTime {
            Self::UNIX_EPOCH
                + time
                    .duration_since(StdSystemTime::UNIX_EPOCH)
                    .expect("found `SystemTime` earlier then unix epoch")
        }
    }
}

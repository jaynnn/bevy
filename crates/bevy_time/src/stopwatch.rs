use bevy_reflect::{prelude::*, Reflect};
use bevy_utils::Duration;

/// 秒表是一个在启动时跟踪经过时间的结构。
///
/// # 例子
///
/// ```
/// # use bevy_time::*;
/// use std::time::Duration;
/// let mut stopwatch = Stopwatch::new();
/// assert_eq!(stopwatch.elapsed_secs(), 0.0);
///
/// stopwatch.tick(Duration::from_secs_f32(1.0)); // tick one second
/// assert_eq!(stopwatch.elapsed_secs(), 1.0);
///
/// stopwatch.pause();
/// stopwatch.tick(Duration::from_secs_f32(1.0)); // paused stopwatches don't tick
/// assert_eq!(stopwatch.elapsed_secs(), 1.0);
///
/// stopwatch.reset(); // reset the stopwatch
/// assert!(stopwatch.paused());
/// assert_eq!(stopwatch.elapsed_secs(), 0.0);
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq, Reflect)]
#[cfg_attr(feature = "serialize", derive(serde::Deserialize, serde::Serialize))]
#[reflect(Default)]
pub struct Stopwatch {
    elapsed: Duration,
    paused: bool,
}

impl Stopwatch {
    /// 创建一个新的未暂停的“秒表”，没有经过时间。
    ///
    /// # 例子
    /// ```
    /// # use bevy_time::*;
    /// let stopwatch = Stopwatch::new();
    /// assert_eq!(stopwatch.elapsed_secs(), 0.0);
    /// assert_eq!(stopwatch.paused(), false);
    /// ```
    pub fn new() -> Self {
        Default::default()
    }

    /// 返回秒表自上次 [`reset`](Stopwatch::reset) 以来经过的时间
    ///
    /// # 例子
    /// ```
    /// # use bevy_time::*;
    /// use std::time::Duration;
    /// let mut stopwatch = Stopwatch::new();
    /// stopwatch.tick(Duration::from_secs(1));
    /// assert_eq!(stopwatch.elapsed(), Duration::from_secs(1));
    /// ```
    ///
    /// # 另请参阅
    ///
    /// [`elapsed_secs`](Stopwatch::elapsed_secs) - if an `f32` value is desirable instead.
    /// [`elapsed_secs_f64`](Stopwatch::elapsed_secs_f64) - if an `f64` is desirable instead.
    #[inline]
    pub fn elapsed(&self) -> Duration {
        self.elapsed
    }

    /// Returns the elapsed time since the last [`reset`](Stopwatch::reset)
    /// of the stopwatch, in seconds.
    ///
    /// # Examples
    /// ```
    /// # use bevy_time::*;
    /// use std::time::Duration;
    /// let mut stopwatch = Stopwatch::new();
    /// stopwatch.tick(Duration::from_secs(1));
    /// assert_eq!(stopwatch.elapsed_secs(), 1.0);
    /// ```
    ///
    /// # 另请参阅
    ///
    /// [`elapsed`](Stopwatch::elapsed) - if a `Duration` is desirable instead.
    /// [`elapsed_secs_f64`](Stopwatch::elapsed_secs_f64) - if an `f64` is desirable instead.
    #[inline]
    pub fn elapsed_secs(&self) -> f32 {
        self.elapsed().as_secs_f32()
    }

    /// Returns the elapsed time since the last [`reset`](Stopwatch::reset)
    /// of the stopwatch, in seconds, as f64.
    ///
    /// # 另请参阅
    ///
    /// [`elapsed`](Stopwatch::elapsed) - if a `Duration` is desirable instead.
    /// [`elapsed_secs`](Stopwatch::elapsed_secs) - if an `f32` is desirable instead.
    #[inline]
    pub fn elapsed_secs_f64(&self) -> f64 {
        self.elapsed().as_secs_f64()
    }

    /// Sets the elapsed time of the stopwatch.
    ///
    /// # Examples
    /// ```
    /// # use bevy_time::*;
    /// use std::time::Duration;
    /// let mut stopwatch = Stopwatch::new();
    /// stopwatch.set_elapsed(Duration::from_secs_f32(1.0));
    /// assert_eq!(stopwatch.elapsed_secs(), 1.0);
    /// ```
    #[inline]
    pub fn set_elapsed(&mut self, time: Duration) {
        self.elapsed = time;
    }

    /// Advance the stopwatch by `delta` seconds.
    /// If the stopwatch is paused, ticking will not have any effect
    /// on elapsed time.
    ///
    /// # Examples
    /// ```
    /// # use bevy_time::*;
    /// use std::time::Duration;
    /// let mut stopwatch = Stopwatch::new();
    /// stopwatch.tick(Duration::from_secs_f32(1.5));
    /// assert_eq!(stopwatch.elapsed_secs(), 1.5);
    /// ```
    pub fn tick(&mut self, delta: Duration) -> &Self {
        if !self.paused() {
            self.elapsed += delta;
        }
        self
    }

    /// Pauses the stopwatch. Any call to [`tick`](Stopwatch::tick) while
    /// paused will not have any effect on the elapsed time.
    ///
    /// # Examples
    /// ```
    /// # use bevy_time::*;
    /// use std::time::Duration;
    /// let mut stopwatch = Stopwatch::new();
    /// stopwatch.pause();
    /// stopwatch.tick(Duration::from_secs_f32(1.5));
    /// assert!(stopwatch.paused());
    /// assert_eq!(stopwatch.elapsed_secs(), 0.0);
    /// ```
    #[inline]
    pub fn pause(&mut self) {
        self.paused = true;
    }

    /// Unpauses the stopwatch. Resume the effect of ticking on elapsed time.
    ///
    /// # Examples
    /// ```
    /// # use bevy_time::*;
    /// use std::time::Duration;
    /// let mut stopwatch = Stopwatch::new();
    /// stopwatch.pause();
    /// stopwatch.tick(Duration::from_secs_f32(1.0));
    /// stopwatch.unpause();
    /// stopwatch.tick(Duration::from_secs_f32(1.0));
    /// assert!(!stopwatch.paused());
    /// assert_eq!(stopwatch.elapsed_secs(), 1.0);
    /// ```
    #[inline]
    pub fn unpause(&mut self) {
        self.paused = false;
    }

    /// Returns `true` if the stopwatch is paused.
    ///
    /// # Examples
    /// ```
    /// # use bevy_time::*;
    /// let mut stopwatch = Stopwatch::new();
    /// assert!(!stopwatch.paused());
    /// stopwatch.pause();
    /// assert!(stopwatch.paused());
    /// stopwatch.unpause();
    /// assert!(!stopwatch.paused());
    /// ```
    #[inline]
    pub fn paused(&self) -> bool {
        self.paused
    }

    /// Resets the stopwatch. The reset doesn't affect the paused state of the stopwatch.
    ///
    /// # Examples
    /// ```
    /// # use bevy_time::*;
    /// use std::time::Duration;
    /// let mut stopwatch = Stopwatch::new();
    /// stopwatch.tick(Duration::from_secs_f32(1.5));
    /// stopwatch.reset();
    /// assert_eq!(stopwatch.elapsed_secs(), 0.0);
    /// ```
    #[inline]
    pub fn reset(&mut self) {
        self.elapsed = Default::default();
    }
}

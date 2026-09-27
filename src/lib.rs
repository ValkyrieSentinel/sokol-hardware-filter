#![no_std]

use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};


const fn is_power_of_two(n: usize) -> bool {
    n > 0 && (n & (n - 1)) == 0
}


#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SpectralOilFilter {
    pub orthogonal_mask: u64,
}

impl SpectralOilFilter {
    pub const fn new(mask: u64) -> Self {
        Self { orthogonal_mask: mask }
    }

    #[inline(always)]
    pub fn apply(&self, raw_signal: u64) -> u64 {
        raw_signal & self.orthogonal_mask
    }
}


#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuantumStateVector(pub u64);

impl QuantumStateVector {
    pub const EMPTY: Self = Self(0);

    #[inline(always)]
    pub fn accumulate(self, hypothesis_flag: u64, active: bool) -> Self {
        let mask = (active as u64).wrapping_neg();
        Self(self.0 | (hypothesis_flag & mask))
    }

    #[inline(always)]
    pub fn collapse(self, decision_table: &[u8; 64]) -> u8 {
        let idx = (self.0.trailing_zeros() as usize) & 63;
        unsafe { *decision_table.get_unchecked(idx) }
    }
}


pub struct FastCffBuffer<T, const N: usize> {
    data: [UnsafeCell<MaybeUninit<T>>; N],
    head: AtomicUsize,
}

unsafe impl<T: Send, const N: usize> Send for FastCffBuffer<T, N> {}
unsafe impl<T: Sync, const N: usize> Sync for FastCffBuffer<T, N> {}

impl<T: Copy, const N: usize> FastCffBuffer<T, N> {
    pub const MASK: usize = N - 1;

    pub const fn new() -> Self {
        assert!(is_power_of_two(N), "Buffer size N must be a power of two!");
        Self {
            data: [const { UnsafeCell::new(MaybeUninit::uninit()) }; N],
            head: AtomicUsize::new(0),
        }
    }

    #[inline(always)]
    pub fn push_event(&self, item: T) {
        let current = self.head.fetch_add(1, Ordering::Relaxed);
        let bounded_idx = current & Self::MASK;

        unsafe {
            let ptr = self.data.get_unchecked(bounded_idx).get();
            (*ptr).write(item);
        }
    }
}


#[repr(C)]
pub struct DualFoveaPipeline {
    pub fast_temporal_mask: u64,
    pub fovea_threshold: u64,
}

impl DualFoveaPipeline {
    #[inline(always)]
    pub fn process_event(&self, state: QuantumStateVector) -> (bool, u64) {
        let fast_features = state.0 & self.fast_temporal_mask;
        let needs_deep_scan = fast_features > self.fovea_threshold;
        (needs_deep_scan, fast_features)
    }
}


#[repr(C)]
pub struct PectenTelemetry {
    pub processed_packets: AtomicU64,
    pub dropped_anomalies: AtomicU64,
}

impl PectenTelemetry {
    pub const fn new() -> Self {
        Self {
            processed_packets: AtomicU64::new(0),
            dropped_anomalies: AtomicU64::new(0),
        }
    }

    #[inline(always)]
    pub fn record_pass(&self) {
        self.processed_packets.fetch_add(1, Ordering::Relaxed);
    }

    #[inline(always)]
    pub fn record_drop(&self) {
        self.dropped_anomalies.fetch_add(1, Ordering::Relaxed);
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
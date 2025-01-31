use crate::arch;
use crate::synch::spinlock::Spinlock;

static PARK_MILLER_LEHMER_SEED: Spinlock<u32> = Spinlock::new(0);
const RAND_MAX: u64 = 2_147_483_647;

fn generate_park_miller_lehmer_random_number() -> u32 {
	let mut seed = PARK_MILLER_LEHMER_SEED.lock();
	let random = ((u64::from(*seed) * 48271) % RAND_MAX) as u32;
	*seed = random;
	random
}

#[allow(improper_ctypes_definitions)]
extern "C" fn __sys_rand32() -> Option<u32> {
	arch::processor::generate_random_number32()
}

extern "C" fn __sys_rand64(ret: &mut Option<u64>) {
	*ret = arch::processor::generate_random_number64();
}

extern "C" fn __sys_rand() -> u32 {
	generate_park_miller_lehmer_random_number()
}

/// Create a cryptographicly secure 32bit random number with the support of
/// the underlying hardware. If the required hardware isn't available,
/// the function returns `None`.
#[cfg(not(feature = "newlib"))]
#[no_mangle]
pub fn sys_secure_rand32() -> Option<u32> {
	kernel_function!(__sys_rand32())
}

/// Create a cryptographicly secure 64bit random number with the support of
/// the underlying hardware. If the required hardware isn't available,
/// the function returns `None`.
#[cfg(not(feature = "newlib"))]
#[no_mangle]
pub fn sys_secure_rand64() -> Option<u64> {
	let mut ret = None;
	kernel_function!(__sys_rand64(&mut ret));
	ret
}

/// The function computes a sequence of pseudo-random integers
/// in the range of 0 to RAND_MAX
#[no_mangle]
pub extern "C" fn sys_rand() -> u32 {
	kernel_function!(__sys_rand())
}

extern "C" fn __sys_srand(seed: u32) {
	*(PARK_MILLER_LEHMER_SEED.lock()) = seed;
}

/// The function sets its argument as the seed for a new sequence
/// of pseudo-random numbers to be returned by rand()
#[no_mangle]
pub extern "C" fn sys_srand(seed: u32) {
	kernel_function!(__sys_srand(seed))
}

pub fn random_init() {
	let seed: u32 = arch::processor::get_timestamp() as u32;

	*PARK_MILLER_LEHMER_SEED.lock() = seed;
}

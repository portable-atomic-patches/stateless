#![no_std]
#![no_main]

extern crate alloc;

use reth_evm_ethereum::EthEvmConfig;
use reth_payload_validator as _;
use stateless::{StatelessInput, stateless_validation};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let evm_config = EthEvmConfig::mainnet();
    let chain_spec = evm_config.chain_spec().clone();
    let input = StatelessInput::default();
    stateless_validation(
        input.block,
        alloc::vec::Vec::new(),
        input.witness,
        chain_spec,
        evm_config,
    )
    .unwrap();
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

struct StubAlloc;

unsafe impl core::alloc::GlobalAlloc for StubAlloc {
    #[inline(never)]
    unsafe fn alloc(&self, _: core::alloc::Layout) -> *mut u8 {
        core::ptr::null_mut()
    }

    #[inline(never)]
    unsafe fn dealloc(&self, _: *mut u8, _: core::alloc::Layout) {}
}

#[global_allocator]
static ALLOCATOR: StubAlloc = StubAlloc;

struct SingleCoreCS;

critical_section::set_impl!(SingleCoreCS);

unsafe impl critical_section::Impl for SingleCoreCS {
    unsafe fn acquire() -> () {}
    unsafe fn release(_: ()) {}
}

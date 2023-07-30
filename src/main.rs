#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(mvos::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(default_alloc_error_handler)]

use core::panic::PanicInfo;
use mvos::{println, memory::{self, allocators::BootInfoFrameAllocator}, init_heap};
use bootloader::{BootInfo, entry_point};
use x86_64::{structures::paging::Page, VirtAddr};

extern crate alloc;

use alloc::{boxed::Box, vec::Vec, rc::Rc};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello, World {}", "!");

    mvos::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap init failed");

    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }

    println!("vec at {:p}", vec.as_slice());

    use alloc::vec;
    let rct = Rc::new(vec![1,2,3]);
    let cloned_ref = rct.clone();

    println!("current rct is {}", Rc::strong_count(&cloned_ref));
    core::mem::drop(rct);
    println!("current now rct is {}", Rc::strong_count(&cloned_ref));

    #[cfg(test)]
    test_main();

    println!("it did not crash!");
    mvos::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    mvos::hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    mvos::test_panic_handler(info)
}


#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}


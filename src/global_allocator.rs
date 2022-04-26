use std::alloc::{GlobalAlloc, System, Layout};
use std::time::Instant;

struct ReportingAllocator;

#[global_allocator]
static ALLOCATOR: ReportingAllocator = ReportingAllocator;

unsafe impl GlobalAlloc for ReportingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let start = Instant::now();
        let ptr = System.alloc(layout);
        let end = Instant::now();
        let time_taken = end - start;
        let bytes_requested = layout.size();

        eprintln!("{}\t{}", bytes_requested, time_taken.as_nanos());
        ptr
    }

    unsafe fn dealloc(&self, prt: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
    }
}
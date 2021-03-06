use segmentation;

pub type TaskStateDescriptorLow = segmentation::SegmentDescriptor;
pub type TaskStateDescriptorHigh = u64;

/// In 64-bit mode the TSS holds information that is not
/// directly related to the task-switch mechanism,
/// but is used for finding kernel level stack
/// if interrupts arrive while in kernel mode.
#[derive(Debug)]
#[repr(C, packed)]
pub struct TaskStateSegment {
    pub reserved: u32,
    /// The full 64-bit canonical forms of the stack pointers (RSP) for privilege levels 0-2.
    pub rsp: [u64; 3],
    pub reserved2: u64,
    /// The full 64-bit canonical forms of the interrupt stack table (IST) pointers.
    pub ist: [u64; 7],
    pub reserved3: u64,
    pub reserved4: u16,
    /// The 16-bit offset to the I/O permission bit map from the 64-bit TSS base.
    pub iomap_base: u16,
}

impl TaskStateSegment {
    pub fn new() -> TaskStateSegment {
        TaskStateSegment{
            reserved: 0,
            rsp: [0,0,0],
            reserved2: 0,
            ist: [0,0,0,0,0,0,0],
            reserved3: 0,
            reserved4: 0,
            iomap_base: 0,
        }
    }
}

/// Load the task state register.
pub unsafe fn load_ltr(sel: segmentation::SegmentSelector) {
    asm!("ltr $0" :: "r" (sel));
}
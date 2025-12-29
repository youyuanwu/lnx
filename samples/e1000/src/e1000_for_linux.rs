#![no_std]
//! Rust e1000 network device.

#![allow(unused)]
#![allow(missing_docs)]

use core::slice::from_raw_parts_mut;
use core::sync::atomic::{AtomicPtr, AtomicU64, Ordering};
use kernel::transmute::AsBytes;
use kernel::{
    bindings, c_str, device,
    devres::Devres,
    dma, driver, pci,
    sync::{Arc, ArcBorrow, CondVar, SpinLock, UniqueArc},
    types::ARef,
};
use kernel::{prelude::*, workqueue};

#[macro_use]
pub mod linux;
pub mod e1000;
pub mod utils;

use e1000::E1000Device;

use core::{
    option::Option::{self, None, Some},
    result::Result::{Err, Ok},
    *,
};

// const RXBUFFER: u32 = 2048;
// /// Intel E1000 ID
// const VENDOR_ID_INTEL: u32 = 0x8086;
// const DEVICE_ID_INTEL_I219: u32 = 0x15fc;
// const DEVICE_ID_INTEL_82540EM: u32 = 0x100e;
// const DEVICE_ID_INTEL_82574L: u32 = 0x10d3;
// //const MAC_HWADDR: [u8; 6] = [0x52, 0x54, 0x00, 0x12, 0x34, 0x56];
// //const MAC_HWADDR: [u8; 6] = [0x90, 0xe2, 0xfc, 0xb5, 0x36, 0x95];
// const MAC_HWADDR: [u8; 6] = [0x52, 0x54, 0x00, 0x6c, 0xf8, 0x88];

kernel::module_pci_driver! {
    type: E1000Driver,
    name: "rust_e1000dev",
    authors: ["Luoyuan Xiao"],
    description: "Rust e1000 device driver",
    license: "GPL",
}

struct Regs;

impl Regs {
    const TEST: usize = 0x0;
    const OFFSET: usize = 0x4;
    const DATA: usize = 0x8;
    const COUNT: usize = 0xC;
    const END: usize = 0x10;
}

type Bar0 = pci::Bar<{ Regs::END }>;

struct E1000Driver {
    _pdev: ARef<pci::Device>,
    _bar: Pin<KBox<Devres<Bar0>>>,
    inner: E1000Device<'static, Kernfn>,
}

struct Kernfn {
    dev: ARef<pci::Device>,
    // TODO: use linked list.
    alloc_coherent: KVec<Option<dma::CoherentAllocation<u8>>>,
}

impl e1000::KernelFunc for Kernfn {
    const PAGE_SIZE: usize = 1 << 12;

    fn dma_alloc_coherent(&mut self, pages: usize) -> (usize, usize) {
        // SAFETY: We transmute the device reference to Bound state. This is safe because
        // the device was properly bound during probe() when enable_device_mem() was called.
        // The pci device is valid for DMA operations.
        let dev_ref = self.dev.as_ref() as *const _ as *const device::Device<device::Bound>;
        let alloc = unsafe {
            dma::CoherentAllocation::<u8>::alloc_coherent(
                &*dev_ref,
                pages * Self::PAGE_SIZE,
                GFP_KERNEL,
            )
        }
        .unwrap();

        let vaddr = alloc.start_ptr() as usize;
        let paddr = alloc.dma_handle() as usize;
        self.alloc_coherent.push(Some(alloc), GFP_KERNEL);
        pr_info!(
            "Allocated {} pages, vaddr: {:#x}, paddr: {:#x}\n",
            pages,
            vaddr,
            paddr
        );

        (vaddr, paddr)
    }

    fn dma_free_coherent(&mut self, vaddr: usize, pages: usize) {
        pr_info!("Deallocating addr: {:#x}\n", vaddr);
        if let Some(i) = self.alloc_coherent.iter().position(|opt| match opt {
            None => false,
            Some(i) => i.start_ptr() as usize == vaddr,
        }) {
            // move to last
            let old_len = self.alloc_coherent.len();
            self.alloc_coherent.swap(i, old_len - 1);
            self.alloc_coherent.pop();
        }
    }
}

pub(crate) const E1000_DEVICE_ID: u32 = 0x100E;

const DEVICE_ID_INTEL_I219: u32 = 0x15fc;
const DEVICE_ID_INTEL_82540EM: u32 = 0x100e;
const DEVICE_ID_INTEL_82574L: u32 = 0x10d3;

kernel::pci_device_table!(
    PCI_TABLE,
    MODULE_PCI_TABLE,
    <E1000Driver as pci::Driver>::IdInfo,
    // Id for the device.
    [
        (
            (pci::DeviceId::from_id(pci::Vendor::INTEL, DEVICE_ID_INTEL_I219)),
            ()
        ),
        (
            (pci::DeviceId::from_id(pci::Vendor::INTEL, DEVICE_ID_INTEL_82540EM)),
            ()
        ),
        (
            (pci::DeviceId::from_id(pci::Vendor::INTEL, DEVICE_ID_INTEL_82574L)),
            ()
        )
    ]
);

impl pci::Driver for E1000Driver {
    type IdInfo = ();
    const ID_TABLE: pci::IdTable<Self::IdInfo> = &PCI_TABLE;

    fn probe(pdev: &pci::Device<device::Core>, id_info: &Self::IdInfo) -> Result<Pin<KBox<Self>>> {
        pr_info!("PCI Driver probing {:?}\n", id_info);

        pdev.enable_device_mem()?;
        pdev.set_master();

        // Initialize the bar resource first
        let bar_init = pdev.iomap_region_sized::<{ Regs::END }>(0, c_str!("rust_e1000dev"));
        let bar_box = KBox::pin_init(bar_init, GFP_KERNEL)?;

        // Get the register address from the bar
        let lk_bar = bar_box.try_access().ok_or(ENXIO)?;
        let regs = lk_bar.addr();

        // Create the pdev ARef
        let pdev_aref: ARef<pci::Device> = pdev.into();

        // Create the kernel functions
        let kfn = Kernfn {
            dev: pdev_aref.clone(),
            alloc_coherent: Vec::new(),
        };

        // Initialize the E1000 device
        let e1000_device = E1000Device::<Kernfn>::new(kfn, regs).unwrap();

        // Create the driver data structure
        let drvdata = KBox::pin_init(
            Self {
                _pdev: pdev_aref,
                _bar: bar_box,
                inner: e1000_device,
            },
            GFP_KERNEL,
        )?;

        let wq = workqueue::system();
        wq.try_spawn(GFP_KERNEL, || {
            unsafe { bindings::msleep(5000) };
            pr_info!("e1000 background");
        })
        .unwrap();

        Ok(drvdata)
    }
}

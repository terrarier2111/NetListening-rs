use parking_lot::Mutex;
use std::sync::Arc;
use arc_swap::ArcSwap;
use std::sync::atomic::AtomicUsize;

pub struct TSComposedBuffer {

    inner: ArcSwap<Vec<Arc<Mutex<Box<[u8]>>>>>, // TODO: Can this size be inlined into Mutex?
    rdx: AtomicUsize, // reader index
    wrx: AtomicUsize, // writer index

}
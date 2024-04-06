use core::cell::Cell;

use kernel::debug;
use kernel::hil::digest;
use kernel::ErrorCode;
use kernel::utilities::leasable_buffer::{SubSlice, SubSliceMut};
use kernel::utilities::cells::TakeCell;

const KEY_BUFFER: [u8; 32] = [42; 32];

pub struct HmacBench<'a, const L: usize, H: digest::Digest<'a, L>> {
    hmac: &'a H,
    data_slice: &'static [u8],
    add_data_rounds: usize,
    add_data_cnt: Cell<usize>,
    hash_buf: TakeCell<'static, [u8; L]>,
}

impl<'a, const L: usize, H: digest::Digest<'a, L> + digest::HmacSha256> HmacBench<'a, L, H> {
    pub fn new(
        hmac: &'a H,
        data_slice: &'static [u8],
        add_data_rounds: usize,
        hash_buf: &'static mut [u8; L],
    ) -> Self {
        HmacBench {
            hmac,
            data_slice,
            add_data_rounds,
            add_data_cnt: Cell::new(0),
            hash_buf: TakeCell::new(hash_buf),
        }
    }

    pub fn start(&self) {
        self.hmac.set_mode_hmacsha256(&KEY_BUFFER).unwrap();
        debug!("Set HMAC mode with key!");
        self.add_data_iter();
    }

    fn add_data_iter(&self) {
        if self.add_data_cnt.get() < self.add_data_rounds {
            self.hmac.add_data(SubSlice::new(self.data_slice)).unwrap();
        } else {
            self.hmac.run(self.hash_buf.take().unwrap()).unwrap();
        }
    }
}

impl<'a, const L: usize, H: digest::Digest<'a, L> + digest::HmacSha256> digest::ClientData<L> for HmacBench<'a, L, H> {
    fn add_data_done(&self, _result: Result<(), ErrorCode>, _data: SubSlice<'static, u8>) {
        self.add_data_cnt.set(self.add_data_cnt.get() + 1);
        self.add_data_iter();
    }
    
    fn add_mut_data_done(&self, _result: Result<(), ErrorCode>, _data: SubSliceMut<'static, u8>) {
        unimplemented!();
    }
}

impl<'a, const L: usize, H: digest::Digest<'a, L> + digest::HmacSha256> digest::ClientHash<L> for HmacBench<'a, L, H> {
    fn hash_done(&self, _result: Result<(), ErrorCode>, digest: &'static mut [u8; L]) {
        unimplemented!("Hash done: {:x?}", digest);
    }
}

impl<'a, const L: usize, H: digest::Digest<'a, L> + digest::HmacSha256> digest::ClientVerify<L> for HmacBench<'a, L, H> {
    fn verification_done(&self, _result: Result<bool, ErrorCode>, _compare: &'static mut [u8; L]) {
        unimplemented!();
    }
}


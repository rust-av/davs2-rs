use crate::data::*;

use davs2_sys::*;
use std::ffi::c_void;
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum Error {
    Error = -1,
    Default = 0,
    GotFrame = 1,
    GotHeader = 2,
    End = 3,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = match self {
            Error::Error => "Decoding error occurs.",
            Error::Default => "Decoding but no output.",
            Error::GotFrame => "Decoding gets a frame.",
            Error::GotHeader => "Decoding gets the sequence header.",
            Error::End => "Decoding ended.",
        };

        write!(f, "{}", v)
    }
}

impl Error {
    fn from_i32(v: i32) -> Self {
        match v {
            -1 => Error::Error,
            0 => Error::Default,
            1 => Error::GotFrame,
            2 => Error::GotHeader,
            3 => Error::End,
            _ => unreachable!(),
        }
    }
}

pub struct Decoder {
    dec: *mut c_void,
}

impl Decoder {
    pub fn new(param: &mut Param) -> Result<Self, Error> {
        let dec = unsafe { davs2_decoder_open(param.get_mut_ptr()) };
        if dec.is_null() {
            Err(Error::Error)
        } else {
            Ok(Self { dec })
        }
    }

    pub fn send_packet(
        &mut self,
        packet: &mut Packet,
    ) -> Result<Error, Error> {
        let ret = unsafe {
            davs2_decoder_send_packet(self.dec, packet.get_mut_ptr())
        };
        let err = Error::from_i32(ret);
        if let Error::Error = err {
            Err(err)
        } else {
            Ok(err)
        }
    }

    pub fn recv_frame(
        &mut self,
        headerset: &mut SeqInfo,
        out_frame: &mut Picture,
    ) -> Result<Error, Error> {
        let ret = unsafe {
            davs2_decoder_recv_frame(
                self.dec,
                headerset.get_mut_ptr(),
                out_frame.get_mut_ptr(),
            )
        };
        let err = Error::from_i32(ret);
        if let Error::Error = err {
            Err(err)
        } else {
            Ok(err)
        }
    }

    pub fn flush(
        &mut self,
        headerset: &mut SeqInfo,
        out_frame: &mut Picture,
    ) -> Result<Error, Error> {
        let ret = unsafe {
            davs2_decoder_flush(
                self.dec,
                headerset.get_mut_ptr(),
                out_frame.get_mut_ptr(),
            )
        };
        let err = Error::from_i32(ret);
        if let Error::Error = err {
            Err(err)
        } else {
            Ok(err)
        }
    }

    pub fn frame_unref(&mut self, picture: &mut Picture) {
        unsafe { davs2_decoder_frame_unref(self.dec, picture.get_mut_ptr()) };
    }
}

impl Drop for Decoder {
    fn drop(&mut self) {
        unsafe { davs2_decoder_close(self.dec) };
    }
}

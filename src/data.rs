use crate::*;

use davs2_sys::*;

use std::ffi::c_void;
use std::ptr::{null, null_mut};
use std::slice;

pub type PictureType = davs2_picture_type_e;
pub type ProfileId = davs2_profile_id_e;
pub type LogLevel = davs2_log_level_e;

macro_rules! check_plane {
    ($plane:ident) => {
        if $plane > 3 {
            panic!("There are only 3 planes!");
        }
    };
}

create_struct!(
    SeqInfo,
    seq_info,
    davs2_seq_info_t,
    (
        profile_id,
        level_id,
        progressive,
        width,
        height,
        chroma_format,
        aspect_ratio,
        low_delay,
        bitrate,
        internal_bit_depth,
        output_bit_depth,
        bytes_per_sample,
        frame_rate,
        frame_rate_id
    ),
    (
        usize, usize, usize, usize, usize, usize, usize, usize, usize, usize,
        usize, usize, f32, usize
    ),
    (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0.0, 0),
    (
        profile_id as u32,
        level_id as u32,
        progressive as u32,
        width as u32,
        height as u32,
        chroma_format as u32,
        aspect_ratio as u32,
        low_delay as u32,
        bitrate as u32,
        internal_bit_depth as u32,
        output_bit_depth as u32,
        bytes_per_sample as u32,
        frame_rate as f32,
        frame_rate_id as u32
    )
);

default_struct!(
    Picture,
    picture,
    davs2_picture_t,
    (
        magic,
        planes,
        widths,
        lines,
        strides,
        pic_order_count,
        type_,
        qp,
        pts,
        dts,
        num_planes,
        bytes_per_sample,
        bit_depth,
        b_decode_error,
        dec_frame
    ),
    (
        null_mut(),
        [null_mut(); 3],
        [0; 3],
        [0; 3],
        [0; 3],
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        null_mut()
    )
);

set_and_get_params!(
    Picture,
    picture,
    (
        pic_order_count,
        type_,
        qp,
        pts,
        dts,
        num_planes,
        bytes_per_sample,
        bit_depth,
    ),
    (usize, usize, usize, i64, i64, usize, usize, usize,),
    (
        pic_order_count as i32,
        type_ as i32,
        qp as i32,
        pts as i64,
        dts as i64,
        num_planes as i32,
        bytes_per_sample as i32,
        bit_depth as i32,
    )
);

impl Picture {
    pub fn planes(&self, plane: usize) -> &mut [u8] {
        check_plane!(plane);

        let p_size = self.picture.widths[plane] * self.picture.lines[plane];
        unsafe {
            slice::from_raw_parts_mut(
                self.picture.planes[plane],
                p_size as usize,
            )
        }
    }

    pub fn widths(&self, plane: usize) -> usize {
        check_plane!(plane);
        self.picture.widths[plane] as usize
    }

    pub fn lines(&self, plane: usize) -> usize {
        check_plane!(plane);
        self.picture.lines[plane] as usize
    }

    pub fn strides(&self, plane: usize) -> usize {
        check_plane!(plane);
        self.picture.strides[plane] as usize
    }

    pub fn b_decode_error(&self) -> bool {
        self.picture.b_decode_error != 0
    }
}

pub struct Packet {
    data: Vec<u8>,
    packet: davs2_packet_t,
}

impl Packet {
    pub fn new(data: &[u8], pts: i64, dts: i64) -> Self {
        let packet = davs2_packet_t {
            data: null(),
            len: data.len() as i32 * 8,
            pts: pts,
            dts: dts,
        };
        let mut pack = Self {
            data: data.to_owned(),
            packet,
        };
        pack.packet.data = pack.data.as_ptr();
        pack
    }
}

pub struct Param {
    param: davs2_param_t,
}

impl Param {
    pub fn new(
        threads: usize,
        info_level: usize,
        num_frames: usize,
        disable_avx: bool,
    ) -> Self {
        let param = davs2_param_t {
            threads: threads as i32,
            info_level: info_level as i32,
            opaque: num_frames as *mut c_void,
            disable_avx: disable_avx as i32,
        };
        Self { param }
    }
}

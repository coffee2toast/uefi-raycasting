#![no_main]
#![no_std]

use core::slice;

use tiny_skia::{Color, FillRule, Paint, PathBuilder, Transform, Pixmap};
use uefi::{
    prelude::*,
    proto::console::{gop::{GraphicsOutput, BltOp, BltPixel, BltRegion}, text::Input},
    table::boot::{OpenProtocolAttributes, OpenProtocolParams, ScopedProtocol},
};

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    let bt = system_table.boot_services();

    let input_handle = bt.get_handle_for_protocol::<Input>().unwrap();
    let _input = bt.open_protocol_exclusive::<Input>(input_handle).unwrap();

    let gop_handle = bt.get_handle_for_protocol::<GraphicsOutput>().unwrap();
    let mut gop: ScopedProtocol<GraphicsOutput> = unsafe {
        bt.open_protocol(
            OpenProtocolParams {
                handle: gop_handle,
                agent: bt.image_handle(),
                controller: None,
            },
            OpenProtocolAttributes::GetProtocol,
        )
    }
    .unwrap();
    let (width, height) = gop.current_mode_info().resolution();

    let mut paint = Paint::default();
    paint.set_color_rgba8(39, 174, 96, 255);
    paint.anti_alias = true;

    let mid_x = width as f32 / 2.0;
    let mid_y = height as f32 / 2.0;

    let mut pb = PathBuilder::new();
    pb.move_to(-200.0, 200.0);
    pb.line_to(200.0, 200.0);
    pb.line_to(0.0, -200.0);
    pb.close();
    let path = pb.finish().unwrap();

    let mut loop_count = 0;
    let mut pixmap = Pixmap::new(width as u32, height as u32).unwrap();
    loop {
        loop_count += 10;
        pixmap.fill(Color::BLACK);
        pixmap.fill_path(
            &path,
            &paint,
            FillRule::Winding,
            Transform::from_translate(((mid_x + loop_count as f32) as usize % width) as f32, mid_y),
            None,
        );
        let pixmap_data = pixmap.data();
        let pixmap_data_ptr = pixmap.data().as_ptr();
        let pixel_data: *const BltPixel = unsafe { core::mem::transmute(pixmap_data_ptr) };                         // evil type cast bit level hack
        let pixel_data_slice = unsafe { slice::from_raw_parts(pixel_data, pixmap_data.len() / 4) };    // what the fuck?
        let op = BltOp::BufferToVideo { buffer: pixel_data_slice, src: BltRegion::Full, dest: (0, 0), dims: (width, height) };
        gop.blt(op).unwrap();
    }
}

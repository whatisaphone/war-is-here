use crate::{
    library::fonts::gnu_unifont,
    utils::{
        cstr::CStrExt,
        imgui_ext::{FontIdExt, ImFontConfigExt},
    },
};
use imgui::internal::RawCast;
use std::{convert::TryInto, ffi::CStr, mem, ptr};

pub fn add_gnu_unifont(atlas: &mut imgui::FontAtlas) -> imgui::FontId {
    add_font(
        atlas,
        cstr!("GNU Unifont"),
        gnu_unifont::WIDTH,
        gnu_unifont::HEIGHT,
        gnu_unifont::STRIDE,
        gnu_unifont::DATA,
        1,
    )
}

#[allow(clippy::needless_range_loop)]
fn add_font(
    atlas: &mut imgui::FontAtlas,
    name: &CStr,
    width: usize,
    height: usize,
    stride: usize,
    data: &[u8],
    scale: usize,
) -> imgui::FontId {
    unsafe {
        let mut font_config = imgui::sys::ImFontConfig::new();
        let name = name.to_slice_with_nul();
        font_config.Name[..name.len()].copy_from_slice(name);
        let font = imgui::sys::ImFontAtlas_AddFontDefault(atlas.raw_mut(), &font_config);

        let mut rect_ids = [0; 256];
        for ch in 0..256 {
            rect_ids[ch] = imgui::sys::ImFontAtlas_AddCustomRectFontGlyph(
                atlas.raw_mut(),
                font,
                ch.try_into().unwrap(),
                (width * scale).try_into().unwrap(),
                (height * scale).try_into().unwrap(),
                (width * scale) as f32,
                <_>::default(),
            );
        }

        let mut tex_pixels = mem::MaybeUninit::uninit();
        let mut tex_width = mem::MaybeUninit::uninit();
        imgui::sys::ImFontAtlas_GetTexDataAsAlpha8(
            atlas.raw_mut(),
            tex_pixels.as_mut_ptr(),
            tex_width.as_mut_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
        );
        let tex_pixels = tex_pixels.assume_init();
        let tex_width: usize = tex_width.assume_init().try_into().unwrap();

        for (ch, &rect_id) in rect_ids.iter().enumerate() {
            let rect = imgui::sys::ImFontAtlas_GetCustomRectByIndex(atlas.raw_mut(), rect_id);

            // Source: 1 bit per pixel
            let src = &data[ch * height * stride..];

            // Dest: 8 bits (1 byte) per pixel
            let dest = tex_pixels.add((*rect).Y as usize * tex_width + (*rect).X as usize);

            match scale {
                1 => blit_char(src, stride, dest, tex_width, width, height),
                2 => blit_char_2x(src, stride, dest, tex_width, width, height),
                _ => unreachable!(),
            }
        }

        imgui::FontId::from_raw(imgui::Font::from_raw(font.as_ref().unwrap()))
    }
}

#[allow(clippy::needless_range_loop)]
unsafe fn blit_char(
    // Source is 1-bit
    src: &[u8],
    src_stride: usize,
    // Dest is 8-bit
    dest: *mut u8,
    dest_stride: usize,
    width: usize,
    height: usize,
) {
    for y in 0..height {
        for x in 0..width {
            // Read 1 bit, write 8 bits
            assert!(src_stride == 1);
            let value = ((src[y] >> (7 - x)) & 1) * 0xff;
            dest.add(y * dest_stride + x).write(value);
        }
    }
}

#[allow(clippy::needless_range_loop)]
unsafe fn blit_char_2x(
    // Source is 1-bit
    src: &[u8],
    src_stride: usize,
    // Dest is 8-bit
    dest: *mut u8,
    dest_stride: usize,
    width: usize,
    height: usize,
) {
    let scale = 2;
    for y in 0..height {
        for x in 0..width {
            // Read 1 bit, write 8 bits
            assert!(src_stride == 1);
            let value = ((src[y] >> (7 - x)) & 1) * 0xff;

            dest.add(y * scale * dest_stride + x * scale).write(value);
            dest.add(y * scale * dest_stride + (x * scale + 1))
                .write(value);
            dest.add((y * scale + 1) * dest_stride + x * scale)
                .write(value);
            dest.add((y * scale + 1) * dest_stride + (x * scale + 1))
                .write(value);
        }
    }
}

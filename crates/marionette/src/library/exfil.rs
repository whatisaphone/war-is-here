use crate::darksiders1::{gfc, new};
use std::{fs, slice};

const PATH: &str = r"C:\Users\John\AppData\Local\Temp";

pub fn dump_byteinputstream_to_file(bis: &gfc::ByteInputStream, prefix: &str) {
    let raw_full = unsafe {
        slice::from_raw_parts((*bis.as_ptr()).mBytes, (*bis.as_ptr()).mNumBytes as usize)
    };
    let raw_full_path = format!("{}/{}.raw.full", PATH, prefix);
    if fs::write(&raw_full_path, raw_full).is_ok() {
        println!("wrote to {:?}", raw_full_path);
    } else {
        println!("failed to write {:?}", raw_full_path);
    }

    let raw_offset = unsafe {
        slice::from_raw_parts(
            (*bis.as_ptr()).mBufferPtr,
            (*bis.as_ptr()).mBufferAvail as usize,
        )
    };
    let raw_offset_path = format!("{}/{}.raw.offset", PATH, prefix);
    if fs::write(&raw_offset_path, raw_offset).is_ok() {
        println!("wrote to {:?}", raw_offset_path);
    } else {
        println!("failed to write {:?}", raw_offset_path);
    }
}

pub fn dump_object_to_file(object: &gfc::Object, prefix: &str) {
    let oo = dump_object_to_bytes(object);
    let oo_path = format!("{}/{}.oo", PATH, prefix);
    if fs::write(&oo_path, oo.bytes()).is_ok() {
        println!("wrote to {:?}", oo_path);
    } else {
        println!("failed to write {:?}", oo_path);
    }
}

pub fn dump_object_to_bytes(object: &gfc::Object) -> gfc::AutoRef<gfc::ByteOutputStream> {
    let mut writer = gfc::OOObjectWriter::new();

    let stream = new(gfc::ByteOutputStream::new);
    let mut stream = unsafe {
        gfc::AutoRef::<gfc::ByteOutputStream>::from_ptr(
            (*(*(*(*stream).as_ptr()).as_gfc__OutputStream_mut_ptr()).as_gfc__Stream_mut_ptr())
                .as_gfc__IRefObject_mut_ptr(),
        )
    };
    writer.write_object(object, &mut stream, false);

    stream
}

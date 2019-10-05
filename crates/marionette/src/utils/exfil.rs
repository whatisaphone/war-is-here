use crate::darksiders1::{gfc, new};
use std::fs;

pub fn dump_object(object: &gfc::Object) {
    let mut writer = gfc::OOObjectWriter::new();

    let stream = new(gfc::ByteOutputStream::new);
    let mut stream = unsafe {
        gfc::AutoRef::<gfc::ByteOutputStream>::from_ptr(
            (*(*(*(*stream).as_ptr()).as_gfc__OutputStream_mut_ptr()).as_gfc__Stream_mut_ptr())
                .as_gfc__IRefObject_mut_ptr(),
        )
    };

    writer.write_object(object, &mut stream, false);

    let path = format!(
        r"C:\Users\John\AppData\Local\Temp\{}-{:?}.oo",
        object.class().name().c_str().to_string_lossy(),
        object.as_ptr(),
    );
    fs::write(&path, stream.bytes()).unwrap();
    println!("wrote object to {:?}", &path);
}

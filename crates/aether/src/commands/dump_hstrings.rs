use crate::darksiders1::Lift1;
use darksiders1_sys::target;
use std::{
    borrow::Cow,
    env::temp_dir,
    error::Error,
    ffi::CStr,
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

pub fn run(_command: &str) -> Cow<'static, str> {
    match run_inner() {
        Ok(path) => format!("wrote to {}", path.display()).into(),
        Err(_) => "could not write file".into(),
    }
}

pub fn run_inner() -> Result<PathBuf, Box<dyn Error>> {
    let path = temp_dir().join("hstrings.txt");
    let output = File::create(&path)?;
    let mut output = BufWriter::new(output);
    unsafe {
        let hstring_manager = *target::gfc__Singleton_gfc__HStringManager_gfc__CreateStatic_gfc__SingletonLongevity__DieLast___InstanceHandle;
        let pairs = (*hstring_manager).mHashTable.mPairs.lift1_ref();
        for pair in pairs {
            let cstr = CStr::from_ptr(pair.mValue.Str);
            writeln!(
                output,
                "{:016x},{}",
                pair.mKey,
                cstr.to_str().unwrap_or("<invalid utf-8>"),
            )?;
        }
    }
    Ok(path)
}

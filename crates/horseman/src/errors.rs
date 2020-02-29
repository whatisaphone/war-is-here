use std::io;

pub fn assert_or_last_os_error(cond: bool) -> Result<(), io::Error> {
    if !cond {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

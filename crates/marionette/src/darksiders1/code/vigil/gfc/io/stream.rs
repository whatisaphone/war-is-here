use crate::darksiders1::gfc;
use darksiders1_sys::target;

struct_wrapper!(Stream, target::gfc__Stream);
struct_wrapper_super!(Stream, gfc::IRefObject, as_gfc__IRefObject_mut_ptr);

struct_wrapper!(InputStream, target::gfc__InputStream);
struct_wrapper_super!(InputStream, gfc::Stream, as_gfc__Stream_mut_ptr);

struct_wrapper!(OutputStream, target::gfc__OutputStream);
struct_wrapper_super!(OutputStream, gfc::Stream, as_gfc__Stream_mut_ptr);

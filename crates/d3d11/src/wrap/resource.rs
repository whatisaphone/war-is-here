use winapi::um::d3d11::ID3D11Resource;

wrap_iunknown!(pub Resource, *mut ID3D11Resource);

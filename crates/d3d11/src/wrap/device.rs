use winapi::um::d3d11::ID3D11Device;

wrap_iunknown!(pub Device, *mut ID3D11Device);

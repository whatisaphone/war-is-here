use winapi::um::d3d11::ID3D11DeviceContext;

wrap_iunknown!(pub DeviceContext, *mut ID3D11DeviceContext);

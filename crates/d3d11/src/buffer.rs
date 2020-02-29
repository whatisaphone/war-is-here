use winapi::um::d3d11::ID3D11Buffer;

wrap_iunknown!(pub Buffer, *mut ID3D11Buffer);

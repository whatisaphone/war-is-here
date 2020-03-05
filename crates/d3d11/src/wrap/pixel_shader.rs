use winapi::um::d3d11::ID3D11PixelShader;

wrap_iunknown!(pub PixelShader, *mut ID3D11PixelShader);

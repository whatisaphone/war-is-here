use winapi::um::d3d11::ID3D11RasterizerState;

wrap_iunknown!(pub RasterizerState, *mut ID3D11RasterizerState);

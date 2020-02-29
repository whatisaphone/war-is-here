use winapi::um::d3d11::ID3D11SamplerState;

wrap_iunknown!(pub SamplerState, *mut ID3D11SamplerState);

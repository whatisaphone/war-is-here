use winapi::um::d3d11::ID3D11ShaderResourceView;

wrap_iunknown!(pub ShaderResourceView, *mut ID3D11ShaderResourceView);

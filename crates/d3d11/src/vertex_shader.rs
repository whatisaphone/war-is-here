use winapi::um::d3d11::ID3D11VertexShader;

wrap_iunknown!(pub VertexShader, *mut ID3D11VertexShader);

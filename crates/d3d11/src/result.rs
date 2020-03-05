use winapi::shared::winerror::HRESULT;

pub type Result<T, E = HRESULT> = ::std::result::Result<T, E>;

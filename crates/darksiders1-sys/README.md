# darksiders1-hook

Low-level bindings to `darksiders1.exe`.

## How to generate bindings

The developers were generous enough to include a PDB with the game, so creating bindings is super easy! Just run this command:

```sh
pdbindgen \
    /path/to/darksiders1.pdb \
    -o src \
    /s/Games/Darksiders/Builds/Darksiders\ Warmastered\ Edition/darksiders1.pdb \
    -o ../war-is-here/crates/darksiders1-sys/src \
    -i ^gfc::HString::HString$ \
    -i ^gfc::Singleton_gfc::WindowHelper_.+::_InstanceHandle$ \
    -i ^gfc::Darksiders::onPostUpdateInterval$ \
    -i ^gfc::Graphics::getInstance$ \
    -i ^gfc::LoadMapMenu::LoadMapMenu$ \
    -i ^gfc::WindowHelper::pushWindow$
```

Or, if you're a pdbindgen developer:

```sh
cargo run --release \
    /s/Games/Darksiders/Builds/Darksiders\ Warmastered\ Edition/darksiders1.pdb \
    -o ../war-is-here/crates/darksiders1-sys/src \
    -i '^gfc::HString::HString$' \
    -i '^gfc::Player::Player$' \
    -i '^gfc::Singleton<gfc::ClassRegistry,.+>::InstanceHandle$' \
    -i '^gfc::Singleton<gfc::Darksiders,.+>::InstanceHandle$' \
    -i '^gfc::Singleton<gfc::WindowHelper,.+>::InstanceHandle$' \
    -i '^gfc::MemAlloc$' \
    -i '^gfc::Actor::setPosition$' \
    -i '^gfc::Class::\w+$' \
    -i '^gfc::ClassRegistry::classForName$' \
    -i '^gfc::Darksiders::onPostUpdateInterval$' \
    -i '^gfc::Graphics::getInstance$' \
    -i '^gfc::LoadMapMenu::LoadMapMenu$' \
    -i '^gfc::Object3D::\w+$' \
    -i '^gfc::OmniLight::\w+$' \
    -i '^gfc::StaticObject::\w+$' \
    -i '^gfc::Value$' \
    -i '^gfc::WindowHelper::pushWindow$' \
    -i '^gfc::WorldObject::\w+$'
```

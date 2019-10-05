# darksiders1-hook

Low-level bindings to `darksiders1.exe`.

## How to generate bindings

The developers were generous enough to include a PDB with the game, so creating bindings is super easy! Just run this command:

```sh
pdbindgen \
    /path/to/darksiders1.pdb \
    -o src \
    -i '^gfc::Player$' \
    -i '^gfc::RegionLayer$' \
    -i '^gfc::StaticMesh$' \
    -i '^gfc::StaticMeshVisual$' \
    -i '^gfc::TriggerRegion$' \
    -i '^gfc::WorldRegion$' \
    -i '^gfc::Singleton<gfc::ClassRegistry,.+>::InstanceHandle$' \
    -i '^gfc::Singleton<gfc::Darksiders,.+>::InstanceHandle$' \
    -i '^gfc::Singleton<gfc::KGMeshCache,.+>::InstanceHandle$' \
    -i '^gfc::Singleton<gfc::Object3DCache,.+>::InstanceHandle$' \
    -i '^gfc::Singleton<gfc::TeleportHelper,.+>::InstanceHandle$' \
    -i '^gfc::Singleton<gfc::WindowHelper,.+>::InstanceHandle$' \
    -i '^gfc::TriggerRegion::_Class$' \
    -i '^gfc::WorldGroup::_Class$' \
    -i '^operator new$' \
    -i '^gfc::MemAlloc$' \
    -i '^gfc::MemFree$' \
    -i '^gfc::AutoRef<gfc::IRefObject>::~?AutoRef<gfc::IRefObject>$' \
    -i '^gfc::ByteOutputStream::~?ByteOutputStream$' \
    -i '^gfc::ClassRegistry::classForName$' \
    -i '^gfc::Darksiders::onPostUpdateInterval$' \
    -i '^gfc::Darksiders::processInputEvent$' \
    -i '^gfc::HString::~?HString$' \
    -i '^gfc::HString::c_str$' \
    -i '^gfc::LoadMapMenu::LoadMapMenu$' \
    -i '^gfc::MeshCache::getStaticMesh$' \
    -i '^gfc::MeshReader::readObject$' \
    -i '^gfc::Object3DCache::get$' \
    -i '^gfc::ObjectWriter::~?ObjectWriter$' \
    -i '^gfc::OOObjectWriter::writeObject$' \
    -i '^gfc::OblivionGame::getWorld$' \
    -i '^gfc::OmniLight::\w+$' \
    -i '^gfc::RegionLayer::getRoot$' \
    -i '^gfc::ResourceCache::getResource$' \
    -i '^gfc::StaticObject::setObjectName$' \
    -i '^gfc::StaticObject::setPackageName$' \
    -i '^gfc::TeleportHelper::warpToMap$' \
    -i '^gfc::WindowHelper::pushWindow$' \
    -i '^gfc::World::getRegion$' \
    -i '^gfc::WorldGroup::getObjects$' \
    -i '^gfc::WorldObject::setLayerID$' \
    -i '^gfc::WorldObject::setRegionID$' \
    -i '^gfc::WorldRegion::getLayer$'
```

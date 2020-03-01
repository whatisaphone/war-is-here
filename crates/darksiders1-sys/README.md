# darksiders1-hook

Low-level bindings to `darksiders1.exe`.

## How to generate bindings

The developers were generous enough to include a PDB with the game, so creating bindings is super easy! Just run this command:

```sh
pdbindgen \
    /path/to/darksiders1.pdb \
    -o src \
    -i '^gfc::ByteInputStream::~?ByteInputStream$' \
    -i '^gfc::ByteOutputStream::~?ByteOutputStream$' \
    -i '^gfc::CShape$' \
    -i '^gfc::CShape::_Class$' \
    -i '^gfc::CShapeBox$' \
    -i '^gfc::CShapeBox::_Class$' \
    -i '^gfc::CShapeMesh$' \
    -i '^gfc::CShapeMesh::_Class$' \
    -i '^gfc::ClassRegistry::classForName$' \
    -i '^gfc::Darksiders::processInputEvent$' \
    -i '^gfc::GameCamera$' \
    -i '^gfc::HString::c_str$' \
    -i '^gfc::HString::~?HString$' \
    -i '^gfc::Inventory::addItem$' \
    -i '^gfc::Item::_Class$' \
    -i '^gfc::KGGraphics::createStaticMesh$' \
    -i '^gfc::KGStaticMesh$' \
    -i '^gfc::LoadMapMenu::LoadMapMenu$' \
    -i '^gfc::MBSubMesh::MBSubMesh$' \
    -i '^gfc::Material::setParameter$' \
    -i '^gfc::Material::~?Material$' \
    -i '^gfc::MaterialCache::get$' \
    -i '^gfc::MemAlloc$' \
    -i '^gfc::MemFree$' \
    -i '^gfc::MeshBuilder::~?MeshBuilder$' \
    -i '^gfc::MeshCache::getStaticMesh$' \
    -i '^gfc::MeshCache::loadMesh$' \
    -i '^gfc::MeshReader::MeshReader$' \
    -i '^gfc::MeshReader::readObject$' \
    -i '^gfc::Node3D::getMatrix$' \
    -i '^gfc::Node3D::getPosition$' \
    -i '^gfc::OOObjectWriter::writeObject$' \
    -i '^gfc::Object3D::~?Object3D$' \
    -i '^gfc::Object3DCache::get$' \
    -i '^gfc::ObjectWriter::~?ObjectWriter$' \
    -i '^gfc::OblivionGame::getProjMatrix$' \
    -i '^gfc::OblivionGame::getViewMatrix$' \
    -i '^gfc::OblivionGame::getWorld$' \
    -i '^gfc::OblivionGame::update$' \
    -i '^gfc::PhysMeshCache::get$' \
    -i '^gfc::PhysicsManager$' \
    -i '^gfc::PhysicsShapeObject::getTransform$' \
    -i '^gfc::Player::pickupItem$' \
    -i '^gfc::RegionLayer::getRoot$' \
    -i '^gfc::RegionLayerData$' \
    -i '^gfc::ResourceCache::getResource$' \
    -i '^gfc::ResourceManager::getPackageID$' \
    -i '^gfc::ResourceManager::getPermanentID$' \
    -i '^gfc::ResourceManager::loadPackages$' \
    -i '^gfc::ScriptClass::~?ScriptClass$' \
    -i '^gfc::Singleton<gfc::ClassRegistry,.+>::InstanceHandle$' \
    -i '^gfc::Singleton<gfc::DSUIManager,.+>::InstanceHandle$' \
    -i '^gfc::Singleton<gfc::Darksiders,.+>::InstanceHandle$' \
    -i '^gfc::Singleton<gfc::KGGraphics,.+>::InstanceHandle$' \
    -i '^gfc::Singleton<gfc::KGMeshCache,.+>::InstanceHandle$' \
    -i '^gfc::Singleton<gfc::Object3DCache,.+>::InstanceHandle$' \
    -i '^gfc::Singleton<gfc::PhysMeshCache,.+>::InstanceHandle$' \
    -i '^gfc::Singleton<gfc::ResourceManager,.+>::InstanceHandle$' \
    -i '^gfc::Singleton<gfc::TeleportHelper,.+>::InstanceHandle$' \
    -i '^gfc::Singleton<gfc::WindowHelper,.+>::InstanceHandle$' \
    -i '^gfc::Skeleton3D$' \
    -i '^gfc::StaticMeshVisual::~?StaticMeshVisual$' \
    -i '^gfc::StaticObject::setObjectName$' \
    -i '^gfc::StaticObject::setPackageName$' \
    -i '^gfc::StaticObject::~?StaticObject$' \
    -i '^gfc::String::~?String$' \
    -i '^gfc::TeleportHelper::warpToMap$' \
    -i '^gfc::TriggerRegion$' \
    -i '^gfc::TriggerRegion::_Class$' \
    -i '^gfc::UIRenderer::\w+$' \
    -i '^gfc::Vector4Parameter::~?Vector4Parameter$' \
    -i '^gfc::Weapon$' \
    -i '^gfc::WindowHelper::pushWindow$' \
    -i '^gfc::World::World$' \
    -i '^gfc::World::getRegion$' \
    -i '^gfc::WorldGroup$' \
    -i '^gfc::WorldGroup::_Class$' \
    -i '^gfc::WorldObject::_Class$' \
    -i '^gfc::WorldObject::attachToObject$' \
    -i '^gfc::WorldObject::removeObjectFromWorld' \
    -i '^gfc::WorldRegion::getLayer$' \
    -i '^gfc::WorldRegionData$' \
    -i '^gfc::_UIManager::draw$' \
    -i '^hkpConvexVerticesShape::getOriginalVertices$' \
    -i '^hkpExtendedMeshShape$' \
    -i '^hkpMoppBvTreeShape$' \
    -i '^keen::KeyEventData$' \
    -i '^keen::MouseEventData$' \
    -i '^keen::MouseWheelEventData$'
```

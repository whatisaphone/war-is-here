# darksiders1-hook

Low-level bindings to `darksiders1.exe`.

## How to generate bindings

The developers were generous enough to include a PDB with the game, so creating bindings is super easy! Just run this command:

```sh
pdbindgen \
    /path/to/darksiders1.pdb \
    -o src \
    -i '^gfc::CShape$' \
    -i '^gfc::CShapeBox$' \
    -i '^gfc::CShapeMesh$' \
    -i '^gfc::GameCamera$' \
    -i '^gfc::PhysicsManager$' \
    -i '^gfc::RegionLayerData$' \
    -i '^gfc::Skeleton3D$' \
    -i '^gfc::StaticMesh$' \
    -i '^gfc::TriggerRegion$' \
    -i '^gfc::Weapon$' \
    -i '^gfc::WorldGroup$' \
    -i '^gfc::WorldRegionData$' \
    -i '^gfc::CShape::_Class$' \
    -i '^gfc::CShapeBox::_Class$' \
    -i '^gfc::CShapeMesh::_Class$' \
    -i '^gfc::Item::_Class$' \
    -i '^gfc::Singleton<gfc::ClassRegistry,.+>::InstanceHandle$' \
    -i '^gfc::Singleton<gfc::Darksiders,.+>::InstanceHandle$' \
    -i '^gfc::Singleton<gfc::DSUIManager,.+>::InstanceHandle$' \
    -i '^gfc::Singleton<gfc::KGGraphics,.+>::InstanceHandle$' \
    -i '^gfc::Singleton<gfc::KGMeshCache,.+>::InstanceHandle$' \
    -i '^gfc::Singleton<gfc::Object3DCache,.+>::InstanceHandle$' \
    -i '^gfc::Singleton<gfc::PhysMeshCache,.+>::InstanceHandle$' \
    -i '^gfc::Singleton<gfc::TeleportHelper,.+>::InstanceHandle$' \
    -i '^gfc::Singleton<gfc::WindowHelper,.+>::InstanceHandle$' \
    -i '^gfc::TriggerRegion::_Class$' \
    -i '^gfc::WorldGroup::_Class$' \
    -i '^gfc::WorldObject::_Class$' \
    -i '^gfc::MemAlloc$' \
    -i '^gfc::MemFree$' \
    -i '^gfc::ByteInputStream::~?ByteInputStream$' \
    -i '^gfc::ByteOutputStream::~?ByteOutputStream$' \
    -i '^gfc::ClassRegistry::classForName$' \
    -i '^gfc::Darksiders::onPostUpdateInterval$' \
    -i '^gfc::Darksiders::processInputEvent$' \
    -i '^gfc::HString::~?HString$' \
    -i '^gfc::HString::c_str$' \
    -i '^gfc::Inventory::addItem$' \
    -i '^gfc::KGGraphics::createStaticMesh$' \
    -i '^gfc::LoadMapMenu::LoadMapMenu$' \
    -i '^gfc::Material::~?Material$' \
    -i '^gfc::Material::setParameter$' \
    -i '^gfc::MaterialCache::get$' \
    -i '^gfc::MBSubMesh::MBSubMesh$' \
    -i '^gfc::MeshBuilder::~?MeshBuilder$' \
    -i '^gfc::MeshCache::getStaticMesh$' \
    -i '^gfc::MeshCache::loadMesh$' \
    -i '^gfc::MeshReader::MeshReader$' \
    -i '^gfc::MeshReader::readObject$' \
    -i '^gfc::Node3D::getMatrix$' \
    -i '^gfc::Node3D::getPosition$' \
    -i '^gfc::Object3D::~?Object3D$' \
    -i '^gfc::Object3DCache::get$' \
    -i '^gfc::ObjectWriter::~?ObjectWriter$' \
    -i '^gfc::OOObjectWriter::writeObject$' \
    -i '^gfc::OblivionGame::getProjMatrix$' \
    -i '^gfc::OblivionGame::getViewMatrix$' \
    -i '^gfc::OblivionGame::getWorld$' \
    -i '^gfc::PhysicsShapeObject::getTransform$' \
    -i '^gfc::Player::pickupItem$' \
    -i '^gfc::PhysMeshCache::get$' \
    -i '^gfc::RegionLayer::getRoot$' \
    -i '^gfc::ResourceCache::getResource$' \
    -i '^gfc::StaticMeshVisual::~?StaticMeshVisual$' \
    -i '^gfc::StaticObject::~?StaticObject$' \
    -i '^gfc::StaticObject::setObjectName$' \
    -i '^gfc::StaticObject::setPackageName$' \
    -i '^gfc::String::~?String$' \
    -i '^gfc::TeleportHelper::warpToMap$' \
    -i '^gfc::UIRenderer::\w+$' \
    -i '^gfc::Vector4Parameter::~?Vector4Parameter$' \
    -i '^gfc::WindowHelper::pushWindow$' \
    -i '^gfc::World::getRegion$' \
    -i '^gfc::WorldObject::attachToObject$' \
    -i '^gfc::WorldObject::removeObjectFromWorld' \
    -i '^gfc::WorldObject::setLayerID$' \
    -i '^gfc::WorldObject::setRegionID$' \
    -i '^gfc::WorldRegion::getLayer$' \
    -i '^gfc::_UIManager::draw$' \
    -i '^hkpConvexVerticesShape::getOriginalVertices$'
```

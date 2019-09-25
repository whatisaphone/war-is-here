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
    -i ^gfc__HString__HString$ \
    -i ^gfc__Singleton_gfc__WindowHelper_.+___InstanceHandle$ \
    -i ^gfc__Darksiders__onPostUpdateInterval$ \
    -i ^gfc__LoadMapMenu__LoadMapMenu$ \
    -i ^gfc__WindowHelper__pushWindow$
```

# Change Log

## [Unreleased]

## [0.0.5]

* Bump minimum supported Rust version to 1.40.
* Add an FBX version field to `any::AnyDocument::V7400`.
* Add `any::AnyDocument::fbx_version()`.

### Fixes
* Now fbxcel-dom handles non-exhaustive types from external crates correctly.
    + Actually, users would never be affected by this bug, because fbxcel-0.5.1 won't be released
      (next release would be 0.6.0).

### Breaking changes
* Add an FBX version field to `any::AnyDocument`.
    + This may be useful to emit meaningful error message when users get unknown variant of
      `AnyDocument`.

### Non-breaking changes
* Use `#[non_exhaustive]` instead of hidden dummy variants for enums.
    + Users won't affected by this internal change.

### Added
* Add `any::Error::UnsupportedVersion` error vaniant.
    + This indicates the loaded document is supported by `fbxcel` crate, but not by `fbxcel-dom`
      crate.
* Add `any::AnyDocument::fbx_version()`.
    + This enable users to get FBX version of the document, even when the enum variant of
      `AnyDocument` is unknown and inaccessible to the users.

## [0.0.4]

* Bump dependencies.
    + Now `fbxcel-dom` depends on `fbxcel-0.5`.
* Error types returned by many methods are changed.
    + Previously `failure::Error` was used, but now migrated to `anyhow::Error`.

### Breaking changes
* Bump dependencies (fec62fe09d3c35acbb8e934192a1a315a0376098).
    + Now `fbxcel-dom` depends on `fbxcel-0.5`.
* Error types are changed (c99727523d9c5ffc40ff041ad02373b8168882f2).
    + Previously `failure::Error` was used, but now migrated to `anyhow::Error`.
    + Note that `anyhow::Error` does not implement `std::error::Error`, while `failure::Error` does.
        - Instead, it implements `Deref<Target = std::error::Error>` and convertible to
          `Box<Error + Send + Sync + 'static>`.
          See <https://github.com/dtolnay/anyhow/issues/10>.

## [0.0.3]
* Added basic mesh and texture data access support.
* `v7400::object::ObjectId::raw()` is added.
* `v7400::data::mesh::*::get_{u32,usize}` is renamed to `to_{u32,usize}`.
* Changed handling of the absent `NormalsW` for layer element normal.
* `Implement `TryFrom` for some types.
    + Due to this change, now `fbxcel-dom` **requires Rust 1.34.0 or above**.
* More aggressive `mint` integration support.
* Added `rgb` integration support.

### Added
* `v7400::object::ObjectId::raw()` is added.
    + This returns raw integer value.
* `v7400::object::video::ClipHandle::{content,relative_filepath}()` is added.
    + With `content()`, users can get texture data.
    + With `relative_filepath()`, users can get relative filepath data.
        - Note that this is raw data and may require some processing before use.
* `v7400::object::texture::TextureProperties` type is added.
    + It provides easy access to texture properties.
* `v7400::object::property::loaders::MintLoader` now supports `Point{2,3}`
  types.
* `v7400::object::property::loaders::RgbLoader` is added.
    + It is a loader type for `rgb::{RGB,RGBA}<{f32,f64}>` types.
* `Implement `TryFrom` for some types.
    + `v7400::data::material::ShadingModel` (from `&str`).
    + `v7400::data::mesh::layer::LayerElementType` (from `&str`).
    + `v7400::data::mesh::layer::MappingMode` (from `&str`)
    + `v7400::data::mesh::layer::ReferenceMode` (from `&str`).
    + `v7400::data::texture::BlendMode` (from `i32`).
    + `v7400::data::texture::WrapMode` (from `i32`).

#### `v7400::data` module
* `v7400::data::material` module is added.
    + It contains material-related types.
* `v7400::data::mesh` module is added.
    + It (currently) contains mesh-related types.
      They will be read or created from FBX data, but they might not reflect
      structures of raw FBX data.
    + Polygon vertices triangulation is supported, but triangulator is currently
      not included in this crate.
      Users should prepare by themselves.
* `v7400::data::texture` module is added.
    + It contains texture-related types.

#### Mesh data access
`v7400::object::geometry::MeshHandle` now supports access to some data including
position vertices and normals.

* Control points and polygon vertices:
    + `MeshHandle::polygon_vertices()` returns proxy to control points and
      polygon vertices.
      Control points are maybe-deduplicated vertices.
      Polygon vertices are indices of control points.
      Using them, users can access or enumerate vertices.
    + Polygons are specified through polygon vertices, not only by control
      points.
    + Usually user may want to triangulated polygons, not direct polygon
      vertices.
* Triangulated polygons:
    + `PolygonVertices` (returned by `MeshHandle::polygon_vertices()`) has
      `triangulate_each` method.
      Using this, polygons can be triangulated.
* Layer elements:
    + `MeshHandle::layers()` returns layer elements, which contains data of
      normals, UVs, materials, etc.
    + See `v7400::data::mesh::layer::TypedLayerElementHandle` to see what kind
      of data are supported.

### Non-breaking change
* `v7400::data::mesh::*::get_{u32,usize}` is renamed to `to_{u32,usize}`.
    + They are simple non-consuming type conversion.
    + See <https://doc.rust-lang.org/1.0.0/style/style/naming/conversions.html>.
    + Old `get_*` functions will exist for a while, but will be removed in
      future.

#### Fixed
* Changed handling of the absent `NormalsW` for layer element normal.
    + Previously, absence of `NormalsW` is handled as an error.
    + Now the absence is non-error.
      All functionalities correctly work without `NormalsW`.

## [0.0.2]

* Docs are improved a little.
* Fixed object traversal bug.
    + Not crash, not vulnerability, but simply very wrong.
* No API changes.

### Non-breaking change
#### Fixed
* Fixed object traversal.
    + For all objects (1060790890a8).
        * Previously, some objects were not iterated as source objects and
          destination objects.
          Now this is fixed.
    + For objects access specialized for some types of object handles (1b6b6e904ba5).
        * For example, `v7400::object::model::ModelHandle::parent_model()` now
          returns the correct parent model.
        * Previously, child and parent check was wrong and for some functions.
          Now this is fixed.

## [0.0.1]

Split from `fbxcel` crate, because objects-related APIs would be frequently
updated or fixed as breaking changes.

The changelog below is change from `fbxcel::dom` module as of `fbxcel-0.3.0`.

* Object properties are supported.
    + Very basic support, but would be useful.
* `any` module is added.
    + They provide mostly version-independent way to read and load the FBX data.

### Added
* `v7400::Document::objects()` is added.
* `v7400::ObjectNodeId` now implements
  `Deref<Target=fbxcel::tree::v7400::NodeId>`.
* `v7400::object::property` module and related features are added.
    + This module contains things related to object properties.
    + `v7400::ObjectHandle::direct_properties()` is added.
        * By this method, users can access object properties.
    + `v7400::ObjectHandle::properties_by_native_typename()` is added.
        * By this method, users can access object properties.
* Object-node-type-specific handle types are added.
    + Modules corresponding to node name are added under `v7400::object`.
        * `deformer`: `Deformer` and `SubDeformer` node.
        * `geometry`: `Geometry` node.
        * `material`: `Material` node.
        * `model`: `Model` node.
        * `nodeattribute`: `NodeAttribute` node.
        * `texture`: `Texture` node.
        * `video`: `Video` node.
        * Some new types of them have the same name (for example
          `model::MeshHandle` and `geometry::MeshHandle`).
          Use with care.
    + Meaningful objects traversing is partially supported, but they are
      unstable and subject to change.
      Be careful.

### Non-breaking change
* Debug (`{:?}`) formatting for `v7400::object::ObjectHandle` became much
  simpler and "usable".
    + Previously it uses default format and dumps contents of `Document` data,
      which can be very large and not so much useful.
    + Now it simply dumps object node ID and object metadata.
      Simple, small, and human-readable.

[Unreleased]: <https://github.com/lo48576/fbxcel/compare/v0.0.5...develop>
[0.0.5]: <https://github.com/lo48576/fbxcel/releases/tag/v0.0.5>
[0.0.4]: <https://github.com/lo48576/fbxcel/releases/tag/v0.0.4>
[0.0.3]: <https://github.com/lo48576/fbxcel/releases/tag/v0.0.3>
[0.0.2]: <https://github.com/lo48576/fbxcel/releases/tag/v0.0.2>
[0.0.1]: <https://github.com/lo48576/fbxcel/releases/tag/v0.0.1>

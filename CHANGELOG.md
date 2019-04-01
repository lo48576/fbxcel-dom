# Change Log

## [Unreleased]
* Added basic mesh data access support.

### Added
* `v7400::data::mesh` module is added.
    + It (currently) contains mesh-related types.
      They will be read or created from FBX data, but they might not reflect
      structures of raw FBX data.
    + Polygon vertices triangulation is supported, but triangulator is currently
      not included in this crate.
      Users should prepare by themselves.
* `v7400::object::video::ClipHandle::{content,relative_filepath}()` is added.
    + With `content()`, users can get texture data.
    + With `relative_filepath()`, users can get relative filepath data.
        - Note that this is raw data and may require some processing before use.

### Mesh data access
`v7400::object::geometry::MeshHandle` now supports access to some data including
position vertices and normals.

* Control points:
    + `MeshHandle::control_points()` returns control points.
      Control points are maybe-deduplicated vertices.
    + `MeshHandle::polygon_vertex_indices()` returns polygon vertices.
      Polygon vertices are indices of control points.
    + Polygons are specified through polygon vertices, not only by control
      points.
* Layer elements:
    + `MeshHandle::layers()` returns layer elements, which contains data of
      normals, UVs, materials, etc.
    + See `v7400::data::mesh::layer::TypedLayerElementHandle` to see what kind
      of data are supported.

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

[Unreleased]: <https://github.com/lo48576/fbxcel/compare/v0.0.2...develop>
[0.0.2]: <https://github.com/lo48576/fbxcel/releases/tag/v0.0.2>
[0.0.1]: <https://github.com/lo48576/fbxcel/releases/tag/v0.0.1>

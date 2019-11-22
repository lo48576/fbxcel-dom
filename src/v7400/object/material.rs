//! `Material` object.

use rgb::RGB;

use crate::v7400::{
    data::material::{ShadingModel, ShadingModelLoader},
    object::{
        model,
        property::{
            loaders::{F64Arr3Loader, PrimitiveLoader, RgbLoader},
            ObjectProperties,
        },
        texture, ObjectHandle, TypedObjectHandle,
    },
};

define_object_subtype! {
    /// `Material` node handle.
    MaterialHandle: ObjectHandle
}

impl<'a> MaterialHandle<'a> {
    /// Returns an iterator of parent model mesh objects.
    pub fn meshes(&self) -> impl Iterator<Item = model::MeshHandle<'a>> {
        self.destination_objects()
            .filter(|obj| obj.label().is_none())
            .filter_map(|obj| obj.object_handle())
            .filter_map(|obj| match obj.get_typed() {
                TypedObjectHandle::Model(model::TypedModelHandle::Mesh(o)) => Some(o),
                _ => None,
            })
    }

    /// Returns a diffuse color texture object if available.
    pub fn diffuse_texture(&self) -> Option<texture::TextureHandle<'a>> {
        get_texture_node(self, "DiffuseColor")
    }

    /// Returns a transparent color texture object if available.
    pub fn transparent_texture(&self) -> Option<texture::TextureHandle<'a>> {
        get_texture_node(self, "TransparentColor")
    }

    /// Returns properties.
    pub fn properties(&self) -> MaterialProperties<'a> {
        // Find phong properties, then lambert.
        let phong_props = self.properties_by_native_typename("FbxSurfacePhong");
        if phong_props.has_default_properties() {
            return MaterialProperties {
                properties: phong_props,
            };
        }
        MaterialProperties {
            properties: self.properties_by_native_typename("FbxSurfaceLambert"),
        }
    }
}

/// Returns a texture object connected with the given label, if available.
fn get_texture_node<'a>(
    obj: &MaterialHandle<'a>,
    label: &str,
) -> Option<texture::TextureHandle<'a>> {
    obj.source_objects()
        .filter(|obj| obj.label() == Some(label))
        .filter_map(|obj| obj.object_handle())
        .filter_map(|obj| match obj.get_typed() {
            TypedObjectHandle::Texture(o) => Some(o),
            _ => None,
        })
        .next()
}

/// Proxy type to material properties.
#[derive(Debug, Clone, Copy)]
pub struct MaterialProperties<'a> {
    /// Properties.
    properties: ObjectProperties<'a>,
}

impl<'a> MaterialProperties<'a> {
    impl_prop_proxy_getters! {
        /// Returns shading model.
        shading_model -> ShadingModel {
            name = "ShadingModel",
            loader = ShadingModelLoader::default(),
            description = "shading model",
            default: {
                /// Returns shading model.
                ///
                /// Returns default if the value is not set.
                shading_model_or_default = ShadingModel::Unknown
            }
        }

        /// Returns multi layer flag.
        multi_layer -> bool {
            name = "MultiLayer",
            loader = PrimitiveLoader::<bool>::new(),
            description = "multi layer flag",
            default: {
                /// Returns multi layer flag.
                ///
                /// Returns default if the value is not set.
                multi_layer_or_default = false
            }
        }

        /// Returns emissive color.
        emissive_color -> RGB<f64> {
            name = "EmissiveColor",
            loader = RgbLoader::<RGB<f64>>::new(),
            description = "emissive color",
            default: {
                /// Returns emissive color.
                ///
                /// Returns default if the value is not set.
                emissive_color_or_default = RGB::from([0.0; 3])
            }
        }

        /// Returns emissive factor.
        emissive_factor -> f64 {
            name = "EmissiveFactor",
            loader = PrimitiveLoader::<f64>::new(),
            description = "emissive factor",
            default: {
                /// Returns emissive factor.
                ///
                /// Returns default if the value is not set.
                emissive_factor_or_default = 1.0
            }
        }

        /// Returns ambient color.
        ambient_color -> RGB<f64> {
            name = "AmbientColor",
            loader = RgbLoader::<RGB<f64>>::new(),
            description = "ambient color",
            default: {
                /// Returns ambient color.
                ///
                /// Returns default if the value is not set.
                ambient_color_or_default = RGB::from([0.2; 3])
            }
        }

        /// Returns ambient factor.
        ambient_factor -> f64 {
            name = "AmbientFactor",
            loader = PrimitiveLoader::<f64>::new(),
            description = "ambient factor",
            default: {
                /// Returns ambient factor.
                ///
                /// Returns default if the value is not set.
                ambient_factor_or_default = 1.0
            }
        }

        /// Returns diffuse color.
        diffuse_color -> RGB<f64> {
            name = "DiffuseColor",
            loader = RgbLoader::<RGB<f64>>::new(),
            description = "diffuse color",
            default: {
                /// Returns diffuse color.
                ///
                /// Returns default if the value is not set.
                diffuse_color_or_default = RGB::from([0.8; 3])
            }
        }

        /// Returns diffuse factor.
        diffuse_factor -> f64 {
            name = "DiffuseFactor",
            loader = PrimitiveLoader::<f64>::new(),
            description = "diffuse factor",
            default: {
                /// Returns diffuse factor.
                ///
                /// Returns default if the value is not set.
                diffuse_factor_or_default = 1.0
            }
        }

        /// Returns bump vector.
        // TODO: Is this vector? How is this intended to use?
        bump -> [f64; 3] {
            name = "Bump",
            loader = F64Arr3Loader::new(),
            description = "bump vector",
            default: {
                /// Returns bump vector.
                ///
                /// Returns default if the value is not set.
                bump_or_default = [0.0; 3]
            }
        }

        /// Returns bump factor.
        bump_factor -> f64 {
            name = "BumpFactor",
            loader = PrimitiveLoader::<f64>::new(),
            description = "bump factor",
            default: {
                /// Returns bump factor.
                ///
                /// Returns default if the value is not set.
                bump_factor_or_default = 1.0
            }
        }

        /// Returns normal map.
        // TODO: Is this vector? How is this intended to use?
        normal_map -> [f64; 3] {
            name = "NormalMap",
            loader = F64Arr3Loader::new(),
            description = "normal map",
            default: {
                /// Returns normal map.
                ///
                /// Returns default if the value is not set.
                normal_map_or_default = [0.0; 3]
            }
        }

        /// Returns transparent color.
        transparent_color -> RGB<f64> {
            name = "TransparentColor",
            loader = RgbLoader::<RGB<f64>>::new(),
            description = "transparent color",
            default: {
                /// Returns transparent color.
                ///
                /// Returns default if the value is not set.
                transparent_color_or_default = RGB::from([0.0; 3])
            }
        }

        /// Returns transparency factor.
        transparency_factor -> f64 {
            name = "TransparencyFactor",
            loader = PrimitiveLoader::<f64>::new(),
            description = "transparency factor",
            default: {
                /// Returns transparency factor.
                ///
                /// Returns default if the value is not set.
                transparency_factor_or_default = 0.0
            }
        }

        /// Returns displacement color.
        displacement_color -> RGB<f64> {
            name = "DisplacementColor",
            loader = RgbLoader::<RGB<f64>>::new(),
            description = "displacement color",
            default: {
                /// Returns displacement color.
                ///
                /// Returns default if the value is not set.
                displacement_color_or_default = RGB::from([0.0; 3])
            }
        }

        /// Returns displacement factor.
        displacement_factor -> f64 {
            name = "DisplacementFactor",
            loader = PrimitiveLoader::<f64>::new(),
            description = "displacement factor",
            default: {
                /// Returns displacement factor.
                ///
                /// Returns default if the value is not set.
                displacement_factor_or_default = 1.0
            }
        }

        /// Returns vector displacement color.
        vector_displacement_color -> RGB<f64> {
            name = "VectorDisplacementColor",
            loader = RgbLoader::<RGB<f64>>::new(),
            description = "vector displacement color",
            default: {
                /// Returns vector displacement color.
                ///
                /// Returns default if the value is not set.
                vector_displacement_color_or_default = RGB::from([0.0; 3])
            }
        }

        /// Returns vector displacement factor.
        vector_displacement_factor -> f64 {
            name = "VectorDisplacementFactor",
            loader = PrimitiveLoader::<f64>::new(),
            description = "vector displacement factor",
            default: {
                /// Returns vector displacement factor.
                ///
                /// Returns default if the value is not set.
                vector_displacement_factor_or_default = 1.0
            }
        }

        /// Returns specular color.
        specular -> RGB<f64> {
            name = "SpecularColor",
            loader = RgbLoader::<RGB<f64>>::new(),
            description = "specular color",
            default: {
                /// Returns specular color.
                ///
                /// Returns default if the value is not set.
                specular_or_default = RGB::from([0.2; 3])
            }
        }

        /// Returns specular factor.
        specular_factor -> f64 {
            name = "SpecularFactor",
            loader = PrimitiveLoader::<f64>::new(),
            description = "specular factor",
            default: {
                /// Returns specular color.
                ///
                /// Returns default if the value is not set.
                specular_factor_or_default = 1.0
            }
        }

        /// Returns shininess.
        shininess -> f64 {
            name = "ShininessExponent",
            loader = PrimitiveLoader::<f64>::new(),
            description = "shininess",
            default: {
                /// Returns shininess.
                ///
                /// Returns default if the value is not set.
                shininess_or_default = 20.0
            }
        }

        /// Returns reflection color.
        reflection -> RGB<f64> {
            name = "ReflectionColor",
            loader = RgbLoader::<RGB<f64>>::new(),
            description = "reflection color",
            default: {
                /// Returns reflection color.
                ///
                /// Returns default if the value is not set.
                reflection_or_default = RGB::from([0.2; 3])
            }
        }

        /// Returns reflection factor.
        reflection_factor -> f64 {
            name = "ReflectionFactor",
            loader = PrimitiveLoader::<f64>::new(),
            description = "reflection factor",
            default: {
                /// Returns reflection color.
                ///
                /// Returns default if the value is not set.
                reflection_factor_or_default = 1.0
            }
        }
    }
}

impl<'a> std::ops::Deref for MaterialProperties<'a> {
    type Target = ObjectProperties<'a>;

    fn deref(&self) -> &Self::Target {
        &self.properties
    }
}

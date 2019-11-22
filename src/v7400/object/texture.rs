//! `Texture` object.

use mint::{Point3, Vector3};

use crate::v7400::{
    data::texture::{BlendMode, BlendModeLoader, WrapMode, WrapModeLoader},
    object::{
        property::{
            loaders::{BorrowedStringLoader, F64Arr3Loader, MintLoader, PrimitiveLoader},
            ObjectProperties,
        },
        video, ObjectHandle, TypedObjectHandle,
    },
};

define_object_subtype! {
    /// `Texture` node handle.
    TextureHandle: ObjectHandle
}

impl<'a> TextureHandle<'a> {
    /// Returns a video clip object if available.
    pub fn video_clip(&self) -> Option<video::ClipHandle<'a>> {
        self.source_objects()
            .filter(|obj| obj.label().is_none())
            .filter_map(|obj| obj.object_handle())
            .filter_map(|obj| match obj.get_typed() {
                TypedObjectHandle::Video(video::TypedVideoHandle::Clip(o)) => Some(o),
                _ => None,
            })
            .next()
    }

    /// Returns properties.
    pub fn properties(&self) -> TextureProperties<'a> {
        TextureProperties {
            properties: self.properties_by_native_typename("FbxFileTexture"),
        }
    }
}

/// Proxy type to texture properties.
#[derive(Debug, Clone, Copy)]
pub struct TextureProperties<'a> {
    /// Properties.
    properties: ObjectProperties<'a>,
}

impl<'a> TextureProperties<'a> {
    impl_prop_proxy_getters! {
        /// Returns default alpha value.
        alpha -> f64 {
            name = "Texture alpha",
            loader = PrimitiveLoader::<f64>::new(),
            description = "texture alpha value",
            default: {
                /// Returns default alpha value.
                ///
                /// Returns default if the value is not set.
                alpha_or_default = 1.0
            }
        }

        /// Returns wrap mode U.
        wrap_mode_u -> WrapMode {
            name = "WrapModeU",
            loader = WrapModeLoader::default(),
            description = "wrap mode U",
            default: {
                /// Returns wrap mode U.
                ///
                /// Returns default if the value is not set.
                wrap_mode_u_or_default = WrapMode::Repeat
            }
        }

        /// Returns wrap mode V.
        wrap_mode_v -> WrapMode {
            name = "WrapModeV",
            loader = WrapModeLoader::default(),
            description = "wrap mode V",
            default: {
                /// Returns wrap mode V.
                ///
                /// Returns default if the value is not set.
                wrap_mode_v_or_default = WrapMode::Repeat
            }
        }

        /// Returns whether the UV should be swapped or not.
        ///
        /// Returns `Some(true)` if UV should be swapped.
        uv_swap -> bool {
            name = "UVSwap",
            loader = PrimitiveLoader::<bool>::new(),
            description = "UV swap flag",
            default: {
                /// Returns whether the UV should be swapped or not.
                ///
                /// Returns `true` if UV should be swapped.
                ///
                /// Returns default if the value is not set.
                uv_swap_or_default = false
            }
        }

        /// Returns premultiply-alpha flag.
        ///
        /// Returns `Some(true)` if the alpha is premultiplied.
        premultiply_alpha -> bool {
            name = "PremultiplyAlpha",
            loader = PrimitiveLoader::<bool>::new(),
            description = "premultiply-alpha flag",
            default: {
                /// Returns premultiply-alpha flag.
                ///
                /// Returns `true` if the alpha is premultiplied.
                ///
                /// Returns default if the value is not set.
                premultiply_alpha_or_default = false
            }
        }

        /// Returns default translation vector.
        translation -> Vector3<f64> {
            name = "Translation",
            loader = MintLoader::<Vector3<f64>>::new(),
            description = "translation vector",
            default: {
                /// Returns default translation vector.
                ///
                /// Returns default if the value is not set.
                translation_or_default = Vector3 { x: 0.0, y: 0.0, z: 0.0 }
            }
        }

        /// Returns default rotation vector.
        // TODO: I'm not sure which type to use here, `mint::Vector3` or
        // `mint::Euler`.
        rotation -> [f64; 3] {
            name = "Rotation",
            loader = F64Arr3Loader::new(),
            description = "rotation vector",
            default: {
                /// Returns default rotation vector.
                ///
                /// Returns default if the value is not set.
                rotation_or_default = [0.0; 3]
            }
        }

        /// Returns default scaling vector.
        scaling -> Vector3<f64> {
            name = "Scaling",
            loader = MintLoader::<Vector3<f64>>::new(),
            description = "scaling vector",
            default: {
                /// Returns default scaling vector.
                ///
                /// Returns default if the value is not set.
                scaling_or_default = Vector3 { x: 1.0, y: 1.0, z: 1.0 }
            }
        }

        /// Returns rotation pivot vector.
        rotation_pivot -> Point3<f64> {
            name = "TextureRotationPivot",
            loader = MintLoader::<Point3<f64>>::new(),
            description = "rotation pivot vector",
            default: {
                /// Returns rotation pivot vector.
                ///
                /// Returns default if the value is not set.
                rotation_pivot_or_default = Point3 { x: 0.0, y: 0.0, z: 0.0 }
            }
        }

        /// Returns rotation pivot vector.
        scaling_pivot -> Point3<f64> {
            name = "TextureScalingPivot",
            loader = MintLoader::<Point3<f64>>::new(),
            description = "scaling pivot vector",
            default: {
                /// Returns scaling pivot vector.
                ///
                /// Returns default if the value is not set.
                scaling_pivot_or_default = Point3 { x: 0.0, y: 0.0, z: 0.0 }
            }
        }

        /// Returns texture blend mode.
        blend_mode -> BlendMode {
            name = "CurrentTextureBlendMode",
            loader = BlendModeLoader::default(),
            description = "texture blend mode",
            default: {
                /// Returns texture blend mode.
                ///
                /// Returns default if the value is not set.
                blend_mode_or_default = BlendMode::Additive
            }
        }

        /// Returns UV set name.
        uv_set -> &'a str {
            name = "UVSet",
            loader = BorrowedStringLoader::new(),
            description = "UV set name",
            default: {
                /// Returns UV set name.
                ///
                /// Returns default if the value is not set.
                uv_set_or_default = "default"
            }
        }
    }
}

impl<'a> std::ops::Deref for TextureProperties<'a> {
    type Target = ObjectProperties<'a>;

    fn deref(&self) -> &Self::Target {
        &self.properties
    }
}

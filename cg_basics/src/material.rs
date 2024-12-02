use colors::Color;
use image::Image;

pub struct UnshadedMaterial<I: Image> {
    pub texture: I,
}

impl<I: Image> UnshadedMaterial<I> {
    pub fn new(texture: I) -> UnshadedMaterial<I> {
        UnshadedMaterial { texture }
    }
}

pub struct LambertMaterial<I: Image> {
    pub texture: I,
}

impl<I: Image> LambertMaterial<I> {
    pub fn new(texture: I) -> LambertMaterial<I> {
        LambertMaterial { texture }
    }
}

pub struct PhongMaterial<I: Image> {
    pub diffuse_texture: I,
    pub specular_texture: I,
    pub exponent: <<I as Image>::ColorType as Color>::ChannelType,
}

impl<I: Image> PhongMaterial<I> {
    pub fn new(
        diffuse_texture: I,
        specular_texture: I,
        exponent: <<I as Image>::ColorType as Color>::ChannelType,
    ) -> PhongMaterial<I> {
        PhongMaterial {
            diffuse_texture,
            specular_texture,
            exponent,
        }
    }
}

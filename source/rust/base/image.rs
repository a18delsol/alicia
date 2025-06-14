/*
* Copyright (c) 2025 luxreduxdelux
*
* Redistribution and use in source and binary forms, with or without
* modification, are permitted provided that the following conditions are met:
*
* 1. Redistributions of source code must retain the above copyright notice,
* this list of conditions and the following disclaimer.
*
* 2. Redistributions in binary form must reproduce the above copyright notice,
* this list of conditions and the following disclaimer in the documentation
* and/or other materials provided with the distribution.
*
* Subject to the terms and conditions of this license, each copyright holder
* and contributor hereby grants to those receiving rights under this license
* a perpetual, worldwide, non-exclusive, no-charge, royalty-free, irrevocable
* (except for failure to satisfy the conditions of this license) patent license
* to make, have made, use, offer to sell, sell, import, and otherwise transfer
* this software, where such license applies only to those patent claims, already
* acquired or hereafter acquired, licensable by such copyright holder or
* contributor that are necessarily infringed by:
*
* (a) their Contribution(s) (the licensed copyrights of copyright holders and
* non-copyrightable additions of contributors, in source or binary form) alone;
* or
*
* (b) combination of their Contribution(s) with the work of authorship to which
* such Contribution(s) was added by such copyright holder or contributor, if,
* at the time the Contribution is added, such addition causes such combination
* to be necessarily infringed. The patent license shall not apply to any other
* combinations which include the Contribution.
*
* Except as expressly stated above, no rights or licenses from any copyright
* holder or contributor is granted under this license, whether expressly, by
* implication, estoppel or otherwise.
*
* DISCLAIMER
*
* THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
* AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
* IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
* DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDERS OR CONTRIBUTORS BE LIABLE
* FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
* DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
* SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
* CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
* OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
* OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

use crate::script::*;
use crate::status::*;

//================================================================

use crate::base::helper::*;
use mlua::prelude::*;

//================================================================

//================================================================

/* class
{ "version": "1.0.0", "name": "alicia.image", "info": "The image API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table, _: &StatusInfo, _: Option<&ScriptInfo>) -> mlua::Result<()> {
    let image = lua.create_table()?;

    image.set("new",             lua.create_async_function(self::LuaImage::new)?)?;       // LoadImage
    image.set("new_from_memory", lua.create_function(self::LuaImage::new_from_memory)?)?; // LoadImageFromMemory
    image.set("new_from_screen", lua.create_function(self::LuaImage::new_from_screen)?)?; // LoadImageFromScreen

    //================================================================

    image.set("new_color",           lua.create_function(self::LuaImage::new_color)?)?;           // GenImageColor
    image.set("new_gradient_linear", lua.create_function(self::LuaImage::new_gradient_linear)?)?; // GenImageGradientLinear
    image.set("new_gradient_radial", lua.create_function(self::LuaImage::new_gradient_radial)?)?; // GenImageGradientRadial
    image.set("new_gradient_square", lua.create_function(self::LuaImage::new_gradient_square)?)?; // GenImageGradientSquare
    image.set("new_check",           lua.create_function(self::LuaImage::new_check)?)?;           // GenImageChecked
    image.set("new_white_noise",     lua.create_function(self::LuaImage::new_white_noise)?)?;     // GenImageWhiteNoise
    image.set("new_perlin_noise",    lua.create_function(self::LuaImage::new_perlin_noise)?)?;    // GenImagePerlinNoise
    image.set("new_cellular",        lua.create_function(self::LuaImage::new_cellular)?)?;        // GenImageCellular
    image.set("new_text",            lua.create_function(self::LuaImage::new_text)?)?;            // GenImageText

    table.set("image", image)?;

    Ok(())
}

/* class
{
    "version": "1.0.0",
    "name": "image",
    "info": "An unique handle for a image in memory.",
    "member": [
        { "name": "shape_x", "info": "Shape of the image (X).", "kind": "number" },
        { "name": "shape_y", "info": "Shape of the image (Y).", "kind": "number" }
    ]
}
*/
pub struct LuaImage(pub Image);

impl Drop for LuaImage {
    fn drop(&mut self) {
        unsafe {
            UnloadImage(self.0);
        }
    }
}

unsafe impl Send for LuaImage {}

impl mlua::UserData for LuaImage {
    fn add_fields<F: mlua::UserDataFields<Self>>(field: &mut F) {
        field.add_field_method_get("shape_x", |_: &Lua, this| Ok(this.0.width));
        field.add_field_method_get("shape_y", |_: &Lua, this| Ok(this.0.height));
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        // TO-DO add Export, ExportToMemory

        /* entry
        {
            "version": "1.0.0",
            "name": "image:to_texture",
            "info": "Get a texture resource from an image.",
            "result": [
                { "name": "texture", "info": "Texture resource.", "kind": "texture" }
            ]
        }
        */
        method.add_method_mut("to_texture", |_: &Lua, this, _: ()| {
            Ok(crate::base::texture::LuaTexture::new_from_image(this.0))
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "image:power_of_two",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("power_of_two", |lua: &Lua, this, color: LuaValue| {
            let color: Color = lua.from_value(color)?;

            unsafe {
                ImageToPOT(&mut this.0, color);
                Ok(())
            }
        });

        // add ImageFormat

        /* entry
        {
            "version": "1.0.0",
            "name": "image:crop",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("crop", |lua: &Lua, this, box_a: LuaValue| {
            let box_a: Rectangle = lua.from_value(box_a)?;

            unsafe {
                ImageCrop(&mut this.0, box_a);
                Ok(())
            }
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "image:crop_alpha",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("crop_alpha", |_: &Lua, this, threshold: f32| unsafe {
            ImageAlphaCrop(&mut this.0, threshold);
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "image:crop_alpha",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "crop_alpha_clear",
            |lua: &Lua, this, (color, threshold): (LuaValue, f32)| {
                let color: Color = lua.from_value(color)?;

                unsafe {
                    ImageAlphaClear(&mut this.0, color, threshold);
                    Ok(())
                }
            },
        );

        // TO-DO add ImageAlphaMask, ImageAlphaPremultiply

        /* entry
        {
            "version": "1.0.0",
            "name": "image:blur_gaussian",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("blur_gaussian", |_: &Lua, this, amount: i32| unsafe {
            ImageBlurGaussian(&mut this.0, amount);
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "image:kernel_convolution",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "kernel_convolution",
            |_: &Lua, this, kernel: Vec<f32>| unsafe {
                ImageKernelConvolution(&mut this.0, kernel.as_ptr(), kernel.len() as i32);
                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "image:resize",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "resize",
            |lua: &Lua, this, (shape, bicubic): (LuaValue, bool)| unsafe {
                let shape: Vector2 = lua.from_value(shape)?;

                if bicubic {
                    ImageResize(&mut this.0, shape.x as i32, shape.y as i32);
                } else {
                    ImageResizeNN(&mut this.0, shape.x as i32, shape.y as i32);
                }
                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "image:extend",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "extend",
            |lua: &Lua, this, (shape, shift, color): (LuaValue, LuaValue, LuaValue)| unsafe {
                let shape: Vector2 = lua.from_value(shape)?;
                let shift: Vector2 = lua.from_value(shift)?;
                let color: Color = lua.from_value(color)?;

                ImageResizeCanvas(
                    &mut this.0,
                    shape.x as i32,
                    shape.y as i32,
                    shift.x as i32,
                    shift.y as i32,
                    color,
                );
                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "image:mipmap",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("mipmap", |_: &Lua, this, _: ()| unsafe {
            ImageMipmaps(&mut this.0);
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "image:dither",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "dither",
            |_: &Lua, this, (r_bpp, g_bpp, b_bpp, a_bpp): (i32, i32, i32, i32)| unsafe {
                ImageDither(&mut this.0, r_bpp, g_bpp, b_bpp, a_bpp);
                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "image:flip",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("flip", |_: &Lua, this, vertical: bool| unsafe {
            if vertical {
                ImageFlipVertical(&mut this.0);
            } else {
                ImageFlipHorizontal(&mut this.0);
            }
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "image:rotate",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("rotate", |_: &Lua, this, angle: i32| unsafe {
            ImageRotate(&mut this.0, angle);
            Ok(())
        });

        // ImageRotateCW/CCW don't really need to exist when ImageRotate is already a thing...not bound on purpose.

        /* entry
        {
            "version": "1.0.0",
            "name": "image:color_tint",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("color_tint", |lua: &Lua, this, color: LuaValue| unsafe {
            let color: Color = lua.from_value(color)?;

            ImageColorTint(&mut this.0, color);
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "image:color_invert",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("color_invert", |_: &Lua, this, _: ()| unsafe {
            ImageColorInvert(&mut this.0);
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "image:color_gray_scale",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("color_gray_scale", |_: &Lua, this, _: ()| unsafe {
            ImageColorGrayscale(&mut this.0);
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "image:color_contrast",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("color_contrast", |_: &Lua, this, contrast: f32| unsafe {
            ImageColorContrast(&mut this.0, contrast);
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "image:color_contrast",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "color_replace",
            |lua: &Lua, this, (color_a, color_b): (LuaValue, LuaValue)| unsafe {
                let color_a: Color = lua.from_value(color_a)?;
                let color_b: Color = lua.from_value(color_b)?;

                ImageColorReplace(&mut this.0, color_a, color_b);
                Ok(())
            },
        );

        // TO-DO add LoadImageColors/Palette?

        /* entry
        {
            "version": "1.0.0",
            "name": "image:get_alpha_border",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_alpha_border", |_: &Lua, this, threshold: f32| unsafe {
            let value = GetImageAlphaBorder(this.0, threshold);
            Ok((value.x, value.y, value.width, value.height))
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "image:get_alpha_border",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_color", |lua: &Lua, this, point: LuaValue| unsafe {
            let point: Vector2 = lua.from_value(point)?;

            let value = GetImageColor(this.0, point.x as i32, point.y as i32);
            Ok((value.r, value.g, value.b, value.a))
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "image:draw_pixel",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("draw_pixel", |lua: &Lua, this, (point, color)| unsafe {
            let point: Vector2 = lua.from_value(point)?;
            let color: Color = lua.from_value(color)?;

            ImageDrawPixelV(&mut this.0, point, color);
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "image:draw_line",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "draw_line",
            |lua: &Lua, this, (point_a, point_b, thickness, color)| unsafe {
                let point_a: Vector2 = lua.from_value(point_a)?;
                let point_b: Vector2 = lua.from_value(point_b)?;
                let color: Color = lua.from_value(color)?;

                ImageDrawLineEx(&mut this.0, point_a, point_b, thickness, color);
                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "image:draw_circle",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "draw_circle",
            |lua: &Lua, this, (point, radius, color)| unsafe {
                let point: Vector2 = lua.from_value(point)?;
                let color: Color = lua.from_value(color)?;

                ImageDrawCircleV(&mut this.0, point, radius, color);
                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "image:draw_circle_line",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "draw_circle_line",
            |lua: &Lua, this, (point, radius, color)| unsafe {
                let point: Vector2 = lua.from_value(point)?;
                let color: Color = lua.from_value(color)?;

                ImageDrawCircleLinesV(&mut this.0, point, radius, color);
                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "image:draw_box_2",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("draw_box_2", |lua: &Lua, this, (box_a, color)| unsafe {
            let box_a: Rectangle = lua.from_value(box_a)?;
            let color: Color = lua.from_value(color)?;

            ImageDrawRectangleRec(&mut this.0, box_a, color);
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "image:draw_box_2_line",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "draw_box_2_line",
            |lua: &Lua, this, (box_a, thickness, color)| unsafe {
                let box_a: Rectangle = lua.from_value(box_a)?;
                let color: Color = lua.from_value(color)?;

                ImageDrawRectangleLines(&mut this.0, box_a, thickness, color);
                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "image:draw_triangle",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "draw_triangle",
            |lua: &Lua, this, (point_a, point_b, point_c, color_a, color_b, color_c)| unsafe {
                let point_a: Vector2 = lua.from_value(point_a)?;
                let point_b: Vector2 = lua.from_value(point_b)?;
                let point_c: Vector2 = lua.from_value(point_c)?;
                let color_a: Color = lua.from_value(color_a)?;
                let color_b: Color = lua.from_value(color_b)?;
                let color_c: Color = lua.from_value(color_c)?;

                ImageDrawTriangleEx(
                    &mut this.0,
                    point_a,
                    point_b,
                    point_c,
                    color_a,
                    color_b,
                    color_c,
                );
                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "image:draw_triangle_line",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "draw_triangle_line",
            |lua: &Lua, this, (point_a, point_b, point_c, color_a)| unsafe {
                let point_a: Vector2 = lua.from_value(point_a)?;
                let point_b: Vector2 = lua.from_value(point_b)?;
                let point_c: Vector2 = lua.from_value(point_c)?;
                let color_a: Color = lua.from_value(color_a)?;

                ImageDrawTriangleLines(&mut this.0, point_a, point_b, point_c, color_a);
                Ok(())
            },
        );

        // TO-DO add ImageDrawTriangleFan/Strip, ImageDraw, DrawTextEx
    }
}

impl LuaImage {
    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.image.new",
        "info": "Create a new image resource.",
        "member": [
            { "name": "path", "info": "Path to image file.", "kind": "string" }
        ],
        "result": [
            { "name": "image", "info": "LuaImage resource.", "kind": "image" }
        ],
        "routine": true
    }
    */
    async fn new(lua: Lua, path: String) -> mlua::Result<Self> {
        let name = ScriptData::get_path(&lua, &path)?;
        let name = Script::rust_to_c_string(&name)?;

        tokio::task::spawn_blocking(move || unsafe {
            let data = LoadImage(name.as_ptr());

            if IsImageValid(data) {
                Ok(Self(data))
            } else {
                Err(mlua::Error::RuntimeError(format!(
                    "LuaImage::new(): Could not load file \"{path}\"."
                )))
            }
        })
        .await
        .unwrap()
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.image.new_from_memory",
        "info": "Create a new image resource, from memory.",
        "member": [
            { "name": "data", "info": "The data buffer.",                     "kind": "data"   },
            { "name": "kind", "info": "The kind of image file (.png, etc.).", "kind": "string" }
        ],
        "result": [
            { "name": "image", "info": "LuaImage resource.", "kind": "image" }
        ],
        "routine": true
    }
    */
    pub fn new_from_memory(_: &Lua, (data, kind): (LuaValue, String)) -> mlua::Result<Self> {
        let data = crate::base::data::Data::get_buffer(data)?;

        unsafe {
            let data = &*data.0;
            let kind = Script::rust_to_c_string(&kind)?;

            let data = LoadImageFromMemory(kind.as_ptr(), data.as_ptr(), data.len() as i32);

            if IsImageValid(data) {
                Ok(Self(data))
            } else {
                Err(mlua::Error::RuntimeError(
                    "LuaImage::new_from_memory(): Could not load file.".to_string(),
                ))
            }
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.image.new_from_screen",
        "info": "Create a new image resource, from the current screen buffer.",
        "result": [
            { "name": "image", "info": "LuaImage resource.", "kind": "image" }
        ],
        "routine": true
    }
    */
    fn new_from_screen(_: &Lua, _: ()) -> mlua::Result<Self> {
        unsafe {
            let data = LoadImageFromScreen();

            if IsImageValid(data) {
                Ok(Self(data))
            } else {
                Err(mlua::Error::RuntimeError(
                    "LuaImage::new_from_screen(): Could not create image.".to_string(),
                ))
            }
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.image.new_color",
        "info": "TO-DO",
        "routine": true
    }
    */
    fn new_color(lua: &Lua, (shape, color): (LuaValue, LuaValue)) -> mlua::Result<Self> {
        let shape: Vector2 = lua.from_value(shape)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            let data = GenImageColor(shape.x as i32, shape.y as i32, color);

            if IsImageValid(data) {
                Ok(Self(data))
            } else {
                Err(mlua::Error::RuntimeError(
                    "LuaImage::new_color(): Could not create image.".to_string(),
                ))
            }
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.image.new_gradient_linear",
        "info": "TO-DO",
        "routine": true
    }
    */
    fn new_gradient_linear(
        lua: &Lua,
        (shape, direction, color_a, color_b): (LuaValue, i32, LuaValue, LuaValue),
    ) -> mlua::Result<Self> {
        let shape: Vector2 = lua.from_value(shape)?;
        let color_a: Color = lua.from_value(color_a)?;
        let color_b: Color = lua.from_value(color_b)?;

        unsafe {
            let data =
                GenImageGradientLinear(shape.x as i32, shape.y as i32, direction, color_a, color_b);

            if IsImageValid(data) {
                Ok(Self(data))
            } else {
                Err(mlua::Error::RuntimeError(
                    "LuaImage::new_gradient_linear(): Could not create image.".to_string(),
                ))
            }
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.image.new_gradient_radial",
        "info": "TO-DO",
        "routine": true
    }
    */
    fn new_gradient_radial(
        lua: &Lua,
        (shape, density, color_a, color_b): (LuaValue, f32, LuaValue, LuaValue),
    ) -> mlua::Result<Self> {
        let shape: Vector2 = lua.from_value(shape)?;
        let color_a: Color = lua.from_value(color_a)?;
        let color_b: Color = lua.from_value(color_b)?;

        unsafe {
            let data =
                GenImageGradientRadial(shape.x as i32, shape.y as i32, density, color_a, color_b);

            if IsImageValid(data) {
                Ok(Self(data))
            } else {
                Err(mlua::Error::RuntimeError(
                    "LuaImage::new_gradient_radial(): Could not create image.".to_string(),
                ))
            }
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.image.new_gradient_square",
        "info": "TO-DO",
        "routine": true
    }
    */
    fn new_gradient_square(
        lua: &Lua,
        (shape, density, color_a, color_b): (LuaValue, f32, LuaValue, LuaValue),
    ) -> mlua::Result<Self> {
        let shape: Vector2 = lua.from_value(shape)?;
        let color_a: Color = lua.from_value(color_a)?;
        let color_b: Color = lua.from_value(color_b)?;

        unsafe {
            let data =
                GenImageGradientSquare(shape.x as i32, shape.y as i32, density, color_a, color_b);

            if IsImageValid(data) {
                Ok(Self(data))
            } else {
                Err(mlua::Error::RuntimeError(
                    "LuaImage::new_gradient_square(): Could not create image.".to_string(),
                ))
            }
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.image.new_check",
        "info": "TO-DO",
        "routine": true
    }
    */
    fn new_check(
        lua: &Lua,
        (shape, check, color_a, color_b): (LuaValue, LuaValue, LuaValue, LuaValue),
    ) -> mlua::Result<Self> {
        let shape: Vector2 = lua.from_value(shape)?;
        let check: Vector2 = lua.from_value(check)?;
        let color_a: Color = lua.from_value(color_a)?;
        let color_b: Color = lua.from_value(color_b)?;

        unsafe {
            let data = GenImageChecked(
                shape.x as i32,
                shape.y as i32,
                check.x as i32,
                check.y as i32,
                color_a,
                color_b,
            );

            if IsImageValid(data) {
                Ok(Self(data))
            } else {
                Err(mlua::Error::RuntimeError(
                    "LuaImage::new_check(): Could not create image.".to_string(),
                ))
            }
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.image.new_white_noise",
        "info": "TO-DO",
        "routine": true
    }
    */
    fn new_white_noise(lua: &Lua, (shape, factor): (LuaValue, f32)) -> mlua::Result<Self> {
        let shape: Vector2 = lua.from_value(shape)?;

        unsafe {
            let data = GenImageWhiteNoise(shape.x as i32, shape.y as i32, factor);

            if IsImageValid(data) {
                Ok(Self(data))
            } else {
                Err(mlua::Error::RuntimeError(
                    "LuaImage::new_white_noise(): Could not create image.".to_string(),
                ))
            }
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.image.new_perlin_noise",
        "info": "TO-DO",
        "routine": true
    }
    */
    fn new_perlin_noise(
        lua: &Lua,
        (shape, shift, scale): (LuaValue, LuaValue, f32),
    ) -> mlua::Result<Self> {
        let shape: Vector2 = lua.from_value(shape)?;
        let shift: Vector2 = lua.from_value(shift)?;

        unsafe {
            let data = GenImagePerlinNoise(
                shape.x as i32,
                shape.y as i32,
                shift.x as i32,
                shift.y as i32,
                scale,
            );

            if IsImageValid(data) {
                Ok(Self(data))
            } else {
                Err(mlua::Error::RuntimeError(
                    "LuaImage::new_perlin_noise(): Could not create image.".to_string(),
                ))
            }
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.image.new_cellular",
        "info": "TO-DO",
        "routine": true
    }
    */
    fn new_cellular(lua: &Lua, (shape, tile_size): (LuaValue, i32)) -> mlua::Result<Self> {
        let shape: Vector2 = lua.from_value(shape)?;

        unsafe {
            let data = GenImageCellular(shape.x as i32, shape.y as i32, tile_size);

            if IsImageValid(data) {
                Ok(Self(data))
            } else {
                Err(mlua::Error::RuntimeError(
                    "LuaImage::new_cellular(): Could not create image.".to_string(),
                ))
            }
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.image.new_text",
        "info": "TO-DO",
        "routine": true
    }
    */
    fn new_text(lua: &Lua, (shape, text): (LuaValue, String)) -> mlua::Result<Self> {
        let shape: Vector2 = lua.from_value(shape)?;

        unsafe {
            let data = GenImageText(
                shape.x as i32,
                shape.y as i32,
                Script::rust_to_c_string(&text)?.as_ptr(),
            );

            if IsImageValid(data) {
                Ok(Self(data))
            } else {
                Err(mlua::Error::RuntimeError(
                    "LuaImage::new_text(): Could not create image.".to_string(),
                ))
            }
        }
    }

    pub fn new_from_texture(texture: Texture) -> mlua::Result<Self> {
        unsafe {
            let data = LoadImageFromTexture(texture);

            if IsImageValid(data) {
                Ok(Self(data))
            } else {
                Err(mlua::Error::RuntimeError(
                    "LuaImage::new_from_texture(): Could not load file.".to_string(),
                ))
            }
        }
    }
}

/*
* Copyright (c) 2025 a18delsol
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

/* class
{ "version": "1.0.0", "name": "alicia.texture", "info": "The texture API.", "head": true }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table, _: &StatusInfo, _: Option<&ScriptInfo>) -> mlua::Result<()> {
    let texture = lua.create_table()?;

    texture.set("new",             lua.create_function(self::LuaTexture::new)?)?;
    //texture.set("new_from_memory", lua.create_function(self::LuaTexture::new_from_memory)?)?;

    table.set("texture", texture)?;

    let render_texture = lua.create_table()?;

    render_texture.set("new", lua.create_function(self::LuaRenderTexture::new)?)?;

    table.set("render_texture", render_texture)?;

    Ok(())
}

pub fn texture_draw(
    lua: &Lua,
    (texture, point, angle, scale, color): (Texture, LuaValue, f32, f32, LuaValue),
) -> mlua::Result<()> {
    let point: Vector2 = lua.from_value(point)?;
    let color: Color = lua.from_value(color)?;

    unsafe {
        DrawTextureEx(texture, point, angle, scale, color);
        Ok(())
    }
}

pub fn texture_pro_draw(
    lua: &Lua,
    (texture, rec_a, rec_b, point, angle, color): (
        Texture,
        LuaValue,
        LuaValue,
        LuaValue,
        f32,
        LuaValue,
    ),
) -> mlua::Result<()> {
    let rec_a: Rectangle = lua.from_value(rec_a)?;
    let rec_b: Rectangle = lua.from_value(rec_b)?;
    let point: Vector2 = lua.from_value(point)?;
    let color: Color = lua.from_value(color)?;

    unsafe {
        DrawTexturePro(texture, rec_a, rec_b, point, angle, color);
        Ok(())
    }
}

pub fn texture_draw_billboard(
    lua: &Lua,
    (texture, camera, point, scale, color): (Texture, LuaValue, LuaValue, f32, LuaValue),
) -> mlua::Result<()> {
    let camera: Camera3D = lua.from_value(camera)?;
    let point: Vector3 = lua.from_value(point)?;
    let color: Color = lua.from_value(color)?;

    unsafe {
        DrawBillboard(camera, texture, point, scale, color);
        Ok(())
    }
}

pub fn texture_draw_billboard_pro(
    lua: &Lua,
    (texture, camera, source, point, up, scale, origin, angle, color): (
        Texture,
        LuaValue,
        LuaValue,
        LuaValue,
        LuaValue,
        LuaValue,
        LuaValue,
        f32,
        LuaValue,
    ),
) -> mlua::Result<()> {
    let camera: Camera3D = lua.from_value(camera)?;
    let source: Rectangle = lua.from_value(source)?;
    let point: Vector3 = lua.from_value(point)?;
    let up: Vector3 = lua.from_value(up)?;
    let scale: Vector2 = lua.from_value(scale)?;
    let origin: Vector2 = lua.from_value(origin)?;
    let color: Color = lua.from_value(color)?;

    unsafe {
        DrawBillboardPro(
            camera, texture, source, point, up, scale, origin, angle, color,
        );
        Ok(())
    }
}

#[rustfmt::skip]
fn texture_draw_plane(lua: &Lua,
    (texture, source, point_a, point_b, point_c, point_d, color): (
        Texture,
        LuaValue,
        LuaValue,
        LuaValue,
        LuaValue,
        LuaValue,
        LuaValue,
    ), ) -> mlua::Result<()> {
    let source: Rectangle = lua.from_value(source)?;
    let point_a: Vector3 = lua.from_value(point_a)?;
    let point_b: Vector3 = lua.from_value(point_b)?;
    let point_c: Vector3 = lua.from_value(point_c)?;
    let point_d: Vector3 = lua.from_value(point_d)?;
    let color: Color = lua.from_value(color)?;

    unsafe {
        // TO-DO port
        /*
        rlSetTexture(texture.id);

            let tx = source.x;
            let ty = source.y;
            let tw = source.width;
            let th = source.height;

            rlBegin(RL_QUADS as i32);

                rlColor4ub(color.r, color.g, color.b, color.a);

                rlNormal3f(0.0, 1.0, 0.0);
                rlTexCoord2f(tx, ty); rlVertex3f(point_a.x, point_a.y, point_a.z);
                rlTexCoord2f(tx, th); rlVertex3f(point_b.x, point_b.y, point_b.z);
                rlTexCoord2f(tw, th); rlVertex3f(point_c.x, point_c.y, point_c.z);
                rlTexCoord2f(tw, ty); rlVertex3f(point_d.x, point_d.y, point_d.z);

            rlEnd();

        rlSetTexture(0);
        */

        Ok(())
    }
}

/* class
{
    "version": "1.0.0",
    "name": "texture",
    "info": "An unique handle for a texture in memory.",
    "member": [
        { "name": "shape_x", "info": "Shape of the texture (X).", "kind": "number" },
        { "name": "shape_y", "info": "Shape of the texture (Y).", "kind": "number" }
    ]
}
*/
struct LuaTexture(Texture);

impl Drop for LuaTexture {
    fn drop(&mut self) {
        unsafe {
            println!("dropping texture");
            UnloadTexture(self.0);
        }
    }
}

unsafe impl Send for LuaTexture {}

impl mlua::UserData for LuaTexture {
    fn add_fields<F: mlua::UserDataFields<Self>>(field: &mut F) {
        field.add_field_method_get("ID", |_: &Lua, this| Ok(this.0.id));
        field.add_field_method_get("shape_x", |_: &Lua, this| Ok(this.0.width));
        field.add_field_method_get("shape_y", |_: &Lua, this| Ok(this.0.height));
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* entry
        {
            "version": "1.0.0",
            "name": "texture:to_image",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("to_image", |_: &Lua, this, _: ()| {
            // TO-DO port
            //crate::base::image::Image::new_from_texture(this.0)
            todo!();
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "texture:set_mipmap",
            "info": "Set the mipmap for a texture."
        }
        */
        method.add_method_mut("set_mipmap", |_: &Lua, this, _: ()| {
            unsafe {
                GenTextureMipmaps(&mut this.0);
            }
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "texture:set_filter",
            "info": "Set the filter for a texture.",
            "member": [
                { "name": "filter", "info": "LuaTexture filter.", "kind": "texture_filter" }
            ]
        }
        */
        method.add_method_mut("set_filter", |_: &Lua, this, filter: i32| {
            unsafe {
                SetTextureFilter(this.0, filter);
            }
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "texture:set_wrap",
            "info": "Set the wrap for a texture.",
            "member": [
                { "name": "wrap", "info": "LuaTexture wrap.", "kind": "texture_wrap" }
            ]
        }
        */
        method.add_method_mut("set_wrap", |_: &Lua, this, wrap: i32| {
            unsafe {
                SetTextureWrap(this.0, wrap);
            }
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "texture:draw_plane",
            "info": "TO-DO"
        }
        */
        method.add_method(
            "draw_plane",
            |lua: &Lua,
             this,
             (source, point_a, point_b, point_c, point_d, color): (
                LuaValue,
                LuaValue,
                LuaValue,
                LuaValue,
                LuaValue,
                LuaValue,
            )| {
                Ok(texture_draw_plane(
                    lua,
                    (this.0, source, point_a, point_b, point_c, point_d, color),
                ))
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "texture:draw",
            "info": "Draw a texture.",
            "member": [
                { "name": "point", "info": "TO-DO", "kind": "vector_2" },
                { "name": "angle", "info": "TO-DO", "kind": "number"   },
                { "name": "scale", "info": "TO-DO", "kind": "number"   },
                { "name": "color", "info": "TO-DO", "kind": "color"    }
            ]
        }
        */
        method.add_method(
            "draw",
            |lua: &Lua, this, (point, angle, scale, color): (LuaValue, f32, f32, LuaValue)| {
                Ok(texture_draw(lua, (this.0, point, angle, scale, color)))
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "texture:draw_pro",
            "info": "Draw a texture (pro).",
            "member": [
                { "name": "box_a", "info": "TO-DO", "kind": "box_2"    },
                { "name": "box_b", "info": "TO-DO", "kind": "box_2"    },
                { "name": "point", "info": "TO-DO", "kind": "vector_2" },
                { "name": "angle", "info": "TO-DO", "kind": "number"   },
                { "name": "color", "info": "TO-DO", "kind": "color"    }
            ]
        }
        */
        method.add_method(
                "draw_pro",
                |lua: &Lua,
                 this,
                 (box_a, box_b, point, angle, color): (
                    LuaValue,
                    LuaValue,
                    LuaValue,
                    f32,
                    LuaValue,
                )| {
                    Ok(texture_pro_draw(
                        lua,
                        (this.0, box_a, box_b, point, angle, color),
                    ))
                },
            );

        /* entry
        {
            "version": "1.0.0",
            "name": "texture:draw_billboard",
            "info": "Draw a billboard texture.",
            "member": [
                { "name": "camera", "info": "TO-DO", "kind": "camera_3d" },
                { "name": "point",  "info": "TO-DO", "kind": "vector_3"  },
                { "name": "scale",  "info": "TO-DO", "kind": "number"    },
                { "name": "color",  "info": "TO-DO", "kind": "color"     }
            ]
        }
        */
        method.add_method(
            "draw_billboard",
            |lua: &Lua,
             this,
             (camera, point, scale, color): (LuaValue, LuaValue, f32, LuaValue)| {
                Ok(texture_draw_billboard(
                    lua,
                    (this.0, camera, point, scale, color),
                ))
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "texture:draw_billboard_pro",
            "info": "Draw a billboard texture (pro).",
            "member": [
                { "name": "camera", "info": "TO-DO", "kind": "camera_3d" },
                { "name": "source", "info": "TO-DO", "kind": "box_3"     },
                { "name": "point",  "info": "TO-DO", "kind": "vector_3"  },
                { "name": "up",     "info": "TO-DO", "kind": "vector_3"  },
                { "name": "scale",  "info": "TO-DO", "kind": "vector_2"  },
                { "name": "origin", "info": "TO-DO", "kind": "vector_2"  },
                { "name": "angle",  "info": "TO-DO", "kind": "number"    },
                { "name": "color",  "info": "TO-DO", "kind": "color"     }
            ]
        }
        */
        method.add_method(
            "draw_billboard_pro",
            |lua: &Lua,
             this,
             (camera, source, point, up, scale, origin, angle, color): (
                LuaValue,
                LuaValue,
                LuaValue,
                LuaValue,
                LuaValue,
                LuaValue,
                f32,
                LuaValue,
            )| {
                Ok(texture_draw_billboard_pro(
                    lua,
                    (
                        this.0, camera, source, point, up, scale, origin, angle, color,
                    ),
                ))
            },
        );
    }
}

impl LuaTexture {
    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.texture.new",
        "info": "Create a new texture resource.",
        "member": [
            { "name": "path", "info": "Path to texture file.", "kind": "string" }
        ],
        "result": [
            { "name": "texture", "info": "LuaTexture resource.", "kind": "texture" }
        ]
    }
    */
    fn new(lua: &Lua, path: String) -> mlua::Result<Self> {
        let name = Script::rust_to_c_string(&ScriptData::get_path(lua, &path)?)?;

        unsafe {
            let data = LoadTexture(name.as_ptr());

            if IsTextureValid(data) {
                Ok(Self(data))
            } else {
                Err(mlua::Error::RuntimeError(format!(
                    "LuaTexture::new(): Could not load file \"{path}\"."
                )))
            }
        }
    }

    // TO-DO port
    /*
    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.texture.new_from_memory",
        "info": "TO-DO"
    }
    */
    fn new_from_memory(lua: &Lua, (data, kind): (LuaValue, String)) -> mlua::Result<Self> {
        let image = crate::base::image::Image::new_from_memory(lua, (data, kind))?;

        unsafe {
            let data = LoadTextureFromImage(image.0);

            if IsTextureValid(data) {
                Ok(data)
            } else {
                Err(mlua::Error::RuntimeError(
                    "LuaTexture::new_from_memory(): Could not load file.".to_string(),
                ))
            }
        }
    }

    pub fn new_from_image(image: Image) -> mlua::Result<Self> {
        unsafe {
            let data = LoadTextureFromImage(image);

            if IsTextureValid(data) {
                Ok(Self(data))
            } else {
                Err(mlua::Error::RuntimeError(
                    "LuaTexture::new_from_image(): Could not load file.".to_string(),
                ))
            }
        }
    }
    */
}

/* class
{
    "version": "1.0.0",
    "name": "render_texture",
    "info": "An unique handle for a render texture in memory.",
    "member": [
        { "name": "shape_x", "info": "Shape of the texture (X).", "kind": "number" },
        { "name": "shape_y", "info": "Shape of the texture (Y).", "kind": "number" }
    ]
}
*/
struct LuaRenderTexture(RenderTexture);

impl Drop for LuaRenderTexture {
    fn drop(&mut self) {
        unsafe {
            UnloadRenderTexture(self.0);
        }
    }
}

unsafe impl Send for LuaRenderTexture {}

impl mlua::UserData for LuaRenderTexture {
    fn add_fields<F: mlua::UserDataFields<Self>>(field: &mut F) {
        field.add_field_method_get("ID", |_: &Lua, this| Ok(this.0.texture.id));
        field.add_field_method_get("shape_x", |_: &Lua, this| Ok(this.0.texture.width));
        field.add_field_method_get("shape_y", |_: &Lua, this| Ok(this.0.texture.height));
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* entry
        {
            "version": "1.0.0",
            "name": "render_texture:begin",
            "info": "Initialize drawing to the render texture.",
            "member": [
                { "name": "call", "info": "The draw code.", "kind": "function" }
            ]
        }
        */
        method.add_method("begin", |_: &Lua, this, call: mlua::Function| {
            unsafe {
                BeginTextureMode(this.0);

                let call = call.call::<()>(());

                EndTextureMode();

                call?;
            }

            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "render_texture:draw_plane",
            "info": "TO-DO"
        }
        */
        method.add_method(
            "draw_plane",
            |lua: &Lua,
             this,
             (source, point_a, point_b, point_c, point_d, color): (
                LuaValue,
                LuaValue,
                LuaValue,
                LuaValue,
                LuaValue,
                LuaValue,
            )| {
                Ok(texture_draw_plane(
                    lua,
                    (
                        this.0.texture,
                        source,
                        point_a,
                        point_b,
                        point_c,
                        point_d,
                        color,
                    ),
                ))
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "render_texture:draw",
            "info": "Draw a texture.",
            "member": [
                { "name": "point", "info": "TO-DO", "kind": "vector_2" },
                { "name": "angle", "info": "TO-DO", "kind": "number"   },
                { "name": "scale", "info": "TO-DO", "kind": "number"   },
                { "name": "color", "info": "TO-DO", "kind": "color"    }
            ]
        }
        */
        method.add_method(
            "draw",
            |lua: &Lua, this, (point, angle, scale, color): (LuaValue, f32, f32, LuaValue)| {
                Ok(texture_draw(
                    lua,
                    (this.0.texture, point, angle, scale, color),
                ))
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "render_texture:draw_pro",
            "info": "Draw a texture (pro).",
            "member": [
                { "name": "box_a", "info": "TO-DO", "kind": "box_2"    },
                { "name": "box_b", "info": "TO-DO", "kind": "box_2"    },
                { "name": "point", "info": "TO-DO", "kind": "vector_2" },
                { "name": "angle", "info": "TO-DO", "kind": "number"   },
                { "name": "color", "info": "TO-DO", "kind": "color"    }
            ]
        }
        */
        method.add_method(
                "draw_pro",
                |lua: &Lua,
                 this,
                 (box_a, box_b, point, angle, color): (
                    LuaValue,
                    LuaValue,
                    LuaValue,
                    f32,
                    LuaValue,
                )| {
                    Ok(texture_pro_draw(
                        lua,
                        (this.0.texture, box_a, box_b, point, angle, color),
                    ))
                },
            );
    }
}

impl LuaRenderTexture {
    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.render_texture.new",
        "info": "Create a new render texture resource.",
        "member": [
            { "name": "shape", "info": "TO-DO", "kind": "vector_2" }
        ],
        "result": [
            { "name": "render_texture", "info": "Render texture resource.", "kind": "render_texture" }
        ]
    }
    */
    fn new(lua: &Lua, shape: LuaValue) -> mlua::Result<Self> {
        let shape: Vector2 = lua.from_value(shape)?;

        unsafe {
            let mut data = LoadRenderTexture(shape.x as i32, shape.y as i32);

            if IsRenderTextureValid(data) {
                // TO-DO expose this as method
                //GenTextureMipmaps(&mut data.texture);
                //SetTextureFilter(data.texture, TextureFilter::TEXTURE_FILTER_TRILINEAR as i32);

                Ok(Self(data))
            } else {
                Err(mlua::Error::RuntimeError(
                    "LuaRenderTexture::new(): Could not load render texture.".to_string(),
                ))
            }
        }
    }
}

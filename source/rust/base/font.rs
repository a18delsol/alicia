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

use super::helper;

//================================================================

/* class
{ "version": "1.0.0", "name": "alicia.font", "info": "The font API.", "head": true }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table, _: &StatusInfo, _: Option<&ScriptInfo>) -> mlua::Result<()> {
    let font = lua.create_table()?;

    font.set("new_default",         lua.create_function(self::LuaFont::new_default)?)?;     // GetFontDefault
    font.set("new",                 lua.create_function(self::LuaFont::new)?)?;             // LoadFont/*Ex TO-DO convert to Ex
    //font.set("new_from_image",      lua.create_function(self::LuaFont::new_from_image)?)?;  // LoadFontFromImage
    font.set("new_from_memory",     lua.create_function(self::LuaFont::new_from_memory)?)?; // LoadFontFromMemory
    
    //================================================================

    font.set("draw_frame_rate", lua.create_function(self::draw_frame_rate)?)?; // DrawFPS
    font.set("draw_text",       lua.create_function(self::draw_text)?)?;       // DrawText

    //================================================================

    font.set("set_text_line_space", lua.create_function(self::set_text_line_space)?)?; // SetTextLineSpacing

    table.set("font", font)?;

    Ok(())
}

/* class
{ "version": "1.0.0", "name": "font", "info": "An unique handle to a font in memory." }
*/
struct LuaFont(Font);

impl Drop for LuaFont {
    fn drop(&mut self) {
        unsafe {
            UnloadFont(self.0);
        }
    }
}

unsafe impl Send for LuaFont {}

impl mlua::UserData for LuaFont {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* entry
        {
            "version": "1.0.0",
            "name": "font:draw",
            "info": "Draw a font.",
            "member": [
                { "name": "label",  "info": "Label of font to draw.", "kind": "string"   },
                { "name": "point",  "info": "Point of font to draw.", "kind": "vector_2" },
                { "name": "origin", "info": "TO-DO", "kind": "vector_2" },
                { "name": "angle",  "info": "TO-DO", "kind": "number" },
                { "name": "scale",  "info": "Scale of font to draw.", "kind": "number"   },
                { "name": "space",  "info": "Space of font to draw.", "kind": "number"   },
                { "name": "color",  "info": "Color of font to draw.", "kind": "color"    }
            ]
        }
        */
        method.add_method(
            "draw",
            |lua: &Lua,
             this,
             (text, point, origin, angle, scale, space, color): (
                String,
                LuaValue,
                LuaValue,
                f32,
                f32,
                f32,
                LuaValue,
            )| {
                let point: Vector2 = lua.from_value(point)?;
                let origin: Vector2 = lua.from_value(origin)?;
                let color: Color = lua.from_value(color)?;
                let text = Script::rust_to_c_string(&text)?;

                unsafe {
                    DrawTextPro(
                        this.0,
                        text.as_ptr(),
                        point,
                        origin,
                        angle,
                        scale,
                        space,
                        color,
                    );
                    Ok(())
                }
            },
        );

        method.add_method(
            "draw_box",
            |lua: &Lua,
             this,
             (text, shape, scale, space, wrap, color): (
                String,
                LuaValue,
                f32,
                f32,
                bool,
                LuaValue,
            )| {
                let shape: Rectangle = lua.from_value(shape)?;
                let color: Color = lua.from_value(color)?;
                let text = Script::rust_to_c_string(&text)?;

                unsafe {
                    let result = helper::DrawTextBoxed(
                        this.0,
                        text.as_ptr(),
                        helper::Rectangle {
                            x: shape.x,
                            y: shape.y,
                            width: shape.width,
                            height: shape.height,
                        },
                        scale,
                        space,
                        wrap,
                        helper::Color {
                            r: color.r,
                            g: color.g,
                            b: color.b,
                            a: color.a,
                        },
                    );

                    Ok(result)
                }
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "font:measure_text",
            "info": "Measure the size of a given text on screen, with a given font.",
            "member": [
                { "name": "label", "info": "Label of font to measure.", "kind": "string" },
                { "name": "scale", "info": "Scale of font to measure.", "kind": "number" },
                { "name": "space", "info": "Space of font to measure.", "kind": "number" }
            ],
            "result": [
                { "name": "size_x", "info": "Size of text (X).", "kind": "number" },
                { "name": "size_y", "info": "Size of text (Y).", "kind": "number" }
            ]
        }
        */
        method.add_method(
            "measure_text",
            |_: &Lua, this, (text, scale, space): (String, f32, f32)| {
                let text = Script::rust_to_c_string(&text)?;

                unsafe {
                    let result = MeasureTextEx(this.0, text.as_ptr(), scale, space);
                    Ok((result.x, result.y))
                }
            },
        );

        method.add_method(
            "measure_text_box",
            |lua: &Lua,
             this,
             (text, shape, scale, space, wrap): (String, LuaValue, f32, f32, bool)| {
                let shape: Rectangle = lua.from_value(shape)?;
                let text = Script::rust_to_c_string(&text)?;

                unsafe {
                    let result = helper::MeasureTextBoxed(
                        this.0,
                        text.as_ptr(),
                        helper::Rectangle {
                            x: shape.x,
                            y: shape.y,
                            width: shape.width,
                            height: shape.height,
                        },
                        scale,
                        space,
                        wrap,
                    );

                    Ok(result)
                }
            },
        );
    }
}

impl LuaFont {
    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.font.new",
        "info": "Create a new font resource.",
        "member": [
            { "name": "path", "info": "Path to font file.", "kind": "string" },
            { "name": "size", "info": "Size for font.",     "kind": "number" }
        ],
        "result": [
            { "name": "font", "info": "LuaFont resource.", "kind": "font" }
        ]
    }
    */
    fn new(lua: &Lua, (path, size): (String, i32)) -> mlua::Result<Self> {
        unsafe {
            let name = Script::rust_to_c_string(&ScriptData::get_path(lua, &path)?)?;

            let data = LoadFontEx(name.as_ptr(), size, std::ptr::null_mut(), 0);

            if IsFontValid(data) {
                Ok(Self(data))
            } else {
                Err(mlua::Error::RuntimeError(format!(
                    "LuaFont::new(): Could not load file \"{path}\"."
                )))
            }
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.font.new_from_memory",
        "info": "Create a new font resource, from memory.",
        "member": [
            { "name": "data", "info": "The data buffer.",                    "kind": "data"   },
            { "name": "kind", "info": "The kind of font file (.ttf, etc.).", "kind": "string" },
            { "name": "size", "info": "Size for font.",                      "kind": "number" }
        ],
        "result": [
            { "name": "font", "info": "LuaFont resource.", "kind": "font" }
        ]
    }
    */
    fn new_from_memory(_: &Lua, (data, kind, size): (LuaValue, String, i32)) -> mlua::Result<Self> {
        let data = crate::base::data::Data::get_buffer(data)?;

        unsafe {
            let data = &data.0;

            let data = LoadFontFromMemory(
                Script::rust_to_c_string(&kind)?.as_ptr(),
                data.as_ptr(),
                data.len() as i32,
                size,
                std::ptr::null_mut(),
                0,
            );

            if IsFontValid(data) {
                Ok(Self(data))
            } else {
                Err(mlua::Error::RuntimeError(
                    "LuaFont::new_from_memory(): Could not load file.".to_string(),
                ))
            }
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.font.new_default",
        "info": "Create a new font resource, from the default font.",
        "member": [
            { "name": "size", "info": "Size for font.", "kind": "number" }
        ],
        "result": [
            { "name": "font", "info": "LuaFont resource.", "kind": "font" }
        ]
    }
    */
    fn new_default(_: &Lua, size: i32) -> mlua::Result<Self> {
        let data = Status::FONT;

        unsafe {
            let data = LoadFontFromMemory(
                Script::rust_to_c_string(".ttf")?.as_ptr(),
                data.as_ptr(),
                data.len() as i32,
                size,
                std::ptr::null_mut(),
                0,
            );

            if IsFontValid(data) {
                Ok(Self(data))
            } else {
                Err(mlua::Error::RuntimeError(
                    "LuaFont::new_from_default(): Could not load file.".to_string(),
                ))
            }
        }
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.font.draw_frame_rate",
    "info": "TO-DO"
}
*/
fn draw_frame_rate(lua: &Lua, point: LuaValue) -> mlua::Result<()> {
    let point: Vector2 = lua.from_value(point)?;

    unsafe {
        DrawFPS(point.x as i32, point.y as i32);
        Ok(())
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.font.draw_text",
    "info": "Draw text.",
    "member": [
        { "name": "point", "info": "The point of the text.", "kind": "vector_2" },
        { "name": "label", "info": "The label of the text.", "kind": "string"   },
        { "name": "scale", "info": "The angle of the text.", "kind": "number"   },
        { "name": "color", "info": "The color of the text.", "kind": "color"    }
    ]
}
*/
fn draw_text(
    lua: &Lua,
    (point, label, scale, color): (LuaValue, String, i32, LuaValue),
) -> mlua::Result<()> {
    let point: Vector2 = lua.from_value(point)?;
    let label = Script::rust_to_c_string(&label)?;
    let color: Color = lua.from_value(color)?;

    unsafe {
        DrawText(label.as_ptr(), point.x as i32, point.y as i32, scale, color);
        Ok(())
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.font.set_text_line_space",
    "info": "Set the vertical space between each line-break.",
    "member": [
        { "name": "space", "info": "Vertical space.", "kind": "number" }
    ]
}
*/
fn set_text_line_space(_: &Lua, space: i32) -> mlua::Result<()> {
    unsafe {
        SetTextLineSpacing(space);
    }

    Ok(())
}

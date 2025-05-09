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

use mlua::prelude::*;
use raylib::prelude::*;

//================================================================

/* class
{ "version": "1.0.0", "name": "alicia.shader", "info": "The shader API.", "head": true }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table, _: &StatusInfo, _: Option<&ScriptInfo>) -> mlua::Result<()> {
    let shader = lua.create_table()?;

    shader.set("new",             lua.create_function(self::Shader::new)?)?;
    shader.set("new_from_memory", lua.create_function(self::Shader::new_from_memory)?)?;

    table.set("shader", shader)?;

    Ok(())
}

pub type RLShader = raylib::shaders::Shader;

/* class
{ "version": "1.0.0", "name": "shader", "info": "An unique handle for a shader in memory." }
*/
pub struct Shader(pub RLShader);

unsafe impl Send for Shader {}

impl Shader {
    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.shader.new",
        "info": "Create a new shader resource.",
        "member": [
            { "name": "v_path", "info": "Path to .vs file.", "kind": "string" },
            { "name": "f_path", "info": "Path to .fs file.", "kind": "string" }
        ],
        "result": [
            { "name": "shader", "info": "Shader resource.", "kind": "shader" }
        ]
    }
    */
    fn new(lua: &Lua, (v_path, f_path): (Option<String>, Option<String>)) -> mlua::Result<Self> {
        unsafe {
            let v_path = match v_path {
                Some(name) => {
                    let pointer = Script::rust_to_c_string(&ScriptData::get_path(lua, &name)?)?;

                    pointer.into_raw()
                }
                None => std::ptr::null(),
            };

            let f_path = match f_path {
                Some(name) => {
                    let pointer = Script::rust_to_c_string(&ScriptData::get_path(lua, &name)?)?;

                    pointer.into_raw()
                }
                None => std::ptr::null(),
            };

            let data = ffi::LoadShader(v_path, f_path);

            if ffi::IsShaderValid(data) {
                Ok(Self(RLShader::from_raw(data)))
            } else {
                Err(mlua::Error::RuntimeError(
                    "Shader::new(): Could not load file.".to_string(),
                ))
            }
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.shader.new_from_memory",
        "info": "TO-DO"
    }
    */
    fn new_from_memory(
        lua: &Lua,
        (v_path, f_path): (Option<String>, Option<String>),
    ) -> mlua::Result<Self> {
        unsafe {
            let v_path = match v_path {
                Some(name) => {
                    let pointer = Script::rust_to_c_string(&ScriptData::get_path(lua, &name)?)?;

                    pointer.into_raw()
                }
                None => std::ptr::null(),
            };

            let f_path = match f_path {
                Some(name) => {
                    let pointer = Script::rust_to_c_string(&ScriptData::get_path(lua, &name)?)?;

                    pointer.into_raw()
                }
                None => std::ptr::null(),
            };

            let data = ffi::LoadShaderFromMemory(v_path, f_path);

            if ffi::IsShaderValid(data) {
                Ok(Self(RLShader::from_raw(data)))
            } else {
                Err(mlua::Error::RuntimeError(
                    "Shader::new_from_memory(): Could not load file.".to_string(),
                ))
            }
        }
    }
}

impl mlua::UserData for Shader {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* entry
        {
            "version": "1.0.0",
            "name": "shader:begin",
            "info": "TO-DO",
            "member": [
                { "name": "call", "info": "The draw code.", "kind": "function" }
            ]
        }
        */
        method.add_method("begin", |_: &Lua, this, call: mlua::Function| {
            unsafe {
                ffi::BeginShaderMode(*this.0);

                call.call::<()>(())?;

                ffi::EndShaderMode();
            }

            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "shader:get_location_name",
            "info": "TO-DO",
            "member": [
                { "name": "name", "info": "TO-DO", "kind": "string" }
            ],
            "result": [
                { "name": "location", "info": "TO-DO", "kind": "number" }
            ]
        }
        */
        method.add_method("get_location_name", |_, this, name: String| {
            Ok(this.0.get_shader_location(&name))
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "shader:get_location_attribute_name",
            "info": "TO-DO",
            "member": [
                { "name": "name", "info": "TO-DO", "kind": "string" }
            ],
            "result": [
                { "name": "location", "info": "TO-DO", "kind": "number" }
            ]
        }
        */
        method.add_method("get_location_attribute_name", |_, this, name: String| {
            Ok(this.0.get_shader_location_attribute(&name))
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "shader:get_location",
            "info": "TO-DO",
            "member": [
                { "name": "location", "info": "TO-DO", "kind": "number" }
            ],
            "result": [
                { "name": "location", "info": "TO-DO", "kind": "number" }
            ]
        }
        */
        method.add_method("get_location", |_, this, location: usize| {
            Ok(this.0.locs()[location])
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "shader:set_location",
            "info": "TO-DO",
            "member": [
                { "name": "location", "info": "TO-DO", "kind": "number" },
                { "name": "value",    "info": "TO-DO", "kind": "number" }
            ]
        }
        */
        method.add_method_mut(
            "set_location",
            |_, this, (location, value): (usize, i32)| {
                this.0.locs_mut()[location] = value;
                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "shader:set_shader_value",
            "info": "TO-DO",
            "member": [
                { "name": "location", "info": "TO-DO", "kind": "number" },
                { "name": "kind",     "info": "TO-DO", "kind": "number" },
                { "name": "value",    "info": "TO-DO", "kind": "any"    }
            ]
        }
        */
        method.add_method_mut(
            "set_shader_value",
            |lua, this, (location, kind, value): (i32, i32, LuaValue)| unsafe {
                match kind {
                    0 => {
                        let value: i32 = lua.from_value(value)?;
                        this.0.set_shader_value(location, value);
                    }
                    1 => {
                        let value: f32 = lua.from_value(value)?;
                        this.0.set_shader_value(location, value);
                    }
                    2 => {
                        let value: Vector2 = lua.from_value(value)?;
                        this.0.set_shader_value(location, value);
                    }
                    3 => {
                        let value: Vector3 = lua.from_value(value)?;
                        this.0.set_shader_value(location, value);
                    }
                    4 => {
                        let value: Vector4 = lua.from_value(value)?;
                        this.0.set_shader_value(location, value);
                    }
                    5 => {
                        let value: Matrix = lua.from_value(value)?;
                        ffi::SetShaderValueMatrix(*this.0, location, value.into());
                    }
                    _ => {
                        if let Some(data) = value.as_userdata() {
                            if let Ok(data) = data.borrow::<crate::base::texture::Texture>() {
                                ffi::SetShaderValueTexture(*this.0, location, (*data).0);
                            } else {
                                return Err(mlua::Error::RuntimeError(
                                    "set_shader_value(): Error borrowing texture.".to_string(),
                                ));
                            }
                        } else {
                            return Err(mlua::Error::RuntimeError(
                                "set_shader_value(): Value is not a Texture user-data.".to_string(),
                            ));
                        }
                    }
                };

                Ok(())
            },
        );
    }
}

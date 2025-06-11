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

use crate::base::helper::*;
use crate::script::*;
use crate::status::*;

//================================================================

use mlua::prelude::*;
use std::ffi::c_void;

//================================================================

/* class
{ "version": "1.0.0", "name": "alicia.shader", "info": "The shader API.", "head": true }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table, _: &StatusInfo, _: Option<&ScriptInfo>) -> mlua::Result<()> {
    let shader = lua.create_table()?;

    shader.set("new",             lua.create_function(self::LuaShader::new)?)?;
    shader.set("new_from_memory", lua.create_function(self::LuaShader::new_from_memory)?)?;

    table.set("shader", shader)?;

    Ok(())
}

/* class
{ "version": "1.0.0", "name": "shader", "info": "An unique handle for a shader in memory." }
*/
pub struct LuaShader(pub Shader);

unsafe impl Send for LuaShader {}

impl LuaShader {
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
            { "name": "shader", "info": "LuaShader resource.", "kind": "shader" }
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

            let data = LoadShader(v_path, f_path);

            if IsShaderValid(data) {
                Ok(Self(data))
            } else {
                Err(mlua::Error::RuntimeError(
                    "LuaShader::new(): Could not load file.".to_string(),
                ))
            }
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.shader.new_from_memory",
        "info": "Create a new shader resource, from memory.",
        "member": [
            { "name": "v_data", "info": ".vs file data.", "kind": "string" },
            { "name": "f_data", "info": ".fs file data.", "kind": "string" }
        ],
        "result": [
            { "name": "shader", "info": "LuaShader resource.", "kind": "shader" }
        ]
    }
    */
    fn new_from_memory(
        lua: &Lua,
        (v_data, f_data): (Option<String>, Option<String>),
    ) -> mlua::Result<Self> {
        unsafe {
            let v_data = match v_data {
                Some(name) => {
                    let pointer = Script::rust_to_c_string(&ScriptData::get_path(lua, &name)?)?;

                    pointer.into_raw()
                }
                None => std::ptr::null(),
            };

            let f_data = match f_data {
                Some(name) => {
                    let pointer = Script::rust_to_c_string(&ScriptData::get_path(lua, &name)?)?;

                    pointer.into_raw()
                }
                None => std::ptr::null(),
            };

            let data = LoadShaderFromMemory(v_data, f_data);

            if IsShaderValid(data) {
                Ok(Self(data))
            } else {
                Err(mlua::Error::RuntimeError(
                    "LuaShader::new_from_memory(): Could not load file.".to_string(),
                ))
            }
        }
    }
}

impl mlua::UserData for LuaShader {
    fn add_fields<F: mlua::UserDataFields<Self>>(_: &mut F) {}

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* entry
        {
            "version": "1.0.0",
            "name": "shader:begin",
            "info": "Initialize the shader.",
            "member": [
                { "name": "call", "info": "The draw code.", "kind": "function" },
                { "name": "...",  "info": "Variadic data.", "kind": "any"      }
            ]
        }
        */
        method.add_method(
            "begin",
            |_: &Lua, this, (call, variadic): (mlua::Function, mlua::Variadic<LuaValue>)| {
                unsafe {
                    BeginShaderMode(this.0);

                    let call = call.call::<()>(variadic);

                    EndShaderMode();

                    call?;
                }

                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "shader:get_location_name",
            "info": "Get the location of a shader variable, by name.",
            "member": [
                { "name": "name", "info": "Variable name.", "kind": "string" }
            ],
            "result": [
                { "name": "location", "info": "Shader variable location.", "kind": "number" }
            ]
        }
        */
        method.add_method("get_location_name", |_, this, name: String| unsafe {
            Ok(GetShaderLocation(
                this.0,
                Script::rust_to_c_string(&name)?.as_ptr(),
            ))
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "shader:get_location_attribute_name",
            "info": "Get the location of a shader attribute, by name.",
            "member": [
                { "name": "name", "info": "Attribute name.", "kind": "string" }
            ],
            "result": [
                { "name": "location", "info": "Shader attribute location.", "kind": "number" }
            ]
        }
        */
        method.add_method(
            "get_location_attribute_name",
            |_, this, name: String| unsafe {
                Ok(GetShaderLocationAttrib(
                    this.0,
                    Script::rust_to_c_string(&name)?.as_ptr(),
                ))
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "shader:get_location",
            "info": "Get the location of a shader variable, by index.",
            "member": [
                { "name": "location", "info": "Variable index.", "kind": "number" }
            ],
            "result": [
                { "name": "location", "info": "Shader variable location.", "kind": "number" }
            ]
        }
        */
        method.add_method("get_location", |_, this, location: usize| unsafe {
            if location <= 31 {
                Ok(*this.0.locs.wrapping_add(location))
            } else {
                Err(mlua::Error::runtime(
                    "Shader::get_location(): Invalid location index.",
                ))
            }
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "shader:set_location",
            "info": "Set the location of a shader variable.",
            "member": [
                { "name": "location", "info": "Variable index.", "kind": "number" },
                { "name": "value",    "info": "Variable value.", "kind": "number" }
            ]
        }
        */
        method.add_method_mut(
            "set_location",
            |_, this, (location, value): (usize, i32)| unsafe {
                if location <= 31 {
                    *this.0.locs.wrapping_add(location) = value;
                    Ok(())
                } else {
                    Err(mlua::Error::runtime(
                        "Shader::set_location(): Invalid location index.",
                    ))
                }
            },
        );

        // TO-DO declare shader_value_kind in Lua
        /* entry
        {
            "version": "1.0.0",
            "name": "shader:set_shader_value",
            "info": "Set the value of a shader variable.",
            "member": [
                { "name": "location", "info": "Variable index.", "kind": "number" },
                { "name": "class",    "info": "Variable class.", "kind": "number" },
                { "name": "value",    "info": "Variable value.", "kind": "any"    }
            ]
        }
        */
        method.add_method_mut(
            "set_shader_value",
            |lua, this, (location, class, value): (i32, i32, LuaValue)| unsafe {
                match class {
                    0 => {
                        let mut value: f32 = lua.from_value(value)?;
                        let value_p: *mut c_void = &mut value as *mut _ as *mut c_void;
                        SetShaderValue(this.0, location, value_p, class);
                    }
                    1 => {
                        let mut value: Vector2 = lua.from_value(value)?;
                        let value_p: *mut c_void = &mut value as *mut _ as *mut c_void;
                        SetShaderValue(this.0, location, value_p, class);
                    }
                    2 => {
                        let mut value: Vector3 = lua.from_value(value)?;
                        let value_p: *mut c_void = &mut value as *mut _ as *mut c_void;
                        SetShaderValue(this.0, location, value_p, class);
                    }
                    3 => {
                        let mut value: Vector4 = lua.from_value(value)?;
                        let value_p: *mut c_void = &mut value as *mut _ as *mut c_void;
                        SetShaderValue(this.0, location, value_p, class);
                    }
                    _ => {
                        let mut value: i32 = lua.from_value(value)?;
                        let value_p: *mut c_void = &mut value as *mut _ as *mut c_void;
                        SetShaderValue(this.0, location, value_p, class);
                    }
                };

                // TO-DO add every other value type (ivec2, ivec3, etc.)

                Ok(())
            },
        );
    }
}

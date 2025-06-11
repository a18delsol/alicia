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
{ "version": "1.0.0", "name": "alicia.r3d", "info": "The R3D API.", "head": true }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table, _: &StatusInfo, script_info: Option<&ScriptInfo>) -> mlua::Result<()> {
    // part of head API. only run on head API pass, if we are running in head mode.
    if let Some(info) = script_info {
        if !info.head {
            return Ok(())
        }
    } else {
        return Ok(())
    }

    let r3d = lua.create_table()?;

    // R3D_HasState
    //r3d.set("get_state", lua.create_function(self::get_state)?)?;
    // R3D_SetState
    //r3d.set("set_state", lua.create_function(self::set_state)?)?;
    // R3D_GetResolution
    //r3d.set("get_shape", lua.create_function(self::set_state)?)?;
    // R3D_UpdateResolution
    r3d.set("set_shape", lua.create_function(self::set_shape)?)?;
    // R3D_SetRenderTarget
    // ???
    // R3D_SetSceneBounds
    //r3d.set("set_state", lua.create_function(self::set_state)?)?;
    // R3D_ApplyRenderMode
    //r3d.set("set_state", lua.create_function(self::set_state)?)?;
    // R3D_ApplyBlendMode
    //r3d.set("set_state", lua.create_function(self::set_state)?)?;
    // R3D_ApplyShadowCastMode
    //r3d.set("set_state", lua.create_function(self::set_state)?)?;
    // R3D_ApplyBillboardMode
    //r3d.set("set_state", lua.create_function(self::set_state)?)?;
    // R3D_ApplyAlphaScissorThreshold
    //r3d.set("set_state", lua.create_function(self::set_state)?)?;
    // R3D_Begin/R3D_End
    r3d.set("begin", lua.create_function(self::begin)?)?;
    // R3D_DrawMesh/Instanced/Ex/Pro
    // ??
    // R3D_DrawModel/Ex
    // ??
    // R3D_DrawSprite/Ex/Pro
    // ??
    // R3D_DrawParticleSystem/Ex
    // ??
    // R3D_SetBackgroundColor
    r3d.set("set_back_color", lua.create_function(self::set_back_color)?)?;
    // R3D_SetAmbientColor
    r3d.set("set_base_color", lua.create_function(self::set_base_color)?)?;

    // R3D_IsPointInFrustum
    r3d.set("get_point_frustum", lua.create_function(self::get_point_frustum)?)?;

    //================================================================

    let light = lua.create_table()?;

    light.set("new", lua.create_function(LuaLight::new)?)?;

    r3d.set("light", light)?;

    //================================================================

    table.set("r3d", r3d)?;

    Ok(())
}

fn begin(
    lua: &Lua,
    (call, camera, variadic): (mlua::Function, LuaValue, mlua::Variadic<LuaValue>),
) -> mlua::Result<()> {
    unsafe {
        let camera = lua.from_value(camera)?;

        R3D_Begin(camera);

        let call = call.call::<()>(variadic);

        R3D_End();

        call?;

        Ok(())
    }
}

fn set_shape(lua: &Lua, shape: LuaValue) -> mlua::Result<()> {
    unsafe {
        let shape: Vector2 = lua.from_value(shape)?;
        R3D_UpdateResolution(shape.x as i32, shape.y as i32);

        Ok(())
    }
}

fn set_back_color(lua: &Lua, color: LuaValue) -> mlua::Result<()> {
    unsafe {
        R3D_SetBackgroundColor(lua.from_value(color)?);
        Ok(())
    }
}

fn set_base_color(lua: &Lua, color: LuaValue) -> mlua::Result<()> {
    unsafe {
        R3D_SetAmbientColor(lua.from_value(color)?);
        Ok(())
    }
}

fn get_point_frustum(lua: &Lua, point: LuaValue) -> mlua::Result<bool> {
    unsafe { Ok(R3D_IsPointInFrustum(lua.from_value(point)?)) }
}

//================================================================

struct LuaLight(R3D_Light);

impl Drop for LuaLight {
    fn drop(&mut self) {
        unsafe {
            R3D_DestroyLight(self.0);
        }
    }
}

impl LuaLight {
    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.r3d.light.new",
        "info": "TO-DO"
    }
    */
    fn new(_: &Lua, kind: u32) -> mlua::Result<Self> {
        unsafe {
            let data = R3D_CreateLight(kind);

            Ok(Self(data))
        }
    }
}

impl mlua::UserData for LuaLight {
    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* entry
        {
            "version": "1.0.0",
            "name": "light:get_kind",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_kind", |_, this, _: ()| unsafe {
            Ok(R3D_GetLightType(this.0))
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "light:get_state",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_state", |_, this, _: ()| unsafe {
            Ok(R3D_IsLightActive(this.0))
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "light:get_color",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_color", |_, this, _: ()| unsafe {
            let value = R3D_GetLightColor(this.0);
            Ok((value.r, value.g, value.b, value.a))
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "light:get_point",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_point", |_, this, _: ()| unsafe {
            let value = R3D_GetLightPosition(this.0);
            Ok((value.x, value.y, value.z))
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "light:get_focus",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_focus", |_, this, _: ()| unsafe {
            let value = R3D_GetLightDirection(this.0);
            Ok((value.x, value.y, value.z))
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "light:set_state",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("set_state", |_, this, active: bool| unsafe {
            R3D_SetLightActive(this.0, active);
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "light:set_color",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("set_color", |lua, this, value: LuaValue| unsafe {
            R3D_SetLightColor(this.0, lua.from_value(value)?);
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "light:set_point",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("set_point", |lua, this, value: LuaValue| unsafe {
            R3D_SetLightPosition(this.0, lua.from_value(value)?);
            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "light:set_focus",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("set_focus", |lua, this, value: LuaValue| unsafe {
            R3D_SetLightDirection(this.0, lua.from_value(value)?);
            Ok(())
        });

        // TO-DO add rest.

        /* entry
        {
            "version": "1.0.0",
            "name": "light:get_shadow",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_shadow", |_, this, _: ()| unsafe {
            Ok((R3D_IsShadowEnabled(this.0), R3D_HasShadowMap(this.0)))
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "light:get_shadow_update",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("get_shadow_update", |_, this, _: ()| unsafe {
            Ok((
                R3D_GetShadowUpdateMode(this.0),
                R3D_GetShadowUpdateFrequency(this.0),
            ))
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "light:set_shadow",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "set_shadow",
            |lua, this, (value, data): (bool, LuaValue)| unsafe {
                if value {
                    let data = lua.from_value(data)?;
                    R3D_EnableShadow(this.0, data);
                } else {
                    let data = lua.from_value(data)?;
                    R3D_DisableShadow(this.0, data);
                }

                // TO-DO hack.
                R3D_ApplyShadowCastMode(2);

                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "light:set_shadow_update",
            "info": "TO-DO"
        }
        */
        method.add_method_mut(
            "set_shadow_update",
            |lua, this, (value, data): (u32, Option<LuaValue>)| unsafe {
                R3D_SetShadowUpdateMode(this.0, value);

                if value == 1 {
                    if let Some(interval) = data {
                        let interval = lua.from_value(interval)?;

                        R3D_SetShadowUpdateFrequency(this.0, interval);
                    } else {
                        return Err(mlua::Error::runtime("light::set_shadow_update(): Missing \"interval\" argument for interval update mode."));
                    }
                }

                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "light:shadow_update",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("shadow_update", |_, this, _: ()| unsafe {
            R3D_UpdateShadowMap(this.0);
            Ok(())
        });

        // TO-DO add bias.
    }
}

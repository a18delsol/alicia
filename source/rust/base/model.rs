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

/* class
{ "version": "1.0.0", "name": "alicia.model", "info": "The model API.", "head": true }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table, _: &StatusInfo, _: Option<&ScriptInfo>) -> mlua::Result<()> {
    let model = lua.create_table()?;

    model.set("new", lua.create_function(self::LuaModel::new)?)?;

    table.set("model", model)?;

    let model_animation = lua.create_table()?;

    model_animation.set("new", lua.create_function(self::LuaModelAnimation::new)?)?;

    table.set("model_animation", model_animation)?;

    Ok(())
}

/* class
{
    "version": "1.0.0",
    "name": "model",
    "info": "An unique handle for a model in memory.",
    "member": [
        { "name": "mesh_count",     "info": "Mesh count.",     "kind": "number" },
        { "name": "bone_count",     "info": "Bone count.",     "kind": "number" },
        { "name": "material_count", "info": "Material count.", "kind": "number" }
    ]
}
*/
struct LuaModel(Model);

unsafe impl Send for LuaModel {}

impl Drop for LuaModel {
    fn drop(&mut self) {
        unsafe {
            for x in 0..self.0.materialCount {
                let material = *self.0.materials.wrapping_add(x as usize);

                for i in 0..12 {
                    let map = *material.maps.wrapping_add(i);

                    if IsTextureValid(map.texture) {
                        // apparently unloading either 1 or 13 will cause a nasty bug for some reason.
                        // 1 is raylib's default texture,
                        // 13 might be R3D's own texture.
                        // deallocating either of them will cause memory corruption!
                        if map.texture.id != 1 && map.texture.id != 13 {
                            UnloadTexture(map.texture);
                        }
                    }
                }
            }

            UnloadModel(self.0);
        }
    }
}

impl LuaModel {
    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.model.new",
        "info": "Create a new LuaModel resource.",
        "member": [
            { "name": "path", "info": "Path to model file.", "kind": "string" }
        ],
        "result": [
            { "name": "model", "info": "LuaModel resource.", "kind": "model" }
        ]
    }
    */
    fn new(lua: &Lua, path: String) -> mlua::Result<Self> {
        let name = Script::rust_to_c_string(&ScriptData::get_path(lua, &path)?)?;

        unsafe {
            let data = LoadModel(name.as_ptr());

            // TO-DO extract this out as separate API
            for x in 0..data.materialCount {
                let mut m = *data.materials.wrapping_add(x as usize);

                for i in 0..12 {
                    let mut map = *m.maps.wrapping_add(i);

                    if IsTextureValid(map.texture) {
                        // apparently every model is also bound to the internal RL texture? what?
                        if map.texture.id != 1 {
                            //GenTextureMipmaps(&mut map.texture);
                            //SetTextureFilter(
                            //    map.texture,
                            //    TextureFilter_TEXTURE_FILTER_BILINEAR as i32,
                            //);
                        }
                    }
                }

                let mut a = *m.maps.wrapping_add(0);
                a.color = Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255,
                };

                R3D_SetMaterialOcclusion(&mut m, std::ptr::null_mut(), 1.0);
                //R3D_SetMaterialRoughness(&mut m, std::ptr::null_mut(), 1.0);
                //R3D_SetMaterialMetalness(&mut m, std::ptr::null_mut(), 1.0);

                let mut a = *m.maps.wrapping_add(3);
                a.texture.id = 1;
            }

            for x in 0..data.meshCount {
                let mut m = *data.meshes.wrapping_add(x as usize);
                GenMeshTangents(&mut m);
            }

            if IsModelValid(data) {
                Ok(Self(data))
            } else {
                Err(mlua::Error::RuntimeError(format!(
                    "LuaModel::new(): Could not load file \"{path}\"."
                )))
            }
        }
    }
}

impl mlua::UserData for LuaModel {
    fn add_fields<F: mlua::UserDataFields<Self>>(field: &mut F) {
        field.add_field_method_get("mesh_count", |_, this| Ok(this.0.meshCount));
        field.add_field_method_get("material_count", |_, this| Ok(this.0.materialCount));
        field.add_field_method_get("bone_count", |_, this| Ok(this.0.boneCount));
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* entry
        {
            "version": "1.0.0",
            "name": "model:bind",
            "info": "Bind a texture to the model.",
            "member": [
                { "name": "index",   "info": "TO-DO", "kind": "number" },
                { "name": "which",   "info": "TO-DO", "kind": "number" },
                { "name": "texture", "info": "Texture to bind to model.", "kind": "texture" }
            ]
        }
        */
        method.add_method_mut(
            "bind",
            |_, this, (index, which, texture): (usize, usize, LuaAnyUserData)| unsafe {
                if texture.is::<crate::base::texture::LuaTexture>() {
                    let texture = texture
                        .borrow::<crate::base::texture::LuaTexture>()
                        .unwrap();
                    let texture = &*texture;

                    // TO-DO bound checking
                    let material = *this.0.materials.wrapping_add(index);
                    let mut map = *material.maps.wrapping_add(which);

                    map.texture = texture.0;
                }

                Ok(())
            },
        );

        method.add_method_mut(
            "bind_shader",
            |_, this, (index, shader): (usize, LuaAnyUserData)| unsafe {
                if shader.is::<crate::base::shader::LuaShader>() {
                    let shader = shader.borrow::<crate::base::shader::LuaShader>().unwrap();
                    let shader = &*shader;

                    // TO-DO bound checking
                    let mut material = *this.0.materials.wrapping_add(index);

                    material.shader = shader.0;
                }

                // TO-DO throw error

                Ok(())
            },
        );

        // TO-DO port
        /*
        /* entry
        {
            "version": "1.0.0",
            "name": "model:draw_mesh",
            "info": "TO-DO"
        }
        */
        method.add_method(
            "draw_mesh",
            |lua, this, (index, point, angle, scale): (usize, LuaValue, LuaValue, LuaValue)| {
                let mesh = &this.0.meshes()[index];
                let point: Vector3 = lua.from_value(point)?;
                let angle: Vector3 = lua.from_value(angle)?;
                let scale: Vector3 = lua.from_value(scale)?;
                let angle = Vector3::new(
                    angle.x * DEG2RAD as f32,
                    angle.y * DEG2RAD as f32,
                    angle.z * DEG2RAD as f32,
                );

                let transform =
                    (Matrix::translate(point.x, point.y, point.z) * Matrix::rotate_xyz(angle) * Matrix::scale(scale.x, scale.y, scale.z));

                //DrawMesh(**mesh, *this.0.materials()[0], transform);
                Ok(())
            },
        );
        */

        /* entry
        {
            "version": "1.0.0",
            "name": "model:draw",
            "info": "Draw the model.",
            "member": [
                { "name": "point", "info": "TO-DO", "kind": "vector_3" },
                { "name": "scale", "info": "TO-DO", "kind": "number"   },
                { "name": "color", "info": "TO-DO", "kind": "color"    }
            ]
        }
        */
        method.add_method(
            "draw",
            |lua, this, (point, scale, color, r3d): (LuaValue, f32, LuaValue, bool)| unsafe {
                let point: Vector3 = lua.from_value(point)?;
                let color: Color = lua.from_value(color)?;

                if r3d {
                    R3D_DrawModel(
                        this.0,
                        Vector3 {
                            x: point.x,
                            y: point.y,
                            z: point.z,
                        },
                        1.0,
                    );
                } else {
                    DrawModel(this.0, point, scale, color);
                }
                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "model:draw_wire",
            "info": "Draw the model (wire-frame).",
            "member": [
                { "name": "point", "info": "TO-DO", "kind": "vector_3" },
                { "name": "scale", "info": "TO-DO", "kind": "number"   },
                { "name": "color", "info": "TO-DO", "kind": "color"    }
            ]
        }
        */
        method.add_method(
            "draw_wire",
            |lua, this, (point, scale, color): (LuaValue, f32, LuaValue)| unsafe {
                let point: Vector3 = lua.from_value(point)?;
                let color: Color = lua.from_value(color)?;

                DrawModelWires(this.0, point, scale, color);
                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "model:draw_transform",
            "info": "Draw the model with a transformation.",
            "member": [
                { "name": "point", "info": "TO-DO", "kind": "vector_3" },
                { "name": "angle", "info": "TO-DO", "kind": "vector_3" },
                { "name": "scale", "info": "TO-DO", "kind": "vector_3" },
                { "name": "color", "info": "TO-DO", "kind": "color"    }
            ]
        }
        */
        method.add_method_mut(
            "draw_transform",
            |lua, this, (point, angle, scale, color, r3d): (LuaValue, LuaValue, LuaValue, LuaValue, bool)| unsafe {
                let point: Vector3 = lua.from_value(point)?;
                let angle: Vector4 = lua.from_value(angle)?;
                let scale: Vector3 = lua.from_value(scale)?;
                let color: Color = lua.from_value(color)?;

                let point = MatrixTranslate(point.x, point.y, point.z);
                let angle = MatrixRotate(Vector3 { x: angle.x, y: angle.y, z: angle.z }, angle.w);
                let scale = MatrixScale(scale.x, scale.y, scale.z);

                this.0.transform = MatrixMultiply(MatrixMultiply(scale, angle), point);

                if r3d {
                    R3D_DrawModel(
                        this.0,
                        Vector3 {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        1.0,
                    );
                } else {
                    DrawModel(this.0, Vector3 { x: 0.0, y: 0.0, z: 0.0 }, 1.0, color);
                }

                this.0.transform = MatrixIdentity();

                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "model:get_box_3",
            "info": "TO-DO",
            "result": [
                { "name": "min_x", "info": "Minimum vector. (X)", "kind": "number" },
                { "name": "min_y", "info": "Minimum vector. (Y)", "kind": "number" },
                { "name": "min_z", "info": "Minimum vector. (Z)", "kind": "number" },
                { "name": "max_x", "info": "Maximum vector. (X)", "kind": "number" },
                { "name": "max_y", "info": "Maximum vector. (Y)", "kind": "number" },
                { "name": "max_z", "info": "Maximum vector. (Z)", "kind": "number" }
            ]
        }
        */
        method.add_method("get_box_3", |_, this, _: ()| unsafe {
            let value = GetModelBoundingBox(this.0);
            Ok((
                value.min.x,
                value.min.y,
                value.min.z,
                value.max.x,
                value.max.y,
                value.max.z,
            ))
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "model:mesh_vertex",
            "info": "Get the vertex data of a specific mesh in the model.",
            "member": [
                { "name": "index", "info": "Index of mesh.", "kind": "number" }
            ],
            "result": [
                { "name": "table", "info": "Vector3 table.", "kind": "table" }
            ]
        }
        */
        method.add_method("mesh_vertex", |lua, this, index: usize| unsafe {
            let mesh = *this.0.meshes.wrapping_add(index);
            let work = std::slice::from_raw_parts(
                mesh.vertices as *const Vector3,
                mesh.vertexCount as usize,
            );

            lua.to_value(&work)
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "model:mesh_index",
            "info": "Get the index data of a specific mesh in the model.",
            "member": [
                { "name": "index", "info": "Index of mesh.", "kind": "number" }
            ],
            "result": [
                { "name": "table", "info": "Number table.", "kind": "table" }
            ]
        }
        */
        method.add_method("mesh_index", |lua, this, index: usize| unsafe {
            let mesh = *this.0.meshes.wrapping_add(index);
            let work = std::slice::from_raw_parts(
                mesh.indices as *const u16,
                (mesh.triangleCount * 3) as usize,
            );

            lua.to_value(&work)
        });

        // TO-DO additional API code
    }
}

/* class
{
    "version": "1.0.0",
    "name": "model_animation",
    "info": "An unique handle for a model animation in memory."
}
*/
struct LuaModelAnimation(ModelAnimation);

unsafe impl Send for LuaModelAnimation {}

impl LuaModelAnimation {
    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.model_animation.new",
        "info": "Create a new model animation resource.",
        "member": [
            { "name": "path", "info": "Path to model file.", "kind": "string" }
        ],
        "result": [
            { "name": "model_animation", "info": "Model animation resource.", "kind": "model_animation" }
        ]
    }
    */
    fn new(lua: &Lua, path: String) -> mlua::Result<Vec<Self>> {
        let name = Script::rust_to_c_string(&ScriptData::get_path(lua, &path)?)?;

        unsafe {
            let mut count = 0;
            let data = LoadModelAnimations(name.as_ptr(), &mut count);
            let mut list: Vec<Self> = Vec::new();

            if count == 0 {
                return Err(mlua::Error::RuntimeError(format!(
                    "LuaModelAnimation::new(): Could not load file \"{path}\"."
                )));
            }

            for x in 0..count {
                let animation = *data.wrapping_add(x.try_into().unwrap());

                list.push(Self(animation));
            }

            Ok(list)
        }
    }
}

impl mlua::UserData for LuaModelAnimation {
    fn add_fields<F: mlua::UserDataFields<Self>>(field: &mut F) {
        field.add_field_method_get("bone_count", |_, this| Ok(this.0.boneCount));
        field.add_field_method_get("frame_count", |_, this| Ok(this.0.frameCount));
        field.add_field_method_get("name", |_, this| {
            // TO-DO check if this does work?
            let name: Vec<u8> = this.0.name.iter().map(|x| *x as u8).collect();
            Ok(String::from_utf8(name)
                .map_err(|e| mlua::Error::runtime(e))
                .unwrap())
        });
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* entry
        {
            "version": "1.0.0",
            "name": "model_animation:update",
            "info": "Update model with new model animation data.",
            "member": [
                { "name": "model", "info": "TO-DO", "kind": "model"  },
                { "name": "frame", "info": "TO-DO", "kind": "number" }
            ]
        }
        */
        method.add_method(
            "update",
            |_, this, (model, frame): (LuaAnyUserData, usize)| unsafe {
                if model.is::<LuaModel>() {
                    let model = model.borrow::<LuaModel>().unwrap();
                    UpdateModelAnimation(model.0, this.0, frame.try_into().unwrap());
                }

                // TO-DO throw error

                Ok(())
            },
        );

        // TO-DO additional API code
    }
}

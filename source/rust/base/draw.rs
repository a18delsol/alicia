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
{ "version": "1.0.0", "name": "alicia.draw", "info": "The drawing API.", "head": true }
*/
#[rustfmt::skip]
pub fn set_global(
    lua: &Lua,
    table: &mlua::Table,
    status_info: &StatusInfo,
    script_info: Option<&ScriptInfo>,
) -> mlua::Result<()> {
    let draw = lua.create_table()?;

    draw.set("clear", lua.create_function(self::clear)?)?;                 // ClearBackground
    draw.set("begin", lua.create_function(self::begin)?)?;                 // BeginDrawing/EndDrawing
    draw.set("begin_blend", lua.create_function(self::begin_blend)?)?;     // BeginBlendMode/EndBlendMode
    draw.set("begin_scissor", lua.create_function(self::begin_scissor)?)?; // BeginScissorMode/EndScissorMode

    table.set("draw", draw)?;

    draw_3d::set_global(lua, table, status_info, script_info)?;
    draw_2d::set_global(lua, table, status_info, script_info)?;

    Ok(())
}

/* entry
{
    "version": "1.0.0", "name": "alicia.draw.clear",
    "info": "Clear the screen with a color.",
    "member": [
        { "name": "color", "info": "The color to use for clearing.", "kind": "color" }
    ]
}
*/
fn clear(lua: &Lua, color: LuaValue) -> mlua::Result<()> {
    let value: Color = lua.from_value(color)?;

    unsafe {
        ClearBackground(value);
        Ok(())
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.draw.begin",
    "info": "Initialize drawing to the screen.",
    "member": [
        { "name": "call", "info": "The draw code.", "kind": "function" },
        { "name": "...",  "info": "Variadic data.", "kind": "any"      }
    ]
}
*/
fn begin(
    _: &Lua,
    (call, variadic): (mlua::Function, mlua::Variadic<LuaValue>),
) -> mlua::Result<()> {
    unsafe {
        BeginDrawing();

        let call = call.call::<()>(variadic);

        EndDrawing();

        call?;

        Ok(())
    }
}

// to-do: blend mode enumerator. error if mode is outside of range.
/* entry
{
    "version": "1.0.0",
    "name": "alicia.draw.begin_blend",
    "info": "Initialize drawing (blend mode) to the screen.",
    "member": [
        { "name": "call", "info": "The draw code.", "kind": "function" },
        { "name": "mode", "info": "The draw code.", "kind": "function" },
        { "name": "...",  "info": "Variadic data.", "kind": "any"      }
    ]
}
*/
fn begin_blend(
    _: &Lua,
    (call, mode, variadic): (mlua::Function, i32, mlua::Variadic<LuaValue>),
) -> mlua::Result<()> {
    unsafe {
        BeginBlendMode(mode);

        let call = call.call::<()>(variadic);

        EndBlendMode();

        call?;

        Ok(())
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.draw.begin_scissor",
    "info": "Initialize drawing (scissor mode) to the screen.",
    "member": [
        { "name": "call", "info": "The draw code.",        "kind": "function" },
        { "name": "view", "info": "The clip test region.", "kind": "box_2"    },
        { "name": "...",  "info": "Variadic data.",        "kind": "any"      }
    ]
}
*/
fn begin_scissor(
    lua: &Lua,
    (call, view, variadic): (mlua::Function, LuaValue, mlua::Variadic<LuaValue>),
) -> mlua::Result<()> {
    let view: Rectangle = lua.from_value(view)?;

    unsafe {
        BeginScissorMode(
            view.x as i32,
            view.y as i32,
            view.width as i32,
            view.height as i32,
        );

        let call = call.call::<()>(variadic);

        EndScissorMode();

        call?;

        Ok(())
    }
}

mod draw_3d {
    use super::*;

    /* class
    { "version": "1.0.0", "name": "alicia.draw_3d", "info": "The 3D drawing API." }
    */
    #[rustfmt::skip]
    pub fn set_global(lua: &Lua, table: &mlua::Table, _: &StatusInfo, _: Option<&ScriptInfo>) -> mlua::Result<()> {
        let draw_3d = lua.create_table()?;

        draw_3d.set("begin",                 lua.create_function(self::begin)?)?;               // BeginMode3D/EndMode3D
        draw_3d.set("get_screen_to_world",   lua.create_function(self::get_screen_to_world)?)?; // GetScreenToWorldRay/*Ex
        draw_3d.set("get_world_to_screen",   lua.create_function(self::get_world_to_screen)?)?; // GetWorldToScreen
        draw_3d.set("draw_line",             lua.create_function(self::draw_line)?)?;           // DrawLine3D
        draw_3d.set("draw_point",            lua.create_function(self::draw_point)?)?;          // DrawPoint3D
        draw_3d.set("draw_circle",           lua.create_function(self::draw_circle)?)?;         // DrawCircle3D
        draw_3d.set("draw_triangle",         lua.create_function(self::draw_triangle)?)?;       // DrawTriangle3D
        draw_3d.set("draw_triangle_strip",   lua.create_function(self::draw_triangle_strip)?)?; // DrawTriangleStrip3D
        draw_3d.set("draw_cube",             lua.create_function(self::draw_cube)?)?;           // DrawCube/*V
        draw_3d.set("draw_cube_wire",        lua.create_function(self::draw_cube_wire)?)?;      // DrawCubeWires/*V
        draw_3d.set("draw_sphere",           lua.create_function(self::draw_sphere)?)?;         // DrawSphere/*Ex
        draw_3d.set("draw_sphere_wire",      lua.create_function(self::draw_sphere_wire)?)?;    // DrawSphereWires
        draw_3d.set("draw_cylinder",         lua.create_function(self::draw_cylinder)?)?;       // DrawCylinder/*Ex
        draw_3d.set("draw_cylinder_wire",    lua.create_function(self::draw_cylinder_wire)?)?;  // DrawCylinderWires/*Ex
        draw_3d.set("draw_capsule",          lua.create_function(self::draw_capsule)?)?;        // DrawCapsule
        draw_3d.set("draw_capsule_wire",     lua.create_function(self::draw_capsule_wire)?)?;   // DrawCapsuleWires
        draw_3d.set("draw_plane",            lua.create_function(self::draw_plane)?)?;          // DrawPlane
        draw_3d.set("draw_ray",              lua.create_function(self::draw_ray)?)?;            // DrawRay
        draw_3d.set("draw_grid",             lua.create_function(self::draw_grid)?)?;           // DrawGrid
        draw_3d.set("draw_box_3",            lua.create_function(self::draw_box_3)?)?;          // DrawBoundingBox

        table.set("draw_3d", draw_3d)?;

        Ok(())
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_3d.begin",
        "info": "Initialize the 3D draw mode.",
        "member": [
            { "name": "call",   "info": "The draw code.", "kind": "function"  },
            { "name": "camera", "info": "The 2D camera.", "kind": "camera_3d" },
            { "name": "...",    "info": "Variadic data.", "kind": "any"       }
        ]
    }
    */
    fn begin(
        lua: &Lua,
        (call, camera, variadic): (mlua::Function, LuaValue, mlua::Variadic<LuaValue>),
    ) -> mlua::Result<()> {
        let value: Camera3D = lua.from_value(camera)?;

        unsafe {
            BeginMode3D(value);

            let call = call.call::<()>(variadic);

            EndMode3D();

            call?;

            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_3d.get_screen_to_world",
        "info": "Get a ray for a 2D screen-space point.",
        "member": [
            { "name": "camera", "info": "The current camera.",        "kind": "camera_3d" },
            { "name": "point",  "info": "The screen-space point.",    "kind": "vector_2"  },
            { "name": "shape",  "info": "The size of the view-port.", "kind": "vector_2"  }
        ],
        "result": [
            { "name": "position_x",  "info": "The 3D ray position. (X).",  "kind": "number" },
            { "name": "position_y",  "info": "The 3D ray position. (Y).",  "kind": "number" },
            { "name": "position_z",  "info": "The 3D ray position. (Z).",  "kind": "number" },
            { "name": "direction_x", "info": "The 3D ray direction. (X).", "kind": "number" },
            { "name": "direction_y", "info": "The 3D ray direction. (Y).", "kind": "number" },
            { "name": "direction_z", "info": "The 3D ray direction. (Z).", "kind": "number" }
        ]
    }
    */
    fn get_screen_to_world(
        lua: &Lua,
        (camera, point, shape): (LuaValue, LuaValue, LuaValue),
    ) -> mlua::Result<(f32, f32, f32, f32, f32, f32)> {
        let camera: Camera3D = lua.from_value(camera)?;
        let point: Vector2 = lua.from_value(point)?;
        let shape: Vector2 = lua.from_value(shape)?;

        unsafe {
            let ray = GetScreenToWorldRayEx(point, camera, shape.x as i32, shape.y as i32);

            Ok((
                ray.position.x,
                ray.position.y,
                ray.position.z,
                ray.direction.x,
                ray.direction.y,
                ray.direction.z,
            ))
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_3d.get_world_to_screen",
        "info": "Get a 2D screen-space point for a 3D world-space point.",
        "member": [
            { "name": "camera", "info": "The current camera.",        "kind": "camera_3d" },
            { "name": "point",  "info": "The world-space point.",     "kind": "vector_3"  },
            { "name": "shape",  "info": "The size of the view-port.", "kind": "vector_2"  }
        ],
        "result": [
            { "name": "point_x", "info": "The 2D screen-space point (X).", "kind": "number" },
            { "name": "point_y", "info": "The 2D screen-space point (Y).", "kind": "number" }
        ]
    }
    */
    fn get_world_to_screen(
        lua: &Lua,
        (camera, point, shape): (LuaValue, LuaValue, LuaValue),
    ) -> mlua::Result<(f32, f32)> {
        let camera: Camera3D = lua.from_value(camera)?;
        let point: Vector3 = lua.from_value(point)?;
        let shape: Vector2 = lua.from_value(shape)?;

        unsafe {
            let point = GetWorldToScreenEx(point, camera, shape.x as i32, shape.y as i32);

            Ok((point.x, point.y))
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_3d.draw_line",
        "info": "Draw a line.",
        "member": [
            { "name": "point_a", "info": "The point A of the line.",   "kind": "vector_3" },
            { "name": "point_b", "info": "The point B of the line.",   "kind": "vector_3" },
            { "name": "color",   "info": "The color of the line.",     "kind": "color"    }
        ]
    }
    */
    fn draw_line(
        lua: &Lua,
        (point_a, point_b, color): (LuaValue, LuaValue, LuaValue),
    ) -> mlua::Result<()> {
        let point_a: Vector3 = lua.from_value(point_a)?;
        let point_b: Vector3 = lua.from_value(point_b)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            DrawLine3D(point_a, point_b, color);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_3d.draw_point",
        "info": "Draw a point.",
        "member": [
            { "name": "point", "info": "The point.",              "kind": "vector_3" },
            { "name": "color", "info": "The color of the point.", "kind": "color"    }
        ]
    }
    */
    fn draw_point(lua: &Lua, (point, color): (LuaValue, LuaValue)) -> mlua::Result<()> {
        let point: Vector3 = lua.from_value(point)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            DrawPoint3D(point, color);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_3d.draw_circle",
        "info": "Draw a circle.",
        "member": [
            { "name": "point", "info": "The point of the circle.", "kind": "vector_3" },
            { "name": "range", "info": "The range of the circle.", "kind": "number"   },
            { "name": "axis",  "info": "The axis of the circle.",  "kind": "vector_3" },
            { "name": "range", "info": "The angle of the circle.", "kind": "number"   },
            { "name": "color", "info": "The color of the circle.", "kind": "color"    }
        ]
    }
    */
    fn draw_circle(
        lua: &Lua,
        (point, range, axis, angle, color): (LuaValue, f32, LuaValue, f32, LuaValue),
    ) -> mlua::Result<()> {
        let point: Vector3 = lua.from_value(point)?;
        let axis: Vector3 = lua.from_value(axis)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            DrawCircle3D(point, range, axis, angle, color);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_3d.draw_triangle",
        "info": "Draw a triangle.",
        "member": [
            { "name": "point_a", "info": "The point (A) of the triangle.", "kind": "vector_3" },
            { "name": "point_b", "info": "The point (B) of the triangle.", "kind": "vector_3" },
            { "name": "point_c", "info": "The point (C) of the triangle.", "kind": "vector_3" },
            { "name": "color",   "info": "The color of the triangle.",     "kind": "color"    }
        ]
    }
    */
    fn draw_triangle(
        lua: &Lua,
        (point_a, point_b, point_c, color): (LuaValue, LuaValue, LuaValue, LuaValue),
    ) -> mlua::Result<()> {
        let point_a: Vector3 = lua.from_value(point_a)?;
        let point_b: Vector3 = lua.from_value(point_b)?;
        let point_c: Vector3 = lua.from_value(point_c)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            DrawTriangle3D(point_a, point_b, point_c, color);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_3d.draw_triangle_strip",
        "info": "Draw a triangle strip.",
        "member": [
            { "name": "point", "info": "The point list of the triangle.", "kind": "table" },
            { "name": "color", "info": "The color of the triangle.",      "kind": "color" }
        ]
    }
    */
    fn draw_triangle_strip(lua: &Lua, (point, color): (LuaValue, LuaValue)) -> mlua::Result<()> {
        let point: Vec<Vector3> = lua.from_value(point)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            DrawTriangleStrip3D(point.as_ptr(), point.len() as i32, color);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_3d.draw_cube",
        "info": "Draw a cube.",
        "member": [
            { "name": "point", "info": "The point of the cube.", "kind": "vector_3" },
            { "name": "shape", "info": "The shape of the cube.", "kind": "vector_3" },
            { "name": "color", "info": "The color of the cube.", "kind": "color"    }
        ]
    }
    */
    fn draw_cube(
        lua: &Lua,
        (point, shape, color): (LuaValue, LuaValue, LuaValue),
    ) -> mlua::Result<()> {
        let point: Vector3 = lua.from_value(point)?;
        let shape: Vector3 = lua.from_value(shape)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            DrawCubeV(point, shape, color);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_3d.draw_cube_wire",
        "info": "Draw a cube (wire-frame).",
        "member": [
            { "name": "point", "info": "The point of the cube.", "kind": "vector_3" },
            { "name": "shape", "info": "The shape of the cube.", "kind": "vector_3" },
            { "name": "color", "info": "The color of the cube.", "kind": "color"    }
        ]
    }
    */
    fn draw_cube_wire(
        lua: &Lua,
        (point, shape, color): (LuaValue, LuaValue, LuaValue),
    ) -> mlua::Result<()> {
        let point: Vector3 = lua.from_value(point)?;
        let shape: Vector3 = lua.from_value(shape)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            DrawCubeWiresV(point, shape, color);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_3d.draw_sphere",
        "info": "Draw a sphere.",
        "member": [
            { "name": "point", "info": "The point of the sphere.",   "kind": "vector_3" },
            { "name": "color", "info": "The color of the triangle.", "kind": "color" }
        ]
    }
    */
    fn draw_sphere(
        lua: &Lua,
        (point, range, ring, slice, color): (LuaValue, f32, i32, i32, LuaValue),
    ) -> mlua::Result<()> {
        let point: Vector3 = lua.from_value(point)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            DrawSphereEx(point, range, ring, slice, color);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_3d.draw_sphere_wire",
        "info": "TO-DO"
    }
    */
    fn draw_sphere_wire(
        lua: &Lua,
        (point, range, ring, slice, color): (LuaValue, f32, i32, i32, LuaValue),
    ) -> mlua::Result<()> {
        let point: Vector3 = lua.from_value(point)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            DrawSphereWires(point, range, ring, slice, color);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_3d.draw_cylinder",
        "info": "TO-DO"
    }
    */
    fn draw_cylinder(
        lua: &Lua,
        (point_a, point_b, range_a, range_b, count, color): (
            LuaValue,
            LuaValue,
            f32,
            f32,
            i32,
            LuaValue,
        ),
    ) -> mlua::Result<()> {
        let point_a: Vector3 = lua.from_value(point_a)?;
        let point_b: Vector3 = lua.from_value(point_b)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            DrawCylinderEx(point_a, point_b, range_a, range_b, count, color);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_3d.draw_cylinder_wire",
        "info": "TO-DO"
    }
    */
    fn draw_cylinder_wire(
        lua: &Lua,
        (point_a, point_b, range_a, range_b, count, color): (
            LuaValue,
            LuaValue,
            f32,
            f32,
            i32,
            LuaValue,
        ),
    ) -> mlua::Result<()> {
        let point_a: Vector3 = lua.from_value(point_a)?;
        let point_b: Vector3 = lua.from_value(point_b)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            DrawCylinderWiresEx(point_a, point_b, range_a, range_b, count, color);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_3d.draw_capsule",
        "info": "TO-DO"
    }
    */
    fn draw_capsule(
        lua: &Lua,
        (point_a, point_b, range, ring, slice, color): (
            LuaValue,
            LuaValue,
            f32,
            i32,
            i32,
            LuaValue,
        ),
    ) -> mlua::Result<()> {
        let point_a: Vector3 = lua.from_value(point_a)?;
        let point_b: Vector3 = lua.from_value(point_b)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            DrawCapsule(point_a, point_b, range, slice, ring, color);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_3d.draw_capsule_wire",
        "info": "TO-DO"
    }
    */
    fn draw_capsule_wire(
        lua: &Lua,
        (point_a, point_b, range, ring, slice, color): (
            LuaValue,
            LuaValue,
            f32,
            i32,
            i32,
            LuaValue,
        ),
    ) -> mlua::Result<()> {
        let point_a: Vector3 = lua.from_value(point_a)?;
        let point_b: Vector3 = lua.from_value(point_b)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            DrawCapsule(point_a, point_b, range, slice, ring, color);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_3d.draw_plane",
        "info": "TO-DO"
    }
    */
    fn draw_plane(
        lua: &Lua,
        (point, shape, color): (LuaValue, LuaValue, LuaValue),
    ) -> mlua::Result<()> {
        let point: Vector3 = lua.from_value(point)?;
        let shape: Vector2 = lua.from_value(shape)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            DrawPlane(point, shape, color);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0", "name": "alicia.draw_3d.draw_ray",
        "info": "Draw a ray.",
        "member": [
            { "name": "ray",   "info": "The ray.",              "kind": "ray"   },
            { "name": "color", "info": "The color of the ray.", "kind": "color" }
        ]
    }
    */
    fn draw_ray(lua: &Lua, (ray, color): (LuaValue, LuaValue)) -> mlua::Result<()> {
        let ray: Ray = lua.from_value(ray)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            DrawRay(ray, color);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0", "name": "alicia.draw_3d.draw_grid",
        "info": "Draw a grid.",
        "member": [
            { "name": "slice", "info": "The slice count of the grid.", "kind": "number" },
            { "name": "space", "info": "The space shift of the grid.", "kind": "number" }
        ]
    }
    */
    fn draw_grid(_: &Lua, (slice, space): (i32, f32)) -> mlua::Result<()> {
        unsafe {
            DrawGrid(slice, space);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0", "name": "alicia.draw_3d.draw_box_3",
        "info": "Draw a 3D box.",
        "member": [
            { "name": "shape", "info": "The shape of the ball.", "kind": "box_3" },
            { "name": "color", "info": "The color of the ball.", "kind": "color" }
        ]
    }
    */
    fn draw_box_3(lua: &Lua, (box_3, color): (LuaValue, LuaValue)) -> mlua::Result<()> {
        let box_3: BoundingBox = lua.from_value(box_3)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            DrawBoundingBox(box_3, color);
            Ok(())
        }
    }
}

mod draw_2d {
    use super::*;

    /* class
    { "version": "1.0.0", "name": "alicia.draw_2d", "info": "The 2D drawing API." }
    */
    #[rustfmt::skip]
    pub fn set_global(lua: &Lua, table: &mlua::Table, _: &StatusInfo, _: Option<&ScriptInfo>) -> mlua::Result<()> {
        let draw_2d = lua.create_table()?;

        draw_2d.set("begin",                          lua.create_function(self::begin)?)?;                        // BeginMode2D/EndMode2D
        draw_2d.set("get_world_to_screen",            lua.create_function(self::get_world_to_screen)?)?;          // GetWorldToScreen2D
        draw_2d.set("get_screen_to_world",            lua.create_function(self::get_screen_to_world)?)?;          // GetScreenToWorld2D
        draw_2d.set("draw_pixel",                     lua.create_function(self::draw_pixel)?)?;                   // DrawPixel/*V
        draw_2d.set("draw_line",                      lua.create_function(self::draw_line)?)?;                    // DrawLine/*V/*Ex
        draw_2d.set("draw_line_strip",                lua.create_function(self::draw_line_strip)?)?;              // DrawLineStrip
        draw_2d.set("draw_line_bezier",               lua.create_function(self::draw_line_bezier)?)?;             // DrawLineBezier
        draw_2d.set("draw_circle",                    lua.create_function(self::draw_circle)?)?;                  // DrawCircle/*V
        draw_2d.set("draw_circle_line",               lua.create_function(self::draw_circle_line)?)?;             // DrawCircleLines/*V
        draw_2d.set("draw_circle_sector",             lua.create_function(self::draw_circle_sector)?)?;           // DrawCircleSector
        draw_2d.set("draw_circle_sector_line",        lua.create_function(self::draw_circle_sector_line)?)?;      // DrawCircleSectorLines
        draw_2d.set("draw_circle_gradient",           lua.create_function(self::draw_circle_gradient)?)?;         // DrawCircleGradient
        draw_2d.set("draw_ellipse",                   lua.create_function(self::draw_ellipse)?)?;                 // DrawEllipse
        draw_2d.set("draw_ellipse_line",              lua.create_function(self::draw_ellipse_line)?)?;            // DrawEllipseLines
        draw_2d.set("draw_ring",                      lua.create_function(self::draw_ring)?)?;                    // DrawRing
        draw_2d.set("draw_ring_line",                 lua.create_function(self::draw_ring_line)?)?;               // DrawRingLines
        draw_2d.set("draw_box_2",                     lua.create_function(self::draw_box_2)?)?;                   // DrawRectangle/*V/*Rec/*Pro
        draw_2d.set("draw_box_2_gradient",            lua.create_function(self::draw_box_2_gradient)?)?;          // DrawRectangleGradientV/*H/*Ex
        draw_2d.set("draw_box_2_line",                lua.create_function(self::draw_box_2_line)?)?;              // DrawRectangleLines/*Ex
        draw_2d.set("draw_box_2_round",               lua.create_function(self::draw_box_2_round)?)?;             // DrawRectangleRound
        draw_2d.set("draw_box_2_line_round",          lua.create_function(self::draw_box_2_line_round)?)?;        // DrawRectangleRoundLines/*Ex
        draw_2d.set("draw_triangle",                  lua.create_function(self::draw_triangle)?)?;                // DrawTriangle
        draw_2d.set("draw_triangle_line",             lua.create_function(self::draw_triangle_line)?)?;           // DrawTriangleLines
        //draw_2d.set("draw_triangle_fan",            lua.create_function(self::draw_triangle_list)?)?;           // DrawTriangleStrip
        //draw_2d.set("draw_triangle_strip",          lua.create_function(self::draw_triangle_list)?)?;           // DrawTriangleFan
        //draw_2d.set("draw_poly",                    lua.create_function(self::draw_poly)?)?;                    // DrawPoly
        //draw_2d.set("draw_poly_line",               lua.create_function(self::draw_poly_line)?)?;               // DrawPolyLines/*Ex
        //draw_2d.set("draw_spline_linear",           lua.create_function(self::draw_spline_linear)?)?;           // DrawSplineLinear
        //draw_2d.set("draw_spline_basis",            lua.create_function(self::draw_spline_basis)?)?;            // DrawSplineBasis
        //draw_2d.set("draw_spline_catmull_rom",      lua.create_function(self::draw_spline_catmull_rom)?)?;      // DrawSplineCatmullRom
        //draw_2d.set("draw_spline_bezier_quadratic", lua.create_function(self::draw_spline_bezier_quadratic)?)?; // DrawSplineBezierQuadratic
        //draw_2d.set("draw_spline_bezier_cubic",     lua.create_function(self::draw_spline_bezier_cubic)?)?;     // DrawSplineBezierCubic
        //draw_2d.set("get_spline_linear",            lua.create_function(self::get_spline_linear)?)?;            // GetSplinePointLinear
        //draw_2d.set("get_spline_basis",             lua.create_function(self::get_spline_basis)?)?;             // GetSplinePointBasis
        //draw_2d.set("get_spline_catmull_rom",       lua.create_function(self::get_spline_catmull_rom)?)?;       // GetSplinePointCatmullRom
        //draw_2d.set("get_spline_bezier_quadratic",  lua.create_function(self::get_spline_bezier_quadratic)?)?;  // GetSplinePointBezierQuadratic
        //draw_2d.set("get_spline_bezier_cubic",      lua.create_function(self::get_spline_bezier_cubic)?)?;      // GetSplinePointBezierCubic

        table.set("draw_2d", draw_2d)?;

        Ok(())
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_2d.begin",
        "info": "Initialize the 2D draw mode.",
        "member": [
            { "name": "call",   "info": "The draw code.", "kind": "function"  },
            { "name": "camera", "info": "The 2D camera.", "kind": "camera_2d" },
            { "name": "...",    "info": "Variadic data.", "kind": "any"       }
        ]
    }
    */
    fn begin(
        lua: &Lua,
        (call, camera, variadic): (mlua::Function, LuaValue, mlua::Variadic<LuaValue>),
    ) -> mlua::Result<()> {
        let value: Camera2D = lua.from_value(camera)?;

        unsafe {
            BeginMode2D(value);

            let call = call.call::<()>(variadic);

            EndMode2D();

            call?;

            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_2d.get_world_to_screen",
        "info": "Get a screen-space point for a 2D world-space point.",
        "member": [
            { "name": "camera", "info": "The current camera.",    "kind": "camera_2d" },
            { "name": "point",  "info": "The world-space point.", "kind": "vector_2"  }
        ],
        "result": [
            { "name": "point_x", "info": "The 2D screen-space point (X).", "kind": "number" },
            { "name": "point_y", "info": "The 2D screen-space point (Y).", "kind": "number" }
        ]
    }
    */
    fn get_world_to_screen(
        lua: &Lua,
        (camera, point): (LuaValue, LuaValue),
    ) -> mlua::Result<(f32, f32)> {
        let camera: Camera2D = lua.from_value(camera)?;
        let point: Vector2 = lua.from_value(point)?;

        unsafe {
            let point = GetWorldToScreen2D(point, camera);

            Ok((point.x, point.y))
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_2d.get_screen_to_world",
        "info": "Get a world-space point for a 2D screen-space point.",
        "member": [
            { "name": "camera", "info": "The current camera.",     "kind": "camera_2d" },
            { "name": "point",  "info": "The screen-space point.", "kind": "vector_2"  }
        ],
        "result": [
            { "name": "point_x", "info": "The 2D world-space point (X).", "kind": "number" },
            { "name": "point_y", "info": "The 2D world-space point (Y).", "kind": "number" }
        ]
    }
    */
    fn get_screen_to_world(
        lua: &Lua,
        (camera, point): (LuaValue, LuaValue),
    ) -> mlua::Result<(f32, f32)> {
        let camera: Camera2D = lua.from_value(camera)?;
        let point: Vector2 = lua.from_value(point)?;

        unsafe {
            let point = GetScreenToWorld2D(point, camera);

            Ok((point.x, point.y))
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_2d.draw_pixel",
        "info": "Draw pixel.",
        "member": [
            { "name": "point", "info": "The point of the pixel.", "kind": "vector_2" },
            { "name": "color", "info": "The color of the pixel.", "kind": "color"    }
        ]
    }
    */
    fn draw_pixel(lua: &Lua, (point, color): (LuaValue, LuaValue)) -> mlua::Result<()> {
        let point: Vector2 = lua.from_value(point)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            DrawPixelV(point, color);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_2d.draw_line",
        "info": "Draw a line.",
        "member": [
            { "name": "point_a", "info": "The point A of the line.",   "kind": "vector_2" },
            { "name": "point_b", "info": "The point B of the line.",   "kind": "vector_2" },
            { "name": "thick",   "info": "The thickness of the line.", "kind": "number"   },
            { "name": "color",   "info": "The color of the line.",     "kind": "color"    }
        ]
    }
    */
    fn draw_line(
        lua: &Lua,
        (point_a, point_b, thick, color): (LuaValue, LuaValue, f32, LuaValue),
    ) -> mlua::Result<()> {
        let point_a: Vector2 = lua.from_value(point_a)?;
        let point_b: Vector2 = lua.from_value(point_b)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            DrawLineEx(point_a, point_b, thick, color);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_2d.draw_line_strip",
        "info": "TO-DO"
    }
    */
    fn draw_line_strip(lua: &Lua, (point, color): (LuaValue, LuaValue)) -> mlua::Result<()> {
        let point: Vec<Vector2> = lua.from_value(point)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            DrawLineStrip(point.as_ptr(), point.len() as i32, color);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_2d.draw_line_bezier",
        "info": "TO-DO"
    }
    */
    fn draw_line_bezier(
        lua: &Lua,
        (point_a, point_b, thickness, color): (LuaValue, LuaValue, f32, LuaValue),
    ) -> mlua::Result<()> {
        let point_a: Vector2 = lua.from_value(point_a)?;
        let point_b: Vector2 = lua.from_value(point_b)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            DrawLineBezier(point_a, point_b, thickness, color);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_2d.draw_circle",
        "info": "Draw a circle.",
        "member": [
            { "name": "point",  "info": "TO-DO", "kind": "vector_2" },
            { "name": "range", "info": "TO-DO", "kind": "number"   },
            { "name": "color",  "info": "TO-DO", "kind": "color"    }
        ]
    }
    */
    fn draw_circle(
        lua: &Lua,
        (point, range, color): (LuaValue, f32, LuaValue),
    ) -> mlua::Result<()> {
        let point: Vector2 = lua.from_value(point)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            DrawCircleV(point, range, color);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_2d.draw_circle_line",
        "info": "TO-DO"
    }
    */
    fn draw_circle_line(
        lua: &Lua,
        (point, range, color): (LuaValue, f32, LuaValue),
    ) -> mlua::Result<()> {
        let point: Vector2 = lua.from_value(point)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            DrawCircleLinesV(point, range, color);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_2d.draw_circle_sector",
        "info": "Draw the sector of a circle.",
        "member": [
            { "name": "point",         "info": "TO-DO", "kind": "vector_2" },
            { "name": "range",        "info": "TO-DO", "kind": "number"   },
            { "name": "begin_angle",   "info": "TO-DO", "kind": "number"   },
            { "name": "close_angle",   "info": "TO-DO", "kind": "number"   },
            { "name": "segment_count", "info": "TO-DO", "kind": "number"   },
            { "name": "color",         "info": "TO-DO", "kind": "color"    }
        ]
    }
    */
    fn draw_circle_sector(
        lua: &Lua,
        (point, range, begin_angle, close_angle, segment_count, color): (
            LuaValue,
            f32,
            f32,
            f32,
            i32,
            LuaValue,
        ),
    ) -> mlua::Result<()> {
        let point: Vector2 = lua.from_value(point)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            DrawCircleSector(point, range, begin_angle, close_angle, segment_count, color);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_2d.draw_circle_sector_line",
        "info": "TO-DO"
    }
    */
    fn draw_circle_sector_line(
        lua: &Lua,
        (point, range, angle_a, angle_b, count, color): (LuaValue, f32, f32, f32, i32, LuaValue),
    ) -> mlua::Result<()> {
        let point: Vector2 = lua.from_value(point)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            DrawCircleSectorLines(point, range, angle_a, angle_b, count, color);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_2d.draw_circle_gradient",
        "info": "TO-DO"
    }
    */
    fn draw_circle_gradient(
        lua: &Lua,
        (point, range, color_a, color_b): (LuaValue, f32, LuaValue, LuaValue),
    ) -> mlua::Result<()> {
        let point: Vector2 = lua.from_value(point)?;
        let color_a: Color = lua.from_value(color_a)?;
        let color_b: Color = lua.from_value(color_b)?;

        unsafe {
            DrawCircleGradient(point.x as i32, point.y as i32, range, color_a, color_b);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_2d.draw_ellipse",
        "info": "TO-DO"
    }
    */
    fn draw_ellipse(
        lua: &Lua,
        (point, shape, color): (LuaValue, LuaValue, LuaValue),
    ) -> mlua::Result<()> {
        let point: Vector2 = lua.from_value(point)?;
        let shape: Vector2 = lua.from_value(shape)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            DrawEllipse(point.x as i32, point.y as i32, shape.x, shape.y, color);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_2d.draw_ellipse_line",
        "info": "TO-DO"
    }
    */
    fn draw_ellipse_line(
        lua: &Lua,
        (point, shape, color): (LuaValue, LuaValue, LuaValue),
    ) -> mlua::Result<()> {
        let point: Vector2 = lua.from_value(point)?;
        let shape: Vector2 = lua.from_value(shape)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            DrawEllipseLines(point.x as i32, point.y as i32, shape.x, shape.y, color);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_2d.draw_ring",
        "info": "TO-DO"
    }
    */
    fn draw_ring(
        lua: &Lua,
        (point, range_a, range_b, angle_a, angle_b, count, color): (
            LuaValue,
            f32,
            f32,
            f32,
            f32,
            i32,
            LuaValue,
        ),
    ) -> mlua::Result<()> {
        let point: Vector2 = lua.from_value(point)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            DrawRing(point, range_a, range_b, angle_a, angle_b, count, color);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_2d.draw_ring_line",
        "info": "TO-DO"
    }
    */
    fn draw_ring_line(
        lua: &Lua,
        (point, range_a, range_b, angle_a, angle_b, count, color): (
            LuaValue,
            f32,
            f32,
            f32,
            f32,
            i32,
            LuaValue,
        ),
    ) -> mlua::Result<()> {
        let point: Vector2 = lua.from_value(point)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            DrawRing(point, range_a, range_b, angle_a, angle_b, count, color);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_2d.draw_box_2",
        "info": "Draw 2D box.",
        "member": [
            { "name": "shape", "info": "The shape of the box.", "kind": "box_2"    },
            { "name": "point", "info": "The point of the box.", "kind": "vector_2" },
            { "name": "angle", "info": "The angle of the box.", "kind": "number"   },
            { "name": "color", "info": "The color of the box.", "kind": "color"    }
        ]
    }
    */
    fn draw_box_2(
        lua: &Lua,
        (shape, point, angle, color): (LuaValue, LuaValue, f32, LuaValue),
    ) -> mlua::Result<()> {
        let shape: Rectangle = lua.from_value(shape)?;
        let point: Vector2 = lua.from_value(point)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            DrawRectanglePro(shape, point, angle, color);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_2d.draw_box_2_gradient",
        "info": "Draw 2D box with a 4-point gradient.",
        "member": [
            { "name": "shape",   "info": "The shape of the box.",   "kind": "box_2" },
            { "name": "color_a", "info": "The color A (T.L.) of the box.", "kind": "color" },
            { "name": "color_b", "info": "The color B (B.L.) of the box.", "kind": "color" },
            { "name": "color_c", "info": "The color C (T.R.) of the box.", "kind": "color" },
            { "name": "color_d", "info": "The color D (B.R.) of the box.", "kind": "color" }
        ]
    }
    */
    fn draw_box_2_gradient(
        lua: &Lua,
        (shape, color_a, color_b, color_c, color_d): (
            LuaValue,
            LuaValue,
            LuaValue,
            LuaValue,
            LuaValue,
        ),
    ) -> mlua::Result<()> {
        let shape: Rectangle = lua.from_value(shape)?;
        let color_a: Color = lua.from_value(color_a)?;
        let color_b: Color = lua.from_value(color_b)?;
        let color_c: Color = lua.from_value(color_c)?;
        let color_d: Color = lua.from_value(color_d)?;

        unsafe {
            DrawRectangleGradientEx(shape, color_a, color_b, color_c, color_d);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_2d.draw_box_2_line",
        "info": "Draw 2D box (out-line).",
        "member": [
            { "name": "shape", "info": "The shape of the box.",     "kind": "box_2"  },
            { "name": "thick", "info": "The thickness of the box.", "kind": "number" },
            { "name": "color", "info": "The color of the box.",     "kind": "color"  }
        ]
    }
    */
    fn draw_box_2_line(
        lua: &Lua,
        (shape, thick, color): (LuaValue, f32, LuaValue),
    ) -> mlua::Result<()> {
        let shape: Rectangle = lua.from_value(shape)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            DrawRectangleLinesEx(shape, thick, color);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_2d.draw_box_2_round",
        "info": "Draw 2D box (round).",
        "member": [
            { "name": "shape", "info": "The shape of the box.",         "kind": "box_2"  },
            { "name": "round", "info": "The roundness of the box.",     "kind": "number" },
            { "name": "count", "info": "The segment count of the box.", "kind": "number" },
            { "name": "color", "info": "The color of the box.",         "kind": "color"  }
        ]
    }
    */
    fn draw_box_2_round(
        lua: &Lua,
        (shape, round, count, color): (LuaValue, f32, i32, LuaValue),
    ) -> mlua::Result<()> {
        let shape: Rectangle = lua.from_value(shape)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            DrawRectangleRounded(shape, round, count, color);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_2d.draw_box_2_line_round",
        "info": "Draw 2D box (out-line, round).",
        "member": [
            { "name": "shape", "info": "The shape of the box.",         "kind": "box_2"  },
            { "name": "round", "info": "The roundness of the box.",     "kind": "number" },
            { "name": "count", "info": "The segment count of the box.", "kind": "number" },
            { "name": "thick", "info": "The thickness of the box.",     "kind": "number" },
            { "name": "color", "info": "The color of the box.",         "kind": "color"  }
        ]
    }
    */
    fn draw_box_2_line_round(
        lua: &Lua,
        (shape, round, count, thick, color): (LuaValue, f32, i32, f32, LuaValue),
    ) -> mlua::Result<()> {
        let shape: Rectangle = lua.from_value(shape)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            DrawRectangleRoundedLinesEx(shape, round, count, thick, color);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_2d.draw_triangle",
        "info": "Draw 2D triangle.",
        "member": [
            { "name": "point_a", "info": "The point A of the triangle.", "kind": "vector_2" },
            { "name": "point_b", "info": "The point B of the triangle.", "kind": "vector_2" },
            { "name": "point_c", "info": "The point C of the triangle.", "kind": "vector_2" },
            { "name": "color",   "info": "The color of the triangle.",   "kind": "color"    }
        ]
    }
    */
    fn draw_triangle(
        lua: &Lua,
        (point_a, point_b, point_c, color): (LuaValue, LuaValue, LuaValue, LuaValue),
    ) -> mlua::Result<()> {
        let point_a: Vector2 = lua.from_value(point_a)?;
        let point_b: Vector2 = lua.from_value(point_b)?;
        let point_c: Vector2 = lua.from_value(point_c)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            DrawTriangle(point_a, point_b, point_c, color);
            Ok(())
        }
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.draw_2d.draw_triangle_line",
        "info": "Draw 2D triangle (out-line).",
        "member": [
            { "name": "point_a", "info": "The point A of the triangle.", "kind": "vector_2" },
            { "name": "point_b", "info": "The point B of the triangle.", "kind": "vector_2" },
            { "name": "point_c", "info": "The point C of the triangle.", "kind": "vector_2" },
            { "name": "color",   "info": "The color of the triangle.",   "kind": "color"    }
        ]
    }
    */
    fn draw_triangle_line(
        lua: &Lua,
        (point_a, point_b, point_c, color): (LuaValue, LuaValue, LuaValue, LuaValue),
    ) -> mlua::Result<()> {
        let point_a: Vector2 = lua.from_value(point_a)?;
        let point_b: Vector2 = lua.from_value(point_b)?;
        let point_c: Vector2 = lua.from_value(point_c)?;
        let color: Color = lua.from_value(color)?;

        unsafe {
            DrawTriangleLines(point_a, point_b, point_c, color);
            Ok(())
        }
    }
}

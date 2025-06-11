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
{ "version": "1.0.0", "name": "alicia.collision", "info": "The collision API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table, _: &StatusInfo, _: Option<&ScriptInfo>) -> mlua::Result<()> {
    let collision = lua.create_table()?;

    collision.set("get_box_2_box_2",            lua.create_function(self::get_box_2_box_2)?)?;            // CheckCollisionRecs
    collision.set("get_circle_circle",          lua.create_function(self::get_circle_circle)?)?;          // CheckCollisionCircles
    collision.set("get_circle_box_2",           lua.create_function(self::get_circle_box_2)?)?;           // CheckCollisionCircleRec
    collision.set("get_circle_line",            lua.create_function(self::get_circle_line)?)?;            // CheckCollisionCircleLine
    collision.set("get_point_box_2",            lua.create_function(self::get_point_box_2)?)?;            // CheckCollisionPointRec
    collision.set("get_point_circle",           lua.create_function(self::get_point_circle)?)?;           // CheckCollisionPointCircle
    collision.set("get_point_triangle",         lua.create_function(self::get_point_triangle)?)?;         // CheckCollisionPointTriangle
    collision.set("get_point_line",             lua.create_function(self::get_point_line)?)?;             // CheckCollisionPointLine
    collision.set("get_point_poly",             lua.create_function(self::get_point_poly)?)?;             // CheckCollisionPointPoly
    collision.set("get_line_line",              lua.create_function(self::get_line_line)?)?;              // CheckCollisionLines
    collision.set("get_box_2_box_2_difference", lua.create_function(self::get_box_2_box_2_difference)?)?; // GetCollisionRec

    //================================================================

    collision.set("get_sphere_sphere", lua.create_function(self::get_sphere_sphere)?)?; // CheckCollisionSpheres
    collision.set("get_box_3_box_3",   lua.create_function(self::get_box_3_box_3)?)?;   // CheckCollisionBoxes
    collision.set("get_box_3_sphere",  lua.create_function(self::get_box_3_sphere)?)?;  // CheckCollisionBoxSphere
    collision.set("get_ray_sphere",    lua.create_function(self::get_ray_sphere)?)?;    // GetRayCollisionSphere
    collision.set("get_ray_box_3",     lua.create_function(self::get_ray_box_3)?)?;     // GetRayCollisionBox
    collision.set("get_ray_triangle",  lua.create_function(self::get_ray_triangle)?)?;  // GetRayCollisionTriangle
    collision.set("get_ray_quad",      lua.create_function(self::get_ray_quad)?)?;      // GetRayCollisionQuad

    table.set("collision", collision)?;

    Ok(())
}

//================================================================

/* entry
{
    "version": "1.0.0",
    "name": "alicia.collision.get_box_2_box_2",
    "info": "Check if a 2D box is intersecting against another 2D box.",
    "member": [
        { "name": "box_a", "info": "2D box (A).", "kind": "box_2" },
        { "name": "box_b", "info": "2D box (B).", "kind": "box_2" }
    ],
    "result": [
        { "name": "intersect", "info": "Result of intersection.", "kind": "boolean" }
    ]
}
*/
fn get_box_2_box_2(lua: &Lua, (box_a, box_b): (LuaValue, LuaValue)) -> mlua::Result<bool> {
    let box_a: Rectangle = lua.from_value(box_a)?;
    let box_b: Rectangle = lua.from_value(box_b)?;

    unsafe { Ok(CheckCollisionRecs(box_a, box_b)) }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.collision.get_circle_circle",
    "info": "Check if a circle is intersecting against another circle.",
    "member": [
        { "name": "point_a",  "info": "Circle point (A).", "kind": "vector_2" },
        { "name": "range_a",  "info": "Circle range (A).", "kind": "number"   },
        { "name": "point_b",  "info": "Circle point (B).", "kind": "vector_2" },
        { "name": "range_b",  "info": "Circle range (B).", "kind": "number"   }
    ],
    "result": [
        { "name": "intersect", "info": "Result of intersection.", "kind": "boolean" }
    ]
}
*/
fn get_circle_circle(
    lua: &Lua,
    (point_a, range_a, point_b, range_b): (LuaValue, f32, LuaValue, f32),
) -> mlua::Result<bool> {
    let point_a: Vector2 = lua.from_value(point_a)?;
    let point_b: Vector2 = lua.from_value(point_b)?;

    unsafe { Ok(CheckCollisionCircles(point_a, range_a, point_b, range_b)) }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.collision.get_circle_box_2",
    "info": "Check if a circle is intersecting against another 2D box.",
    "member": [
        { "name": "point_a", "info": "Circle point.", "kind": "vector_2" },
        { "name": "range_a", "info": "Circle range.", "kind": "number"   },
        { "name": "box_a",   "info": "2D box.",       "kind": "box_2"    }
    ],
    "result": [
        { "name": "intersect", "info": "Result of intersection.", "kind": "boolean" }
    ]
}
*/
fn get_circle_box_2(
    lua: &Lua,
    (point_a, range_a, box_a): (LuaValue, f32, LuaValue),
) -> mlua::Result<bool> {
    let point_a: Vector2 = lua.from_value(point_a)?;
    let box_a: Rectangle = lua.from_value(box_a)?;

    unsafe { Ok(CheckCollisionCircleRec(point_a, range_a, box_a)) }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.collision.get_circle_line",
    "info": "Check if a circle is intersecting against another line.",
    "member": [
        { "name": "point_a", "info": "Circle point.",   "kind": "vector_2" },
        { "name": "range_a", "info": "Circle range.",   "kind": "number"   },
        { "name": "line_a",  "info": "Line point (A).", "kind": "vector_2" },
        { "name": "line_b",  "info": "Line point (B).", "kind": "vector_2" }
    ],
    "result": [
        { "name": "intersect", "info": "Result of intersection.", "kind": "boolean" }
    ]
}
*/
fn get_circle_line(
    lua: &Lua,
    (point_a, range_a, line_a, line_b): (LuaValue, f32, LuaValue, LuaValue),
) -> mlua::Result<bool> {
    let point_a: Vector2 = lua.from_value(point_a)?;
    let line_a: Vector2 = lua.from_value(line_a)?;
    let line_b: Vector2 = lua.from_value(line_b)?;

    unsafe { Ok(CheckCollisionCircleLine(point_a, range_a, line_a, line_b)) }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.collision.get_point_box_2",
    "info": "Check if a point is intersecting against another 2D box.",
    "member": [
        { "name": "point_a", "info": "Point.",  "kind": "vector_2" },
        { "name": "box_a",   "info": "2D box.", "kind": "box_2"    }
    ],
    "result": [
        { "name": "intersect", "info": "Result of intersection.", "kind": "boolean" }
    ]
}
*/
fn get_point_box_2(lua: &Lua, (point_a, box_a): (LuaValue, LuaValue)) -> mlua::Result<bool> {
    let point_a: Vector2 = lua.from_value(point_a)?;
    let box_a: Rectangle = lua.from_value(box_a)?;

    unsafe { Ok(CheckCollisionPointRec(point_a, box_a)) }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.collision.get_point_circle",
    "info": "Check if a point is intersecting against another circle.",
    "member": [
        { "name": "point_a", "info": "Point (A).",        "kind": "vector_2" },
        { "name": "point_b", "info": "Circle point (B).", "kind": "vector_2" },
        { "name": "range_b", "info": "Circle range (B).", "kind": "number"   }
    ],
    "result": [
        { "name": "intersect", "info": "Result of intersection.", "kind": "boolean" }
    ]
}
*/
fn get_point_circle(
    lua: &Lua,
    (point_a, point_b, range_b): (LuaValue, LuaValue, f32),
) -> mlua::Result<bool> {
    let point_a: Vector2 = lua.from_value(point_a)?;
    let point_b: Vector2 = lua.from_value(point_b)?;

    unsafe { Ok(CheckCollisionPointCircle(point_a, point_b, range_b)) }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.collision.get_point_triangle",
    "info": "Check if a point is intersecting against another triangle.",
    "member": [
        { "name": "point_a", "info": "Point (A).",          "kind": "vector_2" },
        { "name": "point_b", "info": "Triangle point (B).", "kind": "vector_2" },
        { "name": "point_c", "info": "Triangle point (C).", "kind": "vector_2" },
        { "name": "point_d", "info": "Triangle point (D).", "kind": "vector_2" }
    ],
    "result": [
        { "name": "intersect", "info": "Result of intersection.", "kind": "boolean" }
    ]
}
*/
fn get_point_triangle(
    lua: &Lua,
    (point_a, point_b, point_c, point_d): (LuaValue, LuaValue, LuaValue, LuaValue),
) -> mlua::Result<bool> {
    let point_a: Vector2 = lua.from_value(point_a)?;
    let point_b: Vector2 = lua.from_value(point_b)?;
    let point_c: Vector2 = lua.from_value(point_c)?;
    let point_d: Vector2 = lua.from_value(point_d)?;

    unsafe {
        Ok(CheckCollisionPointTriangle(
            point_a, point_b, point_c, point_d,
        ))
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.collision.get_point_line",
    "info": "TO-DO"
}
*/
fn get_point_line(
    lua: &Lua,
    (point, point_a, point_b, threshold): (LuaValue, LuaValue, LuaValue, i32),
) -> mlua::Result<bool> {
    let point: Vector2 = lua.from_value(point)?;
    let point_a: Vector2 = lua.from_value(point_a)?;
    let point_b: Vector2 = lua.from_value(point_b)?;

    unsafe { Ok(CheckCollisionPointLine(point, point_a, point_b, threshold)) }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.collision.get_point_poly",
    "info": "TO-DO"
}
*/
fn get_point_poly(lua: &Lua, (point, point_list): (LuaValue, LuaValue)) -> mlua::Result<bool> {
    let point: Vector2 = lua.from_value(point)?;
    let point_list: Vec<Vector2> = lua.from_value(point_list)?;

    unsafe {
        Ok(CheckCollisionPointPoly(
            point,
            point_list.as_ptr(),
            point_list.len() as i32,
        ))
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.collision.get_line_line",
    "info": "TO-DO"
}
*/
fn get_line_line(
    lua: &Lua,
    (point_a, point_b, point_c, point_d): (LuaValue, LuaValue, LuaValue, LuaValue),
) -> mlua::Result<(Option<f32>, Option<f32>)> {
    let point_a: Vector2 = lua.from_value(point_a)?;
    let point_b: Vector2 = lua.from_value(point_b)?;
    let point_c: Vector2 = lua.from_value(point_c)?;
    let point_d: Vector2 = lua.from_value(point_d)?;
    let mut point = Vector2 { x: 0.0, y: 0.0 };

    unsafe {
        if CheckCollisionLines(point_a, point_b, point_c, point_d, &mut point) {
            Ok((Some(point.x), Some(point.y)))
        } else {
            Ok((None, None))
        }
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.collision.get_box_2_box_2_difference",
    "info": "TO-DO"
}
*/
fn get_box_2_box_2_difference(
    lua: &Lua,
    (box_a, box_b): (LuaValue, LuaValue),
) -> mlua::Result<(f32, f32, f32, f32)> {
    let box_a: Rectangle = lua.from_value(box_a)?;
    let box_b: Rectangle = lua.from_value(box_b)?;

    unsafe {
        let value = GetCollisionRec(box_a, box_b);
        Ok((value.x, value.y, value.width, value.height))
    }
}

//================================================================

/* entry
{
    "version": "1.0.0",
    "name": "alicia.collision.get_sphere_sphere",
    "info": "TO-DO"
}
*/
fn get_sphere_sphere(
    lua: &Lua,
    (point_a, point_b, range_a, range_b): (LuaValue, LuaValue, f32, f32),
) -> mlua::Result<bool> {
    let point_a: Vector3 = lua.from_value(point_a)?;
    let point_b: Vector3 = lua.from_value(point_b)?;

    unsafe { Ok(CheckCollisionSpheres(point_a, range_a, point_b, range_b)) }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.collision.get_box_3_box_3",
    "info": "TO-DO"
}
*/
fn get_box_3_box_3(lua: &Lua, (box_a, box_b): (LuaValue, LuaValue)) -> mlua::Result<bool> {
    let box_a: BoundingBox = lua.from_value(box_a)?;
    let box_b: BoundingBox = lua.from_value(box_b)?;

    unsafe { Ok(CheckCollisionBoxes(box_a, box_b)) }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.collision.get_box_3_sphere",
    "info": "TO-DO"
}
*/
fn get_box_3_sphere(
    lua: &Lua,
    (box_a, point_a, range_a): (LuaValue, LuaValue, f32),
) -> mlua::Result<bool> {
    let box_a: BoundingBox = lua.from_value(box_a)?;
    let point_a: Vector3 = lua.from_value(point_a)?;

    unsafe { Ok(CheckCollisionBoxSphere(box_a, point_a, range_a)) }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.collision.get_ray_sphere",
    "info": "TO-DO"
}
*/
fn get_ray_sphere(
    lua: &Lua,
    (ray_a, point_a, range_a): (LuaValue, LuaValue, f32),
) -> mlua::Result<(
    Option<f32>,
    Option<f32>,
    Option<f32>,
    Option<f32>,
    Option<f32>,
    Option<f32>,
    Option<f32>,
)> {
    let ray_a: Ray = lua.from_value(ray_a)?;
    let point_a: Vector3 = lua.from_value(point_a)?;

    unsafe {
        let value = GetRayCollisionSphere(ray_a, point_a, range_a);

        if value.hit {
            Ok((
                Some(value.point.x),
                Some(value.point.y),
                Some(value.point.z),
                Some(value.normal.x),
                Some(value.normal.y),
                Some(value.normal.z),
                Some(value.distance),
            ))
        } else {
            Ok((None, None, None, None, None, None, None))
        }
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.collision.get_ray_box_3",
    "info": "TO-DO"
}
*/
fn get_ray_box_3(
    lua: &Lua,
    (ray_a, box_a): (LuaValue, LuaValue),
) -> mlua::Result<(
    Option<f32>,
    Option<f32>,
    Option<f32>,
    Option<f32>,
    Option<f32>,
    Option<f32>,
    Option<f32>,
)> {
    let ray_a: Ray = lua.from_value(ray_a)?;
    let box_a: BoundingBox = lua.from_value(box_a)?;

    unsafe {
        let value = GetRayCollisionBox(ray_a, box_a);

        if value.hit {
            Ok((
                Some(value.point.x),
                Some(value.point.y),
                Some(value.point.z),
                Some(value.normal.x),
                Some(value.normal.y),
                Some(value.normal.z),
                Some(value.distance),
            ))
        } else {
            Ok((None, None, None, None, None, None, None))
        }
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.collision.get_ray_triangle",
    "info": "TO-DO"
}
*/
fn get_ray_triangle(
    lua: &Lua,
    (ray_a, point_a, point_b, point_c): (LuaValue, LuaValue, LuaValue, LuaValue),
) -> mlua::Result<(
    Option<f32>,
    Option<f32>,
    Option<f32>,
    Option<f32>,
    Option<f32>,
    Option<f32>,
    Option<f32>,
)> {
    let ray_a: Ray = lua.from_value(ray_a)?;
    let point_a: Vector3 = lua.from_value(point_a)?;
    let point_b: Vector3 = lua.from_value(point_b)?;
    let point_c: Vector3 = lua.from_value(point_c)?;

    unsafe {
        let value = GetRayCollisionTriangle(ray_a, point_a, point_b, point_c);

        if value.hit {
            Ok((
                Some(value.point.x),
                Some(value.point.y),
                Some(value.point.z),
                Some(value.normal.x),
                Some(value.normal.y),
                Some(value.normal.z),
                Some(value.distance),
            ))
        } else {
            Ok((None, None, None, None, None, None, None))
        }
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.collision.get_ray_quad",
    "info": "TO-DO"
}
*/
fn get_ray_quad(
    lua: &Lua,
    (ray_a, point_a, point_b, point_c, point_d): (LuaValue, LuaValue, LuaValue, LuaValue, LuaValue),
) -> mlua::Result<(
    Option<f32>,
    Option<f32>,
    Option<f32>,
    Option<f32>,
    Option<f32>,
    Option<f32>,
    Option<f32>,
)> {
    let ray_a: Ray = lua.from_value(ray_a)?;
    let point_a: Vector3 = lua.from_value(point_a)?;
    let point_b: Vector3 = lua.from_value(point_b)?;
    let point_c: Vector3 = lua.from_value(point_c)?;
    let point_d: Vector3 = lua.from_value(point_d)?;

    unsafe {
        let value = GetRayCollisionQuad(ray_a, point_a, point_b, point_c, point_d);

        if value.hit {
            Ok((
                Some(value.point.x),
                Some(value.point.y),
                Some(value.point.z),
                Some(value.normal.x),
                Some(value.normal.y),
                Some(value.normal.z),
                Some(value.distance),
            ))
        } else {
            Ok((None, None, None, None, None, None, None))
        }
    }
}

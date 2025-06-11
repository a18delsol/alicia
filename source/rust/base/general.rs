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

use std::time::{SystemTime, UNIX_EPOCH};

use crate::script::*;
use crate::status::*;

//================================================================

use crate::base::helper::*;
use mlua::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "system_info")]
use sysinfo::System;

//================================================================

/* class
{ "version": "1.0.0", "name": "alicia.general", "info": "The general API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table, _: &StatusInfo, _: Option<&ScriptInfo>) -> mlua::Result<()> {
    let general = lua.create_table()?;

    general.set("load_base",       lua.create_function(self::load_base)?)?;
    general.set("set_log_level",   lua.create_function(self::set_log_level)?)?;
    general.set("open_link",       lua.create_function(self::open_link)?)?;
    general.set("standard_input",  lua.create_function(self::standard_input)?)?;
    general.set("get_frame_time",  lua.create_function(self::get_frame_time)?)?;
    general.set("get_frame_rate",  lua.create_function(self::get_frame_rate)?)?;
    general.set("set_frame_rate",  lua.create_function(self::set_frame_rate)?)?;
    general.set("get_time",        lua.create_function(self::get_time)?)?;
    general.set("get_time_unix",   lua.create_function(self::get_time_unix)?)?;
    general.set("get_argument",    lua.create_function(self::get_argument)?)?;

    #[cfg(feature = "system_info")]
    general.set("get_system", lua.create_function(self::get_system)?)?;

    general.set("get_memory", lua.create_function(self::get_memory)?)?;
    general.set("get_info",   lua.create_function(self::get_info)?)?;

    table.set("general", general)?;

    Ok(())
}

//================================================================

/* entry
{
    "version": "1.0.0",
    "name": "alicia.general.standard_input",
    "info": "Get the standard input.",
    "result": [
        { "name": "input", "info": "The standard input.", "kind": "string" }
    ]
}
*/
fn standard_input(_: &Lua, _: ()) -> mlua::Result<String> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_string())
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.general.load_base",
    "info": "Load the standard Lua library."
}
*/
fn load_base(lua: &Lua, _: ()) -> mlua::Result<()> {
    // TO-DO only for debug. do not re-load from disk on release.
    for base in crate::script::Script::FILE_BASE {
        // load the base library from disk if using a debug build.
        let data = if cfg!(debug_assertions) {
            &std::fs::read_to_string(format!("../source/lua/{}", base.name)).unwrap()
        } else {
            base.data
        };

        lua.load(data).set_name(format!("@{}", base.name)).exec()?;
    }

    Ok(())
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.general.set_log_level",
    "info": "Set the log level.",
    "member": [
        { "name": "level", "info": "The log level.", "kind": "number" }
    ]
}
*/
fn set_log_level(_: &Lua, level: i32) -> mlua::Result<()> {
    unsafe {
        SetTraceLogLevel(level);
        Ok(())
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.general.open_link",
    "info": "Open an URL link.",
    "member": [
        { "name": "link", "info": "The URL link.", "kind": "string" }
    ]
}
*/
fn open_link(_: &Lua, link: String) -> mlua::Result<()> {
    unsafe {
        OpenURL(link.as_ptr() as *const i8);
        Ok(())
    }
}

/* entry
{
    "version": "1.0.0", "name": "alicia.general.get_time",
    "info": "Get the current time. Will count up since the initialization of the window.",
    "result": [
        { "name": "time", "info": "Current time.", "kind": "number" }
    ]
}
*/
fn get_time(_: &Lua, _: ()) -> mlua::Result<f64> {
    unsafe { Ok(GetTime()) }
}

/* entry
{
    "version": "1.0.0", "name": "alicia.general.get_time_unix",
    "info": "Get the time in UNIX time-stamp format.",
    "member": [
        { "name": "add", "info": "OPTIONAL: Add (or subtract) by this amount.", "kind": "number?" }
    ]
}
*/
fn get_time_unix(_: &Lua, add: Option<i64>) -> mlua::Result<String> {
    let time = SystemTime::now();
    let time = time.duration_since(UNIX_EPOCH).unwrap();
    let time = time.as_secs() + (add.unwrap_or_default() as u64);

    Ok(time.to_string())
}

/* entry
{
    "version": "1.0.0", "name": "alicia.general.get_frame_time",
    "info": "Get the current frame time.",
    "result": [
        { "name": "frame_time", "info": "Current frame time.", "kind": "number" }
    ]
}
*/
fn get_frame_time(_: &Lua, _: ()) -> mlua::Result<f32> {
    unsafe { Ok(GetFrameTime()) }
}

/* entry
{
    "version": "1.0.0", "name": "alicia.general.get_frame_rate",
    "info": "Get the current frame rate.",
    "result": [
        { "name": "frame_rate", "info": "Current frame rate.", "kind": "number" }
    ]
}
*/
fn get_frame_rate(_: &Lua, _: ()) -> mlua::Result<i32> {
    unsafe { Ok(GetFPS()) }
}

/* entry
{
    "version": "1.0.0", "name": "alicia.general.set_frame_rate",
    "info": "Set the current frame rate.",
    "member": [
        { "name": "frame_rate", "info": "Current frame rate.", "kind": "number" }
    ]
}
*/
fn set_frame_rate(_: &Lua, rate: i32) -> mlua::Result<()> {
    unsafe {
        SetTargetFPS(rate);
        Ok(())
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.general.get_argument",
    "info": "Get the argument list.",
    "result": [
        { "name": "list", "info": "The list of every argument.", "kind": "table" }
    ]
}
*/
fn get_argument(lua: &Lua, _: ()) -> mlua::Result<LuaValue> {
    let value: Vec<String> = std::env::args().collect();

    lua.to_value(&value)
}

/* entry
{
    "version": "1.0.0",
    "feature": "system_info",
    "name": "alicia.general.get_system",
    "info": "Get the system info.",
    "result": [
        { "name": "info", "info": "The system info.", "kind": "table" }
    ]
}
*/
#[cfg(feature = "system_info")]
fn get_system(lua: &Lua, _: ()) -> mlua::Result<LuaValue> {
    let mut system = System::new_all();
    system.refresh_all();

    lua.to_value(&system)
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.general.get_memory",
    "info": "Get the currently in-use memory by the Lua VM.",
    "result": [
        { "name": "memory", "info": "The currently in-use memory.", "kind": "number" }
    ]
}
*/
fn get_memory(lua: &Lua, _: ()) -> mlua::Result<usize> {
    Ok(lua.used_memory())
}

// TO-DO "get info" might be kind of misleading? it's a lot more than just the info manifest.
/* entry
{
    "version": "1.0.0",
    "name": "alicia.general.get_info",
    "info": "Get the current info manifest.",
    "result": [
        { "name": "info", "info": "The info manifest.", "kind": "table" }
    ]
}
*/
fn get_info(lua: &Lua, _: ()) -> mlua::Result<LuaValue> {
    let script_data = lua.app_data_ref::<crate::script::ScriptData>().unwrap();

    lua.to_value(&*script_data)
}

//================================================================

use serde::ser::SerializeMap;
use serde::{Deserializer, Serializer, de::MapAccess, de::Visitor};
use std::fmt;

impl<'de> Deserialize<'de> for Vector2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(Vector2Visitor)
    }
}

struct Vector2Visitor;

impl<'de> Visitor<'de> for Vector2Visitor {
    type Value = Vector2;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a map with keys 'first' and 'second'")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut x = None;
        let mut y = None;

        while let Some(k) = map.next_key::<String>()? {
            match k.as_str() {
                "x" => x = Some(map.next_value()?),
                "y" => y = Some(map.next_value()?),
                _ => {}
            }
        }

        if x.is_none() {
            return Err(serde::de::Error::custom("vector_2: Missing \"x\" key."));
        }

        if y.is_none() {
            return Err(serde::de::Error::custom("vector_2: Missing \"y\" key."));
        }

        Ok(Vector2 {
            x: x.unwrap(),
            y: y.unwrap(),
        })
    }
}

impl Serialize for Vector3 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_map(Some(3))?;
        seq.serialize_entry("x", &self.x)?;
        seq.serialize_entry("y", &self.y)?;
        seq.serialize_entry("z", &self.z)?;
        seq.end()
    }
}

impl<'de> Deserialize<'de> for Vector3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(Vector3Visitor)
    }
}

struct Vector3Visitor;

impl<'de> Visitor<'de> for Vector3Visitor {
    type Value = Vector3;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a map with keys 'first' and 'second'")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut x = None;
        let mut y = None;
        let mut z = None;

        while let Some(k) = map.next_key::<String>()? {
            match k.as_str() {
                "x" => x = Some(map.next_value()?),
                "y" => y = Some(map.next_value()?),
                "z" => z = Some(map.next_value()?),
                _ => {}
            }
        }

        if x.is_none() {
            return Err(serde::de::Error::custom("vector_3: Missing \"x\" key."));
        }

        if y.is_none() {
            return Err(serde::de::Error::custom("vector_3: Missing \"y\" key."));
        }

        if z.is_none() {
            return Err(serde::de::Error::custom("vector_3: Missing \"z\" key."));
        }

        Ok(Vector3 {
            x: x.unwrap(),
            y: y.unwrap(),
            z: z.unwrap(),
        })
    }
}

impl<'de> Deserialize<'de> for Vector4 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(Vector4Visitor)
    }
}

struct Vector4Visitor;

impl<'de> Visitor<'de> for Vector4Visitor {
    type Value = Vector4;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a map with keys 'first' and 'second'")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut x = None;
        let mut y = None;
        let mut z = None;
        let mut w = None;

        while let Some(k) = map.next_key::<String>()? {
            match k.as_str() {
                "x" => x = Some(map.next_value()?),
                "y" => y = Some(map.next_value()?),
                "z" => z = Some(map.next_value()?),
                "w" => w = Some(map.next_value()?),
                _ => {}
            }
        }

        if x.is_none() {
            return Err(serde::de::Error::custom("vector_4: Missing \"x\" key."));
        }

        if y.is_none() {
            return Err(serde::de::Error::custom("vector_4: Missing \"y\" key."));
        }

        if z.is_none() {
            return Err(serde::de::Error::custom("vector_4: Missing \"z\" key."));
        }

        if w.is_none() {
            return Err(serde::de::Error::custom("vector_4: Missing \"z\" key."));
        }

        Ok(Vector4 {
            x: x.unwrap(),
            y: y.unwrap(),
            z: z.unwrap(),
            w: w.unwrap(),
        })
    }
}

impl<'de> Deserialize<'de> for Rectangle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(RectangleVisitor)
    }
}

struct RectangleVisitor;

impl<'de> Visitor<'de> for RectangleVisitor {
    type Value = Rectangle;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a map with keys 'first' and 'second'")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut point: Option<Vector2> = None;
        let mut shape: Option<Vector2> = None;

        while let Some(k) = map.next_key::<String>()? {
            match k.as_str() {
                "point" => point = Some(map.next_value()?),
                "shape" => shape = Some(map.next_value()?),
                _ => {}
            }
        }

        if point.is_none() {
            return Err(serde::de::Error::custom("box_2: Missing \"point\" key."));
        }

        if shape.is_none() {
            return Err(serde::de::Error::custom("box_2: Missing \"shape\" key."));
        }

        let point = point.unwrap();
        let shape = shape.unwrap();

        Ok(Rectangle {
            x: point.x,
            y: point.y,
            width: shape.x,
            height: shape.y,
        })
    }
}

impl<'de> Deserialize<'de> for BoundingBox {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(BoundingBoxVisitor)
    }
}

struct BoundingBoxVisitor;

impl<'de> Visitor<'de> for BoundingBoxVisitor {
    type Value = BoundingBox;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a map with keys 'first' and 'second'")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut min = None;
        let mut max = None;

        while let Some(k) = map.next_key::<String>()? {
            match k.as_str() {
                "min" => min = Some(map.next_value()?),
                "max" => max = Some(map.next_value()?),
                _ => {}
            }
        }

        if min.is_none() {
            return Err(serde::de::Error::custom("box_3: Missing \"min\" key."));
        }

        if max.is_none() {
            return Err(serde::de::Error::custom("box_3: Missing \"max\" key."));
        }

        Ok(BoundingBox {
            min: min.unwrap(),
            max: max.unwrap(),
        })
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(ColorVisitor)
    }
}

struct ColorVisitor;

impl<'de> Visitor<'de> for ColorVisitor {
    type Value = Color;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a map with keys 'first' and 'second'")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut r = None;
        let mut g = None;
        let mut b = None;
        let mut a = None;

        while let Some(k) = map.next_key::<String>()? {
            match k.as_str() {
                "r" => r = Some(map.next_value()?),
                "g" => g = Some(map.next_value()?),
                "b" => b = Some(map.next_value()?),
                "a" => a = Some(map.next_value()?),
                _ => {}
            }
        }

        if r.is_none() {
            return Err(serde::de::Error::custom("color: Missing \"r\" key."));
        }

        if g.is_none() {
            return Err(serde::de::Error::custom("color: Missing \"g\" key."));
        }

        if b.is_none() {
            return Err(serde::de::Error::custom("color: Missing \"b\" key."));
        }

        if a.is_none() {
            return Err(serde::de::Error::custom("color: Missing \"a\" key."));
        }

        Ok(Color {
            r: r.unwrap(),
            g: g.unwrap(),
            b: b.unwrap(),
            a: a.unwrap(),
        })
    }
}

impl<'de> Deserialize<'de> for Ray {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(RayVisitor)
    }
}

struct RayVisitor;

impl<'de> Visitor<'de> for RayVisitor {
    type Value = Ray;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a map with keys 'first' and 'second'")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut position = None;
        let mut direction = None;

        while let Some(k) = map.next_key::<String>()? {
            match k.as_str() {
                "point" => position = Some(map.next_value()?),
                "focus" => direction = Some(map.next_value()?),
                _ => {}
            }
        }

        if position.is_none() {
            return Err(serde::de::Error::custom("ray: Missing \"position\" key."));
        }

        if direction.is_none() {
            return Err(serde::de::Error::custom("ray: Missing \"direction\" key."));
        }

        Ok(Ray {
            position: position.unwrap(),
            direction: direction.unwrap(),
        })
    }
}

impl<'de> Deserialize<'de> for Camera2D {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(Camera2DVisitor)
    }
}

struct Camera2DVisitor;

impl<'de> Visitor<'de> for Camera2DVisitor {
    type Value = Camera2D;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a map with keys 'first' and 'second'")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut offset = None;
        let mut target = None;
        let mut rotation = None;
        let mut zoom = None;

        while let Some(k) = map.next_key::<String>()? {
            match k.as_str() {
                "shift" => offset = Some(map.next_value()?),
                "focus" => target = Some(map.next_value()?),
                "angle" => rotation = Some(map.next_value()?),
                "zoom" => zoom = Some(map.next_value()?),
                _ => {}
            }
        }

        if offset.is_none() {
            return Err(serde::de::Error::custom(
                "camera_2d: Missing \"shift\" key.",
            ));
        }

        if target.is_none() {
            return Err(serde::de::Error::custom(
                "camera_2d: Missing \"focus\" key.",
            ));
        }

        if rotation.is_none() {
            return Err(serde::de::Error::custom(
                "camera_2d: Missing \"angle\" key.",
            ));
        }

        if zoom.is_none() {
            return Err(serde::de::Error::custom("camera_2d: Missing \"zoom\" key."));
        }

        Ok(Camera2D {
            offset: offset.unwrap(),
            target: target.unwrap(),
            rotation: rotation.unwrap(),
            zoom: zoom.unwrap(),
        })
    }
}

impl<'de> Deserialize<'de> for Camera3D {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(Camera3DVisitor)
    }
}

struct Camera3DVisitor;

impl<'de> Visitor<'de> for Camera3DVisitor {
    type Value = Camera3D;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a map with keys 'first' and 'second'")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut position = None;
        let mut target = None;
        let mut up = None;
        let mut fovy = None;
        let mut projection = None;

        while let Some(k) = map.next_key::<String>()? {
            match k.as_str() {
                "point" => position = Some(map.next_value()?),
                "focus" => target = Some(map.next_value()?),
                "angle" => up = Some(map.next_value()?),
                "zoom" => fovy = Some(map.next_value()?),
                "kind" => projection = Some(map.next_value()?),
                _ => {}
            }
        }

        if position.is_none() {
            return Err(serde::de::Error::custom(
                "camera_3d: Missing \"point\" key.",
            ));
        }

        if target.is_none() {
            return Err(serde::de::Error::custom(
                "camera_3d: Missing \"focus\" key.",
            ));
        }

        if up.is_none() {
            return Err(serde::de::Error::custom(
                "camera_3d: Missing \"angle\" key.",
            ));
        }

        if fovy.is_none() {
            return Err(serde::de::Error::custom("camera_3d: Missing \"zoom\" key."));
        }

        if projection.is_none() {
            return Err(serde::de::Error::custom("camera_3d: Missing \"kind\" key."));
        }

        Ok(Camera3D {
            position: position.unwrap(),
            target: target.unwrap(),
            up: up.unwrap(),
            fovy: fovy.unwrap(),
            projection: projection.unwrap(),
        })
    }
}

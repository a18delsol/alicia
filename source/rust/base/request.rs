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

use mlua::prelude::*;

//================================================================

/* class
{ "version": "1.0.0", "name": "alicia.request", "info": "The request API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table, _: &StatusInfo, _: Option<&ScriptInfo>) -> mlua::Result<()> {
    let request = lua.create_table()?;

    request.set("get",  lua.create_async_function(self::get)?)?;
    request.set("post", lua.create_async_function(self::post)?)?;

    table.set("request", request)?;

    Ok(())
}

//================================================================

/* entry
{
    "version": "1.0.0",
    "name": "alicia.request.get",
    "info": "Perform a GET request.",
    "member": [
        { "name": "link",   "info": "The target URL link.", "kind": "string"  },
        { "name": "binary", "info": "Receive as binary.",   "kind": "boolean" }
    ],
    "result": [
        { "name": "value", "info": "The return value.", "kind": "string | data" }
    ],
    "test": "request/get.lua",
    "routine": true
}
*/
async fn get(lua: Lua, (link, binary): (String, bool)) -> mlua::Result<LuaValue> {
    if binary {
        lua.to_value(
            &reqwest::get(link)
                .await
                .map_err(|e| mlua::Error::runtime(e.to_string()))?
                .bytes()
                .await
                .map_err(|e| mlua::Error::runtime(e.to_string()))?
                .to_vec(),
        )
    } else {
        lua.to_value(
            &reqwest::get(link)
                .await
                .map_err(|e| mlua::Error::runtime(e.to_string()))?
                .text()
                .await
                .map_err(|e| mlua::Error::runtime(e.to_string()))?,
        )
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.request.post",
    "info": "Perform a POST request.",
    "member": [
        { "name": "link",   "info": "The target URL link.",            "kind": "string"  },
        { "name": "data",   "info": "OPTIONAL: The \"data\" payload.", "kind": "string?" },
        { "name": "form",   "info": "OPTIONAL: The \"form\" payload.", "kind": "table?"  },
        { "name": "json",   "info": "OPTIONAL: The \"json\" payload.", "kind": "table?"  },
        { "name": "binary", "info": "Receive as binary.",              "kind": "boolean" }
    ],
    "result": [
        { "name": "value", "info": "The return value.", "kind": "string | data" }
    ],
    "test": "request/post.lua",
    "routine": true
}
*/
async fn post(
    lua: Lua,
    (link, data, form, json, binary): (
        String,
        Option<LuaValue>,
        Option<mlua::Table>,
        Option<mlua::Table>,
        bool,
    ),
) -> mlua::Result<LuaValue> {
    let client = reqwest::Client::new();
    let result = client.post(link);

    let result = if let Some(data) = data {
        match data {
            LuaValue::String(data) => Ok(result.body(data.to_string_lossy())),
            LuaValue::UserData(data) => {
                let data = crate::base::data::Data::get_buffer(mlua::Value::UserData(data))?;
                let data = data.0.clone();
                Ok(result.body(data))
            }
            _ => Err(mlua::Error::runtime(
                "alicia.request.post(): Unknown type for \"data\" argument.",
            )),
        }
    } else {
        Ok(result)
    }?;

    let result = if let Some(form) = form {
        result.form(&form)
    } else {
        result
    };

    let result = if let Some(json) = json {
        result.json(&json)
    } else {
        result
    };

    let result = result
        .send()
        .await
        .map_err(|e| mlua::Error::runtime(e.to_string()))?;

    if binary {
        let data = result
            .bytes()
            .await
            .map_err(|e| mlua::Error::runtime(e.to_string()))?
            .to_vec();

        let data = crate::base::data::Data::new(&lua, data)?;
        let data = lua.create_userdata(data)?;

        Ok(mlua::Value::UserData(data))
    } else {
        lua.to_value(
            &result
                .text()
                .await
                .map_err(|e| mlua::Error::runtime(e.to_string()))?,
        )
    }
}

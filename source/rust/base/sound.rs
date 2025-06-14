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
{ "version": "1.0.0", "name": "alicia.sound", "info": "The sound API.", "head": true }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table, _: &StatusInfo, _: Option<&ScriptInfo>) -> mlua::Result<()> {
    let sound = lua.create_table()?;

    sound.set("new",             lua.create_async_function(self::LuaSound::new)?)?;
    sound.set("new_from_memory", lua.create_async_function(self::LuaSound::new_from_memory)?)?;

    table.set("sound", sound)?;

    Ok(())
}

/* class
{ "version": "1.0.0", "name": "sound", "info": "An unique handle for sound in memory." }
*/
struct LuaSound(Sound, Vec<Sound>);

impl Drop for LuaSound {
    fn drop(&mut self) {
        unsafe {
            for alias in &self.1 {
                UnloadSoundAlias(*alias);
            }

            UnloadSound(self.0);
        }
    }
}

unsafe impl Send for LuaSound {}

impl LuaSound {
    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.sound.new",
        "info": "Create a new sound resource.",
        "member": [
            { "name": "path",  "info": "Path to sound file.",                                          "kind": "string"  },
            { "name": "alias", "info": "OPTIONAL: The total sound alias count to load for the sound.", "kind": "number?" }
        ],
        "result": [
            { "name": "sound", "info": "LuaSound resource.", "kind": "sound" }
        ],
        "routine": true
    }
    */
    async fn new(lua: Lua, (path, alias): (String, Option<usize>)) -> mlua::Result<Self> {
        //tokio::task::spawn_blocking(move || unsafe {
        unsafe {
            let name = Script::rust_to_c_string(&ScriptData::get_path(&lua, &path)?)?;
            let data = LoadSound(name.as_ptr());
            let alias = alias.unwrap_or_default();
            let mut array = Vec::with_capacity(alias);

            if IsSoundValid(data) {
                for _ in 0..alias {
                    let data = LoadSoundAlias(data);
                    println!("Pushing alias...");
                    array.push(data);
                }

                Ok(Self(data, array))
            } else {
                Err(mlua::Error::RuntimeError(format!(
                    "LuaSound::new(): Could not load file \"{path}\"."
                )))
            }
        }
        //})
        //.await
        //.unwrap()
    }

    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.sound.new_from_memory",
        "info": "Create a new sound resource, from memory.",
        "member": [
            { "name": "data",  "info": "The data buffer.",                                             "kind": "data"    },
            { "name": "alias", "info": "OPTIONAL: The total sound alias count to load for the sound.", "kind": "number?" },
            { "name": "kind",  "info": "The kind of sound file (.wav, etc.).",                         "kind": "string"  }
        ],
        "result": [
            { "name": "music", "info": "LuaSound resource.", "kind": "sound" }
        ],
        "routine": true
    }
    */
    async fn new_from_memory(
        _: Lua,
        (data, alias, kind): (LuaValue, Option<usize>, String),
    ) -> mlua::Result<Self> {
        let data = crate::base::data::Data::get_buffer(data)?;

        tokio::task::spawn_blocking(move || unsafe {
            let data = &data.0;

            let data = LoadWaveFromMemory(
                Script::rust_to_c_string(&kind)?.as_ptr(),
                data.as_ptr(),
                data.len() as i32,
            );

            if IsWaveValid(data) {
                let sound = LoadSoundFromWave(data);
                let alias = alias.unwrap_or_default();
                let mut array = Vec::with_capacity(alias);

                UnloadWave(data);

                if IsSoundValid(sound) {
                    for _ in 0..alias {
                        let data = LoadSoundAlias(sound);
                        println!("Pushing alias...");
                        array.push(data);
                    }

                    Ok(Self(sound, array))
                } else {
                    Err(mlua::Error::RuntimeError(
                        "LuaSound::new_from_memory(): Could not load file.".to_string(),
                    ))
                }
            } else {
                Err(mlua::Error::RuntimeError(
                    "LuaSound::new_from_memory(): Could not load file.".to_string(),
                ))
            }
        })
        .await
        .unwrap()
    }
}

impl mlua::UserData for LuaSound {
    fn add_fields<F: mlua::UserDataFields<Self>>(field: &mut F) {
        field.add_field_method_get("alias_count", |_: &Lua, this| Ok(this.1.len()));
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* entry
        { "version": "1.0.0", "name": "sound:create_alias", "info": "Create a sound alias." }
        */
        method.add_method_mut("create_alias", |_, this, _: ()| unsafe {
            let data = LoadSoundAlias(this.0);
            this.1.push(data);

            Ok(())
        });

        /* entry
        { "version": "1.0.0", "name": "sound:remove_alias", "info": "Remove a sound alias." }
        */
        method.add_method_mut("remove_alias", |_, this, _: ()| unsafe {
            if !this.1.is_empty() {
                if let Some(alias) = this.1.first() {
                    UnloadSoundAlias(*alias);
                    this.1.remove(0);
                }
            }

            Ok(())
        });

        /* entry
        { "version": "1.0.0", "name": "sound:remove_alias", "info": "Clear every sound alias." }
        */
        method.add_method_mut("clear_alias", |_, this, _: ()| unsafe {
            for alias in &this.1 {
                UnloadSoundAlias(*alias);
            }

            this.1.clear();

            Ok(())
        });

        /* entry
        { "version": "1.0.0", "name": "sound:play", "info": "Play the sound." }
        */
        method.add_method("play", |_, this, alias: Option<usize>| unsafe {
            if let Some(alias) = alias {
                if let Some(alias) = this.1.get(alias) {
                    PlaySound(*alias);
                } else {
                    return Err(mlua::Error::runtime("sound::play(): Invalid alias index."));
                }
            } else {
                PlaySound(this.0);
            }

            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "sound:get_playing",
            "info": "Check if sound is currently playing.",
            "result": [
                { "name": "state", "info": "State of the sound.", "kind": "boolean" }
            ]
        }
        */
        method.add_method("get_playing", |_, this, alias: Option<usize>| unsafe {
            if let Some(alias) = alias {
                if let Some(alias) = this.1.get(alias) {
                    Ok(IsSoundPlaying(*alias))
                } else {
                    Err(mlua::Error::runtime(
                        "sound::get_playing(): Invalid alias index.",
                    ))
                }
            } else {
                Ok(IsSoundPlaying(this.0))
            }
        });

        /* entry
        { "version": "1.0.0", "name": "sound:stop", "info": "Stop the sound." }
        */
        method.add_method("stop", |_, this, alias: Option<usize>| unsafe {
            if let Some(alias) = alias {
                if let Some(alias) = this.1.get(alias) {
                    StopSound(*alias);
                } else {
                    return Err(mlua::Error::runtime("sound::stop(): Invalid alias index."));
                }
            } else {
                StopSound(this.0);
            }

            Ok(())
        });

        /* entry
        { "version": "1.0.0", "name": "sound:pause", "info": "Pause the sound." }
        */
        method.add_method("pause", |_, this, alias: Option<usize>| unsafe {
            if let Some(alias) = alias {
                if let Some(alias) = this.1.get(alias) {
                    PauseSound(*alias);
                } else {
                    return Err(mlua::Error::runtime("sound::pause(): Invalid alias index."));
                }
            } else {
                PauseSound(this.0);
            }

            Ok(())
        });

        /* entry
        { "version": "1.0.0", "name": "sound:resume", "info": "Resume the sound." }
        */
        method.add_method("resume", |_, this, alias: Option<usize>| unsafe {
            if let Some(alias) = alias {
                if let Some(alias) = this.1.get(alias) {
                    ResumeSound(*alias);
                } else {
                    return Err(mlua::Error::runtime(
                        "sound::resume(): Invalid alias index.",
                    ));
                }
            } else {
                ResumeSound(this.0);
            }

            Ok(())
        });

        /* entry
        {
            "version": "1.0.0",
            "name": "sound:set_volume",
            "info": "Set volume for the sound. (range: 0.0 - 1.0)",
            "member": [
                { "name": "volume", "info": "Current volume.", "kind" : "number" }
            ]
        }
        */
        method.add_method(
            "set_volume",
            |_, this, (value, alias): (f32, Option<usize>)| unsafe {
                if let Some(alias) = alias {
                    if let Some(alias) = this.1.get(alias) {
                        SetSoundVolume(*alias, value);
                    } else {
                        return Err(mlua::Error::runtime(
                            "sound::set_volume(): Invalid alias index.",
                        ));
                    }
                } else {
                    SetSoundVolume(this.0, value);
                }

                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "sound:set_pitch",
            "info": "Set pitch for the sound.",
            "member": [
                { "name": "pitch", "info": "Current pitch.", "kind" : "number" }
            ]
        }
        */
        method.add_method(
            "set_pitch",
            |_, this, (value, alias): (f32, Option<usize>)| unsafe {
                if let Some(alias) = alias {
                    if let Some(alias) = this.1.get(alias) {
                        SetSoundPitch(*alias, value);
                    } else {
                        return Err(mlua::Error::runtime(
                            "sound::set_pitch(): Invalid alias index.",
                        ));
                    }
                } else {
                    SetSoundPitch(this.0, value);
                }

                Ok(())
            },
        );

        /* entry
        {
            "version": "1.0.0",
            "name": "sound:set_pan",
            "info": "Set pan for the sound. (range: 0.0 - 1.0; 0.5 is center)",
            "member": [
                { "name": "pan", "info": "Current pan.", "kind" : "number" }
            ]
        }
        */
        method.add_method(
            "set_pan",
            |_, this, (value, alias): (f32, Option<usize>)| unsafe {
                if let Some(alias) = alias {
                    if let Some(alias) = this.1.get(alias) {
                        SetSoundPan(*alias, value);
                    } else {
                        return Err(mlua::Error::runtime(
                            "sound::set_pan(): Invalid alias index.",
                        ));
                    }
                } else {
                    SetSoundPan(this.0, value);
                }

                Ok(())
            },
        );
    }
}

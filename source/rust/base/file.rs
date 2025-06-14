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
use std::sync::mpsc::Receiver;
use std::thread::JoinHandle;

//================================================================

use crate::base::helper::*;
use mlua::prelude::*;

//================================================================

/* class
{ "version": "1.0.0", "name": "alicia.file", "info": "The file API." }
*/
#[rustfmt::skip]
pub fn set_global(lua: &Lua, table: &mlua::Table, _: &StatusInfo, _: Option<&ScriptInfo>) -> mlua::Result<()> {
    let file = lua.create_table()?;

    file.set("get_file",        lua.create_function(self::get_file)?)?;
    file.set("set_file",        lua.create_function(self::set_file)?)?;
    file.set("move_file",       lua.create_function(self::move_file)?)?;
    file.set("copy_file",       lua.create_function(self::copy_file)?)?;
    file.set("remove_file",     lua.create_function(self::remove_file)?)?;
    file.set("remove_path",     lua.create_function(self::remove_path)?)?;
    file.set("set_path_escape", lua.create_function(self::set_path_escape)?)?;

    //================================================================
    
    file.set("set_call_save_file",        lua.create_function(self::set_call_save_file)?)?;
    file.set("set_call_load_file",        lua.create_function(self::set_call_load_file)?)?;
    file.set("set_call_save_text",        lua.create_function(self::set_call_save_text)?)?;
    file.set("set_call_load_text",        lua.create_function(self::set_call_load_text)?)?;
    file.set("get_file_exist",            lua.create_function(self::get_file_exist)?)?;            // FileExists
    file.set("get_path_exist",            lua.create_function(self::get_path_exist)?)?;            // DirectoryExists
    file.set("get_file_extension_check",  lua.create_function(self::get_file_extension_check)?)?;  // IsFileExtension
    file.set("get_file_size",             lua.create_function(self::get_file_size)?)?;             // GetFileLength
    file.set("get_file_extension",        lua.create_function(self::get_file_extension)?)?;        // GetFileExtension
    file.set("get_file_name",             lua.create_function(self::get_file_name)?)?;             // GetFileName/GetFileNameWithoutExt
    file.set("get_absolute_path",         lua.create_function(self::get_absolute_path)?)?;         // GetDirectoryPath
    file.set("get_previous_path",         lua.create_function(self::get_previous_path)?)?;         // GetPrevDirectoryPath
    file.set("get_work_directory",        lua.create_function(self::get_work_directory)?)?;        // GetWorkingDirectory
    file.set("get_application_directory", lua.create_function(self::get_application_directory)?)?; // GetApplicationDirectory
    file.set("create_path",               lua.create_function(self::create_path)?)?;               // MakeDirectory
    file.set("change_path",               lua.create_function(self::change_path)?)?;               // ChangeDirectory
    file.set("get_path_file",             lua.create_function(self::get_path_file)?)?;             // IsPathFile
    file.set("get_file_name_valid",       lua.create_function(self::get_file_name_valid)?)?;       // IsFileNameValid
    file.set("scan_path",                 lua.create_function(self::scan_path)?)?;                 // LoadDirectoryFiles/LoadDirectoryFilesEx
    file.set("get_file_drop",             lua.create_function(self::get_file_drop)?)?;             // IsFileDropped
    file.set("get_file_drop_list",        lua.create_function(self::get_file_drop_list)?)?;        // LoadDroppedFiles
    file.set("get_file_modification",     lua.create_function(self::get_file_modification)?)?;     // GetFileModTime

    table.set("file", file)?;

    //================================================================

    let file_watcher = lua.create_table()?;

    file_watcher.set("new", lua.create_function(FileWatcher::new)?)?;

    table.set("file_watcher", file_watcher)?;

    Ok(())
}

use notify::{Config, PollWatcher, RecursiveMode, Watcher};

/* class
{
    "version": "1.0.0",
    "name": "file_watcher",
    "info": "TO-DO"
}
*/
#[allow(dead_code)]
pub struct FileWatcher(PollWatcher, JoinHandle<()>, Receiver<notify::Event>);

impl FileWatcher {
    /* entry
    {
        "version": "1.0.0",
        "name": "alicia.file_watcher.new",
        "info": "TO-DO"
    }
    */
    fn new(lua: &Lua, path: String) -> mlua::Result<Self> {
        let path = ScriptData::get_path(lua, &path)?;

        let (tx_a, rx_a) = std::sync::mpsc::channel();
        let (tx, rx) = std::sync::mpsc::channel();
        // use the PollWatcher and disable automatic polling
        let mut watcher = PollWatcher::new(tx, Config::default().with_manual_polling()).unwrap();

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher
            .watch(path.as_ref(), RecursiveMode::Recursive)
            .unwrap();

        // run event receiver on a different thread, we want this one for user input
        let handle = std::thread::spawn(move || {
            for res in rx {
                match res {
                    Ok(event) => tx_a.send(event).unwrap(),
                    _ => {} //Err(e) => println!("watch error: {:?}", e),
                }
            }
        });

        // manually poll for changes, received by the spawned thread
        //watcher.poll().unwrap();

        Ok(Self(watcher, handle, rx_a))
    }
}

impl mlua::UserData for FileWatcher {
    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        /* entry
        {
            "version": "1.0.0",
            "name": "file_watcher:poll",
            "info": "TO-DO"
        }
        */
        method.add_method_mut("poll", |lua, this, ()| {
            this.0.poll().unwrap();

            if let Ok(event) = this.2.try_recv() {
                let path: Vec<String> = event
                    .paths
                    .iter()
                    .map(|x| x.to_str().unwrap().to_string())
                    .collect();

                lua.to_value(&path)
            } else {
                Ok(mlua::Nil)
            }
        });
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.file.get_file",
    "info": "Get the data of a file.",
    "test": "file/get_set.lua",
    "member": [
        { "name": "path",   "info": "Path to file.",   "kind": "string"  },
        { "name": "binary", "info": "Read as binary.", "kind": "boolean" }
    ],
    "result": [
        { "name": "data", "info": "File data.", "kind": "string" }
    ]
}
*/
// TO-DO swap binary and buffer around.
fn get_file(lua: &Lua, (path, binary, buffer): (String, bool, bool)) -> mlua::Result<LuaValue> {
    if binary {
        let data = std::fs::read(ScriptData::get_path(lua, &path)?)
            .map_err(|e| mlua::Error::runtime(e.to_string()))?;
        let data = crate::base::data::Data::new(lua, data)?;
        let data = lua.create_userdata(data)?;

        Ok(mlua::Value::UserData(data))
    } else {
        if buffer {
            let data: Vec<String> = std::fs::read_to_string(ScriptData::get_path(lua, &path)?)
                .map_err(|e| mlua::Error::runtime(e.to_string()))?
                .lines()
                .map(String::from)
                .collect();

            lua.to_value(&data)
        } else {
            let data = std::fs::read_to_string(ScriptData::get_path(lua, &path)?)
                .map_err(|e| mlua::Error::runtime(e.to_string()))?;

            lua.to_value(&data)
        }
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.file.set_file",
    "info": "Set the data of a file.",
    "test": "file/get_set.lua",
    "member": [
        { "name": "path", "info": "Path to file.", "kind": "string"        },
        { "name": "data", "info": "Data to copy.", "kind": "string | data" }
    ]
}
*/
fn set_file(lua: &Lua, (path, data): (String, LuaValue)) -> mlua::Result<()> {
    match data {
        LuaValue::String(data) => {
            std::fs::write(ScriptData::get_path(lua, &path)?, data.to_string_lossy())
                .map_err(|e| mlua::Error::runtime(e.to_string()))
        }
        LuaValue::UserData(data) => {
            let data = crate::base::data::Data::get_buffer(mlua::Value::UserData(data))?;
            let data = &data.0;

            std::fs::write(ScriptData::get_path(lua, &path)?, data)
                .map_err(|e| mlua::Error::runtime(e.to_string()))
        }
        _ => Err(mlua::Error::runtime("set_file(): Unknown data type.")),
    }?;

    Ok(())
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.file.move_file",
    "info": "Move a file.",
    "member": [
        { "name": "source", "info": "The source path.", "kind": "string" },
        { "name": "target", "info": "The target path.", "kind": "string" }
    ]
}
*/
fn move_file(lua: &Lua, (source, target): (String, String)) -> mlua::Result<()> {
    let source = ScriptData::get_path(lua, &source)?;
    let target = ScriptData::get_path(lua, &target)?;

    std::fs::rename(source, target).map_err(mlua::Error::runtime)?;

    Ok(())
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.file.copy_file",
    "info": "Copy a file.",
    "member": [
        { "name": "source", "info": "The source path.", "kind": "string" },
        { "name": "target", "info": "The target path.", "kind": "string" }
    ]
}
*/
fn copy_file(lua: &Lua, (source, target): (String, String)) -> mlua::Result<()> {
    let source = ScriptData::get_path(lua, &source)?;
    let target = ScriptData::get_path(lua, &target)?;

    std::fs::copy(source, target).map_err(mlua::Error::runtime)?;

    Ok(())
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.file.remove_file",
    "info": "Remove a file.",
    "member": [
        { "name": "path", "info": "The path to the file to remove.", "kind": "string" }
    ]
}
*/
fn remove_file(lua: &Lua, path: String) -> mlua::Result<()> {
    let path = ScriptData::get_path(lua, &path)?;

    std::fs::remove_file(path).map_err(mlua::Error::runtime)?;

    Ok(())
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.file.remove_path",
    "info": "Remove a folder.",
    "member": [
        { "name": "path", "info": "The path to the folder to remove.", "kind": "string" }
    ]
}
*/
fn remove_path(lua: &Lua, path: String) -> mlua::Result<()> {
    let path = ScriptData::get_path(lua, &path)?;

    std::fs::remove_dir_all(path).map_err(mlua::Error::runtime)?;

    Ok(())
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.file.set_path_escape",
    "info": "Set the state of the path sand-box.",
    "member": [
        { "name": "state", "info": "The state of the path sand-box.", "kind": "boolean" }
    ]
}
*/
fn set_path_escape(lua: &Lua, state: bool) -> mlua::Result<()> {
    ScriptData::set_path_escape(lua, state)
}

//================================================================

unsafe extern "C" fn call_save_file(
    file_name: *const i8,
    data: *mut std::ffi::c_void,
    size: i32,
) -> bool {
    unsafe {
        let pointer = &raw const CALL_SAVE_FILE;

        if let Some(Some(call)) = pointer.as_ref() {
            let file_name = Script::c_to_rust_string(file_name)
                .map_err(|x| Status::panic(&x.to_string()))
                .unwrap();

            let data = data as *mut u8;
            let data = Vec::from_raw_parts(data, size as usize, size as usize);

            let value = call
                .call::<bool>((file_name, data.clone()))
                .map_err(|x| Status::panic(&x.to_string()))
                .unwrap();

            std::mem::forget(data);

            return value;
        }

        false
    }
}

unsafe extern "C" fn call_load_file(file_name: *const i8, data_size: *mut i32) -> *mut u8 {
    unsafe {
        let pointer = &raw const CALL_LOAD_FILE;

        if let Some(Some(call)) = pointer.as_ref() {
            let file_name = Script::c_to_rust_string(file_name)
                .map_err(|x| Status::panic(&x.to_string()))
                .unwrap();

            let value = call
                .call::<LuaValue>(file_name)
                .map_err(|x| Status::panic(&x.to_string()))
                .unwrap();

            if let LuaValue::UserData(value) = value {
                let data = crate::base::data::Data::<u8>::get_buffer(mlua::Value::UserData(value))
                    .map_err(|x| Status::panic(&x.to_string()))
                    .unwrap();

                let mut data = data.0.clone();

                let buffer = data.as_mut_ptr();
                let length = data.len() as i32;

                std::mem::forget(data);

                *data_size = length;

                return buffer;
            }
        }

        *data_size = 0;

        std::ptr::null_mut()
    }
}

unsafe extern "C" fn call_save_text(file_name: *const i8, data: *mut i8) -> bool {
    unsafe {
        let pointer = &raw const CALL_SAVE_TEXT;

        if let Some(Some(call)) = pointer.as_ref() {
            let file_name = Script::c_to_rust_string(file_name)
                .map_err(|x| Status::panic(&x.to_string()))
                .unwrap();

            let data = Script::c_to_rust_string(data)
                .map_err(|x| Status::panic(&x.to_string()))
                .unwrap();

            let value = call
                .call::<bool>((file_name, data))
                .map_err(|x| Status::panic(&x.to_string()))
                .unwrap();

            return value;
        }

        false
    }
}

unsafe extern "C" fn call_load_text(file_name: *const i8) -> *mut i8 {
    unsafe {
        let pointer = &raw const CALL_LOAD_TEXT;

        if let Some(Some(call)) = pointer.as_ref() {
            let file_name = Script::c_to_rust_string(file_name)
                .map_err(|x| Status::panic(&x.to_string()))
                .unwrap();

            let value = call
                .call::<LuaValue>(file_name)
                .map_err(|x| Status::panic(&x.to_string()))
                .unwrap();

            if let LuaValue::String(value) = value {
                let value = Script::rust_to_c_string(&value.to_string_lossy())
                    .map_err(|x| Status::panic(&x.to_string()))
                    .unwrap();

                return value.into_raw();
            }
        }

        std::ptr::null_mut()
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.file.set_call_save_file",
    "info": "Set the file save call-back.",
    "member": [
        { "name": "call", "info": "The call-back. Must accept a file-name and a data parameter, and return a boolean (true on success, false on failure).", "kind": "function" }
    ]
}
*/
fn set_call_save_file(_: &Lua, call: mlua::Function) -> mlua::Result<()> {
    unsafe {
        SetSaveFileDataCallback(Some(call_save_file));

        CALL_SAVE_FILE = Some(call);

        Ok(())
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.file.set_call_load_file",
    "info": "Set the file load call-back.",
    "member": [
        { "name": "call", "info": "The call-back. Must accept a file-name, and return a data buffer. Return anything else to indicate failure.", "kind": "function" }
    ]
}
*/
fn set_call_load_file(_: &Lua, call: mlua::Function) -> mlua::Result<()> {
    unsafe {
        SetLoadFileDataCallback(Some(call_load_file));

        CALL_LOAD_FILE = Some(call);

        Ok(())
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.file.set_call_save_text",
    "info": "Set the file text save call-back.",
    "member": [
        { "name": "call", "info": "The call-back. Must accept a file-name and a string parameter, and return a boolean (true on success, false on failure).", "kind": "function" }
    ]
}
*/
fn set_call_save_text(_: &Lua, call: mlua::Function) -> mlua::Result<()> {
    unsafe {
        SetSaveFileTextCallback(Some(call_save_text));

        CALL_SAVE_TEXT = Some(call);

        Ok(())
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.file.set_call_load_text",
    "info": "Set the file load call-back.",
    "member": [
        { "name": "call", "info": "The call-back. Must accept a file-name, and return a string. Return anything else to indicate failure.", "kind": "function" }
    ]
}
*/
fn set_call_load_text(_: &Lua, call: mlua::Function) -> mlua::Result<()> {
    unsafe {
        SetLoadFileTextCallback(Some(call_load_text));

        CALL_LOAD_TEXT = Some(call);

        Ok(())
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.file.get_file_exist",
    "info": "Check if a file does exist.",
    "test": "file/get_file_exist.lua",
    "member": [
        { "name": "path", "info": "Path to file.", "kind": "string" }
    ],
    "result": [
        { "name": "exist", "info": "True if file does exist, false otherwise.", "kind": "boolean" }
    ]
}
*/
fn get_file_exist(lua: &Lua, path: String) -> mlua::Result<bool> {
    let path = Script::rust_to_c_string(&ScriptData::get_path(lua, &path)?)?;

    unsafe { Ok(FileExists(path.as_ptr())) }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.file.get_path_exist",
    "info": "Check if a path does exist.",
    "test": "file/get_path_exist.lua",
    "member": [
        { "name": "path", "info": "Path.", "kind": "string" }
    ],
    "result": [
        { "name": "exist", "info": "True if path does exist, false otherwise.", "kind": "boolean" }
    ]
}
*/
fn get_path_exist(lua: &Lua, path: String) -> mlua::Result<bool> {
    let path = Script::rust_to_c_string(&ScriptData::get_path(lua, &path)?)?;

    unsafe { Ok(DirectoryExists(path.as_ptr())) }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.file.get_file_extension_check",
    "info": "Check if a file's extension is the same as a given one.",
    "member": [
        { "name": "path",      "info": "Path to file.",                                   "kind": "string" },
        { "name": "extension", "info": "Extension. MUST include dot (.png, .wav, etc.).", "kind": "string" }
    ],
    "result": [
        { "name": "check", "info": "True if file extension is the same as the given one, false otherwise.", "kind": "boolean" }
    ]
}
*/
fn get_file_extension_check(lua: &Lua, (path, extension): (String, String)) -> mlua::Result<bool> {
    let path = Script::rust_to_c_string(&ScriptData::get_path(lua, &path)?)?;
    let extension =
        Script::rust_to_c_string(&extension).map_err(|e| mlua::Error::runtime(e.to_string()))?;

    unsafe { Ok(IsFileExtension(path.as_ptr(), extension.as_ptr())) }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.file.get_file_size",
    "info": "Get the size of a file.",
    "member": [
        { "name": "path", "info": "Path to file.", "kind": "string" }
    ],
    "result": [
        { "name": "size", "info": "File size.", "kind": "number" }
    ]
}
*/
fn get_file_size(lua: &Lua, path: String) -> mlua::Result<i32> {
    let path = Script::rust_to_c_string(&ScriptData::get_path(lua, &path)?)?;

    unsafe { Ok(GetFileLength(path.as_ptr())) }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.file.get_file_extension",
    "info": "Get the extension of a file.",
    "member": [
        { "name": "path", "info": "Path to file.", "kind": "string" }
    ],
    "result": [
        { "name": "extension", "info": "File extension.", "kind": "string" }
    ]
}
*/
fn get_file_extension(lua: &Lua, path: String) -> mlua::Result<String> {
    let path = Script::rust_to_c_string(&ScriptData::get_path(lua, &path)?)?;

    unsafe {
        let result = GetFileExtension(path.as_ptr());
        Script::c_to_rust_string(result)
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.file.get_file_name",
    "info": "Get the name of a file.",
    "member": [
        { "name": "path",      "info": "Path to file.",                                                      "kind": "string"  },
        { "name": "extension", "info": "File extension. If true, will return file name with the extension.", "kind": "boolean" }
    ],
    "result": [
        { "name": "name", "info": "File name.", "kind": "string" }
    ]
}
*/
fn get_file_name(lua: &Lua, (path, extension): (String, bool)) -> mlua::Result<String> {
    let path = Script::rust_to_c_string(&ScriptData::get_path(lua, &path)?)?;

    unsafe {
        if extension {
            let result = GetFileName(path.as_ptr());
            Script::c_to_rust_string(result)
        } else {
            let result = GetFileNameWithoutExt(path.as_ptr());
            Script::c_to_rust_string(result)
        }
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.file.get_absolute_path",
    "info": "TO-DO"
}
*/
fn get_absolute_path(lua: &Lua, path: String) -> mlua::Result<String> {
    unsafe {
        let value = GetDirectoryPath(
            Script::rust_to_c_string(&ScriptData::get_path(lua, &path)?)?.as_ptr(),
        );

        Script::c_to_rust_string(value)
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.file.get_previous_path",
    "info": "TO-DO"
}
*/
fn get_previous_path(lua: &Lua, path: String) -> mlua::Result<String> {
    unsafe {
        let value = GetPrevDirectoryPath(
            Script::rust_to_c_string(&ScriptData::get_path(lua, &path)?)?.as_ptr(),
        );

        Script::c_to_rust_string(value)
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.file.get_work_directory",
    "info": "Get the current work path.",
    "result": [
        { "name": "path", "info": "Work path.", "kind": "string" }
    ]
}
*/
fn get_work_directory(_: &Lua, _: ()) -> mlua::Result<String> {
    unsafe {
        let result = GetWorkingDirectory();
        Script::c_to_rust_string(result)
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.file.get_application_directory",
    "info": "Get the current application path.",
    "result": [
        { "name": "path", "info": "Application path.", "kind": "string" }
    ]
}
*/
fn get_application_directory(_: &Lua, _: ()) -> mlua::Result<String> {
    unsafe {
        let result = GetApplicationDirectory();
        Script::c_to_rust_string(result)
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.file.create_path",
    "info": "TO-DO"
}
*/
fn create_path(lua: &Lua, path: String) -> mlua::Result<()> {
    unsafe {
        let value =
            MakeDirectory(Script::rust_to_c_string(&ScriptData::get_path(lua, &path)?)?.as_ptr());

        if value == 0 {
            Ok(())
        } else {
            Err(mlua::Error::runtime(format!(
                "create_path(): Error on path \"{path}\" creation."
            )))
        }
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.file.change_path",
    "info": "TO-DO"
}
*/
fn change_path(lua: &Lua, path: String) -> mlua::Result<()> {
    unsafe {
        let value =
            ChangeDirectory(Script::rust_to_c_string(&ScriptData::get_path(lua, &path)?)?.as_ptr());

        if value {
            Ok(())
        } else {
            Err(mlua::Error::runtime(format!(
                "change_path(): Error on path \"{path}\" change."
            )))
        }
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.file.get_path_file",
    "info": "TO-DO"
}
*/
fn get_path_file(lua: &Lua, path: String) -> mlua::Result<bool> {
    unsafe {
        Ok(IsPathFile(
            Script::rust_to_c_string(&ScriptData::get_path(lua, &path)?)?.as_ptr(),
        ))
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.file.get_file_name_valid",
    "info": "TO-DO"
}
*/
fn get_file_name_valid(_: &Lua, path: String) -> mlua::Result<bool> {
    unsafe { Ok(IsFileNameValid(Script::rust_to_c_string(&path)?.as_ptr())) }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.file.scan_path",
    "info": "Scan a path.",
    "member": [
        { "name": "path",      "info": "Path to scan.",                                                                               "kind": "string"  },
        { "name": "filter",    "info": "OPTIONAL: Extension filter. If filter is 'DIR', will include every directory in the result.", "kind": "string?" },
        { "name": "recursive", "info": "If true, recursively scan the directory.",                                                    "kind": "boolean" },
        { "name": "absolute",  "info": "If true, return path relatively.",                                                            "kind": "boolean" }
    ],
    "result": [
        { "name": "list", "info": "File list.", "kind": "table" }
    ]
}
*/
fn scan_path(
    lua: &Lua,
    (path, filter, recursive, relative): (String, Option<String>, bool, bool),
) -> mlua::Result<LuaValue> {
    let mut data: Vec<String> = Vec::new();

    unsafe {
        let result = {
            if let Some(filter) = filter {
                let filter = Script::rust_to_c_string(&filter)
                    .map_err(|e| mlua::Error::runtime(e.to_string()))?;
                let c_path = Script::rust_to_c_string(&ScriptData::get_path(lua, &path)?)?;
                LoadDirectoryFilesEx(c_path.as_ptr(), filter.as_ptr(), recursive)
            } else {
                let c_path = Script::rust_to_c_string(&ScriptData::get_path(lua, &path)?)?;
                LoadDirectoryFilesEx(c_path.as_ptr(), std::ptr::null(), recursive)
            }
        };

        for x in 0..result.count {
            let result_path = *result.paths.wrapping_add(x.try_into().unwrap());

            let result_path = Script::c_to_rust_string(result_path)?;

            if relative {
                let path: Vec<&str> = result_path.split(&path).collect();

                if let Some(path) = path.get(1) {
                    // remove the leading back-slash.
                    let path = &path[1..path.len()];

                    data.push(path.to_string());
                }
            } else {
                data.push(result_path);
            }
        }

        UnloadDirectoryFiles(result);

        lua.to_value(&data)
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.file.get_file_drop",
    "info": "TO-DO"
}
*/
fn get_file_drop(_: &Lua, _: ()) -> mlua::Result<bool> {
    unsafe { Ok(IsFileDropped()) }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.file.get_file_drop_list",
    "info": "TO-DO"
}
*/
fn get_file_drop_list(lua: &Lua, _: ()) -> mlua::Result<LuaValue> {
    let mut data: Vec<String> = Vec::new();

    unsafe {
        let result = LoadDroppedFiles();

        for x in 0..result.count {
            let result_path = *result.paths.wrapping_add(x.try_into().unwrap());

            let result_path = Script::c_to_rust_string(result_path)?;

            data.push(result_path);
        }

        UnloadDroppedFiles(result);

        lua.to_value(&data)
    }
}

/* entry
{
    "version": "1.0.0",
    "name": "alicia.file.get_file_modification",
    "info": "TO-DO"
}
*/
fn get_file_modification(lua: &Lua, path: String) -> mlua::Result<i32> {
    unsafe {
        let time: i32 =
            GetFileModTime(Script::rust_to_c_string(&ScriptData::get_path(lua, &path)?)?.as_ptr())
                .try_into()
                .unwrap();
        Ok(time)
    }
}

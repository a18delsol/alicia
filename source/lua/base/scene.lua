--[[
-- Copyright (c) 2025 a18delsol
--
-- Redistribution and use in source and binary forms, with or without
-- modification, are permitted provided that the following conditions are met:
--
-- 1. Redistributions of source code must retain the above copyright notice,
-- this list of conditions and the following disclaimer.
--
-- 2. Redistributions in binary form must reproduce the above copyright notice,
-- this list of conditions and the following disclaimer in the documentation
-- and/or other materials provided with the distribution.
--
-- Subject to the terms and conditions of this license, each copyright holder
-- and contributor hereby grants to those receiving rights under this license
-- a perpetual, worldwide, non-exclusive, no-charge, royalty-free, irrevocable
-- (except for failure to satisfy the conditions of this license) patent license
-- to make, have made, use, offer to sell, sell, import, and otherwise transfer
-- this software, where such license applies only to those patent claims, already
-- acquired or hereafter acquired, licensable by such copyright holder or
-- contributor that are necessarily infringed by:
--
-- (a) their Contribution(s) (the licensed copyrights of copyright holders and
-- non-copyrightable additions of contributors, in source or binary form) alone;
-- or
--
-- (b) combination of their Contribution(s) with the work of authorship to which
-- such Contribution(s) was added by such copyright holder or contributor, if,
-- at the time the Contribution is added, such addition causes such combination
-- to be necessarily infringed. The patent license shall not apply to any other
-- combinations which include the Contribution.
--
-- Except as expressly stated above, no rights or licenses from any copyright
-- holder or contributor is granted under this license, whether expressly, by
-- implication, estoppel or otherwise.
--
-- DISCLAIMER
--
-- THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
-- AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
-- IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
-- DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDERS OR CONTRIBUTORS BE LIABLE
-- FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
-- DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
-- SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
-- CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
-- OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
-- OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
--]]



local function get_plane_frustum(a, b, c, d)
    return alicia.r3d.get_point_frustum(a)
        or alicia.r3d.get_point_frustum(b)
        or alicia.r3d.get_point_frustum(c)
        or alicia.r3d.get_point_frustum(d)
end

local function get_point_box_3(point, box)
    local x = point.x >= box.min.x and point.x <= box.max.x
    local y = point.y >= box.min.y and point.y <= box.max.y
    local z = point.z >= box.min.z and point.z <= box.max.z

    return x and y and z
end

local function plane_normal(a, b, c)
    return (b - a):cross(c - a):normalize()
end

local function plane_center(a, b, c, d)
    return vector_3:new(
        (a.x + b.x + c.x + d.x) / 4,
        (a.y + b.y + c.y + d.y) / 4,
        (a.z + b.z + c.z + d.z) / 4
    )
end

local function audio_update(audio_list, system_get)
    for i, audio in ipairs(audio_list) do
        local data = system_get(audio.path)

        if data:get_playing(audio.alias) then
            if audio.dynamic then
                local volume = audio.volume
                local pan = 0.5

                if audio.point then
                    if audio.distance_min and audio.distance_max then
                        local fall_off = ((self.camera_3d.point - audio.point):magnitude() - audio.distance_max) /
                            (audio.distance_min - audio.distance_max)
                        volume = math.clamp(0.0, 1.0, fall_off) * volume

                        alicia.draw_3d.draw_ball(audio.point, audio.distance_min, color:new(0.000, 0.000, 255.0, 127.0))
                        alicia.draw_3d.draw_ball(audio.point, audio.distance_max, color:new(255.0, 0.000, 0.000, 33.0))
                    end

                    pan = vector_3:direction_angle(
                        vector_3:new(self.camera_3d.point.x, 0.0, self.camera_3d.point.z),
                        vector_3:new(audio.point.x, 0.0, audio.point.z),
                        vector_3:new(self.camera_3d.focus.x, 0.0, self.camera_3d.focus.z) -
                        vector_3:new(self.camera_3d.point.x, 0.0, self.camera_3d.point.z),
                        self.camera_3d.angle
                    )
                    pan = math.abs(pan) / 3.14
                end

                data:set_pan(pan, audio.alias)
                data:set_volume(volume, audio.alias)
            end
        else
            table.remove(audio_list, i)
        end
    end
end

--[[----------------------------------------------------------------]]

---@class scene
scene = {
    __meta = {}
}

---Create a new scene.
---@param shader shader # The light shader.
---@return scene value # The scene.
function scene:new(shader)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[]]

    i.__type    = "scene"
    i.camera_3d = camera_3d:new(vector_3:new(0.0, 0.0, 0.0), vector_3:new(0.0, 0.0, 0.0), vector_3:new(0.0, 1.0, 0.0),
        90.0, CAMERA_3D_KIND.PERSPECTIVE)
    i.camera_2d = camera_2d:new(vector_2:new(0.0, 0.0), vector_2:new(0.0, 0.0), 0.0, 1.0)
    i.light     = {}
    i.sound     = {}
    i.music     = {}
    i.view_list = {}
    i.room_list = {}

    return i
end

--[[----------------------------------------------------------------]]

function scene:begin(call, system, ...)
    audio_update(self.sound, system.get_sound)
    audio_update(self.music, system.get_music)

    for _, light in ipairs(self.light) do
        light:set_state(1)
    end

    alicia.r3d.begin(function()
        for _, room in ipairs(self.room_list) do
            if get_point_box_3(self.camera_3d.point, room.size) then
                room:draw(self, system, true)
                break
            end
        end
    end, self.camera_3d, ...)

    for _, light in ipairs(self.light) do
        light:set_state(0)
    end
end

--[[----------------------------------------------------------------]]
-- sound management.
--[[----------------------------------------------------------------]]

function scene:play_sound(system, path, point, dynamic, volume, distance_min, distance_max)
    -- load the sound into memory.
    local sound = system:get_sound(path)
    local alias = nil

    if sound:get_playing() then
        local count = sound.alias_count

        if not (count == 0.0) then
            for x = 0, count - 1 do
                if not sound:get_playing(x) then
                    alias = x
                    break
                end
            end
        end
    end

    if not volume then volume = 1.0 end

    local sound_volume = volume
    local pan = 0.5

    if point then
        if distance_min and distance_max then
            local fall_off = ((self.camera_3d.point - point):magnitude() - distance_max) / (distance_min - distance_max)
            sound_volume = math.clamp(0.0, 1.0, fall_off) * volume
        end

        pan = vector_3:direction_angle(
            vector_3:new(self.camera_3d.point.x, 0.0, self.camera_3d.point.z),
            vector_3:new(point.x, 0.0, point.z),
            vector_3:new(self.camera_3d.focus.x, 0.0, self.camera_3d.focus.z) -
            vector_3:new(self.camera_3d.point.x, 0.0, self.camera_3d.point.z),
            self.camera_3d.angle
        )
        pan = math.abs(pan) / 3.14
    end

    sound:set_pan(pan, alias)
    sound:set_volume(sound_volume, alias)
    sound:play(alias)

    table.insert(self.sound, {
        path         = path,
        point        = point and vector_3:new(point.x, 0.0, point.z),
        dynamic      = dynamic,
        volume       = volume,
        distance_min = distance_min,
        distance_max = distance_max,
        alias        = alias
    })
end

function scene:stop_sound(system, path)
    -- load the sound into memory.
    local sound = system:get_sound(path)

    sound:stop()

    local count = sound.alias_count

    if not (count == 0.0) then
        for x = 0, count - 1 do
            sound:stop(x)
        end
    end
end

function scene:stop_sound_all(system)
    for i, sound in ipairs(self.sound) do
        local data = system:get_sound(sound.path)

        if data:get_playing(sound.alias) then
            data:stop(sound.alias)
        end
    end
end

function scene:set_sound_volume(system, path, volume)
    -- load the sound into memory.
    local sound = system:get_sound(path)

    sound:set_volume(volume)
end

--[[----------------------------------------------------------------]]
-- music management.
--[[----------------------------------------------------------------]]

function scene:play_music(system, path, point, dynamic, volume, distance_min, distance_max)
    -- load the music into memory.
    local music = system:get_music(path)

    if not volume then volume = 1.0 end

    local music_volume = volume
    local pan = 0.5

    if point then
        if distance_min and distance_max then
            local fall_off = ((self.camera_3d.point - point):magnitude() - distance_max) / (distance_min - distance_max)
            music_volume = math.clamp(0.0, 1.0, fall_off) * volume
        end

        pan = vector_3:direction_angle(
            vector_3:new(self.camera_3d.point.x, 0.0, self.camera_3d.point.z),
            vector_3:new(point.x, 0.0, point.z),
            vector_3:new(self.camera_3d.focus.x, 0.0, self.camera_3d.focus.z) -
            vector_3:new(self.camera_3d.point.x, 0.0, self.camera_3d.point.z),
            self.camera_3d.angle
        )
        pan = math.abs(pan) / 3.14
    end

    music:set_pan(pan)
    music:set_volume(music_volume)
    music:play()

    table.insert(self.music, {
        path         = path,
        point        = point and vector_3:new(point.x, 0.0, point.z),
        dynamic      = dynamic,
        volume       = volume,
        distance_min = distance_min,
        distance_max = distance_max,
    })
end

function scene:set_music_volume(system, path, volume)
    -- load the music into memory.
    local music = system:get_music(path)

    music:set_volume(volume)
end

function scene:set_music_volume_all(system, volume)
    for i, music in ipairs(self.music) do
        local data = system:get_music(music.path)

        data:set_volume(volume)
    end
end

function scene:stop_music(system, path)
    -- load the music into memory.
    local music = system:get_music(path)

    music:stop()
end

function scene:stop_music_all(system)
    for i, music in ipairs(self.music) do
        local data = system:get_music(music.path)

        if data:get_playing() then
            data:stop()
        end
    end
end

function scene:reset_music_all(system)
    for i, music in ipairs(self.music) do
        local data = system:get_music(music.path)

        data:stop()
        data:set_pitch(1.0)
        data:set_pan(0.5)
        data:set_volume(1.0)
    end
end

--[[----------------------------------------------------------------]]
-- room/view management.
--[[----------------------------------------------------------------]]

---@class room
room = {
    __meta = {}
}

function room:new(scene, system, path)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[]]

    i.__type    = "room"
    i.path      = path
    i.view_list = {}

    --[[]]

    local model = system:set_model(path)

    local x_a, y_a, z_a, x_b, y_b, z_b = model:get_box_3()
    i.size = box_3:new(
        vector_3:new(x_a, y_a, z_a),
        vector_3:new(x_b, y_b, z_b)
    )

    table.insert(scene.room_list, i)

    return i
end

function room:draw(scene, system, main, previous)
    local draw = false

    if main then
        draw = true
    else
        local point = scene.camera_3d.point

        for _, view in ipairs(self.view_list) do
            if get_plane_frustum(view.a, view.b, view.c, view.d) then
                --local m_a = (view.a - point):magnitude()
                --local m_b = (view.b - point):magnitude()
                --local m_c = (view.c - point):magnitude()
                --local m_d = (view.d - point):magnitude()
                --local _, m_a = level_editor.rapier:cast_ray(ray:new(point, view.a - point), m_a, false)
                --local _, m_b = level_editor.rapier:cast_ray(ray:new(point, view.b - point), m_b, false)
                --local _, m_c = level_editor.rapier:cast_ray(ray:new(point, view.c - point), m_c, false)
                --local _, m_d = level_editor.rapier:cast_ray(ray:new(point, view.d - point), m_d, false)

                --if (m_a >= 1.0 or m_b >= 1.0 or m_c >= 1.0 or m_d >= 1.0) then
                draw = true
                --end
            end
        end
    end

    if draw then
        system:get_model(self.path):draw(vector_3:zero(), 1.0, color:white(), true)

        for _, view in ipairs(self.view_list) do
            if view.link_a and not (view.link_a == self) then
                if not (previous and view.link_a == previous) then
                    view.link_a:draw(scene, system, false, self)
                end
            end

            if view.link_b and not (view.link_b == self) then
                if not (previous and view.link_b == previous) then
                    view.link_b:draw(scene, system, false, self)
                end
            end
        end
    end
end

--[[----------------------------------------------------------------]]

---@class view
---@field a      vector_3
---@field b      vector_3
---@field c      vector_3
---@field d      vector_3
---@field link_a room | nil
---@field link_b room | nil
view = {
    __meta = {}
}

function view:new(scene, a, b, c, d)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[]]

    i.__type = "view"
    i.a = a
    i.b = b
    i.c = c
    i.d = d

    table.insert(scene.view_list, i)

    local center = plane_center(i.a, i.b, i.c, i.d)
    local normal = plane_normal(i.a, i.b, i.c)

    local link_a = center + normal
    local link_b = center - normal

    for _, room in ipairs(scene.room_list) do
        if get_point_box_3(link_a, room.size) then
            i.link_a = room
            table.insert(room.view_list, i)
        end
        if get_point_box_3(link_b, room.size) then
            i.link_b = room
            table.insert(room.view_list, i)
        end
    end

    return i
end

function view:draw(state)
    alicia.draw_3d.draw_cube(self.a, vector_3:one() * 0.1, color:red())
    alicia.draw_3d.draw_cube(self.b, vector_3:one() * 0.1, color:red())
    alicia.draw_3d.draw_cube(self.c, vector_3:one() * 0.1, color:red())
    alicia.draw_3d.draw_cube(self.d, vector_3:one() * 0.1, color:red())

    local center = plane_center(self.a, self.b, self.c, self.d)
    local normal = plane_normal(self.a, self.b, self.c)

    alicia.draw_3d.draw_cube(center, vector_3:one() * 0.1, color:green())
    alicia.draw_3d.draw_cube(center + normal, vector_3:one() * 0.1, color:green())
    alicia.draw_3d.draw_cube(center - normal, vector_3:one() * 0.1, color:green())
end

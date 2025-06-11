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

    i.__type = "scene"
    i.camera_3d = camera_3d:new(vector_3:new(0.0, 0.0, 0.0), vector_3:new(0.0, 0.0, 0.0), vector_3:new(0.0, 1.0, 0.0),
        90.0, CAMERA_3D_KIND.PERSPECTIVE)
    i.camera_2d = camera_2d:new(vector_2:new(0.0, 0.0), vector_2:new(0.0, 0.0), 0.0, 1.0)
    --i.light = light_manager:new(shader)
    i.sound = {}
    i.music = {}

    return i
end

function scene:begin(call, system, camera_3d)
    for i, sound in ipairs(self.sound) do
        local data = system:get_sound(sound.path)

        if data:get_playing(sound.alias) then
            if sound.dynamic then
                local volume = sound.volume
                local pan = 0.5

                if sound.point then
                    if sound.distance_min and sound.distance_max then
                        local fall_off = ((self.camera_3d.point - sound.point):magnitude() - sound.distance_max) /
                            (sound.distance_min - sound.distance_max)
                        volume = math.clamp(0.0, 1.0, fall_off) * volume

                        alicia.draw_3d.draw_ball(sound.point, sound.distance_min, color:old(0.000, 0.000, 255.0, 127.0))
                        alicia.draw_3d.draw_ball(sound.point, sound.distance_max, color:old(255.0, 0.000, 0.000, 33.0))
                    end

                    pan = vector_3:direction_angle(
                        vector_3:old(self.camera_3d.point.x, 0.0, self.camera_3d.point.z),
                        vector_3:old(sound.point.x, 0.0, sound.point.z),
                        vector_3:old(self.camera_3d.focus.x, 0.0, self.camera_3d.focus.z) -
                        vector_3:old(self.camera_3d.point.x, 0.0, self.camera_3d.point.z),
                        self.camera_3d.angle
                    )
                    pan = math.abs(pan) / 3.14
                end

                data:set_pan(pan, sound.alias)
                data:set_volume(volume, sound.alias)
            end
        else
            table.remove(self.sound, i)
        end
    end

    for i, music in ipairs(self.music) do
        local data = system:get_music(music.path)

        if data:get_playing() then
            data:update()

            if music.dynamic then
                local volume = music.volume
                local pan = 0.5

                if music.point then
                    if music.distance_min and music.distance_max then
                        local fall_off = ((self.camera_3d.point - music.point):magnitude() - music.distance_max) /
                            (music.distance_min - music.distance_max)
                        volume = math.clamp(0.0, 1.0, fall_off) * volume

                        alicia.draw_3d.draw_ball(music.point, music.distance_min, color:old(0.000, 0.000, 255.0, 127.0))
                        alicia.draw_3d.draw_ball(music.point, music.distance_max, color:old(255.0, 0.000, 0.000, 33.0))
                    end

                    pan = vector_3:direction_angle(
                        vector_3:old(self.camera_3d.point.x, 0.0, self.camera_3d.point.z),
                        vector_3:old(music.point.x, 0.0, music.point.z),
                        vector_3:old(self.camera_3d.focus.x, 0.0, self.camera_3d.focus.z) -
                        vector_3:old(self.camera_3d.point.x, 0.0, self.camera_3d.point.z),
                        self.camera_3d.angle
                    )
                    pan = math.abs(pan) / 3.14
                end

                data:set_pan(pan)
                data:set_volume(volume)
            end
        else
            table.remove(self.music, i)
        end
    end
end

function scene:set_model(system, path)
    -- load the model into memory.
    local model = system:get_model(path)

    if not model then
        model = system:set_model(path)

        -- bind the model with the scene's light shader.
        for x = 0.0, model.material_count - 1.0 do
            model:bind_shader(x, self.light.shader)
        end
    end

    return model
end

function vector_3:direction_angle(point_a, point_b, focus, up)
    local side = focus:cross(up):normalize()
    local x = (point_b - point_a):dot(side)
    local z = (point_b - point_a):dot(focus)

    return math.atan2(z, x)
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
            vector_3:old(self.camera_3d.point.x, 0.0, self.camera_3d.point.z),
            vector_3:old(point.x, 0.0, point.z),
            vector_3:old(self.camera_3d.focus.x, 0.0, self.camera_3d.focus.z) -
            vector_3:old(self.camera_3d.point.x, 0.0, self.camera_3d.point.z),
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

function scene:reset_music_all(system)
    for i, music in ipairs(self.music) do
        local data = system:get_music(music.path)

        data:stop()
        data:set_pitch(1.0)
        data:set_pan(0.5)
        data:set_volume(1.0)
    end
end

function scene:set_sound_volume(system, path, volume)
    -- load the sound into memory.
    local sound = system:get_sound(path)

    sound:set_volume(volume)
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
            vector_3:old(self.camera_3d.point.x, 0.0, self.camera_3d.point.z),
            vector_3:old(point.x, 0.0, point.z),
            vector_3:old(self.camera_3d.focus.x, 0.0, self.camera_3d.focus.z) -
            vector_3:old(self.camera_3d.point.x, 0.0, self.camera_3d.point.z),
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

local LIGHT_MAXIMUM = 32.0

---@class light_manager
light_manager = {
    __meta = {}
}

---Create a new light_manager.
---@param shader shader # The light_manager shader.
---@return light_manager value # The light_manager.
function light_manager:new(shader)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[]]

    i.__type = "light_manager"
    i.shader = shader
    i.point_list = {}
    i.focus_list = {}

    for x = 0, LIGHT_MAXIMUM - 1.0 do
        table.insert(i.point_list, light_instance:new(i, x, true))
    end

    for x = 0, LIGHT_MAXIMUM - 1.0 do
        table.insert(i.focus_list, light_instance:new(i, x, false))
    end

    i.point_amount = 0.0
    i.focus_amount = 0.0

    local location = i.shader:get_location_name("base_color")
    i.shader:set_shader_vector_4(location, vector_4:old(1.0, 1.0, 1.0, 1.0))

    local location = i.shader:get_location_name("view_point")
    i.shader:set_location(11, location)

    i.point_amount_location = i.shader:get_location_name("light_point_count")
    i.focus_amount_location = i.shader:get_location_name("light_focus_count")

    return i
end

function light_manager:set_base_color(color)
    local location = self.shader:get_location_name("base_color")
    self.shader:set_shader_vector_4(location, vector_4:old(
        color.r / 255.0,
        color.g / 255.0,
        color.b / 255.0,
        color.a / 255.0
    ))
end

function light_manager:begin(call, camera_3d)
    local location = self.shader:get_location(11)
    self.shader:set_shader_vector_3(location, camera_3d.point)
    self.shader:set_shader_integer(self.point_amount_location, self.point_amount)
    self.shader:set_shader_integer(self.focus_amount_location, self.focus_amount)

    if call then
        self.shader:begin(call, camera_3d)
    end

    self.point_amount = 0.0
    self.focus_amount = 0.0
end

function light_manager:light_point(point, color, range)
    if self.point_amount < LIGHT_MAXIMUM then
        self.point_amount = self.point_amount + 1.0

        self.point_list[self.point_amount]:set_point(self, point)
        self.point_list[self.point_amount]:set_color(self, color)
        self.point_list[self.point_amount]:set_range(self, range)
    end
end

function light_manager:light_focus(point, focus, color)
    if self.focus_amount < LIGHT_MAXIMUM then
        self.focus_amount = self.focus_amount + 1.0

        self.focus_list[self.focus_amount]:set_point(self, point)
        self.focus_list[self.focus_amount]:set_focus(self, focus)
        self.focus_list[self.focus_amount]:set_color(self, color)
    end
end

---@class light_instance
light_instance = {
    __meta = {}
}

---Create a new light instance.
---@return light_instance value # The light_instance.
function light_instance:new(light, index, point)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[]]

    i.__type = "light_instance"

    local uniform = point and "light_point" or "light_focus"

    i.point_location = light.shader:get_location_name(uniform .. "[" .. index .. "].point")

    if not point then
        i.focus_location = light.shader:get_location_name(uniform .. "[" .. index .. "].focus")
    end

    i.color_location = light.shader:get_location_name(uniform .. "[" .. index .. "].color")
    i.range_location = light.shader:get_location_name(uniform .. "[" .. index .. "].range")

    return i
end

function light_instance:set_point(light, point)
    light.shader:set_shader_vector_3(self.point_location, point)
end

function light_instance:set_focus(light, focus)
    light.shader:set_shader_vector_3(self.focus_location, focus)
end

function light_instance:set_color(light, color)
    light.shader:set_shader_vector_4(self.color_location, vector_4:old(
        color.r / 255.0,
        color.g / 255.0,
        color.b / 255.0,
        color.a / 255.0
    ))
end

function light_instance:set_range(light, range)
    light.shader:set_shader_decimal(self.range_location, range)
end

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

---@class vector_2
---@field x number
---@field y number
vector_2 = {
    __meta = {
        __add = function(a, b) return vector_2:new(a.x + b.x, a.y + b.y) end,
        __sub = function(a, b) return vector_2:new(a.x - b.x, a.y - b.y) end,
        __mul = function(a, b)
            if type(a) == "number" then
                return vector_2:new(a * b.x, a * b.y)
            elseif type(b) == "number" then
                return vector_2:new(a.x * b, a.y * b)
            else
                return vector_2:new(a.x * b.x, a.y * b.y)
            end
        end,
        __div = function(a, b) return vector_2:new(a.x / b.x, a.y / b.y) end,
        __tostring = function(a)
            return string.format("{ x : %.2f, y: %.2f }", a.x, a.y)
        end
    }
}

---Create a new vector (2 dimensional).
---@param x number # "X" component.
---@param y number # "Y" component.
---@return vector_2 value # The vector.
function vector_2:new(x, y)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[]]

    i.__type = "vector_2"
    i.x = x
    i.y = y

    return i
end

---Get the "X" vector.
---@return vector_2 value # The vector.
function vector_2:x()
    return vector_2:new(1.0, 0.0)
end

---Get the "Y" vector.
---@return vector_2 value # The vector.
function vector_2:y()
    return vector_2:new(0.0, 1.0)
end

---Get a vector, with every component set to "1".
---@return vector_2 value # The vector.
function vector_2:one()
    return vector_2:new(1.0, 1.0)
end

---Get a vector, with every component set to "0".
---@return vector_2 value # The vector.
function vector_2:zero()
    return vector_2:new(0.0, 0.0)
end

---Get the magnitude of the current vector.
---@return number value # The magnitude.
function vector_2:magnitude()
    return math.sqrt(self.x * self.x + self.y * self.y)
end

---Get the unit vector of the current vector.
---@return vector_2 value # The unit vector.
function vector_2:normalize()
    local length = math.sqrt(self.x * self.x + self.y * self.y)

    if not (length == 0.0) then
        local length = 1.0 / length
        return vector_2:new(self.x * length, self.y * length)
    else
        return self
    end
end

---Get the angle between the current vector, and a given one.
---@param value vector_2 # The vector to calculate the angle to.
---@return number value # The magnitude.
function vector_2:angle(value)
    local result = 0.0;

    local dot = self.x * value.x + self.y * value.y;
    local det = self.x * value.y - self.y * value.x;

    result = math.atan2(det, dot);

    return result;
end

---Scale a vector by the current 2D camera's zoom scale.
---@param camera camera_2d # The current 2D camera.
---@return vector_2 value # The vector.
function vector_2:scale_zoom(camera)
    return self * (1.0 / camera.zoom)
end

--[[----------------------------------------------------------------]]

---@class vector_3
---@field x number
---@field y number
---@field z number
vector_3 = {
    __meta = {
        __add = function(a, b) return vector_3:new(a.x + b.x, a.y + b.y, a.z + b.z) end,
        __sub = function(a, b) return vector_3:new(a.x - b.x, a.y - b.y, a.z - b.z) end,
        __mul = function(a, b)
            if type(a) == "number" then
                return vector_3:new(a * b.x, a * b.y, a * b.z)
            elseif type(b) == "number" then
                return vector_3:new(a.x * b, a.y * b, a.z * b)
            else
                return vector_3:new(a.x * b.x, a.y * b.y, a.z * b.z)
            end
        end,
        __div = function(a, b) return vector_3:new(a.x / b.x, a.y / b.y, a.z / b.z) end,
        __tostring = function(a)
            return string.format("{ x : %.2f, y: %.2f, z: %.2f }", a.x, a.y, a.z)
        end
    }
}

---Create a new vector (3 dimensional).
---@param x number # "X" component.
---@param y number # "Y" component.
---@param z number # "Z" component.
---@return vector_3 value # The vector.
function vector_3:new(x, y, z)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[]]

    i.__type = "vector_3"
    i.x = x
    i.y = y
    i.z = z

    return i
end

---Get the "X" vector.
---@return vector_3 value # The vector.
function vector_3:x()
    return vector_3:new(1.0, 0.0, 0.0)
end

---Get the "Y" vector.
---@return vector_3 value # The vector.
function vector_3:y()
    return vector_3:new(0.0, 1.0, 0.0)
end

---Get the "Z" vector.
---@return vector_3 value # The vector.
function vector_3:z()
    return vector_3:new(0.0, 0.0, 1.0)
end

---Get a vector, with every component set to "1".
---@return vector_3 value # The vector.
function vector_3:one()
    return vector_3:new(1.0, 1.0, 1.0)
end

---Get a vector, with every component set to "0".
---@return vector_3 value # The vector.
function vector_3:zero()
    return vector_3:new(0.0, 0.0, 0.0)
end

---Get the dot product between the current vector, and another one.
---@param value vector_3 # Vector to perform the dot product with.
---@return number value # The dot product.
function vector_3:dot(value)
    return (self.x * value.x + self.y * value.y + self.z * value.z)
end

---Get the cross product between the current vector, and another one.
---@param value vector_3 # Vector to perform the cross product with.
---@return vector_3 value # The cross product.
function vector_3:cross(value)
    return vector_3:new(self.y * value.z - self.z * value.y, self.z * value.x - self.x * value.z,
        self.x * value.y - self.y * value.x)
end

---Get the magnitude of the current vector.
---@return number value # The magnitude.
function vector_3:magnitude()
    return math.sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
end

---Get the unit vector of the current vector.
---@return vector_3 value # The unit vector.
function vector_3:normalize()
    local length = math.sqrt(self.x * self.x + self.y * self.y + self.z * self.z)

    if not (length == 0.0) then
        local length = 1.0 / length
        return vector_3:new(self.x * length, self.y * length, self.z * length)
    else
        return self
    end
end

---Snap the current to a given step.
---@param step number # Step.
---@return vector_3 value # The vector.
function vector_3:snap(step)
    return vector_3:new(
        math.snap(step, self.x),
        math.snap(step, self.y),
        math.snap(step, self.z)
    )
end

---Interpolate the current vector.
---@param value vector_3 # The vector to interpolate to.
---@return vector_3 value # The vector.
function vector_3:interpolate(value, time)
    return vector_3:new(
        math.interpolate(self.x, value.x, time),
        math.interpolate(self.y, value.y, time),
        math.interpolate(self.z, value.z, time)
    )
end

---Rotate the current vector by an axis and an angle.
---@param axis  vector_3 # The axis.
---@param angle number # The angle.
---@return vector_3 value # The vector.
function vector_3:rotate_axis_angle(axis, angle)
    -- TO-DO clean up

    local axis = axis:normalize()

    angle      = angle / 2.0
    local a    = math.sin(angle)
    local b    = axis.x * a
    local c    = axis.y * a
    local d    = axis.z * a
    a          = math.cos(angle)
    local w    = vector_3:new(b, c, d)

    local wv   = w:cross(self)

    local wwv  = w:cross(wv)

    wv         = wv * a * 2.0

    wwv        = wwv * 2.0

    return vector_3:new(self.x + wv.x + wwv.x, self.y + wv.y + wwv.y, self.z + wv.z + wwv.z)
end

function vector_3:rotate_vector_4(q)
    -- TO-DO clean up

    local result = vector_3:new(0.0, 0.0, 0.0);

    result.x = self.x * (q.x * q.x + q.w * q.w - q.y * q.y - q.z * q.z) + self.y * (2 * q.x * q.y - 2 * q.w * q.z) +
        self.z * (2 * q.x * q.z + 2 * q.w * q.y);
    result.y = self.x * (2 * q.w * q.z + 2 * q.x * q.y) + self.y * (q.w * q.w - q.x * q.x + q.y * q.y - q.z * q.z) +
        self.z * (-2 * q.w * q.x + 2 * q.y * q.z);
    result.z = self.x * (-2 * q.w * q.y + 2 * q.x * q.z) + self.y * (2 * q.w * q.x + 2 * q.y * q.z) +
        self.z * (q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z);

    return result;
end

---Get the angle between the current vector, and a given one.
---@param value vector_3 # The vector to calculate the angle to.
---@return number value # The magnitude.
function vector_3:angle(value)
    local result = 0.0;

    local cross = vector_3:new(
        self.y * value.z - self.z * value.y,
        self.z * value.x - self.x * value.z,
        self.x * value.y - self.y * value.x
    );
    local len = math.sqrt(cross.x * cross.x + cross.y * cross.y + cross.z * cross.z);
    local dot = (self.x * value.x + self.y * value.y + self.z * value.z);
    result = math.atan2(len, dot);

    return result;
end

--[[----------------------------------------------------------------]]

---@class vector_4
---@field x number
---@field y number
---@field z number
---@field w number
vector_4 = {
    __meta = {
        __add = function(a, b) return vector_4:new(a.x + b.x, a.y + b.y, a.z + b.z, a.w + b.w) end,
        __sub = function(a, b) return vector_4:new(a.x - b.x, a.y - b.y, a.z - b.z, a.w - b.w) end,
        __mul = function(a, b)
            if type(a) == "number" then
                return vector_4:new(a * b.x, a * b.y, a * b.z, a * b.w)
            elseif type(b) == "number" then
                return vector_4:new(a.x * b, a.y * b, a.z * b, a.w * b)
            else
                return vector_4:new(a.x * b.x, a.y * b.y, a.z * b.z, a.w * b.w)
            end
        end,
        __div = function(a, b) return vector_4:new(a.x / b.x, a.y / b.y, a.z / b.z, a.w / b.w) end,
        __tostring = function(a)
            return "{ x:" ..
                tostring(a.x) .. " y:" .. tostring(a.y) .. " z:" .. tostring(a.z) .. " w:" .. tostring(a.w) .. " }"
        end
    }
}

---Create a new vector (4 dimensional).
---@param x number # "X" component.
---@param y number # "Y" component.
---@param z number # "Z" component.
---@param w number # "W" component.
---@return vector_4 value # The vector.
function vector_4:new(x, y, z, w)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[]]

    i.__type = "vector_4"
    i.x = x
    i.y = y
    i.z = z
    i.w = w

    return i
end

---Get the "X" vector.
---@return vector_4 value # The vector.
function vector_4:x()
    return vector_4:new(1.0, 0.0, 0.0, 0.0)
end

---Get the "Y" vector.
---@return vector_4 value # The vector.
function vector_4:y()
    return vector_4:new(0.0, 1.0, 0.0, 0.0)
end

---Get the "Z" vector.
---@return vector_4 value # The vector.
function vector_4:z()
    return vector_4:new(0.0, 0.0, 1.0, 0.0)
end

---Get the "W" vector.
---@return vector_4 value # The vector.
function vector_4:w()
    return vector_4:new(0.0, 0.0, 0.0, 1.0)
end

---Get a vector, with every component set to "1".
---@return vector_4 value # The vector.
function vector_4:one()
    return vector_4:new(1.0, 1.0, 1.0, 1.0)
end

---Get a vector, with every component set to "0".
---@return vector_4 value # The vector.
function vector_4:zero()
    return vector_4:new(0.0, 0.0, 0.0, 0.0)
end

---Get the unit vector of the current vector.
---@return vector_4 value # The unit vector.
function vector_4:normalize()
    local length = math.sqrt(self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w)

    if not (length == 0.0) then
        local length = 1.0 / length
        return vector_4:new(self.x * length, self.y * length, self.z * length, self.w * length)
    else
        return self
    end
end

function vector_4:from_euler(pitch, yaw, roll)
    local result = vector_4:new(0.0, 0.0, 0.0, 0.0);

    local x0 = math.cos(pitch * 0.5);
    local x1 = math.sin(pitch * 0.5);
    local y0 = math.cos(yaw * 0.5);
    local y1 = math.sin(yaw * 0.5);
    local z0 = math.cos(roll * 0.5);
    local z1 = math.sin(roll * 0.5);

    result.x = x1 * y0 * z0 - x0 * y1 * z1;
    result.y = x0 * y1 * z0 + x1 * y0 * z1;
    result.z = x0 * y0 * z1 - x1 * y1 * z0;
    result.w = x0 * y0 * z0 + x1 * y1 * z1;

    return result;
end

--[[----------------------------------------------------------------]]

---@class box_2
---@field point vector_2
---@field shape vector_2
box_2 = {
    __meta = {}
}

function box_2:new(point, shape)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[]]

    i.__type = "box_2"
    i.point  = point
    i.shape  = shape

    return i
end

--[[----------------------------------------------------------------]]

---@class box_3
---@field min vector_3
---@field max vector_3
box_3 = {
    __meta = {}
}

function box_3:new(min, max)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[]]

    i.__type = "box_3"
    i.min = min
    i.max = max

    return i
end

--[[----------------------------------------------------------------]]

---@class ray
---@field point vector_3
---@field focus vector_3
ray = {
    __meta = {}
}

function ray:new(point, focus)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[]]

    i.__type = "ray"
    i.point  = point
    i.focus  = focus

    return i
end

--[[----------------------------------------------------------------]]

---@class color
---@field r number
---@field g number
---@field b number
---@field a number
color = {
    __meta = {
        __mul = function(a, b)
            if type(a) == "number" then
                return color:new(
                    math.clamp(0.0, 255.0, math.floor(a * b.r)),
                    math.clamp(0.0, 255.0, math.floor(a * b.g)),
                    math.clamp(0.0, 255.0, math.floor(a * b.b)),
                    b.a
                )
            elseif type(b) == "number" then
                return color:new(
                    math.clamp(0.0, 255.0, math.min(255.0, math.floor(a.r * b))),
                    math.clamp(0.0, 255.0, math.min(255.0, math.floor(a.g * b))),
                    math.clamp(0.0, 255.0, math.min(255.0, math.floor(a.b * b))),
                    a.a
                )
            else
                return color:new(
                    math.clamp(0.0, 255.0, math.floor(a.r * b.r)),
                    math.clamp(0.0, 255.0, math.floor(a.g * b.g)),
                    math.clamp(0.0, 255.0, math.floor(a.b * b.b)),
                    math.clamp(0.0, 255.0, math.floor(a.a * b.a))
                )
            end
        end,
        __tostring = function(a)
            return string.format("{ r : %.2f, g: %.2f, b: %.2f, a: %.2f }", a.r, a.g, a.b, a.a)
        end
    }
}

function color:new(r, g, b, a)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[]]

    i.__type = "color"
    i.r = r
    i.g = g
    i.b = b
    i.a = a

    return i
end

function color:white()
    return color:new(255.0, 255.0, 255.0, 255.0)
end

function color:black()
    return color:new(0.0, 0.0, 0.0, 255.0)
end

function color:grey()
    return color:new(127.0, 127.0, 127.0, 255.0)
end

function color:red()
    return color:new(255.0, 0.0, 0.0, 255.0)
end

function color:green()
    return color:new(0.0, 255.0, 0.0, 255.0)
end

function color:blue()
    return color:new(0.0, 0.0, 255.0, 255.0)
end

--[[----------------------------------------------------------------]]

---@class camera_2d
---@field shift vector_2
---@field focus vector_2
---@field angle number
---@field zoom  number
camera_2d = {
    __meta = {}
}

function camera_2d:new(shift, focus, angle, zoom)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[]]

    i.__type = "camera_2d"
    i.shift = shift
    i.focus = focus
    i.angle = angle
    i.zoom = zoom

    return i
end

--[[----------------------------------------------------------------]]

---@enum camera_3d_kind
CAMERA_3D_KIND = {
    PERSPECTIVE = 0,
    ORTHOGRAPHIC = 1,
}

---@class camera_3d
---@field point vector_3
---@field focus vector_3
---@field angle vector_3
---@field zoom  number
---@field kind  camera_3d_kind
camera_3d = {
    __meta = {}
}

function camera_3d:new(point, focus, angle, zoom, kind)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[]]

    i.__type = "camera_3d"
    i.point = point
    i.focus = focus
    i.angle = angle
    i.zoom = zoom
    i.kind = kind

    return i
end

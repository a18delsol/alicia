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

local LOGGER_LINE_COLOR_HISTORY = color:new(127.0, 127.0, 127.0, 255.0)
local LOGGER_LINE_COLOR_MESSAGE = color:new(255.0, 255.0, 255.0, 255.0)
local LOGGER_LINE_COLOR_FAILURE = color:new(255.0, 0.0, 0.0, 255.0)
local LOGGER_LINE_COUNT         = 4.0
local LOGGER_LINE_DELAY         = 4.0
local LOGGER_LINE_LABEL_TIME    = true
local LOGGER_ACTION_ABOVE       = action:new(
    {
        action_button:new(INPUT_DEVICE.BOARD, INPUT_BOARD.UP),
    }
)
local LOGGER_ACTION_BELOW       = action:new(
    {
        action_button:new(INPUT_DEVICE.BOARD, INPUT_BOARD.DOWN),
    }
)

---@class logger_line
logger_line                     = {
    __meta = {}
}

function logger_line:new(label, color)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[]]

    i.__type = "logger_line"
    i.label = label
    i.color = color
    i.time = alicia.general.get_time()

    return i
end

--[[----------------------------------------------------------------]]

local LOGGER_FONT_SCALE = 24.0
local LOGGER_FONT_SPACE = 2.0
local LOGGER_LINE_CAP   = 4.0

---@class logger
---@field current  table
---@field active  boolean
---@field window  window
logger                  = {
    __meta = {}
}

---Create a new logger.
---@return logger value # The logger.
function logger:new()
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[]]

    i.__type        = "logger"
    i.font          = alicia.font.new_default(LOGGER_FONT_SCALE)
    i.work          = ""
    i.current       = {}
    i.history       = {}
    i.active        = false
    i.window        = window:new()

    local lua_print = print

    -- over-ride default print function with our own.
    print           = function(...)
        lua_print(...)
        i:print(..., color:new(255.0, 255.0, 255.0, 255.0))
    end

    return i
end

---Draw the logger.
function logger:draw()
    local shape = vector_2:old(alicia.window.get_shape())

    if alicia.input.board.get_press(INPUT_BOARD.F2) then
        self.active = not self.active
    end

    if self.active then
        local shift = nil
        local click = false

        if LOGGER_ACTION_ABOVE:press_repeat() then shift = 1 * -1 end
        if LOGGER_ACTION_BELOW:press_repeat() then shift = 1 end

        if shift then
            if #self.history > 0 then
                local match = false

                for i, entry in ipairs(self.history) do
                    if entry == self.work then
                        if shift == -1 then
                            if self.history[i - 1] then
                                self.work = self.history[i - 1]
                            else
                                self.work = self.history[#self.history]
                            end
                        else
                            if self.history[i + 1] then
                                self.work = self.history[i + 1]
                            else
                                self.work = self.history[1]
                            end
                        end

                        match = true
                        break
                    end
                end

                if not match then
                    self.work = self.history[#self.history]
                end
            end
        end

        alicia.draw_2d.draw_box_2(box_2:old(0.0, 0.0, shape.x, shape.y), vector_2:zero(), 0.0,
            color:old(0.0, 0.0, 0.0, 127.0))

        self.window:draw(function()
            self.work, click = self.window:entry(
                box_2:old(8.0, shape.y - (LOGGER_FONT_SCALE + 8.0), shape.x - 16.0, LOGGER_FONT_SCALE), "",
                self.work)
        end)

        if click then
            if not (self.work == "") then
                self:print(self.work, color:new(160.0, 160.0, 160.0, 255.0))

                local call, error = loadstring(self.work)

                if call then
                    local success, message = pcall(call)

                    if not success then
                        self:print(message, color:new(255.0, 0.0, 0.0, 255.0))
                    end
                else
                    self:print(error, color:new(255.0, 0.0, 0.0, 255.0))
                end

                table.insert(self.history, self.work)
                self.work = ""
            end
        end
    end

    -- get the length of the current worker.
    local count        = #self.current
    local text_point_a = vector_2:old(0.0, 0.0)
    local text_point_b = vector_2:old(0.0, 0.0)
    local offset       = 0.0

    -- draw the latest logger current, iterating through the current in reverse.
    for i = 1, (self.active and count or LOGGER_LINE_CAP) do
        local line = self.current[count + 1 - i]

        -- line isn't nil...
        if line then
            -- line is within the time threshold...
            if alicia.general.get_time() < line.time + LOGGER_LINE_DELAY or self.active then
                local label = line.label

                local result = self.font:measure_text_box(label,
                    box_2:new(0.0, 0.0, shape.x - 24.0, shape.y), LOGGER_FONT_SCALE,
                    LOGGER_FONT_SPACE,
                    true)

                if self.active then offset = offset + result end

                text_point_a:set(13.0,
                    (self.active and shape.y - (LOGGER_FONT_SCALE * 2.0 + 1.0) or 13.0) +
                    (offset) * (self.active and -1.0 or 1.0))
                text_point_b:set(12.0,
                    (self.active and shape.y - (LOGGER_FONT_SCALE * 2.0 + 0.0) or 12.0) +
                    (offset) * (self.active and -1.0 or 1.0))

                if not self.active then offset = offset + result end

                -- line with time-stamp is set, add time-stamp to beginning.
                if LOGGER_LINE_LABEL_TIME then
                    label = string.format("(%.2f) %s", line.time, line.label)
                end

                self.font:draw_box(label,
                    box_2:new(text_point_a.x, text_point_a.y, shape.x - text_point_a.x, shape.y), LOGGER_FONT_SCALE,
                    LOGGER_FONT_SPACE,
                    true, line.color * 0.5)

                self.font:draw_box(label,
                    box_2:new(text_point_b.x, text_point_b.y, shape.x - text_point_b.x, shape.y), LOGGER_FONT_SCALE,
                    LOGGER_FONT_SPACE,
                    true, line.color)

                --alicia.draw_2d.draw_box_2(box_2:old(text_point_a.x, text_point_a.y, shape.x - 24.0, result),
                --    vector_2:zero(), 0.0,
                --    color:old(0.0, 255.0, 0.0, 33.0))
            end
        end
    end
end

---Print a new line to the logger.
---@param line_label  string # Line label.
---@param line_color? color  # OPTIONAL: Line color.
function logger:print(line_label, line_color)
    -- if line color is nil, use default color.
    line_color = line_color and line_color or LOGGER_LINE_COLOR_MESSAGE

    -- insert a new logger line.
    table.insert(self.current, logger_line:new(tostring(line_label), line_color))

    -- if logger line count is over the cap...
    if #self.current > 39 then
        -- pop one logger line.
        table.remove(self.current, 1)
    end
end

function logger:clear()
    self.current = {}
end

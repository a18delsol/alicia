--[[
-- Copyright (c) 2025 luxreduxdelux
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

require("bit")

---@enum gizmo_flag
GIZMO_FLAG = {
    IGNORE_BOARD   = 0x00000001,
    IGNORE_MOUSE   = 0x00000010,
    CLICK_ON_PRESS = 0x00000100,
}

--[[----------------------------------------------------------------]]

local WINDOW_FONT_SCALE         = 24.0
local WINDOW_FONT_SPACE         = 2.0
local WINDOW_COLOR_PRIMARY_MAIN = color:new(156, 39, 176, 255)
local WINDOW_COLOR_PRIMARY_SIDE = color:new(123, 31, 162, 255)
local WINDOW_COLOR_TEXT_WHITE   = color:new(255, 255, 255, 255)
local WINDOW_COLOR_TEXT_BLACK   = color:new(33, 33, 33, 255)
local WINDOW_CARD_ROUND_SHAPE   = 0.25
local WINDOW_CARD_ROUND_COUNT   = 4
local WINDOW_ACTION_ABOVE       = action:new(
    {
        action_button:new(INPUT_DEVICE.BOARD, INPUT_BOARD.UP),
        action_button:new(INPUT_DEVICE.PAD, INPUT_PAD.LEFT_FACE_UP),
    }
)
local WINDOW_ACTION_BELOW       = action:new(
    {
        action_button:new(INPUT_DEVICE.BOARD, INPUT_BOARD.DOWN),
        action_button:new(INPUT_DEVICE.PAD, INPUT_PAD.LEFT_FACE_DOWN),
    }
)
local WINDOW_ACTION_FOCUS       = action:new(
    {
        action_button:new(INPUT_DEVICE.BOARD, INPUT_BOARD.SPACE),
        action_button:new(INPUT_DEVICE.BOARD, INPUT_BOARD.RETURN),
        action_button:new(INPUT_DEVICE.MOUSE, INPUT_MOUSE.LEFT),
        action_button:new(INPUT_DEVICE.PAD, INPUT_PAD.RIGHT_FACE_DOWN),
    }
)
local WINDOW_ACTION_ALTERNATE   = action:new(
    {
        action_button:new(INPUT_DEVICE.BOARD, INPUT_BOARD.SPACE),
        action_button:new(INPUT_DEVICE.BOARD, INPUT_BOARD.TAB),
        action_button:new(INPUT_DEVICE.MOUSE, INPUT_MOUSE.LEFT),
        action_button:new(INPUT_DEVICE.MOUSE, INPUT_MOUSE.RIGHT),
        action_button:new(INPUT_DEVICE.PAD, INPUT_PAD.RIGHT_FACE_DOWN),
        action_button:new(INPUT_DEVICE.PAD, INPUT_PAD.RIGHT_FACE_UP),
    }
)
local WINDOW_ACTION_LATERAL     = action:new(
    {
        action_button:new(INPUT_DEVICE.BOARD, INPUT_BOARD.LEFT),
        action_button:new(INPUT_DEVICE.BOARD, INPUT_BOARD.RIGHT),
        action_button:new(INPUT_DEVICE.MOUSE, INPUT_MOUSE.LEFT),
        action_button:new(INPUT_DEVICE.MOUSE, INPUT_MOUSE.RIGHT),
        action_button:new(INPUT_DEVICE.PAD, INPUT_PAD.LEFT_FACE_LEFT),
        action_button:new(INPUT_DEVICE.PAD, INPUT_PAD.LEFT_FACE_RIGHT),
    }
)
local WINDOW_ACTION_ENTRY       = action:new(
    {
        action_button:new(INPUT_DEVICE.BOARD, INPUT_BOARD.LEFT),
        action_button:new(INPUT_DEVICE.BOARD, INPUT_BOARD.RIGHT),
    }
)
local WINDOW_SHIFT              = vector_2:new(6.0, 2.0)

local function window_font(self, scale)
    if self.font[scale] then
        return self.font[scale]
    end

    -- TO-DO make an effort to find the closest match instead of loading a new font from scratch.
    self.font[scale] = alicia.font.new_default(scale)

    return self.font[scale]
end

local function window_gizmo(self, label, hover, index, focus, click)
    local delta = alicia.general.get_frame_time()

    label = label .. self.count

    if not self.data[label] then
        self.data[label] = gizmo:new()
    end

    local data = self.data[label]

    data.hover = math.clamp(0.0, 1.0,
        data.hover + ((hover or index or focus) and delta * 8.0 or delta * -8.0))

    data.focus = math.clamp(0.0, 1.0,
        data.focus + (focus and delta * 8.0 or delta * -8.0))

    return data
end

---Draw a border.
---@param shape  box_2    # The shape of the border.
---@param hover  boolean  # Mouse focus. Whether or not the mouse cursor is over this gizmo.
---@param index  boolean  # Board focus. Whether or not the board cursor is over this gizmo.
---@param focus  boolean  # Gizmo focus. Whether or not the window focus is on this gizmo.
---@param label? string   # OPTIONAL: Text to draw.
---@param move?  vector_2 # OPTIONAL: Text off-set.
local function window_border(self, shape, hover, index, focus, label, move)
    local gizmo = window_gizmo(self, label, hover, index, focus, click)
    local shape = gizmo:move(self, shape)
    local color = gizmo:fade(self, WINDOW_COLOR_PRIMARY_MAIN)
    local shift = vector_2:new(shape.point.x + WINDOW_SHIFT.x, shape.point.y + WINDOW_SHIFT.y)

    -- if move isn't nil...
    if move then
        -- apply text off-set.
        shift = shift + move
    end

    -- draw border.
    alicia.draw_2d.draw_box_2_round(shape, WINDOW_CARD_ROUND_SHAPE, WINDOW_CARD_ROUND_COUNT, color)

    -- if label isn't nil...
    if label then
        local color_a = gizmo:fade(self, color:new(127.0, 127.0, 127.0, 255.0))
        local color_b = gizmo:fade(self, color:new(255.0, 255.0, 255.0, 255.0))

        -- draw text, draw with back-drop.
        window_font(self, shape.shape.y):draw(label, shift + vector_2:new(1.0, 1.0), vector_2:zero(), 0.0, shape.shape.y,
            WINDOW_FONT_SPACE,
            color_a)
        window_font(self, shape.shape.y):draw(label, shift, vector_2:zero(), 0.0, shape.shape.y, WINDOW_FONT_SPACE,
            color_b)
    end

    return gizmo
end

---Get the state of a gizmo.
---@param self   window     # The window.
---@param shape  box_2      # The shape of the gizmo.
---@param flag?  gizmo_flag # OPTIONAL: The flag of the gizmo.
---@param input? action     # OPTIONAL: The input of the gizmo. Will override the default focus action for the gizmo.
local function window_state(self, shape, flag, input)
    local check = true

    -- if there is a view-port shape set...
    if self.shape then
        -- check if the gizmo is within it.
        check = alicia.collision.get_box_2_box_2(shape, self.shape) and
            alicia.collision.get_point_box_2(self.mouse, self.shape)
    end

    -- mouse interaction check.
    local hover = self.device == INPUT_DEVICE.MOUSE and alicia.collision.get_point_box_2(self.mouse, shape) and check
    -- board interaction check.
    local index = self.device ~= INPUT_DEVICE.MOUSE and self.index == self.count
    -- whether or not this gizmo has been set off.
    local click = false
    local which = nil

    -- if flag isn't nil...
    if flag then
        -- gizmo flag set to ignore board/pad input.
        if bit.band(flag, GIZMO_FLAG.IGNORE_BOARD) ~= 0 then
            -- if board/pad is interacting with us...
            if index then
                -- set to false, and scroll away from us, using the last input direction.
                index = false
                self.index = self.index + self.which
            end
        end

        -- gizmo flag set to ignore mouse input.
        if bit.band(flag, GIZMO_FLAG.IGNORE_MOUSE) ~= 0 then
            -- if mouse is interacting with us...
            if hover then
                -- set to false.
                hover = false
            end
        end
    end

    -- if we have any form of interaction with the gizmo...
    if hover or index then
        -- check if the focus button has been set off.
        -- TO-DO change to press_repeat ONLY if input device is not mouse.
        local hover_click = WINDOW_ACTION_FOCUS:press(self.device)

        -- if input over-ride isn't nil...
        if input then
            -- over-ride the default focus button with the given one.
            hover_click, which = input:press(self.device)
        end

        if hover_click then
            if flag and bit.band(flag, GIZMO_FLAG.CLICK_ON_PRESS) ~= 0 then
                click = true
            else
                -- focus button was set off, set us as the focus gizmo.
                self.focus = self.count
            end
        end
    end

    -- check if we are the focus gizmo.
    local focus = self.focus == self.count

    -- if we are the focus gizmo...
    if focus then
        -- check if the focus button has been set up.
        local focus_click = WINDOW_ACTION_FOCUS:release(self.device)

        -- if input over-ride isn't nil...
        if input then
            -- over-ride the default focus button with the given one.
            focus_click, which = input:release(self.device)
        end

        -- focus button was set up, set off click event, release focus gizmo.
        if focus_click then
            click = true
            self.focus = nil
        end
    end

    -- increase gizmo count.
    self.count = self.count + 1
    self.frame = shape.point.y + shape.point.y

    if index then
        self.gizmo = shape
    end

    return hover, index, focus, click, which
end

local function window_check_draw(self, shape)
    if self.shape then
        return alicia.collision.get_box_2_box_2(self.shape, shape)
    end

    return true
end

--[[----------------------------------------------------------------]]

---@class window
---@field index  number
---@field count  number
---@field focus  number | nil
---@field device input_device
window = {
    __meta = {}
}

---Create a new window.
---@return window value # The window.
function window:new()
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[]]

    i.__type = "window"
    i.index  = 0.0
    i.count  = 0.0
    i.point  = 0.0
    i.which  = 0.0
    i.shift  = false
    i.mouse  = vector_2:new(0.0, 0.0)
    i.device = INPUT_DEVICE.MOUSE
    i.data   = {}
    i.font   = {
        [WINDOW_FONT_SCALE] = alicia.font.new_default(WINDOW_FONT_SCALE)
    }

    return i
end

---Begin the window.
---@param lock boolean # If true, will lock user input.
function window:draw(call, lock, mouse, ...)
    self.count = 0.0

    if self.device == INPUT_DEVICE.MOUSE then
        if mouse then
            self.mouse = mouse
        else
            self.mouse = vector_2:new(alicia.input.mouse.get_point())
        end
    end

    call(...)

    local above = WINDOW_ACTION_ABOVE:press(self.device)
    local below = WINDOW_ACTION_BELOW:press(self.device)

    -- roll over the value in case it is not hovering over any valid gizmo.
    self.index = math.roll_over(0.0, self.count - 1.0, self.index)

    -- if temporarily locking navigation input or currently focusing a gizmo...
    if self.shift or self.focus or lock then
        -- remove navigation lock.
        self.shift = false

        -- disregard any input.
        return
    end

    -- scroll above.
    if above then
        self.index = self.index - 1.0
        self.which = -1.0
    end

    -- scroll below.
    if below then
        self.index = self.index + 1.0
        self.which = 1.0
    end

    -- get the latest board press.
    local board_check = alicia.input.board.get_key_code_queue() > 0.0
    local mouse_check = alicia.input.mouse.get_press(INPUT_MOUSE.LEFT)
    local pad_check = alicia.input.pad.get_queue() > 0.0

    -- a board or pad button was set off...
    if board_check or pad_check then
        -- set the active device as either board, or pad.
        self:set_device(board_check and INPUT_DEVICE.BOARD or INPUT_DEVICE.PAD)
    end

    -- a mouse button was set off...
    if mouse_check then
        self:set_device(INPUT_DEVICE.MOUSE)
    end
end

---Draw a button gizmo.
---@param shape box_2            # The shape of the gizmo.
---@param label string | texture # The label of the gizmo.
---@param flag? gizmo_flag       # OPTIONAL: The flag of the gizmo.
---@return boolean click # True on click, false otherwise.
function window:button(shape, label, flag)
    -- scroll.
    shape.point.y = shape.point.y + self.point

    -- get the state of this gizmo.
    local hover, index, focus, click = window_state(self, shape, flag)

    if window_check_draw(self, shape) then
        if type(label) == "string" then
            -- draw a border.
            window_border(self, shape, hover, index, focus, label)
        else
            -- draw a border.
            window_border(self, shape, hover, index, focus, "")
            label:draw_pro(box_2:new(vector_2:zero(), vector_2:new(label.shape_x, label.shape_y)), shape, vector_2:zero(),
                0.0,
                color:white())
        end
    end

    -- return true on click.
    return click
end

---Draw a toggle gizmo.
---@param shape box_2      # The shape of the gizmo.
---@param label string     # The label of the gizmo.
---@param value number     # The value of the gizmo.
---@param flag? gizmo_flag # OPTIONAL: The flag of the gizmo.
---@return number  value # The value.
---@return boolean click # True on click, false otherwise.
function window:toggle(shape, label, value, flag)
    -- scroll.
    shape.point.y = shape.point.y + self.point

    -- get the state of this gizmo.
    local hover, index, focus, click = window_state(self, shape, flag)

    -- toggle value on click.
    if click then
        value = not value
    end

    if window_check_draw(self, shape) then
        -- draw a border, with a text off-set.
        window_border(self, shape, hover, index, focus, label, vector_2:new(shape.shape.x, 0.0))

        -- if value is set on, draw a small box to indicate so.
        if value then
            alicia.draw_2d.draw_box_2_round(
                box_2:new(shape.point.x + 6.0, shape.point.y + 6.0, shape.shape.x - 12.0, shape.shape.y - 12.0),
                WINDOW_CARD_ROUND_SHAPE, WINDOW_CARD_ROUND_COUNT, WINDOW_COLOR_PRIMARY_SIDE)
        end
    end

    -- return value, and click.
    return value, click
end

---Draw a slider gizmo.
---@param shape box_2      # The shape of the gizmo.
---@param label string     # The label of the gizmo.
---@param value number     # The value of the gizmo.
---@param min   number     # The minimum value of the gizmo.
---@param max   number     # The minimum value of the gizmo.
---@param step  number     # The step size of the gizmo.
---@param flag? gizmo_flag # OPTIONAL: The flag of the gizmo.
---@return number  value # The value.
---@return boolean click # True on click, false otherwise.
function window:slider(shape, label, value, min, max, step, flag)
    -- scroll.
    shape.point.y = shape.point.y + self.point

    -- click on press flag is incompatible with this gizmo, remove if present.
    if flag then
        flag = bit.band(flag, bit.bnot(GIZMO_FLAG.CLICK_ON_PRESS))
    end

    -- get the state of this gizmo.
    local hover, index, focus, click, which = window_state(self, shape, flag, WINDOW_ACTION_LATERAL)

    -- special preference for the mouse.
    if self.device == INPUT_DEVICE.MOUSE then
        -- if gizmo is in focus...
        if focus then
            -- calculate value.
            local result = math.percentage_from_value(shape.point.x + 6.0, shape.point.x + 6.0 + shape.shape.x - 12.0,
                self.mouse.x)
            result = math.clamp(0.0, 1.0, result)
            result = math.value_from_percentage(min, max, result)
            result = math.snap(step, result)
            value = result
        end
    else
        -- if there has been input at all...
        if which then
            if which.button == INPUT_BOARD.LEFT or which == INPUT_PAD.LEFT_FACE_LEFT then
                -- decrease value.
                value = value - step
            else
                -- increase value.
                value = value + step
            end

            -- clamp.
            value = math.clamp(min, max, value)
        end
    end

    if window_check_draw(self, shape) then
        -- get the percentage of the value within the minimum/maximum range.
        local percentage = math.clamp(0.0, 1.0, math.percentage_from_value(min, max, value))

        -- draw a border, with a text off-set.
        local gizmo = window_border(self, shape, hover, index, focus, label, vector_2:new(shape.shape.x, 0.0))
        local value = string.format("%.2f", value)

        -- if percentage is above 0.0...
        if percentage > 0.0 then
            alicia.draw_2d.draw_box_2_round(
                box_2:new(shape.point + vector_2:new(6.0, 6.0), vector_2:new((shape.shape.x - 12.0) * percentage,
                    shape.shape.y - 12.0)),
                WINDOW_CARD_ROUND_SHAPE, WINDOW_CARD_ROUND_COUNT, gizmo:fade(self, WINDOW_COLOR_PRIMARY_SIDE))
        end

        -- measure text.
        local measure = window_font(self, shape.shape.y):measure_text(value, shape.shape.y, WINDOW_FONT_SPACE)

        -- draw value.
        window_font(self, shape.shape.y):draw(value,
            vector_2:new(shape.point.x + (shape.shape.x * 0.5) - (measure * 0.5), shape.point.y + WINDOW_SHIFT.y),
            vector_2:zero(),
            0.0,
            shape.shape.y,
            WINDOW_FONT_SPACE,
            gizmo:fade(self, color:white()))
    end

    -- return value, and click.
    return value, click
end

-- TO-DO
function window:spinner(shape, label, value, step, flag)
    -- scroll.
    shape.point.y = shape.point.y + self.point

    -- click on press flag is incompatible with this gizmo, remove if present.
    if flag then
        flag = bit.band(flag, bit.bnot(GIZMO_FLAG.CLICK_ON_PRESS))
    end

    -- get the state of this gizmo.
    local hover, index, focus, click, which = window_state(self, shape, flag, WINDOW_ACTION_LATERAL)

    -- special preference for the mouse.
    if self.device == INPUT_DEVICE.MOUSE then
        -- if gizmo is in focus...
        if focus then
            if WINDOW_ACTION_LATERAL:press(INPUT_DEVICE.MOUSE) then
                print("press")
                alicia.input.mouse.set_active(false)
            end

            if WINDOW_ACTION_LATERAL:release(INPUT_DEVICE.MOUSE) then
                print("release")
                alicia.input.mouse.set_active(true)
            end

            local delta_x = alicia.input.mouse.get_delta()
            value = value + delta_x * step
        end
    else
        -- if there has been input at all...
        if which then
            if which.button == INPUT_BOARD.LEFT or which == INPUT_PAD.LEFT_FACE_LEFT then
                -- decrease value.
                value = value - step
            else
                -- increase value.
                value = value + step
            end
        end
    end

    if window_check_draw(self, shape) then
        -- draw a border, with a text off-set.
        local gizmo = window_border(self, shape, hover, index, focus, label, vector_2:new(shape.shape.x, 0.0))
        local value = string.format("%.2f", value)

        -- measure text.
        local measure = window_font(self, shape.shape.y):measure_text(value, shape.shape.y, WINDOW_FONT_SPACE)

        -- draw value.
        window_font(self, shape.shape.y):draw(value,
            vector_2:new(shape.point.x + (shape.shape.x * 0.5) - (measure * 0.5), shape.point.y + WINDOW_SHIFT.y),
            vector_2:zero(),
            0.0,
            shape.shape.y,
            WINDOW_FONT_SPACE,
            gizmo:fade(self, color:white()))
    end

    -- return value, and click.
    return value, click
end

---Draw a switch gizmo.
---@param shape box_2      # The shape of the gizmo.
---@param label string     # The label of the gizmo.
---@param value number     # The value of the gizmo.
---@param pool  table      # The value pool of the gizmo.
---@param flag? gizmo_flag # OPTIONAL: The flag of the gizmo.
---@return number  value # The value.
---@return boolean click # True on click, false otherwise.
function window:switch(shape, label, value, pool, flag)
    -- scroll.
    shape.point.y = shape.point.y + self.point

    -- get the state of this gizmo.
    local hover, index, focus, click, which = window_state(self, shape, flag, WINDOW_ACTION_LATERAL)

    local value_a = nil
    local value_b = nil
    local value_label = "N/A"

    value_label = pool[value]

    -- if there's an entry below us...
    if pool[value - 1] then
        value_a = value - 1
    end

    -- if there's an entry below us...
    if pool[value + 1] then
        value_b = value + 1
    end

    -- if there has been input at all...
    if which then
        if which.button == INPUT_BOARD.LEFT or which.button == INPUT_MOUSE.LEFT or which.button == INPUT_PAD.LEFT_FACE_LEFT then
            -- if below value is valid...
            if value_a then
                -- decrease value.
                value = value_a
            end
        else
            -- if above value is valid...
            if value_b then
                -- increase value.
                value = value_b
            end
        end
    end

    if window_check_draw(self, shape) then
        -- draw a border, with a text off-set.
        window_border(self, shape, hover, index, focus, label, vector_2:new(shape.shape.x, 0.0))

        -- measure text.
        local measure = window_font(self, shape.shape.y):measure_text(value_label, shape.shape.y, WINDOW_FONT_SPACE)

        -- draw value.
        window_font(self, shape.shape.y):draw(value_label,
            vector_2:new(shape.point.x + (shape.shape.x * 0.5) - (measure * 0.5), shape.point.y + WINDOW_SHIFT.y),
            vector_2:zero(),
            0.0,
            shape.shape.y,
            WINDOW_FONT_SPACE,
            color:white())
    end

    -- return value, and click.
    return value, click
end

---Draw an action gizmo.
---@param shape   box_2      # The shape of the gizmo.
---@param label   string     # The label of the gizmo.
---@param value   action     # The value of the gizmo.
---@param clamp?  number     # OPTIONAL: The maximum button count for the action. If nil, do not clamp.
---@param flag?   gizmo_flag # The flag of the gizmo.
---@return boolean click # True on click, false otherwise.
function window:action(shape, label, value, clamp, flag)
    -- scroll.
    shape.point.y = shape.point.y + self.point

    local pick = false

    -- if pick gizmo is not nil...
    if self.pick then
        -- check if we are the pick gizmo.
        pick = self.pick == self.count
    end

    -- if we are the pick gizmo...
    if pick then
        -- get every button press in the queue.
        local board_queue = alicia.input.board.get_key_code_queue()
        local mouse_queue = alicia.input.mouse.get_queue()

        -- if a button was set off...
        if board_queue > 0.0 or mouse_queue then
            if clamp then
                if #value.list >= clamp then
                    -- remove every button for this action.
                    value.list = {}
                end
            end

            -- if button came from the board, attach board action.
            if board_queue > 0.0 then
                value:button_attach(action_button:new(INPUT_DEVICE.BOARD, board_queue))
            end

            -- if button came from the mouse, attach mouse action.
            if mouse_queue then
                value:button_attach(action_button:new(INPUT_DEVICE.MOUSE, mouse_queue))
            end

            -- remove us from the focus gizmo, lock navigation, and remove us from the pick gizmo.
            self.focus = nil
            self.shift = true
            self.pick = nil
        end
    end

    local action = pick and action:new({}) or WINDOW_ACTION_ALTERNATE

    -- get the state of this gizmo.
    local hover, index, focus, click, which = window_state(self, shape, flag, action)

    -- if there has been input at all...
    if which then
        if which.button == INPUT_BOARD.SPACE or which.button == INPUT_MOUSE.LEFT or which.button == INPUT_PAD.LEFT_FACE_DOWN then
            -- make us the focus/pick gizmo
            self.focus = self.count - 1.0
            self.pick = self.count - 1.0
        else
            -- remove every button for this action.
            value.list = {}
        end
    end

    if window_check_draw(self, shape) then
        -- draw a border.
        window_border(self, shape, hover, index, focus, label, vector_2:new(shape.shape.x, 0.0))

        local label = #value.list > 0.0 and "" or "N/A"

        -- for every button in the action's list...
        for i, button in ipairs(value.list) do
            -- concatenate the button's name.
            label = label .. (i > 1.0 and "/" or "")
                .. button:name()
        end

        -- measure text.
        local measure = window_font(self, shape.shape.y):measure_text(label, shape.shape.y, WINDOW_FONT_SPACE)

        -- draw value.
        window_font(self, shape.shape.y):draw(label,
            vector_2:new(shape.point.x + (shape.shape.x * 0.5) - (measure * 0.5), shape.point.y + WINDOW_SHIFT.y),
            vector_2:zero(),
            0.0,
            shape.shape.y,
            WINDOW_FONT_SPACE,
            color:white())
    end

    return click
end

---Draw an entry gizmo.
---@param shape   box_2      # The shape of the gizmo.
---@param label   string     # The label of the gizmo.
---@param value   string     # The value of the gizmo.
---@param flag?   gizmo_flag # The flag of the gizmo.
---@return boolean click # True on click, false otherwise.
function window:entry(shape, label, value, flag)
    -- scroll.
    shape.point.y = shape.point.y + self.point

    local gizmo = window_gizmo(self, label)
    local pick = false
    local done = false

    -- if pick gizmo is not nil...
    if self.pick then
        -- check if we are the pick gizmo.
        pick = self.pick == self.count
    end

    -- if we are the pick gizmo...
    if pick then
        -- get every button press in the queue.
        local board_queue = alicia.input.board.get_uni_code_queue()

        -- TO-DO change device to keyboard on focus gain.
        local _, which = WINDOW_ACTION_ENTRY:press_repeat()

        if alicia.input.board.get_down(INPUT_BOARD.L_CONTROL) then
            if alicia.input.board.get_press(INPUT_BOARD.V) then
                local value_a = string.sub(value, 0, gizmo.entry_cursor)
                local value_b = string.sub(value, gizmo.entry_cursor + 1)
                local text = alicia.input.get_clipboard_text()

                gizmo.entry_cursor = gizmo.entry_cursor + #text
                value = value_a .. text .. value_b
            end
        end

        if which then
            if which.button == INPUT_BOARD.LEFT then
                if gizmo.entry_cursor > 0 then
                    local stop = false

                    if alicia.input.board.get_down(INPUT_BOARD.L_CONTROL) then
                        while gizmo.entry_cursor > 0 do
                            gizmo.entry_cursor = gizmo.entry_cursor - 1

                            local check = string.sub(value, gizmo.entry_cursor, gizmo.entry_cursor)

                            if check == "." or check == "(" or check == ")" or check == " " then
                                stop = true
                            else
                                if stop then
                                    break
                                end
                            end
                        end
                    else
                        gizmo.entry_cursor = gizmo.entry_cursor - 1
                    end
                end
            elseif which.button == INPUT_BOARD.RIGHT then
                if gizmo.entry_cursor < #value then
                    local stop = false

                    if alicia.input.board.get_down(INPUT_BOARD.L_CONTROL) then
                        while gizmo.entry_cursor < #value do
                            gizmo.entry_cursor = gizmo.entry_cursor + 1

                            local check = string.sub(value, gizmo.entry_cursor + 1, gizmo.entry_cursor + 1)

                            if check == "." or check == "(" or check == ")" or check == " " then
                                stop = true
                            else
                                if stop then
                                    break
                                end
                            end
                        end
                    else
                        gizmo.entry_cursor = gizmo.entry_cursor + 1
                    end
                end
            end
        else
            if alicia.input.board.get_press(INPUT_BOARD.RETURN) then
                -- remove us from the focus gizmo, lock navigation, and remove us from the pick gizmo.
                self.focus = nil
                self.shift = true
                self.pick = nil
                done = true
                gizmo.entry_cursor = 0
            elseif alicia.input.board.get_press(INPUT_BOARD.BACKSPACE) or alicia.input.board.get_press_repeat(INPUT_BOARD.BACKSPACE) then
                if gizmo.entry_cursor > 0 then
                    local value_a = string.sub(value, 0, gizmo.entry_cursor - 1)
                    local value_b = string.sub(value, gizmo.entry_cursor + 1)

                    gizmo.entry_cursor = gizmo.entry_cursor - 1
                    value = value_a .. value_b
                end
            elseif board_queue > 0.0 then
                local value_a = string.sub(value, 0, gizmo.entry_cursor)
                local value_b = string.sub(value, gizmo.entry_cursor + 1)

                gizmo.entry_cursor = gizmo.entry_cursor + 1
                value = value_a .. string.char(board_queue) .. value_b
            end
        end
    end

    local action = pick and action:new({}) or WINDOW_ACTION_FOCUS

    -- get the state of this gizmo.
    local hover, index, focus, click, which = window_state(self, shape, flag, action)

    -- if there has been input at all...
    if which then
        self.focus = self.count - 1.0
        self.pick = self.count - 1.0
    end

    -- draw a border.
    window_border(self, shape, hover, index, focus, label, vector_2:new(shape.shape.x, 0.0))

    if pick then
        local measure = window_font(self, shape.shape.y):measure_text(string.sub(value, 0, gizmo.entry_cursor),
            shape.shape.y,
            WINDOW_FONT_SPACE)

        alicia.draw_2d.draw_box_2(
            box_2:new(shape.point + vector_2:new(measure + 4.0, 0.0), vector_2:new(2.0, shape.shape.y)),
            vector_2:zero(),
            0.0, color:white())
    end

    -- draw value.
    window_font(self, shape.shape.y):draw(value, vector_2:new(shape.point.x + 4.0, shape.point.y + WINDOW_SHIFT.y),
        vector_2:zero(),
        0.0,
        shape.shape.y,
        WINDOW_FONT_SPACE,
        color:white())

    -- TO-DO tidy up return data for every widget
    return value, done, focus
end

---Draw a scroll gizmo.
---@param shape box_2    # The shape of the gizmo.
---@param value number   # The value of the gizmo.
---@param frame number   # The frame of the gizmo.
---@param call  function # The draw function.
---@return number value
---@return number frame
function window:scroll(shape, call)
    local gizmo = window_gizmo(self, self.count)

    local view_size = math.min(0.0, shape.shape.y - gizmo.scroll_frame)
    self.point = view_size * gizmo.scroll_value
    self.shape = shape
    self.frame = 0.0

    local begin = self.count

    alicia.draw.begin_scissor(call, shape)

    local close = self.count

    gizmo.scroll_frame = (self.frame - shape.point.y) - self.point

    self.point = 0.0
    self.shape = nil
    self.frame = 0.0

    if self.gizmo then
        if self.index >= begin and self.index <= close then
            if self.gizmo.y < shape.point.y then
                local subtract = shape.point.y - self.gizmo.y

                gizmo.scroll_value = math.clamp(0.0, 1.0, gizmo.scroll_value + (subtract / view_size))
            end

            if self.gizmo.y + self.gizmo.height > shape.point.y + shape.shape.y then
                local subtract = (self.gizmo.y + self.gizmo.height) - (shape.point.y + shape.shape.y)

                gizmo.scroll_value = math.clamp(0.0, 1.0, gizmo.scroll_value - (subtract / view_size))
            end
        else
            if self.index < begin then gizmo.scroll_value = 0.0 end
            if self.index > close then gizmo.scroll_value = 1.0 end
        end
    end

    self.gizmo = nil

    local delta = vector_2:new(alicia.input.mouse.get_wheel())

    if alicia.collision.get_point_box_2(self.mouse, shape) then
        gizmo.scroll_value = math.clamp(0.0, 1.0, gizmo.scroll_value - delta.y * 0.05)
    end
end

-- TO-DO documentation
function window:border(shape, round, count, b_color)
    -- scroll.
    shape.point.y = shape.point.y + self.point

    round         = round or WINDOW_CARD_ROUND_SHAPE
    count         = count or WINDOW_CARD_ROUND_COUNT
    b_color       = b_color or WINDOW_COLOR_PRIMARY_MAIN

    if window_check_draw(self, shape) then
        alicia.draw_2d.draw_box_2_round(shape,
            round,
            count,
            b_color)
    end
end

---Draw a text gizmo.
---@param point  box_2  # The point of the gizmo.
---@param label  string # The label of the gizmo.
---@param font   font   # The font of the gizmo.
-- TO-DO origin, angle
---@param scale  number # The text scale of the gizmo.
---@param space  number # The text space of the gizmo.
---@param color  number # The text color of the gizmo.
---@return boolean click # True on click, false otherwise.
function window:text(point, label, font, origin, angle, scale, space, color)
    -- scroll.
    point.y = point.y + self.point

    if window_check_draw(self, box_2:new(point, vector_2:new(1.0, scale))) then
        if font then
            font:draw(label, point, origin, angle, scale, space, color)
        else
            window_font(self, scale):draw(label, point, origin, angle, scale, space, color)
        end
    end
end

function window:set_device(device)
    if device == INPUT_DEVICE.BOARD or device == INPUT_DEVICE.PAD then
        if not alicia.input.mouse.get_hidden() then
            -- if mouse wasn't hidden, disable.
            alicia.input.mouse.set_active(false)
        end
    else
        if alicia.input.mouse.get_hidden() then
            -- if mouse was hidden, enable.
            alicia.input.mouse.set_active(true)
        end

        -- reset index.
        self.index = 0.0
    end

    -- set the active device.
    self.device = device
end

--[[----------------------------------------------------------------]]

---@class gizmo
---@field hover       number
---@field sound_hover boolean
---@field sound_focus boolean
gizmo = {
    __meta = {}
}

---Create a new gizmo.
---@return gizmo value # The gizmo.
function gizmo:new()
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[]]

    i.__type       = "gizmo"
    i.hover        = 0.0
    i.focus        = 0.0
    -- TO-DO consider only initializing this for the appropiate gizmo and not before?
    i.scroll_value = 0.0
    i.scroll_frame = 0.0
    i.entry_cursor = 0.0

    return i
end

---Calculate the point of a gizmo with animation.
---@param lobby lobby # The lobby.
---@param shape box_2 # The shape.
---@return box_2 shape # The shape, with animation.
function gizmo:move(window, shape)
    -- move shape horizontally.
    --shape.point.x = shape.point.x + (ease.in_out_quad(self.hover) * 8.0) - 16.0 + math.out_quad(math.min(1.0, window.time * 4.0)) * 16.0

    shape.point.y = shape.point.y - (ease.in_out_quad(self.hover) * 4.0) + (ease.in_out_quad(self.focus) * 4.0)

    return shape
end

---Calculate the color of a gizmo with animation.
---@param lobby lobby # The lobby.
---@param color color # The color.
---@return color color # The color, with animation.
function gizmo:fade(window, color)
    dark = 1.0

    if window.pick and not (window.count == window.pick + 1) then
        dark = 0.5
    end

    -- fade in/out from hover.
    color = color * (ease.in_out_quad(self.hover) * 0.25 + 0.75) * dark

    -- fade in/out from time.
    --color.a = math.floor(math.out_quad(math.min(1.0, window.time * 4.0)) * 255.0)

    return color
end

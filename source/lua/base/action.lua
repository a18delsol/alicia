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

---Check if the currently active device is the same as the button's device.
---@param action_button action_button # The action button.
---@param active_device action_button # The active device.
---@param check_device  action_button # The device kind to check against.
local function check_device(action_button, active_device, check_device)
    if active_device then
        return action_button.device == check_device and active_device == check_device
    else
        return action_button.device == check_device
    end
end

--[[----------------------------------------------------------------]]

---@class action
---@field list table
action = {
    __meta = {}
}

---Create a new action.
---@param  button_list table # A table array of every action button to be bound to this action.
---@return action      value # The action.
function action:new(button_list)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[]]

    i.__type = "action"
    i.list   = button_list

    return i
end

---Attach a new button to the action's list.
---@param button action_button # The action button to attach.
function action:button_attach(button)
    -- iterate over every button in our button list.
    for i, list_button in ipairs(self.list) do
        if list_button.device == button.device and list_button.button == button.button then
            return nil
        end
    end

    table.insert(self.list, button)
end

---Get the input state (up) of the action, depending on the active device.
---@param active_device? # The active device.
---@return boolean state # The state of the action (up).
function action:up(active_device)
    -- iterate over every button in our button list.
    for i, button in ipairs(self.list) do
        if button:up(active_device) then
            return true, button
        end
    end

    return false, nil
end

---Get the input state (down) of the action, depending on the active device.
---@param active_device? # The active device.
---@return boolean state # The state of the action (down).
function action:down(active_device)
    -- iterate over every button in our button list.
    for i, button in ipairs(self.list) do
        if button:down(active_device) then
            return true, button
        end
    end

    return false, nil
end

---Get the input state (press) of the action, depending on the active device.
---@param active_device? # The active device.
---@return boolean state # The state of the action (press).
function action:press(active_device)
    -- iterate over every button in our button list.
    for i, button in ipairs(self.list) do
        if button:press(active_device) then
            return true, button
        end
    end

    return false, nil
end

---Get the input state (press, repeat) of the action, depending on the active device.
---@param active_device? # The active device.
---@return boolean state # The state of the action (press, repeat).
function action:press_repeat(active_device)
    -- iterate over every button in our button list.
    for i, button in ipairs(self.list) do
        if button:press_repeat(active_device) then
            return true, button
        end
    end

    return false, nil
end

---Get the input state (release) of the action, depending on the active device.
---@param active_device? # The active device.
---@return boolean state # The state of the action (release).
function action:release(active_device)
    -- iterate over every button in our button list.
    for i, button in ipairs(self.list) do
        if button:release(active_device) then
            return true, button
        end
    end

    return false, nil
end

---Get the input state (release, repeat) of the action, depending on the active device.
---@param active_device? # The active device.
---@return boolean state # The state of the action (release, repeat).
function action:release_repeat(active_device)
    -- iterate over every button in our button list.
    for i, button in ipairs(self.list) do
        if button:release_repeat(active_device) then
            return true, button
        end
    end

    return false, nil
end

--[[----------------------------------------------------------------]]

---@class action_button
---@field device input_device
---@field button number
---@field bounce number
action_button = {
    __meta = {}
}

---Create a new action button.
---@param device input_device # The device for which the button is for.
---@param button input_device # The button for which the action is for.
---@return action_button value # The action_button.
function action_button:new(device, button)
    local i = {}
    setmetatable(i, self.__meta)
    getmetatable(i).__index = self

    --[[]]

    i.__type = "action_button"
    i.device = device
    i.button = button
    i.bounce = 0.0

    return i
end

---Get the human-readable name of the button.
---@return string name # The human-readable name of the button.
function action_button:name()
    -- check which device the button is bound to, then return the name from the input_* table.
    if self.device == INPUT_DEVICE.BOARD then
        return INPUT_BOARD[self.button]
    end

    if self.device == INPUT_DEVICE.MOUSE then
        return INPUT_MOUSE[self.button]
    end

    if self.device == INPUT_DEVICE.PAD then
        return INPUT_PAD[self.button]
    end

    error("action_button::name(): Unknown device.")
end

---Get the input state (up) of the action button, depending on the active device.
---@param active_device? # The active device.
---@return boolean state # The state of the input (up).
function action_button:up(active_device)
    -- check which device the button is bound to, then return the input state.
    if check_device(self, active_device, INPUT_DEVICE.BOARD) then
        return alicia.input.board.get_up(self.button)
    end

    if check_device(self, active_device, INPUT_DEVICE.MOUSE) then
        return alicia.input.mouse.get_up(self.button)
    end

    if check_device(self, active_device, INPUT_DEVICE.PAD) then
        return alicia.input.pad.get_up(0.0, self.button)
    end

    return false
end

---Get the input state (down) of the action button, depending on the active device.
---@param active_device? # The active device.
---@return boolean state # The state of the input (down).
function action_button:down(active_device)
    -- check which device the button is bound to, then return the input state.
    if check_device(self, active_device, INPUT_DEVICE.BOARD) then
        return alicia.input.board.get_down(self.button)
    end

    if check_device(self, active_device, INPUT_DEVICE.MOUSE) then
        return alicia.input.mouse.get_down(self.button)
    end

    if check_device(self, active_device, INPUT_DEVICE.PAD) then
        return alicia.input.pad.get_down(0.0, self.button)
    end

    return false
end

---Get the input state (press) of the action button, depending on the active device.
---@param active_device? # The active device.
---@return boolean state # The state of the input (press).
function action_button:press(active_device)
    -- check which device the button is bound to, then return the input state.
    if check_device(self, active_device, INPUT_DEVICE.BOARD) then
        return alicia.input.board.get_press(self.button)
    end

    if check_device(self, active_device, INPUT_DEVICE.MOUSE) then
        return alicia.input.mouse.get_press(self.button)
    end

    if check_device(self, active_device, INPUT_DEVICE.PAD) then
        return alicia.input.pad.get_press(0.0, self.button)
    end

    return false
end

---Get the input state (press, repeat) of the action button, depending on the active device.
---@param active_device? # The active device.
---@return boolean state # The state of the input (press, repeat).
function action_button:press_repeat(active_device)
    -- check which device the button is bound to, then return the input state.
    if check_device(self, active_device, INPUT_DEVICE.BOARD) then
        return alicia.input.board.get_press(self.button)
            or alicia.input.board.get_press_repeat(self.button)
    end

    if check_device(self, active_device, INPUT_DEVICE.MOUSE) then
        if alicia.input.mouse.get_down(self.button) then
            local delta = alicia.general.get_frame_time()

            self.bounce = self.bounce + delta

            if self.bounce > 0.5 then
                self.bounce = 0.4
                return true
            end
        else
            self.bounce = 0.0
        end

        return alicia.input.mouse.get_press(self.button)
    end

    if check_device(self, active_device, INPUT_DEVICE.PAD) then
        if alicia.input.pad.get_down(0.0, self.button) then
            local delta = alicia.general.get_frame_time()

            self.bounce = self.bounce + delta

            if self.bounce > 0.5 then
                self.bounce = 0.4
                return true
            end
        else
            self.bounce = 0.0
        end

        return alicia.input.pad.get_press(0.0, self.button)
    end

    return false
end

---Get the input state (release) of the action button, depending on the active device.
---@param active_device? # The active device.
---@return boolean state # The state of the input (release).
function action_button:release(active_device)
    -- check which device the button is bound to, then return the input state.
    if check_device(self, active_device, INPUT_DEVICE.BOARD) then
        return alicia.input.board.get_release(self.button)
    end

    if check_device(self, active_device, INPUT_DEVICE.MOUSE) then
        return alicia.input.mouse.get_release(self.button)
    end

    if check_device(self, active_device, INPUT_DEVICE.PAD) then
        return alicia.input.pad.get_release(0.0, self.button)
    end

    return false
end

---Get the input state (release, repeat) of the action button, depending on the active device.
---@param active_device? # The active device.
---@return boolean state # The state of the input (release, repeat).
function action_button:release_repeat(active_device)
    -- check which device the button is bound to, then return the input state.
    if check_device(self, active_device, INPUT_DEVICE.BOARD) then
        return alicia.input.board.get_release(self.button)
            or alicia.input.board.get_press_repeat(self.button)
    end

    if check_device(self, active_device, INPUT_DEVICE.MOUSE) then
        if alicia.input.mouse.get_down(self.button) then
            local delta = alicia.general.get_frame_time()

            self.bounce = self.bounce + delta

            if self.bounce > 0.5 then
                self.bounce = 0.4
                return true
            end
        else
            self.bounce = 0.0
        end

        return alicia.input.mouse.get_release(self.button)
    end

    if check_device(self, active_device, INPUT_DEVICE.PAD) then
        if alicia.input.pad.get_down(0.0, self.button) then
            local delta = alicia.general.get_frame_time()

            self.bounce = self.bounce + delta

            if self.bounce > 0.5 then
                self.bounce = 0.4
                return true
            end
        else
            self.bounce = 0.0
        end

        return alicia.input.pad.get_release(0.0, self.button)
    end

    return false
end

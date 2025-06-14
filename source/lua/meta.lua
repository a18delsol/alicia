---@meta

---The Alicia API.
---@class alicia
alicia = {}

---Main entry-point. Alicia will call this on initialization.
---@alias alicia.main fun()

---Fail entry-point. Alicia will call this on a script error, with the script error message as the argument. Note that this function is OPTIONAL, and Alicia will use a default crash handler if missing.
---@alias alicia.fail fun(error : string)

---The sound API.
---
--- ---
---*Not available in head-less mode.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/sound.rs#L64)
---@class alicia.sound
alicia.sound = {}

---An unique handle for sound in memory.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/sound.rs#L79)
---@class sound
sound = {}

---Create a new sound resource.
---@param path string # Path to sound file.
---@param alias number? # OPTIONAL: The total sound alias count to load for the sound.
---@return sound sound # LuaSound resource.
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/sound.rs#L111)
function alicia.sound.new(path,alias) end

---Create a new sound resource, from memory.
---@param data data # The data buffer.
---@param alias number? # OPTIONAL: The total sound alias count to load for the sound.
---@param kind string # The kind of sound file (.wav, etc.).
---@return sound music # LuaSound resource.
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/sound.rs#L154)
function alicia.sound.new_from_memory(data,alias,kind) end

---Create a sound alias.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/sound.rs#L209)
function sound:create_alias() end

---Remove a sound alias.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/sound.rs#L219)
function sound:remove_alias() end

---Clear every sound alias.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/sound.rs#L233)
function sound:remove_alias() end

---Play the sound.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/sound.rs#L246)
function sound:play() end

---Check if sound is currently playing.
---@return boolean state # State of the sound.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/sound.rs#L270)
function sound:get_playing() end

---Stop the sound.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/sound.rs#L287)
function sound:stop() end

---Pause the sound.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/sound.rs#L304)
function sound:pause() end

---Resume the sound.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/sound.rs#L321)
function sound:resume() end

---Set volume for the sound. (range: 0.0 - 1.0)
---@param volume number # Current volume.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/sound.rs#L347)
function sound:set_volume(volume) end

---Set pitch for the sound.
---@param pitch number # Current pitch.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/sound.rs#L376)
function sound:set_pitch(pitch) end

---Set pan for the sound. (range: 0.0 - 1.0; 0.5 is center)
---@param pan number # Current pan.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/sound.rs#L405)
function sound:set_pan(pan) end

---The input API.
---
--- ---
---*Not available in head-less mode.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L64)
---@class alicia.input
alicia.input = {}

---The board input API.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L77)
---@class alicia.input.board
alicia.input.board = {}

---The mouse input API.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L95)
---@class alicia.input.mouse
alicia.input.mouse = {}

---The pad input API.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L121)
---@class alicia.input.pad
alicia.input.pad = {}

---Set a key to exit Alicia.
---@param key input_board # Key to exit Alicia with.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L162)
function alicia.input.set_exit_key(key) end

---Set the clipboard text.
---```lua
---alicia.input.set_clipboard_text("hello, world!")
---local text = alicia.input.get_clipboard_text()
---assert(text == "hello, world!")
---
---```
---@param text string # Clipboard text.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L180)
function alicia.input.board.set_clipboard_text(text) end

---Get the clipboard text.
---```lua
---alicia.input.set_clipboard_text("hello, world!")
---local text = alicia.input.get_clipboard_text()
---assert(text == "hello, world!")
---
---```
---@return string text # Clipboard text.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L200)
function alicia.input.board.get_clipboard_text() end

---Get the last unicode glyph in the queue.
---@return number key_code # Key-code. If 0, queue is empty.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L217)
function alicia.input.board.get_key_code_queue() end

---Get the last unicode glyph in the queue.
---@return number uni_code # Uni-code. If 0, queue is empty.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L231)
function alicia.input.board.get_uni_code_queue() end

---Get the name of a given key.
---```lua
---local name = alicia.input.board.get_name(INPUT_BOARD.A)
---assert(name == "a")
---
---```
---@param board input_board # The board button to get a name for.
---@return string name # The name.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L249)
function alicia.input.board.get_name(board) end

---Get the state of an input (up).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L271)
function alicia.input.board.get_up(board) end

---Get the state of an input (down).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L285)
function alicia.input.board.get_down(board) end

---Get the state of an input (press).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L299)
function alicia.input.board.get_press(board) end

---Get the state of an input (repeat-press).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L313)
function alicia.input.board.get_press_repeat(board) end

---Get the state of an input (release).
---@param board input_board # The board button to check for.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L327)
function alicia.input.board.get_release(board) end

---Set the active state of the mouse.
---@param state boolean # Current state.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L343)
function alicia.input.mouse.set_active(state) end

---Set the hidden state of the mouse.
---@param state boolean # Current state.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L365)
function alicia.input.mouse.set_hidden(state) end

---Get the hidden state of the mouse.
---@return boolean state # Current state.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L387)
function alicia.input.mouse.get_hidden() end

---Check if the mouse is currently over the screen.
---@return boolean state # Current state.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L401)
function alicia.input.mouse.get_screen() end

---Get the current point of the mouse.
---@return number point_x # The point of the mouse (X).
---@return number point_y # The point of the mouse (Y).
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L416)
function alicia.input.mouse.get_point() end

---Set the current point of the mouse.
---@param point vector_2 # The point of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L433)
function alicia.input.mouse.set_point(point) end

---Get the current delta (i.e. mouse movement) of the mouse.
---@return number delta_x # The delta of the mouse (X).
---@return number delta_y # The delta of the mouse (Y).
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L452)
function alicia.input.mouse.get_delta() end

---Set the current shift of the mouse.
---@param shift vector_2 # The shift of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L469)
function alicia.input.mouse.set_shift(shift) end

---Set the current scale of the mouse.
---@param scale vector_2 # The scale of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L487)
function alicia.input.mouse.set_scale(scale) end

---Set the current cursor of the mouse.
---@param cursor cursor_mouse # The cursor of the mouse.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L505)
function alicia.input.mouse.set_cursor(cursor) end

---Get the current delta (i.e. mouse wheel movement) of the mouse wheel.
---@return number delta_x # The delta of the mouse wheel (X).
---@return number delta_y # The delta of the mouse wheel (Y).
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L523)
function alicia.input.mouse.get_wheel() end

---Get the last mouse button press.
---@return input_mouse input # The last mouse button press.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L540)
function alicia.input.mouse.get_mouse_queue() end

---Get the state of an input (up).
---@param mouse input_mouse # The mouse button to check for.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L564)
function alicia.input.mouse.get_up(mouse) end

---Get the state of an input (down).
---@param mouse input_mouse # The mouse button to check for.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L578)
function alicia.input.mouse.get_down(mouse) end

---Get the state of an input (press).
---@param mouse input_mouse # The mouse button to check for.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L592)
function alicia.input.mouse.get_press(mouse) end

---Get the state of an input (release).
---@param mouse input_mouse # The mouse button to check for.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L606)
function alicia.input.mouse.get_release(mouse) end

---Get the state of a pad.
---@param index number # The index of the pad to check for.
---@return boolean state # The state of the pad.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L625)
function alicia.input.pad.get_state(index) end

---Get the name of a pad.
---@param index number # The index of the pad to check for.
---@return string name # The name of the pad.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L642)
function alicia.input.pad.get_name(index) end

---Get the state of an input (press).
---@param pad input_pad # The pad button to check for.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L659)
function alicia.input.pad.get_press(pad) end

---Get the state of an input (down).
---@param pad input_pad # The pad button to check for.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L673)
function alicia.input.pad.get_down(pad) end

---Get the state of an input (release).
---@param pad input_pad # The pad button to check for.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L687)
function alicia.input.pad.get_release(pad) end

---Get the state of an input (up).
---@param pad input_pad # The pad button to check for.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L701)
function alicia.input.pad.get_up(pad) end

---Get the last pad button press.
---@return input_pad input # The last pad button press.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L715)
function alicia.input.pad.get_queue() end

---Get the axis count of a pad.
---@param index number # The index of the pad to check for.
---@return number axis_count # The axis count of the pad.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L732)
function alicia.input.pad.get_axis_count(index) end

---Get the axis state of a pad.
---@param index number # The index of the pad to check for.
---@param axis number # The axis of the pad to check for.
---@return number axis_state # The axis state of the pad.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L750)
function alicia.input.pad.get_axis_state(index,axis) end

---Set the rumble of a pad.
---@param index number # The index of the pad to rumble.
---@param motor_a number # The intensity of the L. rumble motor.
---@param motor_b number # The intensity of the R. rumble motor.
---@param duration number # The duration of the rumble.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/input.rs#L767)
function alicia.input.pad.set_rumble(index,motor_a,motor_b,duration) end

---The Rapier API.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/rapier.rs#L72)
---@class alicia.rapier
alicia.rapier = {}

---An unique handle for a Rapier simulation.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/rapier.rs#L86)
---@class rapier
rapier = {}

---Create a new Rapier simulation.
---@return rapier rapier # Rapier simulation.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/rapier.rs#L114)
function alicia.rapier.new() end

---Cast a ray.
---@param ray ray # Ray to cast.
---@param range number # Ray range.
---@param solid boolean # TO-DO
---@param exclude_rigid table # TO-DO
---@param exclude_collider table # TO-DO
---@return table collider_handle # Solid body handle.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/rapier.rs#L163)
function rapier:cast_ray(ray,range,solid,exclude_rigid,exclude_collider) end

---Cast a ray, and also get the normal information..
---@param ray ray # Ray to cast.
---@param range number # Ray range.
---@param solid boolean # TO-DO
---@param exclude_rigid table # TO-DO
---@param exclude_collider table # TO-DO
---@return table rigid_body # Rigid body handle.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/rapier.rs#L222)
function rapier:cast_ray_normal(ray,range,solid,exclude_rigid,exclude_collider) end

---Check if a cuboid is intersecting against another cuboid.
---@param point_a vector_3 # Point of cuboid (A).
---@param angle_a vector_3 # Angle of cuboid (A).
---@param shape_a vector_3 # Shape of cuboid (A).
---@param point_b vector_3 # Point of cuboid (B).
---@param angle_b vector_3 # Angle of cuboid (B).
---@param shape_b vector_3 # Shape of cuboid (B).
---@return boolean intersect # Result of intersection.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/rapier.rs#L288)
function rapier:test_intersect_cuboid_cuboid(point_a,angle_a,shape_a,point_b,angle_b,shape_b) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/rapier.rs#L332)
function rapier:test_intersect_cuboid() end

---Get the shape of a solid body (cuboid).
---@param solid_body table # Solid body handle.
---@return number half_shape_x # Half-shape of the cuboid. (X).
---@return number half_shape_y # Half-shape of the cuboid. (Y).
---@return number half_shape_z # Half-shape of the cuboid. (Z).
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/rapier.rs#L404)
function rapier:get_solid_body_shape_cuboid(solid_body) end

---Set the shape of a solid body.
---@param solid_body table # Solid body handle.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/rapier.rs#L436)
function rapier:set_solid_body_shape(solid_body) end

---Get the parent of a solid body.
---@param solid_body table # Solid body handle.
---@return table rigid_body # Rigid body handle.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/rapier.rs#L488)
function rapier:get_solid_body_parent(solid_body) end

---Get the position of a solid body.
---@param solid_body table # Solid body handle.
---@return number position_x # Solid body position (X).
---@return number position_y # Solid body position (Y).
---@return number position_z # Solid body position (Z).
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/rapier.rs#L519)
function rapier:get_solid_body_position(solid_body) end

---Set the position of a solid body.
---@param solid_body table # Solid body handle.
---@param position vector_3 # Solid body position.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/rapier.rs#L549)
function rapier:set_solid_body_position(solid_body,position) end

---Set the rotation of a solid body.
---@param solid_body table # Solid body handle.
---@param rotation vector_3 # Solid body rotation.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/rapier.rs#L581)
function rapier:set_solid_body_rotation(solid_body,rotation) end

---Set the sensor state of a solid body.
---@param solid_body table # Solid body handle.
---@param sensor boolean # Solid body sensor state.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/rapier.rs#L610)
function rapier:set_solid_body_sensor(solid_body,sensor) end

---Remove a solid body.
---@param solid_body table # Solid body handle.
---@param wake_parent boolean # Whether or not to wake up the rigid body parent this solid body is bound to.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/rapier.rs#L639)
function rapier:solid_body_remove(solid_body,wake_parent) end

---Remove a rigid body.
---@param rigid_body table # Rigid body handle.
---@param remove_solid_body boolean # Whether or not to remove every solid body this rigid body is bound to.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/rapier.rs#L666)
function rapier:rigid_body_remove(rigid_body,remove_solid_body) end

---Create a character controller.
---@return table character_controller # Character controller.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/rapier.rs#L694)
function rapier:character_controller() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/rapier.rs#L705)
function rapier:set_character_controller_up_vector() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/rapier.rs#L722)
function rapier:set_character_controller_slope() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/rapier.rs#L739)
function rapier:set_character_auto_step() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/rapier.rs#L786)
function rapier:set_character_snap_ground() end

---Move a character controller.
---@param step number # TO-DO
---@param character table # TO-DO
---@param solid_body table # TO-DO
---@param translation vector_3 # TO-DO
---@return number movement_x # Translation point (X).
---@return number movement_y # Translation point (Y).
---@return number movement_z # Translation point (Z).
---@return boolean floor # Currently on floor.
---@return boolean slope # Currently on slope.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/rapier.rs#L822)
function rapier:character_controller_move(step,character,solid_body,translation) end

---Create a rigid body.
---@param kind rigid_body_kind # Rigid body kind.
---@return table rigid_body # Rigid body handle.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/rapier.rs#L867)
function rapier:rigid_body(kind) end

---Get the user data of a rigid_body.
---@param rigid_body userdata # Rigid body handle.
---@return number user_data # Rigid body user data.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/rapier.rs#L891)
function rapier:get_rigid_body_user_data(rigid_body) end

---Set the user data of a rigid_body.
---@param rigid_body userdata # Rigid body handle.
---@param user_data number # Rigid body user data.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/rapier.rs#L917)
function rapier:set_rigid_body_user_data(rigid_body,user_data) end

---Set the position of a rigid_body.
---@param rigid_body userdata # rigid_body handle.
---@param position vector_3 # rigid_body position.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/rapier.rs#L944)
function rapier:set_rigid_body_position(rigid_body,position) end

---Set the rotation of a rigid_body.
---@param rigid_body table # rigid_body handle.
---@param rotation vector_3 # rigid_body rotation.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/rapier.rs#L973)
function rapier:set_rigid_body_rotation(rigid_body,rotation) end

---Get the user data of a solid body.
---@param solid_body userdata # Solid body handle.
---@return number user_data # Solid body user data.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/rapier.rs#L1008)
function rapier:get_solid_body_user_data(solid_body) end

---Set the user data of a solid body.
---@param solid_body userdata # Solid body handle.
---@param user_data number # Solid body user data.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/rapier.rs#L1034)
function rapier:set_solid_body_user_data(solid_body,user_data) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/rapier.rs#L1059)
function rapier:solid_body() end

---Step the Rapier simulation.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/rapier.rs#L1146)
function rapier:step() end

---Render the Rapier simulation.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/rapier.rs#L1184)
function rapier:debug_render() end

---The Steam API.
---
--- ---
---*Available with compile feature: `steam`.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L64)
---@class alicia.steam
alicia.steam = {}

---An unique handle to a Steam client.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L78)
---@class steam
steam = {}

---Update the Steam state.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L136)
function steam:update() end

---Get the current AppID.
---@return number ID # The current AppID.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L155)
function steam:get_app_ID() end

---Get the current country code from the user's IP.
---@return string code # The current country code.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L169)
function steam:get_IP_country() end

---Get the current language for the Steam client.
---@return string language # The current language.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L183)
function steam:get_UI_language() end

---Get the current Steam server time, as an UNIX time-stamp.
---@return number time # The current server time.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L197)
function steam:get_server_time() end

---Set the position of the Steam overlay.
---@param position steam_overlay_position # The Steam overlay position.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L211)
function steam:set_overlay_position(position) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L245)
function steam:set_message() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L269)
function steam:get_message() end

---Check if a given AppID is currently on the system.
---@param ID number # The AppID.
---@return boolean install # The state of the AppID.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L297)
function steam:get_app_install(ID) end

---Check if a given DLC is currently on the system.
---@param ID number # The AppID.
---@return boolean install # The state of the AppID.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L314)
function steam:get_DLC_install(ID) end

---Check if the user has a subscription to the given AppID.
---@param ID number # The AppID.
---@return boolean subscription # Subscription state.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L331)
function steam:get_app_subscribe(ID) end

---Check if the user has a VAC ban on record.
---@return boolean ban # The VAC ban state.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L345)
function steam:get_VAC_ban() end

---Check if the user has a VAC ban on record.
---@return boolean ban # The VAC ban state.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L359)
function steam:get_cyber_cafe() end

---Check if the current AppID has support for low violence.
---@return boolean low_violence # Low violence state.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L373)
function steam:get_low_violence() end

---Check if the user has a subscription to the current AppID.
---@return boolean subscription # Subscription state.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L387)
function steam:get_subscribe() end

---Get the build ID for the current AppID.
---@return number ID # The build ID.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L401)
function steam:get_app_build_ID() end

---Get the installation path for the given AppID. NOTE: this will work even if the app is not in disk.
---@param ID number # The AppID.
---@return string path # Installation path.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L418)
function steam:get_app_install_path(ID) end

---Get the SteamID of the owner's current AppID.
---@return string ID # The SteamID.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L432)
function steam:get_app_owner() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L443)
function steam:get_game_language_list() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L454)
function steam:get_game_language() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L465)
function steam:get_beta_name() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L476)
function steam:get_launch_command_line() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L491)
function steam:get_name() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L502)
function steam:activate_overlay() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L514)
function steam:activate_overlay_link() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L526)
function steam:activate_overlay_store() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L547)
function steam:activate_overlay_user() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L563)
function steam:activate_invite_dialog() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L585)
function steam:get_steam_ID() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L596)
function steam:get_level() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L605)
function steam:get_log_on() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L620)
function steam:get_leader_board() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L640)
function steam:get_or_create_leader_board() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L673)
function steam:upload_leader_board_score() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L732)
function steam:get_leader_board_show_kind() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L764)
function steam:get_leader_board_sort_kind() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L795)
function steam:get_leader_board_name() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L822)
function steam:get_leader_board_entry_count() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L849)
function steam:pull_user_statistic() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L862)
function steam:push_user_statistic() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L875)
function steam:reset_user_statistic() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L895)
function steam:get_user_statistic() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L917)
function steam:set_user_statistic() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L941)
function steam:get_achievement() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L960)
function steam:get_achievement_list() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L977)
function steam:set_achievement() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L1003)
function steam:get_achievement_percent() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L1022)
function steam:get_achievement_name() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L1041)
function steam:get_achievement_() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L1060)
function steam:get_achievement_hidden() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L1079)
function steam:get_achievement_icon() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L1108)
function steam:get_session_user() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L1121)
function steam:get_session_client_name() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L1134)
function steam:get_session_client_form_factor() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L1159)
function steam:get_session_client_resolution() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L1179)
function steam:invite_session() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L1203)
function steam:set_cloud_app() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L1215)
function steam:get_cloud_app() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L1226)
function steam:get_cloud_account() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L1237)
function steam:get_file_list() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L1257)
function steam:file_delete() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L1270)
function steam:file_forget() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L1283)
function steam:file_exist() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L1296)
function steam:get_file_persist() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L1309)
function steam:get_file_time_stamp() end

---Create a new Steam client.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/steam.rs#L1331)
function alicia.steam.new() end

---The request API.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/request.rs#L63)
---@class alicia.request
alicia.request = {}

---Perform a GET request.
---```lua
---local link = "https://raw.githubusercontent.com/luxreduxdelux/alicia/refs/heads/main/test/asset/sample.txt"
---
----- Get the data from the link. As we know it's not binary, we pass false to the function.
---local response = alicia.request.get(link, false)
---
---assert(response == "Hello, world!")
---
---```
---@param link string # The target URL link.
---@param binary boolean # Receive as binary.
---@return string | data value # The return value.
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/request.rs#L93)
function alicia.request.get(link,binary) end

---Perform a POST request.
---```lua
----- POST request with a body. We pass false to get the result as a string.
---local body = alicia.request.post("http://httpbin.org/post", "hello, body", nil, nil, false)
---
----- POST request with a form.
---local form = alicia.request.post("http://httpbin.org/post", nil, {
---    foo = "bar",
---    bar = "baz",
---}, nil, false)
---
----- POST request with a JSON.
---local JSON = alicia.request.post("http://httpbin.org/post", nil, nil, {
---    foo = "bar",
---    bar = "baz",
---}, false)
---
----- Deserialize the JSON result to a Lua table.
---body = alicia.data.deserialize(body)
---form = alicia.data.deserialize(form)
---JSON = alicia.data.deserialize(JSON)
---
----- Assert that the request with a body's data is correct.
---assert(body.data, "hello, body")
---
----- Assert that the request with a form's data is correct.
---assert(form.form.foo, "bar")
---assert(form.form.bar, "baz")
---
----- Assert that the request with a JSON's data is correct.
---assert(JSON.json.foo, "bar")
---assert(JSON.json.bar, "baz")
---
----- POST request with binary data as the body.
---local body = alicia.request.post("http://httpbin.org/post", alicia.data.new({ 255, 0, 255, 0 }), nil, nil, false)
---
----- Deserialize the JSON result to a Lua table.
---body = alicia.data.deserialize(body)
---
----- Get the Base64 string representation of the data.
---body = string.tokenize(body.data, ",")[2]
---
----- Convert the string representation to a byte representation.
---body = alicia.data.to_data(body, 2)
---
----- Decode the data from Base64.
---body = alicia.data.decode(body)
---
----- Get the data as a Lua table.
---body = body:get_buffer()
---
----- Assert that the data sent is the same as the data we got.
---assert(body[1], 255)
---assert(body[2], 0)
---assert(body[3], 255)
---assert(body[4], 0)
---
---```
---@param link string # The target URL link.
---@param data string? # OPTIONAL: The "data" payload.
---@param form table? # OPTIONAL: The "form" payload.
---@param json table? # OPTIONAL: The "json" payload.
---@param binary boolean # Receive as binary.
---@return string | data value # The return value.
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/request.rs#L135)
function alicia.request.post(link,data,form,json,binary) end

---The texture API.
---
--- ---
---*Not available in head-less mode.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/texture.rs#L64)
---@class alicia.texture
alicia.texture = {}

---An unique handle for a texture in memory.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/texture.rs#L219)
---@class texture
---@field shape_x number # Shape of the texture (X).
---@field shape_y number # Shape of the texture (Y).
texture = {}

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/texture.rs#L246)
function texture:to_image() end

---Set the mipmap for a texture.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/texture.rs#L257)
function texture:set_mipmap() end

---Set the filter for a texture.
---@param filter texture_filter # LuaTexture filter.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/texture.rs#L272)
function texture:set_filter(filter) end

---Set the wrap for a texture.
---@param wrap texture_wrap # LuaTexture wrap.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/texture.rs#L287)
function texture:set_wrap(wrap) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/texture.rs#L301)
function texture:draw_plane() end

---Draw a texture.
---@param point vector_2 # TO-DO
---@param angle number # TO-DO
---@param scale number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/texture.rs#L334)
function texture:draw(point,angle,scale,color) end

---Draw a texture (pro).
---@param box_a box_2 # TO-DO
---@param box_b box_2 # TO-DO
---@param point vector_2 # TO-DO
---@param angle number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/texture.rs#L355)
function texture:draw_pro(box_a,box_b,point,angle,color) end

---Draw a billboard texture.
---@param camera camera_3d # TO-DO
---@param point vector_3 # TO-DO
---@param scale number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/texture.rs#L386)
function texture:draw_billboard(camera,point,scale,color) end

---Draw a billboard texture (pro).
---@param camera camera_3d # TO-DO
---@param source box_3 # TO-DO
---@param point vector_3 # TO-DO
---@param up vector_3 # TO-DO
---@param scale vector_2 # TO-DO
---@param origin vector_2 # TO-DO
---@param angle number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/texture.rs#L415)
function texture:draw_billboard_pro(camera,source,point,up,scale,origin,angle,color) end

---Create a new texture resource.
---@param path string # Path to texture file.
---@return texture texture # LuaTexture resource.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/texture.rs#L454)
function alicia.texture.new(path) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/texture.rs#L478)
function alicia.texture.new_from_memory() end

---An unique handle for a render texture in memory.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/texture.rs#L520)
---@class render_texture
---@field shape_x number # Shape of the texture (X).
---@field shape_y number # Shape of the texture (Y).
render_texture = {}

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/texture.rs#L555)
function render_texture:set_R3D() end

---Initialize drawing to the render texture.
---@param call function # The draw code.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/texture.rs#L575)
function render_texture:begin(call) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/texture.rs#L598)
function render_texture:draw_plane() end

---Draw a texture.
---@param point vector_2 # TO-DO
---@param angle number # TO-DO
---@param scale number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/texture.rs#L639)
function render_texture:draw(point,angle,scale,color) end

---Draw a texture (pro).
---@param box_a box_2 # TO-DO
---@param box_b box_2 # TO-DO
---@param point vector_2 # TO-DO
---@param angle number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/texture.rs#L663)
function render_texture:draw_pro(box_a,box_b,point,angle,color) end

---Create a new render texture resource.
---@param shape vector_2 # TO-DO
---@return render_texture render_texture # Render texture resource.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/texture.rs#L697)
function alicia.render_texture.new(shape) end

---The ZIP API.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/zip.rs#L64)
---@class alicia.zip
alicia.zip = {}

---An unique handle to a ZIP in memory.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/zip.rs#L83)
---@class zip
zip = {}

---Get a file from the ZIP file.
---@param path string # Path to the file.
---@param binary boolean # Read as binary.
---@return string | data file # The file.
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/zip.rs#L104)
function zip:get_file(path,binary) end

---Get a list of every file in the ZIP file.
---@return table list # The list of every file.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/zip.rs#L141)
function zip:get_list() end

---Check if the given path is a file.
---@param path string # Path to the file.
---@return boolean value # True if the path is a file, false otherwise.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/zip.rs#L160)
function zip:is_file(path) end

---Check if the given path is a folder.
---@param path string # Path to the folder.
---@return boolean value # True if the path is a folder, false otherwise.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/zip.rs#L180)
function zip:is_path(path) end

---Create a new ZIP resource.
---@param path string # Path to ZIP file.
---@return zip zip # ZIP resource.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/zip.rs#L203)
function alicia.zip.new(path) end

---The model API.
---
--- ---
---*Not available in head-less mode.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/model.rs#L64)
---@class alicia.model
alicia.model = {}

---An unique handle for a model in memory.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/model.rs#L93)
---@class model
---@field mesh_count number # Mesh count.
---@field bone_count number # Bone count.
---@field material_count number # Material count.
model = {}

---Create a new LuaModel resource.
---@param path string # Path to model file.
---@return model model # LuaModel resource.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/model.rs#L137)
function alicia.model.new(path) end

---Bind a texture to the model.
---@param index number # TO-DO
---@param which number # TO-DO
---@param texture texture # Texture to bind to model.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/model.rs#L214)
function model:bind(index,which,texture) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/model.rs#L262)
function model:draw_mesh() end

---Draw the model.
---@param point vector_3 # TO-DO
---@param scale number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/model.rs#L296)
function model:draw(point,scale,color) end

---Draw the model (wire-frame).
---@param point vector_3 # TO-DO
---@param scale number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/model.rs#L331)
function model:draw_wire(point,scale,color) end

---Draw the model with a transformation.
---@param point vector_3 # TO-DO
---@param angle vector_3 # TO-DO
---@param scale vector_3 # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/model.rs#L355)
function model:draw_transform(point,angle,scale,color) end

---TO-DO
---@return number min_x # Minimum vector. (X)
---@return number min_y # Minimum vector. (Y)
---@return number min_z # Minimum vector. (Z)
---@return number max_x # Maximum vector. (X)
---@return number max_y # Maximum vector. (Y)
---@return number max_z # Maximum vector. (Z)
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/model.rs#L404)
function model:get_box_3() end

---Get the vertex data of a specific mesh in the model.
---@param index number # Index of mesh.
---@return table table # Vector3 table.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/model.rs#L429)
function model:mesh_vertex(index) end

---Get the index data of a specific mesh in the model.
---@param index number # Index of mesh.
---@return table table # Number table.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/model.rs#L452)
function model:mesh_index(index) end

---An unique handle for a model animation in memory.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/model.rs#L473)
---@class model_animation
model_animation = {}

---Create a new model animation resource.
---@param path string # Path to model file.
---@return model_animation model_animation # Model animation resource.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/model.rs#L491)
function alicia.model_animation.new(path) end

---Update model with new model animation data.
---@param model model # TO-DO
---@param frame number # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/model.rs#L541)
function model_animation:update(model,frame) end

---The drawing API.
---
--- ---
---*Not available in head-less mode.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L64)
---@class alicia.draw
alicia.draw = {}

---Clear the screen with a color.
---@param color color # The color to use for clearing.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L95)
function alicia.draw.clear(color) end

---Initialize drawing to the screen.
---@param call function # The draw code.
---@param ... any # Variadic data.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L115)
function alicia.draw.begin(call,...) end

---Initialize drawing (blend mode) to the screen.
---@param call function # The draw code.
---@param mode function # The draw code.
---@param ... any # Variadic data.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L145)
function alicia.draw.begin_blend(call,mode,...) end

---Initialize drawing (scissor mode) to the screen.
---@param call function # The draw code.
---@param view box_2 # The clip test region.
---@param ... any # Variadic data.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L174)
function alicia.draw.begin_scissor(call,view,...) end

---The 3D drawing API.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L204)
---@class alicia.draw_3d
alicia.draw_3d = {}

---Initialize the 3D draw mode.
---@param call function # The draw code.
---@param camera camera_3d # The 2D camera.
---@param ... any # Variadic data.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L246)
function alicia.draw_3d.begin(call,camera,...) end

---Get a ray for a 2D screen-space point.
---@param camera camera_3d # The current camera.
---@param point vector_2 # The screen-space point.
---@param shape vector_2 # The size of the view-port.
---@return number position_x # The 3D ray position. (X).
---@return number position_y # The 3D ray position. (Y).
---@return number position_z # The 3D ray position. (Z).
---@return number direction_x # The 3D ray direction. (X).
---@return number direction_y # The 3D ray direction. (Y).
---@return number direction_z # The 3D ray direction. (Z).
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L285)
function alicia.draw_3d.get_screen_to_world(camera,point,shape) end

---Get a 2D screen-space point for a 3D world-space point.
---@param camera camera_3d # The current camera.
---@param point vector_3 # The world-space point.
---@param shape vector_2 # The size of the view-port.
---@return number point_x # The 2D screen-space point (X).
---@return number point_y # The 2D screen-space point (Y).
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L323)
function alicia.draw_3d.get_world_to_screen(camera,point,shape) end

---Draw a line.
---@param point_a vector_3 # The point A of the line.
---@param point_b vector_3 # The point B of the line.
---@param color color # The color of the line.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L350)
function alicia.draw_3d.draw_line(point_a,point_b,color) end

---Draw a point.
---@param point vector_3 # The point.
---@param color color # The color of the point.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L375)
function alicia.draw_3d.draw_point(point,color) end

---Draw a circle.
---@param point vector_3 # The point of the circle.
---@param range number # The range of the circle.
---@param axis vector_3 # The axis of the circle.
---@param range number # The angle of the circle.
---@param color color # The color of the circle.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L399)
function alicia.draw_3d.draw_circle(point,range,axis,range,color) end

---Draw a triangle.
---@param point_a vector_3 # The point (A) of the triangle.
---@param point_b vector_3 # The point (B) of the triangle.
---@param point_c vector_3 # The point (C) of the triangle.
---@param color color # The color of the triangle.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L426)
function alicia.draw_3d.draw_triangle(point_a,point_b,point_c,color) end

---Draw a triangle strip.
---@param point table # The point list of the triangle.
---@param color color # The color of the triangle.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L452)
function alicia.draw_3d.draw_triangle_strip(point,color) end

---Draw a cube.
---@param point vector_3 # The point of the cube.
---@param shape vector_3 # The shape of the cube.
---@param color color # The color of the cube.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L474)
function alicia.draw_3d.draw_cube(point,shape,color) end

---Draw a cube (wire-frame).
---@param point vector_3 # The point of the cube.
---@param shape vector_3 # The shape of the cube.
---@param color color # The color of the cube.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L500)
function alicia.draw_3d.draw_cube_wire(point,shape,color) end

---Draw a sphere.
---@param point vector_3 # The point of the sphere.
---@param color color # The color of the triangle.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L525)
function alicia.draw_3d.draw_sphere(point,color) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L545)
function alicia.draw_3d.draw_sphere_wire() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L565)
function alicia.draw_3d.draw_cylinder() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L593)
function alicia.draw_3d.draw_cylinder_wire() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L621)
function alicia.draw_3d.draw_capsule() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L649)
function alicia.draw_3d.draw_capsule_wire() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L677)
function alicia.draw_3d.draw_plane() end

---Draw a ray.
---@param ray ray # The ray.
---@param color color # The color of the ray.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L701)
function alicia.draw_3d.draw_ray(ray,color) end

---Draw a grid.
---@param slice number # The slice count of the grid.
---@param space number # The space shift of the grid.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L721)
function alicia.draw_3d.draw_grid(slice,space) end

---Draw a 3D box.
---@param shape box_3 # The shape of the ball.
---@param color color # The color of the ball.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L738)
function alicia.draw_3d.draw_box_3(shape,color) end

---The 2D drawing API.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L755)
---@class alicia.draw_2d
alicia.draw_2d = {}

---Initialize the 2D draw mode.
---@param call function # The draw code.
---@param camera camera_2d # The 2D camera.
---@param ... any # Variadic data.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L814)
function alicia.draw_2d.begin(call,camera,...) end

---Get a screen-space point for a 2D world-space point.
---@param camera camera_2d # The current camera.
---@param point vector_2 # The world-space point.
---@return number point_x # The 2D screen-space point (X).
---@return number point_y # The 2D screen-space point (Y).
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L848)
function alicia.draw_2d.get_world_to_screen(camera,point) end

---Get a world-space point for a 2D screen-space point.
---@param camera camera_2d # The current camera.
---@param point vector_2 # The screen-space point.
---@return number point_x # The 2D world-space point (X).
---@return number point_y # The 2D world-space point (Y).
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L877)
function alicia.draw_2d.get_screen_to_world(camera,point) end

---Draw pixel.
---@param point vector_2 # The point of the pixel.
---@param color color # The color of the pixel.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L902)
function alicia.draw_2d.draw_pixel(point,color) end

---Draw a line.
---@param point_a vector_2 # The point A of the line.
---@param point_b vector_2 # The point B of the line.
---@param thick number # The thickness of the line.
---@param color color # The color of the line.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L925)
function alicia.draw_2d.draw_line(point_a,point_b,thick,color) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L946)
function alicia.draw_2d.draw_line_strip() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L963)
function alicia.draw_2d.draw_line_bezier() end

---Draw a circle.
---@param point vector_2 # TO-DO
---@param range number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L989)
function alicia.draw_2d.draw_circle(point,range,color) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L1009)
function alicia.draw_2d.draw_circle_line() end

---Draw the sector of a circle.
---@param point vector_2 # TO-DO
---@param range number # TO-DO
---@param begin_angle number # TO-DO
---@param close_angle number # TO-DO
---@param segment_count number # TO-DO
---@param color color # TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L1037)
function alicia.draw_2d.draw_circle_sector(point,range,begin_angle,close_angle,segment_count,color) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L1064)
function alicia.draw_2d.draw_circle_sector_line() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L1084)
function alicia.draw_2d.draw_circle_gradient() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L1105)
function alicia.draw_2d.draw_ellipse() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L1126)
function alicia.draw_2d.draw_ellipse_line() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L1147)
function alicia.draw_2d.draw_ring() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L1175)
function alicia.draw_2d.draw_ring_line() end

---Draw 2D box.
---@param shape box_2 # The shape of the box.
---@param point vector_2 # The point of the box.
---@param angle number # The angle of the box.
---@param color color # The color of the box.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L1209)
function alicia.draw_2d.draw_box_2(shape,point,angle,color) end

---Draw 2D box with a 4-point gradient.
---@param shape box_2 # The shape of the box.
---@param color_a color # The color A (T.L.) of the box.
---@param color_b color # The color B (B.L.) of the box.
---@param color_c color # The color C (T.R.) of the box.
---@param color_d color # The color D (B.R.) of the box.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L1237)
function alicia.draw_2d.draw_box_2_gradient(shape,color_a,color_b,color_c,color_d) end

---Draw 2D box (out-line).
---@param shape box_2 # The shape of the box.
---@param thick number # The thickness of the box.
---@param color color # The color of the box.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L1271)
function alicia.draw_2d.draw_box_2_line(shape,thick,color) end

---Draw 2D box (round).
---@param shape box_2 # The shape of the box.
---@param round number # The roundness of the box.
---@param count number # The segment count of the box.
---@param color color # The color of the box.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L1297)
function alicia.draw_2d.draw_box_2_round(shape,round,count,color) end

---Draw 2D box (out-line, round).
---@param shape box_2 # The shape of the box.
---@param round number # The roundness of the box.
---@param count number # The segment count of the box.
---@param thick number # The thickness of the box.
---@param color color # The color of the box.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L1324)
function alicia.draw_2d.draw_box_2_line_round(shape,round,count,thick,color) end

---Draw 2D triangle.
---@param point_a vector_2 # The point A of the triangle.
---@param point_b vector_2 # The point B of the triangle.
---@param point_c vector_2 # The point C of the triangle.
---@param color color # The color of the triangle.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L1350)
function alicia.draw_2d.draw_triangle(point_a,point_b,point_c,color) end

---Draw 2D triangle (out-line).
---@param point_a vector_2 # The point A of the triangle.
---@param point_b vector_2 # The point B of the triangle.
---@param point_c vector_2 # The point C of the triangle.
---@param color color # The color of the triangle.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/draw.rs#L1378)
function alicia.draw_2d.draw_triangle_line(point_a,point_b,point_c,color) end

---The data API.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/data.rs#L64)
---@class alicia.data
alicia.data = {}

---An unique handle for a data buffer in memory.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/data.rs#L103)
---@class data
data = {}

---Get the length of the data buffer.
---@return number length # The length of the data buffer.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/data.rs#L157)
function data:get_length() end

---Get the data buffer.
---@return table buffer # The data buffer.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/data.rs#L169)
function data:get_buffer() end

---Get a slice out of the data buffer, as another data buffer.
---@param index_a number # Index A into the data buffer.
---@param index_b number # Index B into the data buffer.
---@return data slice # The slice, as another data buffer.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/data.rs#L185)
function data:get_slice(index_a,index_b) end

---Compress a given data buffer (DEFLATE).
---```lua
----- Create a table with no data in it.
---local data = {}
---
----- Insert some dummy data in it.
---for x = 1, 63 do
---    table.insert(data, 0)
---end
---
----- Insert some data of significance at the last index.
---table.insert(data, 1)
---
----- Check that the last value is 1.
---assert(data[64] == 1)
---
----- Get a data buffer user-data from Alicia.
---local data = alicia.data.new(data)
---
----- Compress the data.
---local data = alicia.data.compress(data)
---
----- Decompress the data.
---local data = alicia.data.decompress(data)
---
----- Get the data back as a Lua table.
---local data = data:get_buffer()
---
----- Check that the last value is 1.
---assert(data[64] == 1)
---
---```
---@param data data # The data buffer to compress.
---@return data data # The data buffer.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/data.rs#L216)
function alicia.data.compress(data) end

---Decompress a given data buffer (DEFLATE).
---```lua
----- Create a table with no data in it.
---local data = {}
---
----- Insert some dummy data in it.
---for x = 1, 63 do
---    table.insert(data, 0)
---end
---
----- Insert some data of significance at the last index.
---table.insert(data, 1)
---
----- Check that the last value is 1.
---assert(data[64] == 1)
---
----- Get a data buffer user-data from Alicia.
---local data = alicia.data.new(data)
---
----- Compress the data.
---local data = alicia.data.compress(data)
---
----- Decompress the data.
---local data = alicia.data.decompress(data)
---
----- Get the data back as a Lua table.
---local data = data:get_buffer()
---
----- Check that the last value is 1.
---assert(data[64] == 1)
---
---```
---@param data data # The data buffer to decompress.
---@return data data # The data buffer.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/data.rs#L242)
function alicia.data.decompress(data) end

---Encode a given data buffer (Base64).
---@param data data # The data buffer to encode.
---@return data data # The data buffer.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/data.rs#L269)
function alicia.data.encode(data) end

---Decode a given data buffer (Base64).
---@param data data # The data buffer to decode.
---@return data data # The data buffer.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/data.rs#L294)
function alicia.data.decode(data) end

---Hash a given data buffer.
---@param data data # The data buffer to encode.
---@param data hash_kind? # OPTIONAL: The hash method. Default: CRC32.
---@return data data # The hash code.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/data.rs#L322)
function alicia.data.hash(data,data) end

---Serialize a given Lua value as another format, in the form of a string.
---@param text any # Lua value to serialize.
---@param kind format_kind? # OPTIONAL: The format to serialize to. Default: JSON.
---@return string value # The value, in string form.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/data.rs#L373)
function alicia.data.serialize(text,kind) end

---Deserialize a given format string as a Lua value.
---@param text string # String to deserialize.
---@param kind format_kind? # OPTIONAL: The format to deserialize from. Default: JSON.
---@return any value # The value, in Lua value form.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/data.rs#L421)
function alicia.data.deserialize(text,kind) end

---Convert a given Lua value to a data buffer.
---@param data any # Lua value to convert to a data buffer.
---@param kind data_kind # The data kind to convert to.
---@return data value # The value, in data buffer form.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/data.rs#L477)
function alicia.data.to_data(data,kind) end

---Convert a given data buffer to a Lua value.
---@param data data # Data buffer to convert to a Lua value.
---@param kind data_kind # The data kind to convert to.
---@return number | string value # The value, in Lua value form.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/data.rs#L508)
function alicia.data.from_data(data,kind) end

---Get a file from the embed file.
---@param path string # Path to the file.
---@param binary boolean # Read as binary.
---@return string | data file # The file.
---
--- ---
---*Available with compile feature: `embed`.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/data.rs#L543)
function alicia.data.get_embed_file(path,binary) end

---Get a list of every file in the embed data.
---@return table list # The list of every file.
---
--- ---
---*Available with compile feature: `embed`.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/data.rs#L572)
function alicia.data.get_embed_list() end

---The socket API.
---
--- ---
---*Not available in head-less mode.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/socket.rs#L65)
---@class alicia.socket
alicia.socket = {}

---An unique handle to a TCP (stream) socket in memory.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/socket.rs#L83)
---@class socket_TCP_stream
socket_TCP_stream = {}

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/socket.rs#L99)
function socket_TCP_stream:get() end

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/socket.rs#L119)
function socket_TCP_stream:set() end

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/socket.rs#L137)
function alicia.socket.new_TCP_stream() end

---An unique handle to a TCP (listen) socket in memory.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/socket.rs#L151)
---@class socket_TCP_listen
socket_TCP_listen = {}

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/socket.rs#L167)
function socket_TCP:accept() end

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/socket.rs#L186)
function alicia.socket.new_TCP_listen() end

---An unique handle to a UDP socket in memory.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/socket.rs#L198)
---@class socket_UDP
socket_UDP = {}

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/socket.rs#L214)
function socket_UDP:connect() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/socket.rs#L227)
function socket_UDP:get() end

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/socket.rs#L247)
function socket_UDP:set() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/socket.rs#L261)
function socket_UDP:get_at() end

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/socket.rs#L281)
function socket_UDP:set_at() end

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/socket.rs#L302)
function alicia.socket.new_UDP() end

---The R3D API.
---
--- ---
---*Not available in head-less mode.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/r3d.rs#L63)
---@class alicia.r3d
alicia.r3d = {}

---An unique handle for a light in the R3D scene.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/r3d.rs#L201)
---@class light_handle
light_handle = {}

---Create a new light handle.
---@param kind number # Light kind.
---@return light_handle light # Light handle.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/r3d.rs#L225)
function alicia.r3d.light.new(kind) end

---Get the kind of the light.
---@return number kind # Light kind.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/r3d.rs#L246)
function light:get_kind() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/r3d.rs#L257)
function light:get_state() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/r3d.rs#L268)
function light:get_color() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/r3d.rs#L280)
function light:get_point() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/r3d.rs#L292)
function light:get_focus() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/r3d.rs#L304)
function light:set_state() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/r3d.rs#L316)
function light:set_color() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/r3d.rs#L328)
function light:set_point() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/r3d.rs#L340)
function light:set_focus() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/r3d.rs#L354)
function light:get_shadow() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/r3d.rs#L365)
function light:get_shadow_update() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/r3d.rs#L379)
function light:set_shadow() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/r3d.rs#L401)
function light:set_shadow_update() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/r3d.rs#L427)
function light:shadow_update() end

---The Discord API.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/discord.rs#L74)
---@class alicia.discord
alicia.discord = {}

---An unique handle to a Discord client.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/discord.rs#L88)
---@class discord
discord = {}

---Set the rich presence.
---@param activity table # The rich presence.
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/discord.rs#L145)
function discord:set_rich_presence(activity) end

---Clear the rich presence.
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/discord.rs#L255)
function discord:clear_rich_presence() end

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/discord.rs#L268)
function discord:update() end

---Create a new Discord client.
---@param ID string # Discord App ID.
---@param name string? # OPTIONAL: The application name.
---@param command number? | string? | table? # OPTIONAL: The application command to run on the rich presence state.
---@return string | data file # The file.
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/discord.rs#L308)
function alicia.discord.new(ID,name,command) end

---The file API.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/file.rs#L66)
---@class alicia.file
alicia.file = {}

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/file.rs#L125)
---@class file_watcher
file_watcher = {}

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/file.rs#L136)
function alicia.file_watcher.new() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/file.rs#L176)
function file_watcher:poll() end

---Get the data of a file.
---```lua
----- Write "123" to the file "foo.txt". Since we don't want to write binary, pass false as the third argument.
---alicia.file.set("work/foo.txt", "123", false)
---
----- Read the data back. Again, since we know the file isn't binary, we pass false.
---local data = alicia.file.get("work/foo.txt", false)
---
---assert(data == "123")
---
---```
---@param path string # Path to file.
---@param binary boolean # Read as binary.
---@return string data # File data.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/file.rs#L209)
function alicia.file.get_file(path,binary) end

---Set the data of a file.
---```lua
----- Write "123" to the file "foo.txt". Since we don't want to write binary, pass false as the third argument.
---alicia.file.set("work/foo.txt", "123", false)
---
----- Read the data back. Again, since we know the file isn't binary, we pass false.
---local data = alicia.file.get("work/foo.txt", false)
---
---assert(data == "123")
---
---```
---@param path string # Path to file.
---@param data string | data # Data to copy.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/file.rs#L248)
function alicia.file.set_file(path,data) end

---Move a file.
---@param source string # The source path.
---@param target string # The target path.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/file.rs#L278)
function alicia.file.move_file(source,target) end

---Copy a file.
---@param source string # The source path.
---@param target string # The target path.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/file.rs#L298)
function alicia.file.copy_file(source,target) end

---Remove a file.
---@param path string # The path to the file to remove.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/file.rs#L317)
function alicia.file.remove_file(path) end

---Remove a folder.
---@param path string # The path to the folder to remove.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/file.rs#L335)
function alicia.file.remove_path(path) end

---Set the state of the path sand-box.
---@param state boolean # The state of the path sand-box.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/file.rs#L353)
function alicia.file.set_path_escape(state) end

---Set the file save call-back.
---@param call function # The call-back. Must accept a file-name and a data parameter, and return a boolean (true on success, false on failure).
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/file.rs#L489)
function alicia.file.set_call_save_file(call) end

---Set the file load call-back.
---@param call function # The call-back. Must accept a file-name, and return a data buffer. Return anything else to indicate failure.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/file.rs#L509)
function alicia.file.set_call_load_file(call) end

---Set the file text save call-back.
---@param call function # The call-back. Must accept a file-name and a string parameter, and return a boolean (true on success, false on failure).
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/file.rs#L529)
function alicia.file.set_call_save_text(call) end

---Set the file load call-back.
---@param call function # The call-back. Must accept a file-name, and return a string. Return anything else to indicate failure.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/file.rs#L549)
function alicia.file.set_call_load_text(call) end

---Check if a file does exist.
---```lua
----- Write "123" to the file "foo.txt". Since we don't want to write binary, pass false as the third argument.
---alicia.file.set("work/foo.txt", "123", false)
---
---assert(alicia.file.get_file_exist("work/foo.txt"))
---
---```
---@param path string # Path to file.
---@return boolean exist # True if file does exist, false otherwise.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/file.rs#L573)
function alicia.file.get_file_exist(path) end

---Check if a path does exist.
---```lua
----- Write "123" to the file "foo.txt". Since we don't want to write binary, pass false as the third argument.
---alicia.file.set("work/foo.txt", "123", false)
---
---assert(alicia.file.get_file_exist("work/foo.txt"))
---
---```
---@param path string # Path.
---@return boolean exist # True if path does exist, false otherwise.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/file.rs#L593)
function alicia.file.get_path_exist(path) end

---Check if a file's extension is the same as a given one.
---@param path string # Path to file.
---@param extension string # Extension. MUST include dot (.png, .wav, etc.).
---@return boolean check # True if file extension is the same as the given one, false otherwise.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/file.rs#L613)
function alicia.file.get_file_extension_check(path,extension) end

---Get the size of a file.
---@param path string # Path to file.
---@return number size # File size.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/file.rs#L634)
function alicia.file.get_file_size(path) end

---Get the extension of a file.
---@param path string # Path to file.
---@return string extension # File extension.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/file.rs#L653)
function alicia.file.get_file_extension(path) end

---Get the name of a file.
---@param path string # Path to file.
---@param extension boolean # File extension. If true, will return file name with the extension.
---@return string name # File name.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/file.rs#L676)
function alicia.file.get_file_name(path,extension) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/file.rs#L697)
function alicia.file.get_absolute_path() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/file.rs#L714)
function alicia.file.get_previous_path() end

---Get the current work path.
---@return string path # Work path.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/file.rs#L734)
function alicia.file.get_work_directory() end

---Get the current application path.
---@return string path # Application path.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/file.rs#L751)
function alicia.file.get_application_directory() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/file.rs#L765)
function alicia.file.create_path() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/file.rs#L787)
function alicia.file.change_path() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/file.rs#L809)
function alicia.file.get_path_file() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/file.rs#L824)
function alicia.file.get_file_name_valid() end

---Scan a path.
---@param path string # Path to scan.
---@param filter string? # OPTIONAL: Extension filter. If filter is 'DIR', will include every directory in the result.
---@param recursive boolean # If true, recursively scan the directory.
---@param absolute boolean # If true, return path relatively.
---@return table list # File list.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/file.rs#L844)
function alicia.file.scan_path(path,filter,recursive,absolute) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/file.rs#L895)
function alicia.file.get_file_drop() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/file.rs#L906)
function alicia.file.get_file_drop_list() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/file.rs#L933)
function alicia.file.get_file_modification() end

---The general API.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/general.rs#L70)
---@class alicia.general
alicia.general = {}

---Get the standard input.
---@return string input # The standard input.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/general.rs#L108)
function alicia.general.standard_input() end

---Load the standard Lua library.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/general.rs#L122)
function alicia.general.load_base() end

---Set the log level.
---@param level number # The log level.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/general.rs#L148)
function alicia.general.set_log_level(level) end

---Open an URL link.
---@param link string # The URL link.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/general.rs#L165)
function alicia.general.open_link(link) end

---Get the current time. Will count up since the initialization of the window.
---@return number time # Current time.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/general.rs#L181)
function alicia.general.get_time() end

---Get the time in UNIX time-stamp format.
---@param add number? # OPTIONAL: Add (or subtract) by this amount.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/general.rs#L194)
function alicia.general.get_time_unix(add) end

---Get the current frame time.
---@return number frame_time # Current frame time.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/general.rs#L211)
function alicia.general.get_frame_time() end

---Get the current frame rate.
---@return number frame_rate # Current frame rate.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/general.rs#L224)
function alicia.general.get_frame_rate() end

---Set the current frame rate.
---@param frame_rate number # Current frame rate.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/general.rs#L237)
function alicia.general.set_frame_rate(frame_rate) end

---Get the argument list.
---@return table list # The list of every argument.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/general.rs#L254)
function alicia.general.get_argument() end

---Get the system info.
---@return table info # The system info.
---
--- ---
---*Available with compile feature: `system_info`.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/general.rs#L271)
function alicia.general.get_system() end

---Get the currently in-use memory by the Lua VM.
---@return number memory # The currently in-use memory.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/general.rs#L289)
function alicia.general.get_memory() end

---Get the current info manifest.
---@return table info # The info manifest.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/general.rs#L304)
function alicia.general.get_info() end

---The collision API.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/collision.rs#L64)
---@class alicia.collision
alicia.collision = {}

---Check if a 2D box is intersecting against another 2D box.
---@param box_a box_2 # 2D box (A).
---@param box_b box_2 # 2D box (B).
---@return boolean intersect # Result of intersection.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/collision.rs#L111)
function alicia.collision.get_box_2_box_2(box_a,box_b) end

---Check if a circle is intersecting against another circle.
---@param point_a vector_2 # Circle point (A).
---@param range_a number # Circle range (A).
---@param point_b vector_2 # Circle point (B).
---@param range_b number # Circle range (B).
---@return boolean intersect # Result of intersection.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/collision.rs#L134)
function alicia.collision.get_circle_circle(point_a,range_a,point_b,range_b) end

---Check if a circle is intersecting against another 2D box.
---@param point_a vector_2 # Circle point.
---@param range_a number # Circle range.
---@param box_a box_2 # 2D box.
---@return boolean intersect # Result of intersection.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/collision.rs#L159)
function alicia.collision.get_circle_box_2(point_a,range_a,box_a) end

---Check if a circle is intersecting against another line.
---@param point_a vector_2 # Circle point.
---@param range_a number # Circle range.
---@param line_a vector_2 # Line point (A).
---@param line_b vector_2 # Line point (B).
---@return boolean intersect # Result of intersection.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/collision.rs#L185)
function alicia.collision.get_circle_line(point_a,range_a,line_a,line_b) end

---Check if a point is intersecting against another 2D box.
---@param point_a vector_2 # Point.
---@param box_a box_2 # 2D box.
---@return boolean intersect # Result of intersection.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/collision.rs#L210)
function alicia.collision.get_point_box_2(point_a,box_a) end

---Check if a point is intersecting against another circle.
---@param point_a vector_2 # Point (A).
---@param point_b vector_2 # Circle point (B).
---@param range_b number # Circle range (B).
---@return boolean intersect # Result of intersection.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/collision.rs#L232)
function alicia.collision.get_point_circle(point_a,point_b,range_b) end

---Check if a point is intersecting against another triangle.
---@param point_a vector_2 # Point (A).
---@param point_b vector_2 # Triangle point (B).
---@param point_c vector_2 # Triangle point (C).
---@param point_d vector_2 # Triangle point (D).
---@return boolean intersect # Result of intersection.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/collision.rs#L258)
function alicia.collision.get_point_triangle(point_a,point_b,point_c,point_d) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/collision.rs#L281)
function alicia.collision.get_point_line() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/collision.rs#L299)
function alicia.collision.get_point_poly() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/collision.rs#L319)
function alicia.collision.get_line_line() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/collision.rs#L345)
function alicia.collision.get_box_2_box_2_difference() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/collision.rs#L367)
function alicia.collision.get_sphere_sphere() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/collision.rs#L384)
function alicia.collision.get_box_3_box_3() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/collision.rs#L398)
function alicia.collision.get_box_3_sphere() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/collision.rs#L415)
function alicia.collision.get_ray_sphere() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/collision.rs#L456)
function alicia.collision.get_ray_box_3() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/collision.rs#L497)
function alicia.collision.get_ray_triangle() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/collision.rs#L540)
function alicia.collision.get_ray_quad() end

---The shader API.
---
--- ---
---*Not available in head-less mode.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/shader.rs#L65)
---@class alicia.shader
alicia.shader = {}

---An unique handle for a shader in memory.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/shader.rs#L80)
---@class shader
shader = {}

---Create a new shader resource.
---@param v_path string # Path to .vs file.
---@param f_path string # Path to .fs file.
---@return shader shader # LuaShader resource.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/shader.rs#L99)
function alicia.shader.new(v_path,f_path) end

---Create a new shader resource, from memory.
---@param v_data string # .vs file data.
---@param f_data string # .fs file data.
---@return shader shader # LuaShader resource.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/shader.rs#L145)
function alicia.shader.new_from_memory(v_data,f_data) end

---Initialize the shader.
---@param call function # The draw code.
---@param ... any # Variadic data.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/shader.rs#L196)
function shader:begin(call,...) end

---Get the location of a shader variable, by name.
---@param name string # Variable name.
---@return number location # Shader variable location.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/shader.rs#L226)
function shader:get_location_name(name) end

---Get the location of a shader attribute, by name.
---@param name string # Attribute name.
---@return number location # Shader attribute location.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/shader.rs#L246)
function shader:get_location_attribute_name(name) end

---Get the location of a shader variable, by index.
---@param location number # Variable index.
---@return number location # Shader variable location.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/shader.rs#L269)
function shader:get_location(location) end

---Set the location of a shader variable.
---@param location number # Variable index.
---@param value number # Variable value.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/shader.rs#L290)
function shader:set_location(location,value) end

---Set the value of a shader variable.
---@param location number # Variable index.
---@param class number # Variable class.
---@param value any # Variable value.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/shader.rs#L317)
function shader:set_shader_value(location,class,value) end

---The image API.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L66)
---@class alicia.image
alicia.image = {}

---An unique handle for a image in memory.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L102)
---@class image
---@field shape_x number # Shape of the image (X).
---@field shape_y number # Shape of the image (Y).
image = {}

---Get a texture resource from an image.
---@return texture texture # Texture resource.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L133)
function image:to_texture() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L144)
function image:power_of_two() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L162)
function image:crop() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L178)
function image:crop_alpha() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L190)
function image:crop_alpha() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L211)
function image:blur_gaussian() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L223)
function image:kernel_convolution() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L238)
function image:resize() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L259)
function image:extend() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L285)
function image:mipmap() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L297)
function image:dither() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L312)
function image:flip() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L328)
function image:rotate() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L342)
function image:color_tint() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L356)
function image:color_invert() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L368)
function image:color_gray_scale() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L380)
function image:color_contrast() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L392)
function image:color_contrast() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L412)
function image:get_alpha_border() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L424)
function image:get_alpha_border() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L438)
function image:draw_pixel() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L453)
function image:draw_line() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L472)
function image:draw_circle() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L490)
function image:draw_circle_line() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L508)
function image:draw_box_2() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L523)
function image:draw_box_2_line() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L541)
function image:draw_triangle() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L571)
function image:draw_triangle_line() end

---Create a new image resource.
---@param path string # Path to image file.
---@return image image # LuaImage resource.
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L603)
function alicia.image.new(path) end

---Create a new image resource, from memory.
---@param data data # The data buffer.
---@param kind string # The kind of image file (.png, etc.).
---@return image image # LuaImage resource.
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L637)
function alicia.image.new_from_memory(data,kind) end

---Create a new image resource, from the current screen buffer.
---@return image image # LuaImage resource.
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L667)
function alicia.image.new_from_screen() end

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L689)
function alicia.image.new_color() end

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L714)
function alicia.image.new_gradient_linear() end

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L744)
function alicia.image.new_gradient_radial() end

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L774)
function alicia.image.new_gradient_square() end

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L804)
function alicia.image.new_check() end

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L841)
function alicia.image.new_white_noise() end

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L865)
function alicia.image.new_perlin_noise() end

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L899)
function alicia.image.new_cellular() end

---TO-DO
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/image.rs#L923)
function alicia.image.new_text() end

---The font API.
---
--- ---
---*Not available in head-less mode.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/font.rs#L66)
---@class alicia.font
alicia.font = {}

---An unique handle to a font in memory.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/font.rs#L92)
---@class font
font = {}

---Draw a font.
---@param label string # Label of font to draw.
---@param point vector_2 # Point of font to draw.
---@param origin vector_2 # TO-DO
---@param angle number # TO-DO
---@param scale number # Scale of font to draw.
---@param space number # Space of font to draw.
---@param color color # Color of font to draw.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/font.rs#L124)
function font:draw(label,point,origin,angle,scale,space,color) end

---Measure the size of a given text on screen, with a given font.
---@param label string # Label of font to measure.
---@param scale number # Scale of font to measure.
---@param space number # Space of font to measure.
---@return number size_x # Size of text (X).
---@return number size_y # Size of text (Y).
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/font.rs#L216)
function font:measure_text(label,scale,space) end

---Create a new font resource.
---@param path string # Path to font file.
---@param size number # Size for font.
---@return font font # LuaFont resource.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/font.rs#L273)
function alicia.font.new(path,size) end

---Create a new font resource, from memory.
---@param data data # The data buffer.
---@param kind string # The kind of font file (.ttf, etc.).
---@param size number # Size for font.
---@return font font # LuaFont resource.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/font.rs#L304)
function alicia.font.new_from_memory(data,kind,size) end

---Create a new font resource, from the default font.
---@param size number # Size for font.
---@return font font # LuaFont resource.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/font.rs#L342)
function alicia.font.new_default(size) end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/font.rs#L373)
function alicia.font.draw_frame_rate() end

---Draw text.
---@param point vector_2 # The point of the text.
---@param label string # The label of the text.
---@param scale number # The angle of the text.
---@param color color # The color of the text.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/font.rs#L395)
function alicia.font.draw_text(point,label,scale,color) end

---Set the vertical space between each line-break.
---@param space number # Vertical space.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/font.rs#L419)
function alicia.font.set_text_line_space(space) end

---The automation API.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/automation.rs#L64)
---@class alicia.automation
alicia.automation = {}

---An unique handle to an automation event list.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/automation.rs#L80)
---@class automation_event
automation_event = {}

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/automation.rs#L100)
function alicia.automation.new() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/automation.rs#L131)
function automation_event:save() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/automation.rs#L146)
function automation_event:set_active() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/automation.rs#L158)
function automation_event:set_frame() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/automation.rs#L170)
function automation_event:start() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/automation.rs#L182)
function automation_event:stop() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/automation.rs#L194)
function automation_event:play() end

---TO-DO
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/automation.rs#L213)
function automation_event:get_event() end

---The window API.
---
--- ---
---*Not available in head-less mode.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L65)
---@class alicia.window
alicia.window = {}

---Create a new native OS file dialog.
---@param kind file_dialog_kind # The kind of file dialog to display.
---@param title string? # OPTIONAL: Window title.
---@param path string? # OPTIONAL: Default file path.
---@param name string? # OPTIONAL: Default file name.
---@param filter table? # OPTIONAL: Extension filter table. Must be in { extension = { ".txt", ".ini" } format.
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L173)
function alicia.window.file_dialog(kind,title,path,name,filter) end

---Create a new native OS text dialog.
---@param kind text_dialog_kind? # OPTIONAL: The kind of text dialog to display.
---@param title string? # OPTIONAL: Window title.
---@param label string? # OPTIONAL: Window label.
---@param button text_dialog_button? # OPTIONAL: Window button layout.
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L267)
function alicia.window.text_dialog(kind,title,label,button) end

---Get if the window should close.
---@return boolean close # True if the window should close.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L317)
function alicia.window.get_close() end

---Get the state of the window (full-screen).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L331)
function alicia.window.get_fullscreen() end

---Get the state of the window (hidden).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L345)
function alicia.window.get_hidden() end

---Get the state of the window (minimize).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L359)
function alicia.window.get_minimize() end

---Get the state of the window (maximize).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L373)
function alicia.window.get_maximize() end

---Get the state of the window (focus).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L387)
function alicia.window.get_focus() end

---Get the state of the window (resize).
---@return boolean state # State of the window.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L401)
function alicia.window.get_resize() end

---Get the state of a window flag.
---@param flag window_flag # Window flag.
---@return boolean state # Window flag state.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L418)
function alicia.window.get_state(flag) end

---Set the state of a window flag.
---@param flag window_flag # Window flag.
---@param state boolean # Window flag state.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L433)
function alicia.window.set_state(flag,state) end

---Set the window to full-screen mode.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L448)
function alicia.window.set_fullscreen() end

---Set the window to border-less mode.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L458)
function alicia.window.set_borderless() end

---Minimize the window.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L468)
function alicia.window.set_minimize() end

---Maximize the window.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L478)
function alicia.window.set_maximize() end

---Restore the window.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L488)
function alicia.window.set_restore() end

---Set the window icon.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L498)
function alicia.window.set_icon() end

---Set the window name.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L517)
function alicia.window.set_name() end

---Set the window point.
---@param point vector_2 # Point of the window.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L536)
function alicia.window.set_point(point) end

---Set the window monitor.
---@param index number # Index of monitor to move window to.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L555)
function alicia.window.set_screen(index) end

---Set the minimum window shape.
---@param shape vector_2 # Minimum shape of the window.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L572)
function alicia.window.set_shape_min(shape) end

---Set the maximum window shape.
---@param shape vector_2 # Maximum shape of the window.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L591)
function alicia.window.set_shape_max(shape) end

---Set the current window shape.
---@param shape vector_2 # Shape of the window.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L610)
function alicia.window.set_shape(shape) end

---Set the window alpha.
---@param alpha number # Alpha of the window.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L629)
function alicia.window.set_alpha(alpha) end

---Focus the window.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L639)
function alicia.window.set_focus() end

---Get the shape of the window.
---@return number shape_x # Shape of the window (X).
---@return number shape_y # Shape of the window (Y).
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L657)
function alicia.window.get_shape() end

---Get the shape of the current render view.
---@return number shape_x # Shape of the render view (X).
---@return number shape_y # Shape of the render view (Y).
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L672)
function alicia.window.get_render_shape() end

---Get the available monitor amount.
---@return number count # Monitor count.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L686)
function alicia.window.get_screen_count() end

---Get the current active monitor, where the window is.
---@return number index # Current active monitor index.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L700)
function alicia.window.get_screen_focus() end

---Get the point of the given monitor.
---@param index number # Index of the monitor.
---@return number point_x # Point of the monitor (X).
---@return number point_y # Point of the monitor (Y).
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L718)
function alicia.window.get_screen_point(index) end

---Get the shape of the given monitor.
---@param index number # Index of the monitor.
---@return number shape_x # Shape of the window (X).
---@return number shape_y # Shape of the window (Y).
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L739)
function alicia.window.get_screen_shape(index) end

---Get the physical shape of the given monitor.
---@param index number # Index of the monitor.
---@return number shape_x # Physical shape of the window (X).
---@return number shape_y # Physical shape of the window (Y).
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L757)
function alicia.window.get_screen_shape_physical(index) end

---Get the refresh rate of the given monitor.
---@param index number # Index of the monitor.
---@return number rate # Refresh rate of the monitor.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L779)
function alicia.window.get_screen_rate(index) end

---Get the point of the window.
---@return number point_x # Point of the window (X).
---@return number point_y # Point of the window (Y).
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L794)
function alicia.window.get_point() end

---Get the DPI scale of the window.
---@return number scale_x # Scale of the window (X).
---@return number scale_y # Scale of the window (Y).
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L813)
function alicia.window.get_scale() end

---Get the name of the given monitor.
---@param index number # Index of the monitor.
---@return string name # Name of the monitor.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L834)
function alicia.window.get_screen_name(index) end

---Get a screen-shot of the current frame.
---@param path string # Path to save the screen-shot to.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/window.rs#L851)
function alicia.window.get_screen_shot(path) end

---The music API.
---
--- ---
---*Not available in head-less mode.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/music.rs#L64)
---@class alicia.music
alicia.music = {}

---An unique handle for music in memory.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/music.rs#L79)
---@class music
music = {}

---Create a new music resource.
---@param path string # Path to music file.
---@return music music # LuaMusic resource.
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/music.rs#L107)
function alicia.music.new(path) end

---Create a new music resource, from memory.
---@param data data # The data buffer.
---@param kind string # The kind of music file (.png, etc.).
---@return music music # LuaMusic resource.
---
--- ---
---*This function is asynchronous and can run within a co-routine.*
---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/music.rs#L140)
function alicia.music.new_from_memory(data,kind) end

---Play the music.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/music.rs#L171)
function music:play() end

---Check if music is currently playing.
---@return boolean state # State of the music.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/music.rs#L186)
function music:get_playing() end

---Stop the music.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/music.rs#L193)
function music:stop() end

---Pause the music.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/music.rs#L201)
function music:pause() end

---Resume the music.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/music.rs#L209)
function music:resume() end

---Set volume for the music. (range: 0.0 - 1.0)
---@param volume number # Current volume.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/music.rs#L224)
function music:set_volume(volume) end

---Set pitch for the music.
---@param pitch number # Current pitch.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/music.rs#L239)
function music:set_pitch(pitch) end

---Set pan for the music. (range: 0.0 - 1.0; 0.5 is center)
---@param pan number # Current pan.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/music.rs#L254)
function music:set_pan(pan) end

---Update the music.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/music.rs#L262)
function music:update() end

---Set position for the music.
---@param position number # Current position.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/music.rs#L277)
function music:set_position(position) end

---Get time length for the music.
---@return number length # Time length.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/music.rs#L292)
function music:get_length() end

---Get time played for the music.
---@return number played # Time played.
---
--- ---
---[Source Code Definition](https://github.com/luxreduxdelux/alicia/tree/main/source/rust/base/music.rs#L306)
function music:get_played() end


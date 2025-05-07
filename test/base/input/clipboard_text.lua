alicia.input.set_clipboard_text("hello, world!")
local text = alicia.input.get_clipboard_text()
assert(text == "hello, world!")

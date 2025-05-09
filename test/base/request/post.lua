-- POST request with a body. We pass false to get the result as a string.
local body = alicia.request.post("http://httpbin.org/post", "hello, body", nil, nil, false)

-- POST request with a form.
local form = alicia.request.post("http://httpbin.org/post", nil, {
    foo = "bar",
    bar = "baz",
}, nil, false)

-- POST request with a JSON.
local JSON = alicia.request.post("http://httpbin.org/post", nil, nil, {
    foo = "bar",
    bar = "baz",
}, false)

-- Deserialize the JSON result to a Lua table.
body = alicia.data.deserialize(body)
form = alicia.data.deserialize(form)
JSON = alicia.data.deserialize(JSON)

-- Assert that the request with a body's data is correct.
assert(body.data, "hello, body")

-- Assert that the request with a form's data is correct.
assert(form.form.foo, "bar")
assert(form.form.bar, "baz")

-- Assert that the request with a JSON's data is correct.
assert(JSON.json.foo, "bar")
assert(JSON.json.bar, "baz")

-- POST request with binary data as the body.
local body = alicia.request.post("http://httpbin.org/post", alicia.data.new({ 255, 0, 255, 0 }), nil, nil, false)

-- Deserialize the JSON result to a Lua table.
body = alicia.data.deserialize(body)

-- Get the Base64 string representation of the data.
body = string.tokenize(body.data, ",")[2]

-- Convert the string representation to a byte representation.
body = alicia.data.to_data(body, 2)

-- Decode the data from Base64.
body = alicia.data.decode(body)

-- Get the data as a Lua table.
body = body:get_buffer()

-- Assert that the data sent is the same as the data we got.
assert(body[1], 255)
assert(body[2], 0)
assert(body[3], 255)
assert(body[4], 0)

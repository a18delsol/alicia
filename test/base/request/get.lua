local link = "https://raw.githubusercontent.com/luxreduxdelux/alicia/refs/heads/main/test/asset/sample.txt"

-- Get the data from the link. As we know it's not binary, we pass false to the function.
local response = alicia.request.get(link, false)

assert(response == "Hello, world!")

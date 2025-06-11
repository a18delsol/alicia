-- Write "123" to the file "foo.txt". Since we don't want to write binary, pass false as the third argument.
alicia.file.set("work/foo.txt", "123", false)

assert(alicia.file.get_file_exist("work/foo.txt"))

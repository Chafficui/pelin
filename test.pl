imp std_file
imp std_io
imp std_comp
imp std_logic

fn nun write_file(str path, str content) {
    std_file.write_file(path, content)
}
write_file("test.txt", "Hello, World!")
std_io.print(std_comp.gt(3, 3))
std_io.print(std_comp.gte(3, 3))
std_io.print(std_comp.lt(3, 3))
std_io.print(std_comp.lte(3, 3))
std_io.print(std_comp.eq(3, 3))
std_io.print(std_comp.neq(3, 3))
std_io.print(std_logic.and(true, false))
std_io.print(std_logic.not(true))
std_io.print(std_logic.not(false))
std_io.print(std_logic.xor(true, false))
std_io.print(std_logic.or(true, false))
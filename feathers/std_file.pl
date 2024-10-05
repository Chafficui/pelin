fn str read_file(str path) {
    RUST[std_func::file_read](path)
}

fn nun write_file(str path, str content) {
    RUST[std_func::file_write](path, content)
}
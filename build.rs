use cc;

fn main() {
    let mut build = cc::Build::new();

    build
        .file("liblmdb/libraries/liblmdb/mdb.c")
        .file("liblmdb/libraries/liblmdb/midl.c")
        .opt_level(3)
        .warnings(false)
        .compile("liblmdb.a");
}

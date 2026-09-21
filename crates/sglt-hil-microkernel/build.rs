fn main() {
    let mut build = cc::Build::new();
    build
        .file("../../kernel/src/shbt_causal_kernel.c")
        .include("../../kernel/include")
        .flag("-O3")
        .flag("-ffreestanding")
        .flag("-D_POSIX_C_SOURCE=199309L")
        .flag("-std=c11");
    build.compile("shbt_causal");
}

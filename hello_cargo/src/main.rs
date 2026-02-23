
fn main() {
    let name = "world";
    // Rustのスタイルは、4スペースでインデントする
    // println!はマクロで、!はマクロを呼び出すために使用する記号。展開すると：
    //     { ::std::io::_print(::std::format_args_nl!("Hello, world!")); }
    // 上記の形になる
    println!("Hello, {name}!");
}

//在实际的项目中，有些包需要编译第三方非Rust代码，例如 C库；有些包需要链接到 C库，当然这些库既可以位于系统上， 也可以从源代码构建。其它的需求则有可能是需要构建代码生成 。 在Cargo中，提供了构建脚本，来满足这些需求。
//
//指定的build命令应执行的Rust文件，将在包编译其它内容之前，被编译和调用，从而具备Rust代码所依赖的构建或生成的组件。Build通常被用来：
//
//构建一个捆绑的C库；
//在主机系统上找到C库；
//生成Rust模块；
//为crate执行所需的某平台特定配置。
// fn say_hello() {
//     println!("hello");
// }
include!(concat!(env!("OUT_DIR"), "/hello.rs"));

fn main() {
    println!("{}", say_hello());
}

请写代码演示下面库的功能 :

linkme 的用法

请搜索资料，阅读理解之后再撰写

创建 rust 项目演示的工程 :

- ./example_array/ 这里放 linkme 的数组
- ./example_link/ 这里放函数
- ./main/ 这里演示调用数组

展示如何使用 linkme 的功能，搞一个跨 crate 的异步函数调用的 HOOK_LI（运行时用 tokio）

代码和注释都必须为全英文,请在各自 README.md 中注明使用的 Rust 版本号，Rust 工具链三元组、操作系统版本等环境信息

安装库请进入目录然后 cargo add，禁止手动编辑 example/Cargo.toml

然后运行 ./test.sh 测试

./test.sh 会进入目录，运行

最后，请按照 ./schema.yml 的格式约束生成 ./issue.yml ，需要包括这个库的介绍，优点等等
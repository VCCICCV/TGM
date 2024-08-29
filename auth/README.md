## Rust多模块

前置知识

* 包`package`：`cargo new`出来的根文件夹就是一个项目包
  * 二进制箱子`binary crate`：`cargo new demo`
    * 一个包可以有一个或多个`crate`
    * `main.rs`是`crate root`
  * 箱子库`library crate`：可以和`binary crate`放在同一个包内，也可以`cargo new --lib demo`一个单独的包
    * 一个包只能有0-1个`crate`
    * `lib.rs`是`crate root`
* 工作空间`workspace`：多个项目联合在一起可以组成工作空间
* 箱子`crate`：提供多个功能，相当于dll或者作为第三方依赖
* 货物`cargo`：包管理工具，要用的第三方依赖就是箱子，功能就是里面的货物

* 关键字`pub`：公开模块或方法，默认是私有的
* 关键字`use`：导入模块到作用域，始终从`crate root`开始
* 关键字`pub use`：导入当前作用域并公开模块或方法
* 关键字`mod`：定义或导入模块
* 关键字`super`：引用父模块，相当于`../`
* 关键字`self`：引用自身模块，调用同一模块的内容
* 关键字`crate`：引用同级模块

## 1、 在Rust 1.30之前使用`mod.rs`来定义模块和嵌套子模块

```rust
cargo new demo
```

`src\util\bar.rs`

```rust
pub fn print_src_bar() {
    println!("bar");
}
```

`src\util\foo.rs`

```rust
pub fn print_src_foo() {
    println!("foo");
}
```

`src\util\mod.rs`在`mod.rs`定义的内容编译器会找同级目录的`bar.rs`或`bar\mod.rs`文件

```rust
// 公开模块识别crate
pub mod bar;
pub mod foo;
```

`main.rs`：crate root

```rust
mod util;

use crate::util::{bar, foo};
fn main() {
    bar::print_src_bar();
    foo::print_src_foo();
}
```

模块树

![image-20240827072042761](C:/Users/Administrator/Desktop/ThreeGorgesMotor/tgm/auth/README.assets/image-20240827072042761-1724717977035-1.png)

执行`cargo run`成功调用方法

````
bar
foo
````

也可以直接在`util\mod.rs`里编写方法，比如路由

```rust
pub mod bar;
pub mod foo;
// 公开模块才能识别到crate和方法
pub mod routes{
    pub fn routes(){
        println!("routes");
    }
}
```

`main.rs`：crate root

```rust
mod util;
use crate::util::{ bar, foo };
use crate::util::routes::routes;
fn main() {
    bar::print_src_bar();
    foo::print_src_foo();
    routes();
}
```

模块树

![image-20240827082224381](C:/Users/Administrator/Desktop/ThreeGorgesMotor/tgm/auth/README.assets/image-20240827082224381.png)

## 2、 在Rust 1.30之后，直接创建子模块，不需要`mod.rs`

```rust
cargo new demo
```

`src\util\bar.rs`

```rust
pub fn print_src_bar() {
    println!("bar");
}
```

`src\util\foo.rs`

```rust
pub fn print_src_foo() {
    println!("foo");
}
```

1. `main.rs`声明``crate`

```rust
mod util{
    pub mod bar;
    pub mod foo;
}
use util::bar;
use util::foo;

fn main() {
    bar::print_src_bar();
    foo::print_src_foo();
    println!("main");
}
```

执行`cargo run`成功调用方法

```
bar
foo
main
```

2. 使用`<folder_name>.rs`

新建一个`src\util.rs`

```rust
pub mod bar;
pub mod foo;
```

`main.rs`

```rust
mod util;
use crate::util::bar;
use crate::util::foo;
fn main() {
    bar::print_src_bar();
    foo::print_src_foo();
    println!("main");
}
```

执行`cargo run`

```
bar
foo
main
```

模块树

![image-20240827082425240](C:/Users/Administrator/Desktop/ThreeGorgesMotor/tgm/auth/README.assets/image-20240827082425240.png)

## Rust多模块应用

* 使用`[workspace]`使子模块依赖于一个`Cargo.toml`，共享一个`target`输出目录

* 使用`cargo new --lib`新建子模块，在根目录`Cargo.toml`添加`[workspace]`来嵌套子模块

创建父项目

```rust
cargo new demo
```

创建子模块

```rust
cargo new --lib application
```

`application\src\util\app_bar.rs`

```rust
pub fn print_app_bar() {
    println!("app_bar");
}
```

`application\src\util\app_foo.rs`

```rust
pub fn print_app_foo() {
    println!("app_foo");
}
```

`application\src\lib.rs`

```rust
pub mod util{
    pub mod app_bar;
    pub mod app_foo;
}
```

如果有依赖要导入到当前模块中使用

```rust
// 在lib.rs添加
pub use sea_orm_migration::prelude::*;
// 在要使用的crate中添加
use sea_orm_migration::prelude::*;
```

在父项目根目录的`Cargo.toml`添加

```toml
[workspace]
members = [".","application"]
[dependencies]
application = {path = "application"}
```

`src\main.rs`

```rust
use application::util::app_bar;
use application::util::app_foo;
fn main() {
    app_bar::print_app_bar();
    app_foo::print_app_foo();
    println!("main");
}
```

执行`cargo run`

```rust
app_bar
app_foo
main
```

新建其他模块同样的套路，`cargo new --lib 模块名`，父项目的`Cargo.toml`会自动添加

```toml
[workspace]
members = [".","application", "模块名"]
```

要使用哪个模块的方法就在哪个模块的`Cargo.toml`添加`[dependencies]`并指定路径`path`，比如实体定义在entity模块中，在`service`的`Cargo.toml`添加

```toml
[dependencies]
entity = { path = "../entity" }
```

## Axum DDD开发（整洁架构）



* `domain`：领域层，包含您的所有实体、值对象和业务逻辑，相关且应一起改变的实体应分组为一个聚合，实体可以利用领域事件将变化传达给系统的其他部分，实体可以定义可用于查询它们的接口（interface，叫约定更合适）,对于可变访问，应该通过 Repository 接口访问实体
  
  * interface：定义接口
  * **services**：领域服务，实现接口（洋葱架构中叫Domain Model，Domain Services）
  * **entities/model**：领域实体，封装整个系统的关键业务逻辑（能被其他部分复用的实体及业务逻辑），既可以是带有方法的对象，也可以是主句结构和函数集合
  * value object：值对象
  * eceptions：错误
  * repository：只定义数据库操作接口，用于数据访问抽象
  
* `application`：应用层，该层控制整个应用程序流程，逻辑必须在此层范围内定义，这一层的变化既不影响业务实体，也不受数据库和框架的影响
  
  * **use_case/service**：定义编排业务流程，用例通常按照 CQRS 分组为命令和查询（洋葱架构中叫Application Services）
  * validators：输入验证相关的类
  * repository：只定义数据库操作接口，用于数据访问抽象
  
* `infrastructure`：基础设施层，数据库、文件、邮件、事件处理等相关代码，实现`use case`定义的接口，依赖于`domain`存在的接口。用于创建数据库连接工厂类，负责初始化数据库连接池，配置连接参数如数据库地址、用户名、密码，**用于**处理连接异常，如连接超时、连接失败等情况，提供重试机制或错误处理策略，**用于**处理事务管理，确保数据操作的原子性、一致性、隔离性和持久性，**用于**管理系统的运行环境配置，如开发环境、测试环境、生产环境的切换

  * `persistence`：持久层，数据库连接的初始化配置和管理代码，如Java中的DAO
    * mysql_repository_impl：数据库具体实现，使用orm就把连接和实现放一起
    * postgresql_repository_imp：数据库具体实现，使用orm就把连接和实现放一起
  * config：连接数据库等的配置
  * cache：缓存
    * redis_connection：编写与 Redis 服务器建立连接的代码
    * redis_repository_impl：实现使用 Redis 的具体数据存储和检索逻辑
  * messaging：实现消息的发送和接收功能
  * file-storage：使用 Minio 的具体文件存储和检索逻辑
  * publisher：发布领域事件

* `adapter`：适配器层，作为基础设施层和应用层之间的桥梁，适配器层主要关注基础设施，将基础设施层提供的功能进行封装和适配，使其更符合业务逻辑的需求。访问和连接过程必须限制在此层中。向`infrastructure`提供接口而不是方法（这一层也可以放在`infrastructure`），还可以**用于**防腐，对外部 API 的返回结果进行适配，**用于**转换为系统内部使用的业务对象格式，处理消息的路由和分发，**用于**连接和访问外部中间件、服务或 API

  * `api/controller`：路由，如Java中的Controller
  * BFF：为了前端的后端，为不同平台适配
  * persistence_adapters：持久层适配器
  * cache-adapter：对 Redis 的操作进行封装和适配，以满足`application`的需求
  * messaging-adapter：对消息队列的操作进行封装和适配，以满足应用层的需求


    * convertor：转换器，将DO领域对象转换为`persistence`最方便操作的格式

* 启动应用应该单独使用一个包或模块：例如`COLA`使用`start`，Rust项目中`src`，依赖于`adapter`

* 父模块应该依赖于所有的包

* 中间件：如果中间件的主要作用是对外部请求或响应进行预处理或后处理，以适配特定的外部系统或接口要求，那么可以将其放在 `adapter `层，如`adapter/middleware`；如果中间件主要是处理与技术实现相关的通用功能，比如日志记录、请求验证、错误处理等，可以将其放在 `infrastructure `层，如`infrastructure/middleware`

**其他目录**

* `migrations` ：数据库sql文件存放的位置，也可以放在infrastructure目录下
* `src`：启动应用的入口
* `test`：测试文件

**依赖关系规则**：不允许让外层圆形中的代码影响到内层圆形的代码

* 源码中的依赖关系必须指向同心圆内层，即底层指向高层，如`infrastructure`依赖于`domain`、`application`
* 内层代码不能引用外层代码中的函数、变量等一切有命名的实体
* 外层代码的数据格式不能不应该被内层代码中使用

**跨越边界**

假设`use case`的代码需要调用`adapter`中的代码，直接调用就违反了依赖规则，需要在`use case`声明一个接口，让`adapter`实现这个接口

## 整洁架构



1. **cargo-watch**：在项目源代码发生变化时自动运行 Cargo 命令

```
# 安装
cargo install cargo-watch
# 运行测试
cargo watch -x test
# 运行项目
cargo watch -x run
# 生成文档
cargo watch -x doc
# 运行基准测试
cargo watch -x bench
# 构建发布版本
cargo watch -x "build --release"
# 组合使用
cargo watch -q -c -w src/ -x run
```

* `cargo watch`：文件发生变化时自动执行某些任务的工具。它可以监视指定的文件或目录，当检测到变化时，执行特定的命令
* `-q`：只显示关键的状态变化和错误信息
* `-c`：在执行命令之前先清理（clean）项目
* `-w src/`：指定要监视的目录为 `src/`
* `-x run`：指定要执行的命令为 `run`，`cargo run` 用于编译并运行项目

2. **watchexec**：文件监视工具
3. **systemfd**：开发过程中保持服务器监听端口



创建项目

```
cargo new auth
```

创建模块

```
cargo new --lib domain
cargo new --lib application
cargo new --lib infrastructure
cargo new --lib adapter
```

添加依赖

```toml
[dependencies]
# web框架
axum = "0.7.5"
# 异步运行时
tokio = { version = "1.39.3", features = ["full"] }
serde = { version = "1.0.127", features = ["derive"] }
[dev-dependencies]
# 错误处理
anyhow = "1.0.86"
```

`main.rs`

```rust
use axum::{
    routing::get,
    Router,
};
// 属性宏，将此函数标记为异步程序的入口点，启动一个异步运行时（Tokio 运行时）来执行这个异步函数
#[tokio::main]
async fn main() {
    // 创建路由实例
    let app = Router::new().route("/hello", get(|| async { "Hello, Axum!"}));
    // 使用hyper监听所有地址的9090端口，.await等待异步完成，绑定成功返回TcpListener实例，失败panic并打印错误信息
    let listener = tokio::net::TcpListener::bind("127.0.0.1:9090").await.unwrap();
    println!("->>LISTENING on {:?}",listener);
    // 使用serve函数启动一个异步服务器，监听TcpListener实例，并使用app作为处理函数
    axum::serve(listener, app).await.unwrap();
}
```

使用`postman`请求`0.0.0.0:9090/hello`可以看到返回了数据

```
Hello, Axum!
```

> 0.0.0.0表示所有ipv4地址，但不能被ping通
>
> 127.0.0.1表示回环地址。所有网络号为127的地址都是回环地址

Rust项目源代码发生变化时自动运行 Cargo 命令

```
# 安装（关闭杀毒软件）
cargo install cargo-watch
```
监听src/目录，更改代码自动重新执行`cargo run`

```
cargo watch -q -c -w src/ -x run
```

监听tests/目录，更改代码自动重新执行`cargo test -q quick_dev -- --nocapture`测试并显示所有输出

```
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```

可以将路由处理函数提取出来

```rust
use axum::{
    response::{Html, IntoResponse}, routing::get, Router
};
#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello", get(hello_handler));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:9090").await.unwrap();
    println!("->>LISTENING on {:?}",listener);
    axum::serve(listener, app).await.unwrap();
}
// 实现了IntoResponse trait，可以返回任何能够转换为 HTTP 响应的类型
async fn hello_handler() ->impl IntoResponse{
    println!("->> {:<12} - {:<30}", "GET /hello", "Handler");
    Html("Hello, World!")
}
```

还可以将路由函数提取出来

```rust
use axum::{
    extract::Path, response::{Html, IntoResponse}, routing::get, Router
};
use serde::Deserialize;
#[derive(Debug,Deserialize)]
struct Helloparams{
    name: Option<String>,
}
#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/hello", get(hello_handler))
    .route("/hello2/:name",get(hello_handler2));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:9090").await.unwrap();
    println!("->>LISTENING on {:?}",listener);
    axum::serve(listener, app).await.unwrap();
}
// 实现了IntoResponse trait，可以返回任何能够转换为 HTTP 响应的类型
async fn hello_handler() ->impl IntoResponse{
    // 左对齐并占用 12 个和 30 个字符宽度的位置
    println!("->> {:<12} - {:<30}", "GET /hello", "Handler");
    Html("Hello, World!")
}
async fn hello_handler2(Path(name):Path<String>) ->impl IntoResponse{
    println!("->> {:<12} - hello_handler2 - {name:?}", "Handler");
    Html(format!("Hello, World!<h1>{name}</h1>"))
}
```

`postman`请求`127.0.0.1:9090/hello2/cci`

```html
Hello, World!<h1>cci</h1>
```




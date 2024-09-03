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
  * BFF：为了前端的后端，为不同平台适配通信协议
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
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("->>LISTENING on {:?}",listener);
    // 使用serve函数启动一个异步服务器，监听TcpListener实例，并使用app作为处理函数
    axum::serve(listener, app).await.unwrap();
}
```

使用`postman`请求`0.0.0.0:8080/hello`可以看到返回了数据

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

> 若要监听tests/目录，更改代码自动重新执行`cargo test -q test_dev -- --nocapture`测试并显示所有输出

```
cargo watch -q -c -w tests/ -x "test -q test_dev -- --nocapture"
```

## `axum::Router`路由


闭包传递路由

> 闭包可以捕获调用者作用域中的值

```rust
use axum::{
    routing::get,
    Router,
    extract::Path,
};
// use tracing::info;
#[tokio::main]
async fn main() {

    let app = Router::new()
    .route("/", get(|| async { "Hello, Rust!" }))
    .route("/hello", get(|| async { "Hello, World!" }))
    .route("/tokio/:name", get(|name:Path<String>| async move{ format!("Hello,{:?}",name) }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

还可以将路由函数提取出来，相同路由可以的不同处理可以通过`.`添加处理器并添加自定义方法，如`.get().post().patch().delete()`

```rust
use axum::{
    routing::get,
    Router,
};
use tokio::net::TcpListener;

// 获取
async fn get_handler() -> String {
    "Hello, world!".to_string()
}
// 创建
async fn post_handler() -> String {
    "post".to_string()
}
// 更新
async fn patch_handler() -> String {
    "update".to_string()
}
// 删除
async fn delete_handler() -> String {
    "delete".to_string()
}
#[tokio::main]
async fn main() {
    
    let app = Router::new()
       .route("/", get(get_handler))
       .route("/hello", get(get_handler).post(post_handler).patch(patch_handler).delete(delete_handler));

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

> **幂等**：一个操作被多次重复执行多次，其结果与第一次执行的结果相同
>
> 同样的请求被执行一次与**连续执行多次**，对服务器的预期**影响是相同的**，那么称这个 HTTP 方法是**幂等的**，如`PUT`、`DELETE`
>
> 所有的**安全**方法都是幂等的，如`GET`、`HEAD`、`OPTIONS`
>
> **安全**：一个 HTTP 方法是`安全`的，是指这是个方法不会修改服务器的数据，即只读的方法，如`GET`、`HEAD`、`OPTIONS`

* **GET（获取资源）**：**请求**资源
  * GET 请求是安全、幂等的
* **POST（创建资源）**：通常用于向服务器提交数据以**创建**新的资源
  * POST 请求是不安全、不幂等的
  * POST 请求的主体可以包含任意格式的数据，例如表单数据、JSON 或 XML
* **PUT（更新资源）**：更新服务器上的现有资源，客户端将**完整的资源**表示发送到服务器，服务器用这个表示**替换**现有的资源
  * PUT请求是不安全、幂等的
* **PATCH（部分更新资源）**： PATCH 只需要提供**资源的部分**，服务器只更新指定的部分
  * PATCH请求是不安全、不幂等的
* **DELETE（删除资源）**：删除指定的资源
  * DELETE请求是不安全、幂等的
* **HEAD（获取资源头信息）**：只返回资源的头部信息，不返回资源的主体内容，用于检查资源的存在性、获取资源的大小、最后修改时间等信息，而不需要下载整个资源
* **OPTIONS（获取服务器支持的方法）**：获取服务器支持的 HTTP 方法和其他选项信息，客户端发送 OPTIONS 请求以了解服务器对特定资源的支持情况
  * OPTIONS请求的响应通常包含一个`Allow`头部，列出服务器支持的方法
  * OPTIONS 请求可以用于客户端在发送实际请求之前了解服务器的能力和限制

## 路由匹配

`:`创建**动态路由**，可作为传递的值，必须有值才能匹配到

* `/hello/:id`匹配`/hello/12`
* `/:id/hello`匹配`/12/hello`

`*`创建**通配符路由**，

* `/hello/*file`匹配`/hello/sssssssfile`
* **特殊**`/hello/*key`不匹配`/hello/`但会匹配`/hello/`下的所有路由，如`/hello/cci/cci/cci/`

多个参数传递使用`axum::extract::Path`提取

> 路径只包含一个参数时，可以省略元组

```rust
use axum::{
    extract::Path,
    routing::get,
    Router,
};
use tokio::net::TcpListener;

// 获取
async fn get_handler() -> String {
    "Hello, world!".to_string()
}
async fn show_user(Path((user_id,team_id)):Path<(String,String)>)-> String {
    format!("{}_{}", user_id, team_id)
}
#[tokio::main]
async fn main() {
    
    let app = Router::new()
       .route("/", get(get_handler))
       .route("/users/:user_id/team/:team_id", get(show_user));
        
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

## fallback 后备路由

**fallback**：后备，备用方案或回退机制，主要的操作或功能无法正常执行时，程序可以使用预先定义的 fallback来处理情况，以确保程序不会完全失败或崩溃

后背路由仅适用于路由中任何内容均不匹配的路由，创建路由后使用`.fallback()`添加后备路由

> **callback**：区分回调函数

### nest 嵌套路由

将路由嵌套在另一个路由下，例如将用户相关的路由嵌套在`users`下，请求路径必须包含`users`

* 嵌套路由不会看到原始请求 URI，而是会删除匹配的前缀
* 使用原始URI请使用`axum::extract::OriginalUri`
* 嵌套路由和通配符路由功能类似，嵌套路由会删除前缀，通配符路由保留完整路由

```rust
use axum::{
    extract::Path,
    routing::{get,post},
    Router,
};
use tokio::net::TcpListener;
async fn show_user(Path(id): Path<String>) -> String {
    format!("id: {:?}", id)
}
async fn post_user() -> String {
    "post_user".to_string()
}
#[tokio::main]
async fn main() {
    let user_routes = Router::new().route("/:id", get(show_user));
    let team_routes = Router::new().route("/", post(post_user));

    let api_routes = Router::new()
    .nest("/users", user_routes)// GET /api/users/145632
    .nest("/teams", team_routes);// POST /api/teams/
    let app = Router::new().nest("/api", api_routes);
        
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

如果嵌套路由**没有自己的后备**，那么将**继承外部路由的后备**，以下例子

* 当请求`:8080/api/users/145632`时可以匹配成功
* 当请求`:8080/test`时，由于没有路由可以匹配，会执行`/api`定义的后备
* 当请求`:8080/api/users/`时，由于没有路由可以匹配，会执行`/users`定义的后备
* 当请求`:8080/api/users/145632/test`时，由于没有路由可以匹配，`/:id`路由没有定义后备路由，会执行外部的`/users`定义的后备

```rust
use axum::{
    http::Uri,
    extract::OriginalUri,
    routing::get,
    Router,
    http::StatusCode
};
use tokio::net::TcpListener;
async fn show_user(uri: Uri, OriginalUri(original_uri): OriginalUri) -> String {
    format!("uri: {:?}\noriginal_uri: {:?}\n", uri,original_uri)
}
async fn fallback_api() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not Found /api")
}
async fn fallback_users() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not Found /users")
}

#[tokio::main]
async fn main() {
    let user_routes = Router::new().route("/:id", get(show_user));

    let api_routes = Router::new()
    .nest("/users", user_routes).fallback(fallback_users);

    let app = Router::new().nest("/api", api_routes).fallback(fallback_api);
        
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

## `axum::extract::OriginalUri`

获取原始URI

```rust
use axum::{
    http::Uri,
    extract::OriginalUri,
    routing::{get,post},
    Router,
};
use tokio::net::TcpListener;
async fn show_user(uri: Uri, OriginalUri(original_uri): OriginalUri) -> String {
    //uri: /145632
    //original_uri: /api/users/145632
    format!("uri: {:?}\noriginal_uri: {:?}\n", uri,original_uri)
}
async fn post_user() -> String {
    "post_user".to_string()
}
#[tokio::main]
async fn main() {
    let user_routes = Router::new().route("/:id", get(show_user));
    let team_routes = Router::new().route("/", post(post_user));

    let api_routes = Router::new()
    .nest("/users", user_routes)// GET \api\users\145632
    .nest("/teams", team_routes);// POST \api\teams\
    let app = Router::new().nest("/api", api_routes);
        
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```



使用`Router::with_state`嵌套路由和不同状态

```
```



嵌套路由和后备路由



### merge 合并路由

将多个独立的路由**组合**到一起统一处理，例如两个模块中定义了路由，在主应用中合并为一个路由

```rust
use axum::{
    routing::get,
    Router,
    extract::Path,
};
use tokio::net::TcpListener;
async fn users_list() -> String {
    "users list".to_string()
}
async fn users_show(Path(id):Path<String>) -> String {
    format!("user show: {:?}",id)
}
async fn teams_list() -> String {
    "teams list".to_string()
}
#[tokio::main]
async fn main() {

    let user_routes = Router::new()
        .route("/users", get(users_list)) // GET :8080/users
        .route("/users/:id", get(users_show)); // GET :8080/users/145632

    let team_routes = Router::new()
        .route("/teams", get(teams_list)); // GET :8080/teams

    let app = Router::new()
        .merge(user_routes)
        .merge(team_routes);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

合并路由和状态

 

合并路由和后背路由

## `tower_service::Service`服务路由

用于编写模块化和重复使用的应用，例如`RPC`远程过程调用

* 请求服务时立即返回一个`Future`，代表未来将会完成的任务，此时主线程继续执行，**poll轮询**直到某个时刻处理完成，返回结果或错误
* 

## 路由函数



## Responses响应

任何实现了 [`IntoResponse`](https://docs.rs/axum/latest/axum/response/trait.IntoResponse.html)的类型都可以从处理函数中返回响应体

```rust
use axum::{
    routing::get,
    response::IntoResponse,
    http::{StatusCode, HeaderMap, Uri, header},
};

// `(StatusCode, impl IntoResponse)` 覆盖响应的状态码
async fn with_status(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("Not Found: {}", uri.path()))
}

// 使用`impl IntoResponse` 避免输入类型推导
async fn impl_trait(uri: Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("Not Found: {}", uri.path()))
}

// `(HeaderMap, impl IntoResponse)`添加额外的报头
async fn with_headers() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/plain".parse().unwrap());
    (headers, "foo")
}

// 使用元组添加标头
async fn with_array_headers() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "text/plain")], "foo")
}

// 使用字符串作为报头
async fn with_array_headers_custom() -> impl IntoResponse {
    ([("x-custom", "custom")], "foo")
}

// `(StatusCode, headers, impl IntoResponse)` 设置响应码和报头
// `headers` 报头也可以是Map
async fn with_status_and_array_headers() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        [(header::CONTENT_TYPE, "text/plain")],
        "foo",
    )
}

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
       .route("/with_status", get(with_status))
       .route("/impl_trait", get(impl_trait))
       .route("/with_headers", get(with_headers))
       .route("/with_array_headers", get(with_array_headers))
       .route("/with_array_headers_custom", get(with_array_headers_custom))
       .route("/with_status_and_array_headersm", get(with_status_and_array_headers));

       let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
       axum::serve(listener, app).await.unwrap();
}
```

## Error handling 错误处理

`axum`基于`tower`服务，该服务通过其关联的错误类型捆绑错误。如果您的服务产生错误并且导致该错误一直传到`hyper`，则连接将在不发送响应的情况下终止。这通常是不可取的，因此`axum`确保您始终通过依赖类型系统来生成响应

`axum`通过要求所有服务将`Infallible`作为其错误类型，`Invalliable`是指永远不会发生的错误的错误类型

## anyhow 处理错误

```rust
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
       .route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("->> listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
// 发生错误返回 AppError
async fn handler() -> Result<(), AppError> {
    try_thing()?;
    Ok(())
}
// 模拟一个错误
fn try_thing() -> Result<(), anyhow::Error> {
    anyhow::bail!("it failed!")
}

// 编写自定义的错误来包装Error
struct AppError(anyhow::Error);

// 告诉axum如何将AppError转换为响应体
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// 允许用 ? 处理`Result<_, anyhow::Error>` 减少手动操作
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
```

## tracing 分布式链路追踪

### 日志管理

通常，我们会将应用分为多个服务部署到多台服务器上，一旦其中一台服务器发生问题，查看日志非常麻烦，通过tracing将所有日志集中管理，无需SSH登录到每个节点去查看日志

日志应该包含哪些信息：

```
时间 严重级别 请求ID 用户ID 应用ID
```

核心概念：跨度、事件、订阅者

* [tracing-appender](https://crates.io/crates/tracing-appender)：提供了一个执行非阻塞写入的订阅者
* [tracing-futures](https://crates.io/crates/tracing-futures)：提供与 async/await 的兼容性
* [tracing-subscriber](https://crates.io/crates/tracing-subscriber)：提供了一些辅助函数来构建订阅者
* [tracing-bunyan-formatter](https://crates.io/crates/tracing-bunyan-formatter)：将Bunyan格式的日志转换为JSON
* [tracing-log](https://crates.io/crates/tracing-log)：日志处理库，可以将日志转发给订阅者

**Spans**：跨度，在特定环境中执行的时间段，当程序进入某一个服务中的上下文执行任务时进入跨度，停止执行时退出跨度；线程当前执行的范围称为当前线程的跨度

**Event**：事件，

**Collector**：收集器，当`Span`开始/结束或`Event`发生时，他们的记录会被`Collector`收集，`tracing-subscriber`就是一个`Collector`

```toml
[dependencies]
# 分布式追踪SDK
tracing = "0.1.40"
# 日志过滤器
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

基本示例

```rust
use axum::{
    routing::get,
    Router,
};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
#[tokio::main]
async fn main() {
    // 只有注册 subscriber 订阅者后， 才能在控制台上看到日志输出
    tracing_subscriber::registry()
        .with(fmt::layer())
        .init();

    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    // 调用 `tracing` 包的 `info!`
    tracing::info!("🚀 listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
```

**#[instrument]**将函数标记为`span`，`tracing `会自动为函数创建一个 `span`，`span `名跟函数名相同

```rust
use axum::{ routing::get, Router };
use tracing::{info, instrument};
use tracing_subscriber::{ fmt, layer::SubscriberExt, util::SubscriberInitExt };
#[instrument]
async fn hello()-> String{
    info!("hello tracing");
    "hello".to_string()
}
#[tokio::main]
async fn main() {
    // 只有注册 subscriber 后， 才能在控制台上看到日志输出
    tracing_subscriber::registry().with(fmt::layer()).init();

    let app = Router::new().route("/",get(hello));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    // 调用 `tracing` 包的 `info!`
    info!("🚀 listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
```

访问`:8080/`可以看到日志

```bash
2024-09-03T14:31:14.694933Z  INFO auth: 🚀 listening on 0.0.0.0:8080
2024-09-03T14:31:16.826323Z  INFO hello: auth: hello tracing
```

`tracing::Level`指定跨度的级别

> level：ERROR、WARN、DEBUG、INFO、TRACE或者1-5，级别从低到高

```rust
#[instrument(level = Level::DEBUG)]
async fn hello()-> String{
    info!("hello tracing");
    "hello".to_string()
}
```

`name`覆盖默认创建的`span`名字

```rust
#[instrument(name = "my_name")]
async fn hello()-> String{
    info!("hello tracing");
    "hello".to_string()
}
```

* `target`：覆盖生成的跨度的目标
* `parent`：覆盖生成的跨度的父级
* `follows_from`：覆盖生成的跨度跟随关系
* `skip`：跳过记录参数
* `fields`：向跨度添加其他上下文
* `ret`：函数返回时发出带有函数返回值的事件
* `err`：覆盖事件的级别，事件的级别默认为`ERROR`

```rust
#[instrument(err(level = Level::INFO))]
```

**in_scope**将不能使用`#[instrument]`的函数或第三方库包裹进`span`



根目录创建配置文件`.cargo/config.toml`



## Middleware 中间件

axum可以在任何地方添加中间件

* 路由中间件`Router::layer`)和的整个路由器`Router::route_layer`
* 路由方法`MethodRouter::layer`或[`Handler::layer`
* 单个处理函数`Handler::layer`

### Router::layer

`Router::layer`添加的中间件将在路由后运行，不能用于重写URI的中间件，可以使用`Router::route_layer`重写URI



### Router::route_layer

### MethodRouter::layer

## Sharing state with handlers 处理程序共享状态

如连接数据库的状态需要与其他服务共享

- 使用 `State` 提取器
- 使用请求扩展
- 使用闭包捕获

1. 使用 `State` 提取器

2. 使用请求扩展

3. 使用闭包捕获

## Building integrations for axum 为axum构建集成

如果你要编写`FromRequest` 、`FromRequestParts` 或 `IntoResponse`相关的库，应当依赖于`axum-core`而不是`axum`

## Required dependencies 所需的依赖项

使用`axum`需要引入



```
[dependencies]
# web框架
axum = "0.7.5"
# 提供了一些额外的功能，如处理特定类型的 HTTP 头
axum-extra = { version = "0.9.3", features = ["typed-header"] }
# session
async-session = "3.0.0"
# oauth2
oauth2 = "4.1"
# 错误处理
anyhow = "1.0.86"
# 提供了静态文件服务
tower-http = { version = "0.5", features = ["fs"] }
# cookie
tower-cookies = "0.10"
# 异步运行时
tokio = { version = "1.40.0", features = ["full"] }
```

##  `axum::handler` 处理器

`handler`是一个异步函数，接受0个或多个`extract`提取器作为参数，并且可以转换为响应的内容

* 注意`handler`是`axum`提供的处理路由，像`list_handler`这种是自定义的处理函数

##  `axum::extract` 提取器

从请求中提取数据的类型的`trait`，提取器是实现了`FromRequest`或 `FromRequestParts`的类型

* 提取器总是按照函数参数的顺序从左到右运行

> 注意使用POST请求

```rust
use axum::{
    extract::{Request, Json, Path, Extension, Query},
    routing::{get, post},
    http::header::HeaderMap,
    body::Bytes,
    Router,
    response::IntoResponse
};
use serde_json::Value;
use std::collections::HashMap;

// `Path`提取路由参数
async fn path(Path(user_id): Path<u32>) -> impl IntoResponse {
    format!("User ID: {}", user_id)
}

// `Query`提取参数并序列化 
async fn query(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    format!("Query parameters: {:?}", params)
}

// `HeaderMap`提取请求头
async fn headers(headers: HeaderMap) -> impl IntoResponse {
    format!("Headers: {:?}", headers)
}

// `String`只消费UTF-8字符串
async fn string(body: String) -> impl IntoResponse {
    format!("String body: {}", body)
}

// `Bytes`提供原始请求正文
async fn bytes(body: Bytes) -> impl IntoResponse {
    format!("Bytes body: {:?}", body)
}

// 将正文解析为json
async fn json(Json(payload): Json<Value>) -> impl IntoResponse {
    format!("JSON payload: {:?}", payload)
}

// `Request`可以控制整个请求内容
async fn request(request: Request) -> impl IntoResponse {
    let method = request.method();
    let uri = request.uri();
    format!("Request method: {}, URI: {}", method, uri)
}

// `Extension`从"request extensions"中提取数据，通常用于共享程序状态
async fn extension(Extension(state): Extension<State>) -> impl IntoResponse {
    format!("State: {:?}", state)
}

#[derive(Clone,Debug)]
struct State {
}

async fn handler() -> impl IntoResponse {
    "Hello, world!"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
       .route("/path/:user_id", post(path))
       .route("/query", post(query))
       .route("/headers", post(headers))
       .route("/string", post(string))
       .route("/bytes", post(bytes))
       .route("/json", post(json))
       .route("/request", post(request))
       .route("/extension", post(extension))
       .route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("->> listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
```



## `axum::response`

生成`types`类型和`trait`特征

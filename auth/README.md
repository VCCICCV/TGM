
```toml
[dependencies]
application = { path = "application" }
# migration ={ path = "migration" }
# web框架
axum = "0.7.5"
# 提供了一些额外的功能，如处理特定类型的 HTTP 头
axum-extra = { version = "0.9.3", features = ["typed-header"] }
# orm
sea-orm = { version = "1.0.1", features = [ "sqlx-postgres", "runtime-tokio-rustls", "macros" ] }
# session
async-session = "3.0.0"
# 读取toml文件
toml = "0.8.19"
# oauth2
# oauth2 = "4.1"
# 错误处理
anyhow = "1.0.86"
# 提供了静态文件服务
tower-http = { version = "0.5", features = ["fs"] }
# cookie
tower-cookies = "0.10"
# 异步运行时
tokio = { version = "1.40.0", features = ["full"] }
# 序列化和反序列化数据
serde = { version = "1.0.127", features = ["derive"] }
# 序列化JSON
serde_json = "1.0.128"
# 序列化时间
serde_with = "3.8.2"
# 分布式跟踪的 SDK，用于采集监控数据，这里用其日志功能
tracing = "0.1.40"
tracing-error = "0.2.0"
# 日志过滤器
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
# 正则表达式
lazy-regex = "3"

strum_macros = "0.26.4"
uuid = { version = "1", features = ["v4", "fast-rng"] }
# 以声明式或程序式的方式创建命令行解析器
clap = "4.5.17"
# HTTP客户端
reqwest = { version = "0.12.7", default-features = false, features = [
    "rustls-tls",
    "json",
] }
# 定义了 HTTP 请求和响应的相关类型
http = "1.0.0"
[dev-dependencies]
# 测试
httpc-test = "0.1.1"
```

## Axum DDD开发（整洁架构）

* `domain`：领域层，包含您的所有实体、值对象和业务逻辑，相关且应一起改变的实体应分组为一个聚合，实体可以利用领域事件将变化传达给系统的其他部分，实体可以定义可用于查询它们的接口（interface，叫约定更合适）,对于可变访问，应该通过 Repository 接口访问实体
  * interface：定义接口
  * **services**：领域服务，实现接口（洋葱架构中叫Domain Model，Domain Services），当业务逻辑不能自然地归属于某个实体时，可以创建领域服务
  * **entities/model**：领域实体，代表业务领域核心概念的实体类，封装整个系统的关键业务逻辑（能被其他部分复用的实体及业务逻辑），既可以是带有方法的对象，也可以是主句结构和函数集合
  * value object：不可变的值对象，如地址，值对象可以增强代码的可读性和可维护性，避免在多个地方重复相同的逻辑
  * event：领域事件，如`OrderPlaced`（订单已下单）、`ProductUpdated`（商品已更新）
  * eceptions：错误
  * repository：只定义数据库操作接口，用于数据访问抽象
* `application`：应用层，该层控制整个应用程序流程，逻辑必须在此层范围内定义，这一层的变化既不影响业务实体，也不受数据库和框架的影响
  * **use_case/service**：定义编排业务流程，组合实体的功能，用例通常按照 CQRS 分组为命令和查询（洋葱架构中叫Application Services）
  * validators：输入验证相关的类
  * repository：只定义数据库操作接口，用于数据访问抽象
  * interface_adapters：适配器层，作为基础设施层和应用层之间的桥梁，适配器层主要关注基础设施，将基础设施层提供的功能进行封装和适配，使其更符合业务逻辑的需求。访问和连接过程必须限制在此层中。向`infrastructure`提供接口而不是方法（这一层也可以放在`infrastructure`），还可以**用于**防腐，对外部 API 的返回结果进行适配，**用于**转换为系统内部使用的业务对象格式，处理消息的路由和分发，**用于**连接和访问外部中间件、服务或 API
    *
    * BFF：为了前端的后端，为不同平台适配通信协议
    * persistence_adapters：持久层适配器
    * cache-adapter：对 Redis 的操作进行封装和适配，以满足`application`的需求
    * messaging-adapter：对消息队列的操作进行封装和适配，以满足应用层的需求
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

* `Interface`：
  * `api/controller`：路由，如Java中的Controller

convertor：转换器，将DO领域对象转换为`persistence`最方便操作的格式

* 启动应用应该单独使用一个包或模块：例如`COLA`使用`start`，Rust项目中`src`，依赖于`adapter`

* 父模块应该依赖于所有的包

* 中间件：如果中间件的主要作用是对外部请求或响应进行预处理或后处理，以适配特定的外部系统或接口要求，那么可以将其放在 `adapter`层，如`adapter/middleware`；如果中间件主要是处理与技术实现相关的通用功能，比如日志记录、请求验证、错误处理等，可以将其放在 `infrastructure`层，如`infrastructure/middleware`

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
cargo new --lib interface
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



## `tower_service::Service`服务路由

用于编写模块化和重复使用的应用，例如`RPC`远程过程调用

* 请求服务时立即返回一个`Future`，代表未来将会完成的任务，此时主线程继续执行，**poll轮询**直到某个时刻处理完成，返回结果或错误

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

## Axum整合sea-orm

添加crate

```toml
sea-orm = { version = "1.0.1", features = [ "sqlx-postgres", "runtime-tokio-rustls", "macros" ] }
```

安装cli

```bash
cargo install sea-orm-cli
```

初始化，将会生成`migration`文件夹

> 迁移可以在开发时随时回滚到数据库初始状态，也可以让其他人通过运行命令来初始化一样的开发环境，还可以保留不同版本的数据库设计，随时初始化不同版本的数据库，避免手动导入sql、手动删除开发测试数据

```bash
sea-orm-cli migrate init
```

将`migration`添加到工作区，根目录`Cargo.toml`

```toml
[workspace]
members = [".","migration"]
```

取消驱动的注释`migration\Cargo.toml`

```toml
[dependencies.sea-orm-migration]
version = "1.0.0"
features = [
  # Enable at least one `ASYNC_RUNTIME` and `DATABASE_DRIVER` feature if you want to run migration via CLI.
  # View the list of supported features at https://www.sea-ql.org/SeaORM/docs/install-and-config/database-and-async-runtime.
  # e.g.
  "runtime-tokio-rustls",  # `ASYNC_RUNTIME` feature
  "sqlx-postgres",         # `DATABASE_DRIVER` feature
]
```

添加或修改你需要的字段，这里定义了一个`Post`表，`id`、`title`、`text`三个字段

```rust
#[derive(DeriveIden)]
enum Post {
    Table,// 存储表名但不成为数据库字段
    Id,
    Title,
    #[sea_orm(iden = "full_text")] // 重命名数据库里的字段名
    Text,
}
```

编写建表命令，所有类型和约束都已函数的方式添加

> `todo!()`用于提示未完成的部分，使用`todo!()`会panic，迁移前请删除

```rust
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]//允许为自定义结构体实现异步MigrationTrait
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()// 建表
                    .table(Post::Table)// 表名
                    .if_not_exists()// 不存在则创建
                    .col(pk_auto(Post::Id))// 主键自增
                    .col(string(Post::Title))// string类型的Title
                    .col(string(Post::Text))// string类型的Text
                    .to_owned(),// 创建一个拥有所有权的副本
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}
```

> up应用 down回滚

`SeaQuery`其他定义<https://www.sea-ql.org/SeaORM/docs/migration/writing-migration/>

还可以创建多个表

```rust
use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建用户表
        manager.create_table(
            Table::create()
                .table(User::Table)
                .if_not_exists()
                .col(ColumnDef::new(User::Id).integer().not_null().auto_increment().primary_key())
                .col(ColumnDef::new(User::Username).string().not_null())
                .col(ColumnDef::new(User::Email).string().not_null().unique_key())
                .to_owned()
        ).await?;
        // 创建订单表
        manager.create_table(
            Table::create()
                .table(Order::Table)
                .if_not_exists()
                .col(ColumnDef::new(Order::Id).integer().not_null().auto_increment().primary_key())
                .col(ColumnDef::new(Order::UserId).integer().not_null())
                .col(ColumnDef::new(Order::TotalPrice).decimal().not_null())
                .to_owned()
        ).await?;
        // 创建产品表
        manager.create_table(
            Table::create()
                .table(Product::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Product::Id).integer().not_null().auto_increment().primary_key()
                )
                .col(ColumnDef::new(Product::Name).string().not_null())
                .col(ColumnDef::new(Product::Price).decimal().not_null())
                .to_owned()
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 删除产品表
        manager.drop_table(Table::drop().table(Product::Table).to_owned()).await?;
        // 删除订单表
        manager.drop_table(Table::drop().table(Order::Table).to_owned()).await?;
        // 删除用户表
        manager.drop_table(Table::drop().table(User::Table).to_owned()).await?;
        Ok(())
    }
}
#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Username,
    Email,
}
#[derive(DeriveIden)]
enum Order {
    Table,
    Id,
    UserId,
    TotalPrice,
}
#[derive(DeriveIden)]
enum Product {
    Table,
    Id,
    Name,
    Price,
}
```

在根目录创建`.env`文件，添加以下内容，修改`username:password@host`

```.env
DATABASE_URL=postgres://username:password@host:5432/database
# 例如
DATABASE_URL=postgres://postgres:root123456@localhost:5432/postgres
```

## 运行迁移

迁移将在 Postgres 中以原子方式执行，失败则回滚，MySQL 和 SQLite 不支持原子迁移

* 迁移，除了会创建自定义的表外，还有一个`seaql_migrations`的版本信息表

> 其他目录通过`-d` 来指定`sea-orm-cli migrate COMMAND -d ./other/migration/dir`

```
sea-orm-cli migrate up
```

* 回滚

```
sea-orm-cli migrate down
```

* 检查迁移的状态

```
sea-orm-cli migrate status
```

* 删除**所有表**重新迁移

> 会删除整个数据库的表，不仅仅是迁移定义的表

```
sea-orm-cli migrate fresh
```

* 回滚所有已应用的迁移，然后重新应用所有迁移

```
sea-orm-cli migrate refresh
```

* 回滚所有已应用的迁移

```
sea-orm-cli migrate reset
```

## 创建实体

指定在`entity/src`下创建实体

```
sea-orm-cli generate entity -o entity/src
```

常用参数

* `-o`指定输出目录

* `--with-serde`序列化与反序列化，指定值`none`、`serialize`、`deserialize`、`both`，默认`none`
  * `--serde-skip-deserializing-primary-key`生成主键字段标记为的实体模型`#[serde(skip_deserializing)]`
  * `--serde-skip-hidden-column`：生成带有隐藏列（列名以 开头`_`）字段的实体模型，标记为`#[serde(skip)]`

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

**Event**：事件，某一个时间点发生了什么事

**Collector**：收集器，当`Span`开始/结束或`Event`发生时，他们的记录会被`Collector`收集，`tracing-subscriber`就是一个`Collector`

```toml
[dependencies]
# 分布式追踪SDK
tracing = "0.1.40"
# 日志过滤器
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
tower-http = { version = "0.5", features = ["trace"] }
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

还可以在路由添加

```rust
use axum::{ routing::get, Router };
use tracing::info;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{ fmt, layer::SubscriberExt,util::SubscriberInitExt};
async fn hello()-> String{
    info!("hello tracing");
    "hello".to_string()
}

#[tokio::main]
async fn main() {
    // 只有注册 subscriber 后， 才能在控制台上看到日志输出
    tracing_subscriber::registry().with(fmt::layer()).init();

    let app = Router::new().route("/",get(hello)).layer(TraceLayer::new_for_http());

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

还可以在`.env`文件指定级别`RUST_LOG=trace cargo run`

> ERROR、WARN、DEBUG、INFO、TRACE或者1-5，级别从低到高

**#[instrument]**将函数标记为`span`，`tracing`会自动为函数创建一个 `span`，`span`名跟函数名相同

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

* 使用 `State` 提取器
* 使用请求扩展
* 使用闭包捕获

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

## `axum::handler` 处理器

`handler`是一个异步函数，接受0个或多个`extract`提取器作为参数，并且可以转换为响应的内容

* 注意`handler`是`axum`提供的处理路由，像`list_handler`这种是自定义的处理函数

## `axum::extract` 提取器

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






## RBAC

RBAC（Role-Based Access Control）：基于角色的访问控制，将权限分配给角色，再将角色分配给用户

* Permission：权限
* Role：角色
* Assignment：分配
* User：用户

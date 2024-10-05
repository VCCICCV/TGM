

## MGR（motor getting rusty，锈化动力）

## 项目简介

`MGR`是一个基于DDD整洁架构设计的web3D购车商城，采用Java17、Spring Cloud 2023 & Alibaba、Spring Boot 3.2.7、Mybatis、OAuth2、Elasticsearch、Containerd、Kubernetes等技术

## tgm-front

过滤生产日期

## 后台管理

* 首页
  * 订单数
  * 营销额
  * 商品总览
  * 订单统计
* 商品
  * 商品列表
  * 添加商品
  * 商品分类
  * 商品类型
  * 品牌管理
* 订单
  * 订单管理
  * 订单设置
  * 退货申请处理
  * 退货原因设置
* 营销
  * 优惠券列表
  * 
* 权限
  * 用户列表
  * 角色列表
  * 菜单列表
  * 资源列表

## 数据库

* SKU (Stock Keeping Unit) ：**库存量单位**，是物理上不可分割的最小存货单元，用于库春管理，SKU通常表示：规格、颜色、款式
  * 黑色、500公里续航的电车是一个SKU
  * 银色、800公里续航的电车是一个SKU
* item：**单品**，展示和销售的基本单位，商品条目或单个商品，每一个具体的商品都是一个 item，用于订单管理和库存跟踪
* SPU (Standard Product Unit) ：**标准产品单元**，是商品聚合的最小单元，是一组**可复用、易检索**的**标准化信息的集合**，是一组具有共同属性的商品的集合
  * 电车是一个SPU
  * 油车是一个SPU
  * 机油是一个SPU
1. 商品
    编号，名称，描述，型号，分类，售价，库存，状态
2. 
3. 订单
4. 营销
5. 权限

dsdsdssadasdsadsa
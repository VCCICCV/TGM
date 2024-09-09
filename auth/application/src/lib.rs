/*
 * @Author: cci
 * @Date: 2024-08-27 02:37:18
 * @LastEditors: cci
 * @LastEditTime: 2024-09-07 22:37:27
 * @Description: service/use case 层 用于处理业务逻辑，组合业务逻辑，调用领域层，调用基础设施层，调用其他服务层
 * 
 * Copyright (c) 2024 by TGM All Rights Reserved. 
 */
pub mod common{
    pub mod error;
    pub mod res;
}

pub mod service{
    pub mod user_service;
    pub mod user_service_trait;
}

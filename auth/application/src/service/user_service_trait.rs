use domain::model::user::{SysUserSearchReq, UserWithDept};
use crate::common::res::{ListData, PageParams};

pub trait UserServiceTrait {
    async fn get_users_list(&self, page_params: PageParams, req: SysUserSearchReq) -> Result<ListData<UserWithDept>, String>;
}
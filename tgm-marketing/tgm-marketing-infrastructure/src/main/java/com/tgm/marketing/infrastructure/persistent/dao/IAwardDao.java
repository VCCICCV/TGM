package com.tgm.marketing.infrastructure.persistent.dao;


import com.tgm.marketing.infrastructure.persistent.po.Award;
import org.apache.ibatis.annotations.Mapper;


import java.util.List;

/**
 * @author cci
 * @description 奖品表DAO
 * @create 2023-12-16 13:23
 */
@Mapper
public interface IAwardDao {

    List<Award> queryAwardList();

    String queryAwardConfigByAwardId(Integer awardId);

    String queryAwardKeyByAwardId(Integer awardId);

}

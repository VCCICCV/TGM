package com.tgm.marketing.domain.strategy.service;

import com.tgm.marketing.domain.strategy.model.entity.RaffleAwardEntity;
import com.tgm.marketing.domain.strategy.model.entity.RaffleFactorEntity;

/**
 * @PROJECT_NAME tgm
 * @AUTHOR VCCICCV
 * @DATE 2024/2/6 上午2:31
 * @DESCRIPTION
 */
public interface IRaffleStrategy {
    /**
     * 执行抽奖；用抽奖因子入参，执行抽奖计算，返回奖品信息
     *
     * @param raffleFactorEntity 抽奖因子实体对象，根据入参信息计算抽奖结果
     * @return 抽奖的奖品
     */
    RaffleAwardEntity performRaffle(RaffleFactorEntity raffleFactorEntity);
}

package com.tgm.marketing.domain.strategy.service.armory;

/**
 * @PROJECT_NAME tgm
 * @AUTHOR VCCICCV
 * @DATE 2024/8/3 上午7:15
 * @DESCRIPTION
 */
public interface IStrategyArmory {
    /**
     * 装配抽奖策略配置「触发的时机可以为活动审核通过后进行调用」
     *
     * @param strategyId 策略ID
     * @return 装配结果
     */
    boolean assembleLotteryStrategy(Long strategyId);
}

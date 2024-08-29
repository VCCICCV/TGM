package com.tgm.marketing.domain.strategy.repository;

import com.tgm.marketing.domain.strategy.model.entity.StrategyAwardEntity;
import com.tgm.marketing.domain.strategy.model.entity.StrategyEntity;
import com.tgm.marketing.domain.strategy.model.entity.StrategyRuleEntity;

import java.math.BigDecimal;
import java.util.Date;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

/**
 * @PROJECT_NAME tgm
 * @AUTHOR VCCICCV
 * @DATE 2024/8/3 下午11:16
 * @DESCRIPTION 策略仓租接口
 */
public interface IStrategyRepository {
    //1、查询策略配置
    List<StrategyAwardEntity> queryStrategyAwardList(Long strategyId);
    void storeStrategyAwardSearchRateTable(String key, Integer rateRange, Map<Integer, Integer> strategyAwardSearchRateTable);
    int getRateRange(Long strategyId);
    int getRateRange(String key);

    Integer getStrategyAwardAssemble(String key, int rateKey);

    StrategyEntity queryStrategyEntityByStrategyId(Long strategyId);

    StrategyRuleEntity queryStrategyRule(Long strategyId, String ruleModel);

    Boolean subtractionAwardStock(String cacheKey, Date endDateTime);

    void cacheStrategyAwardCount(String cacheKey, Integer awardCount);

    String queryStrategyRuleValue(Long strategyId, Integer awardId, String ruleModel);
}

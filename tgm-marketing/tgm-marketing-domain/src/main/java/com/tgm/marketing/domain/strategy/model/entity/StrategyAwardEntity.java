package com.tgm.marketing.domain.strategy.model.entity;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

import java.math.BigDecimal;

/**
 * @PROJECT_NAME tgm
 * @AUTHOR VCCICCV
 * @DATE 2024/8/3 下午11:28
 * @DESCRIPTION 策略奖品实体
 */
@Data
@Builder
@AllArgsConstructor
@NoArgsConstructor
public class StrategyAwardEntity {
    /**
     * 自增ID
     */
    private Long id;
    /**
     * 抽奖策略ID
     */
    private Long strategyId;
    /**
     * 抽奖奖品ID - 内部流转使用
     */
    private Integer awardId;
    /**
     * 奖品库存总量
     */
    private Integer awardCount;
    /**
     * 奖品库存剩余
     */
    private Integer awardCountSurplus;
    /**
     * 奖品中奖概率
     */
    private BigDecimal awardRate;
    // /** 排序 */
    // private Integer sort;
    // /** 规则模型，rule配置的模型同步到此表，便于使用 */
    // private String ruleModels;
}

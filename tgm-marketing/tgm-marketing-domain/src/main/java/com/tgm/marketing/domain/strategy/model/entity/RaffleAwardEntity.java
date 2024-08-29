package com.tgm.marketing.domain.strategy.model.entity;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

/**
 * @PROJECT_NAME tgm
 * @AUTHOR VCCICCV
 * @DATE 2024/1/6 上午2:28
 * @DESCRIPTION 抽奖奖品
 */
@Builder
@Data
@AllArgsConstructor
@NoArgsConstructor
public class RaffleAwardEntity {
    /**
     * 抽奖策略ID
     */
    private Long strategyId;
    /**
     * 抽奖奖品ID【规则类型为策略，则不需要奖品ID】
     */
    private Integer awardId;
    /**
     * 奖品对接标识，对于发奖策略
     */
    private String awardKey;
    /**
     * 奖品配置信息
     */
    private String awardConfig;
    /**
     * 抽奖规则描述
     */
    private String ruleDesc;
    // /** 奖品ID */
    // private Integer awardId;
    // /** 抽奖奖品标题 */
    // private String awardTitle;
    // /** 奖品配置信息 */
    // private String awardConfig;
    // /** 奖品顺序号 */
    // private Integer sort;
}

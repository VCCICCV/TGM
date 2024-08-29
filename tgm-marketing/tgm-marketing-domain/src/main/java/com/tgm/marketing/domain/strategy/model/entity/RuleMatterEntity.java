package com.tgm.marketing.domain.strategy.model.entity;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

/**
 * @PROJECT_NAME tgm
 * @AUTHOR VCCICCV
 * @DATE 2024/2/6 上午2:29
 * @DESCRIPTION 规则物料实体
 */
@Builder
@Data
@AllArgsConstructor
@NoArgsConstructor
public class RuleMatterEntity {
    /** 用户ID */
    private String userId;
    /** 策略ID */
    private Long strategyId;
    /** 抽奖奖品ID【规则类型为策略，则不需要奖品ID】 */
    private Integer awardId;
    /** 抽奖规则类型【rule_random - 随机值计算、rule_lock - 抽奖几次后解锁、rule_luck_award - 幸运奖(兜底奖品)】 */
    private String ruleModel;

}

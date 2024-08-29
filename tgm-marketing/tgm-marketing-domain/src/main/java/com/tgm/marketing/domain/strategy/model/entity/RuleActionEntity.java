package com.tgm.marketing.domain.strategy.model.entity;

import com.tgm.marketing.domain.strategy.model.valobj.RuleLogicCheckTypeVO;
import lombok.*;

/**
 * @PROJECT_NAME tgm
 * @AUTHOR VCCICCV
 * @DATE 2024/1/6 上午2:28
 * @DESCRIPTION 规则动作实体
 */
@Builder
@Data
@AllArgsConstructor
@NoArgsConstructor
public class RuleActionEntity<T> {
    private String code = RuleLogicCheckTypeVO.ALLOW.getCode();
    private String info = RuleLogicCheckTypeVO.ALLOW.getInfo();
    private String ruleModel;
    private T data;

    static public class RaffleEntity {

    }

    // 抽奖之前
    @EqualsAndHashCode (callSuper = true)
    @Data
    @Builder
    @AllArgsConstructor
    @NoArgsConstructor
    static public class RaffleBeforeEntity extends RaffleEntity {
        /**
         * 策略ID
         */
        private Long strategyId;

        /**
         * 权重值Key；用于抽奖时可以选择权重抽奖。
         */
        private String ruleWeightValueKey;

        /**
         * 奖品ID；
         */
        private Integer awardId;
    }

    // 抽奖之中
    static public class RaffleCenterEntity extends RaffleEntity {

    }

    // 抽奖之后
    static public class RaffleAfterEntity extends RaffleEntity {

    }
}

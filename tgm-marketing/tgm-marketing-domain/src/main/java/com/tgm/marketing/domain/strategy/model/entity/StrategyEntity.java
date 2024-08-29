package com.tgm.marketing.domain.strategy.model.entity;

import com.tgm.marketing.types.common.Constants;
import io.micrometer.common.util.StringUtils;
import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

/**
 * @PROJECT_NAME tgm
 * @AUTHOR VCCICCV
 * @DATE 2024/8/5 下午10:42
 * @DESCRIPTION 策略实体
 */
@Builder
@Data
@AllArgsConstructor
@NoArgsConstructor
public class StrategyEntity {
    /**
     * 抽奖策略ID
     */
    private Long strategyId;
    /**
     * 抽奖策略描述
     */
    private String strategyDesc;
    /**
     * 抽奖规则模型
     */
    private String ruleModels;
    public String[] ruleModels() {
        if (StringUtils.isBlank(ruleModels)) return null;
        return ruleModels.split(Constants.SPLIT);
    }

    public String getRuleWeight() {
        String[] ruleModels = this.ruleModels();
        if (null == ruleModels) return null;
        for (String ruleModel : ruleModels) {
            if ("rule_weight".equals(ruleModel)) return ruleModel;
        }
        return null;
    }
}

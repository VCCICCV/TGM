package com.tgm.marketing.domain.strategy.service.rule;

import com.tgm.marketing.domain.strategy.model.entity.RuleActionEntity;
import com.tgm.marketing.domain.strategy.model.entity.RuleMatterEntity;

/**
 * @PROJECT_NAME tgm
 * @AUTHOR VCCICCV
 * @DATE 2024/2/6 上午2:30
 * @DESCRIPTION
 */
public interface ILogicFilter<T extends RuleActionEntity.RaffleEntity> {
    RuleActionEntity<T> filter(RuleMatterEntity ruleMatterEntity);
}

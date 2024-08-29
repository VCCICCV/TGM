package com.tgm.marketing.domain.strategy.service.rule.impl;

import com.tgm.marketing.domain.strategy.model.entity.RuleActionEntity;
import com.tgm.marketing.domain.strategy.model.entity.RuleMatterEntity;
import com.tgm.marketing.domain.strategy.model.valobj.RuleLogicCheckTypeVO;
import com.tgm.marketing.domain.strategy.repository.IStrategyRepository;
import com.tgm.marketing.domain.strategy.service.annotation.LogicStrategy;
import com.tgm.marketing.domain.strategy.service.rule.ILogicFilter;
import com.tgm.marketing.domain.strategy.service.rule.factory.DefaultLogicFactory;
import com.tgm.marketing.types.common.Constants;
import jakarta.annotation.Resource;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Component;

/**
 * @PROJECT_NAME tgm
 * @AUTHOR VCCICCV
 * @DATE 2024/2/6 上午2:31
 * @DESCRIPTION 规则黑名单过滤
 */
@Slf4j
@Component
@LogicStrategy (logicMode = DefaultLogicFactory.LogicModel.RULE_BLACKLIST)
public class RuleBackListLogicFilter implements ILogicFilter<RuleActionEntity.RaffleBeforeEntity> {
    @Resource
    private IStrategyRepository repository;

    @Override
    public RuleActionEntity<RuleActionEntity.RaffleBeforeEntity> filter(RuleMatterEntity ruleMatterEntity) {
        log.info("规则过滤-黑名单 userId:{} strategyId:{} ruleModel:{}", ruleMatterEntity.getUserId(), ruleMatterEntity.getStrategyId(), ruleMatterEntity.getRuleModel());

        String userId = ruleMatterEntity.getUserId();

        // 查询规则值配置
        String ruleValue = repository.queryStrategyRuleValue(ruleMatterEntity.getStrategyId(), ruleMatterEntity.getAwardId(), ruleMatterEntity.getRuleModel());
        String[] splitRuleValue = ruleValue.split(Constants.COLON);
        Integer awardId = Integer.parseInt(splitRuleValue[0]);

        // 过滤其他规则
        String[] userBlackIds = splitRuleValue[1].split(Constants.SPLIT);
        for (String userBlackId : userBlackIds) {
            if (userId.equals(userBlackId)) {
                return RuleActionEntity.<RuleActionEntity.RaffleBeforeEntity>builder()
                        .ruleModel(DefaultLogicFactory.LogicModel.RULE_BLACKLIST.getCode())
                        .data(RuleActionEntity.RaffleBeforeEntity.builder()
                                .strategyId(ruleMatterEntity.getStrategyId())
                                .awardId(awardId)
                                .build())
                        .code(RuleLogicCheckTypeVO.TAKE_OVER.getCode())
                        .info(RuleLogicCheckTypeVO.TAKE_OVER.getInfo())
                        .build();
            }
        }

        return RuleActionEntity.<RuleActionEntity.RaffleBeforeEntity>builder()
                .code(RuleLogicCheckTypeVO.ALLOW.getCode())
                .info(RuleLogicCheckTypeVO.ALLOW.getInfo())
                .build();
    }
}

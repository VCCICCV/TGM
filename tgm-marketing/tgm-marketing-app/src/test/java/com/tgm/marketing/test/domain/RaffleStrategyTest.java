package com.tgm.marketing.test.domain;

import com.alibaba.fastjson.JSON;
import com.tgm.marketing.domain.strategy.model.entity.RaffleAwardEntity;
import com.tgm.marketing.domain.strategy.model.entity.RaffleFactorEntity;
import com.tgm.marketing.domain.strategy.service.IRaffleStrategy;
import com.tgm.marketing.domain.strategy.service.rule.impl.RuleWeightLogicFilter;
import jakarta.annotation.Resource;
import lombok.extern.slf4j.Slf4j;
import org.junit.Before;
import org.junit.jupiter.api.Test;
import org.junit.runner.RunWith;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.test.context.junit4.SpringRunner;
import org.springframework.test.util.ReflectionTestUtils;

/**
 * @PROJECT_NAME tgm
 * @AUTHOR VCCICCV
 * @DATE 2024/1/6 上午2:40
 * @DESCRIPTION
 */
@Slf4j
@RunWith (SpringRunner.class)
@SpringBootTest
public class RaffleStrategyTest {
    @Resource
    private IRaffleStrategy raffleStrategy;
    @Resource
    private RuleWeightLogicFilter ruleWeightLogicFilter;
    /**
     * 权重测试
     */
    @Before
    public void setUp() {
        ReflectionTestUtils.setField(ruleWeightLogicFilter, "userScore", 40500L);
    }
    @Test
    public void test_performRaffle() {
        RaffleFactorEntity raffleFactorEntity = RaffleFactorEntity.builder()
                .userId("xiaofuge")
                .strategyId(100001L)
                .build();
        RaffleAwardEntity raffleAwardEntity = raffleStrategy.performRaffle(raffleFactorEntity);
        log.info("请求参数：{}", JSON.toJSONString(raffleFactorEntity));
        log.info("测试结果：{}", JSON.toJSONString(raffleAwardEntity));
    }
    /**
     * 黑名单测试
     */
    @Test
    public void test_performRaffle_blacklist() {
        RaffleFactorEntity raffleFactorEntity = RaffleFactorEntity.builder()
                .userId("user003")  // 黑名单用户 user001,user002,user003
                .strategyId(100001L)
                .build();
        RaffleAwardEntity raffleAwardEntity = raffleStrategy.performRaffle(raffleFactorEntity);
        log.info("请求参数：{}", JSON.toJSONString(raffleFactorEntity));
        log.info("测试结果：{}", JSON.toJSONString(raffleAwardEntity));
    }
}

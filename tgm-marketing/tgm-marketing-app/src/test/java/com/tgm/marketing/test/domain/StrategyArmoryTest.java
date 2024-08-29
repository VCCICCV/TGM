package com.tgm.marketing.test.domain;

import com.tgm.marketing.domain.strategy.service.armory.IStrategyArmory;
import com.tgm.marketing.domain.strategy.service.armory.IStrategyDispatch;
import jakarta.annotation.Resource;
import lombok.extern.slf4j.Slf4j;
import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.test.context.junit4.SpringRunner;

/**
 * @PROJECT_NAME tgm
 * @AUTHOR VCCICCV
 * @DATE 2024/8/4 上午6:25
 * @DESCRIPTION 策略装配测试
 */
@Slf4j
@RunWith (SpringRunner.class)
@SpringBootTest
public class StrategyArmoryTest {
    @Resource
    private IStrategyDispatch strategyDispatch;
    @Resource
    private IStrategyArmory strategyArmory;
    @Before
    public void test_strategyDispatch() {
        boolean success = strategyArmory.assembleLotteryStrategy(100001L);
        log.info("测试结果：{} - 奖品id值", success);
    }
    @Test
    public void test_StrategyArmory() {
        strategyArmory.assembleLotteryStrategy(100001L);

    }

    /**
     * 抽奖测试
     */
    @Test
    public void test_getRandomAwardId() {
        log.info("测试结果：{} - 奖品ID值", strategyDispatch.getRandomAwardId(100001L));
    }


    public void test_getRandomAwardId_ruleWeightValue() {
        log.info("测试结果：{} - 4000 策略配置", strategyDispatch.getRandomAwardId(100001L, "4000:102,103,104,105"));
        log.info("测试结果：{} - 5000 策略配置", strategyDispatch.getRandomAwardId(100001L, "5000:102,103,104,105,106,107"));
        log.info("测试结果：{} - 6000 策略配置", strategyDispatch.getRandomAwardId(100001L, "6000:102,103,104,105,106,107,108,109"));
    }

}

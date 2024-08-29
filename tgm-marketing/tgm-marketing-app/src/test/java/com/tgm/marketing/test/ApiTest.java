package com.tgm.marketing.test;

import com.tgm.marketing.infrastructure.persistent.redis.IRedisService;
import jakarta.annotation.Resource;
import lombok.extern.slf4j.Slf4j;
import org.junit.Test;
import org.junit.runner.RunWith;
import org.redisson.api.RMap;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.test.context.junit4.SpringRunner;

/**
 * @PROJECT_NAME tgm
 * @AUTHOR VCCICCV
 * @DATE 2024/8/3 上午7:02
 * @DESCRIPTION
 */
@Slf4j
@RunWith (SpringRunner.class)
@SpringBootTest
public class ApiTest {
    @Resource
    private IRedisService redisService;
    @Test
    public void test(){
        RMap<Object,Object> map = redisService.getMap("strategy_id_100001");
        map.put(1,101);
        map.put(1,102);
        map.put(1,103);
        map.put(1,104);
        map.put(1,105);
        map.put(4,106);
        map.put(2,107);
        map.put(1,108);
        log.info("测试结果：{}",redisService.getFromMap("strategy_id_100001",1).toString());

    }
}

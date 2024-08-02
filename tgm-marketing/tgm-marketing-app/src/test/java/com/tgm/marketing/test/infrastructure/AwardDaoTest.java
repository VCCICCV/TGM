package com.tgm.marketing.test.infrastructure;


import com.alibaba.fastjson2.JSON;
import com.tgm.marketing.infrastructure.persistent.dao.IAwardDao;
import com.tgm.marketing.infrastructure.persistent.po.Award;
import jakarta.annotation.Resource;
import lombok.extern.slf4j.Slf4j;
import org.junit.jupiter.api.Test;
import org.junit.runner.RunWith;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.test.context.junit4.SpringRunner;
import java.util.List;

/**
 * @author cci
 * Description  奖品持久化单元测试
 * Create  2023-12-16 13:36
 */
@Slf4j
@RunWith (SpringRunner.class)
@SpringBootTest
class AwardDaoTest {
    @Resource
    private IAwardDao awardDao;

    @Test
    public void test_queryAwardList() {
        List<Award> awards = awardDao.queryAwardList();
        log.info("测试结果：{}", JSON.toJSONString(awards));
    }

}

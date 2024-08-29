package com.tgm.marketing.infrastructure.persistent.repository;

import com.tgm.marketing.domain.strategy.model.entity.StrategyAwardEntity;
import com.tgm.marketing.domain.strategy.model.entity.StrategyEntity;
import com.tgm.marketing.domain.strategy.model.entity.StrategyRuleEntity;
import com.tgm.marketing.domain.strategy.repository.IStrategyRepository;
import com.tgm.marketing.infrastructure.persistent.dao.IStrategyAwardDao;
import com.tgm.marketing.infrastructure.persistent.dao.IStrategyDao;
import com.tgm.marketing.infrastructure.persistent.dao.IStrategyRuleDao;
import com.tgm.marketing.infrastructure.persistent.po.Strategy;
import com.tgm.marketing.infrastructure.persistent.po.StrategyAward;
import com.tgm.marketing.infrastructure.persistent.po.StrategyRule;
import com.tgm.marketing.infrastructure.persistent.redis.IRedisService;
import com.tgm.marketing.types.common.Constants;
import com.tgm.marketing.types.exception.AppException;
import jakarta.annotation.Resource;
import org.springframework.stereotype.Repository;
import lombok.extern.slf4j.Slf4j;
import java.util.ArrayList;
import java.util.Date;
import java.util.List;
import java.util.Map;
import java.util.concurrent.TimeUnit;

import static com.tgm.marketing.types.enums.ResponseCode.UN_ASSEMBLED_STRATEGY_ARMORY;

/**
 * @PROJECT_NAME tgm
 * @AUTHOR VCCICCV
 * @DATE 2024/8/3 下午11:22
 * @DESCRIPTION 策略仓储实现
 */
@Slf4j
@Repository
public class StrategyRepository implements IStrategyRepository {
    // @Resource
    // private IRaffleActivityDao raffleActivityDao;
    @Resource
    private IStrategyDao strategyDao;
    @Resource
    private IStrategyRuleDao strategyRuleDao;
    @Resource
    private IStrategyAwardDao strategyAwardDao;
    // @Resource
    // private IRaffleActivityAccountDao raffleActivityAccountDao;
    // @Resource
    // private IRaffleActivityAccountDayDao raffleActivityAccountDayDao;
    @Resource
    private IRedisService redisService;
    // @Resource
    // private IRuleTreeDao ruleTreeDao;
    // @Resource
    // private IRuleTreeNodeDao ruleTreeNodeDao;
    // @Resource
    // private IRuleTreeNodeLineDao ruleTreeNodeLineDao;

    @Override
    public List<StrategyAwardEntity> queryStrategyAwardList(Long strategyId) {
        // 优先从缓存获取
        String cacheKey = Constants.RedisKey.STRATEGY_AWARD_KEY + strategyId;
        List<StrategyAwardEntity> strategyAwardEntities = redisService.getValue(cacheKey);
        if (null != strategyAwardEntities && !strategyAwardEntities.isEmpty()) return strategyAwardEntities;
        // 从库中获取数据
        List<StrategyAward> strategyAwards = strategyAwardDao.queryStrategyAwardListByStrategyId(strategyId);
        strategyAwardEntities = new ArrayList<>(strategyAwards.size());
        for (StrategyAward strategyAward : strategyAwards) {
            StrategyAwardEntity strategyAwardEntity = StrategyAwardEntity.builder()
                    .strategyId(strategyAward.getStrategyId())
                    .awardId(strategyAward.getAwardId())
                    .awardCount(strategyAward.getAwardCount())
                    .awardCountSurplus(strategyAward.getAwardCountSurplus())
                    .awardRate(strategyAward.getAwardRate())
                    .build();
            strategyAwardEntities.add(strategyAwardEntity);
        }
        redisService.setValue(cacheKey, strategyAwardEntities);
        return strategyAwardEntities;
    }

    /**
     * 在 Redisson 中，当你调用 getMap 方法时，如果指定的 key 不存在，Redisson 并不会立即在 Redis 数据库中创建这个 key。相反，它会返回一个 RMap 对象的实例，这个实例是一个本地的 Java 对象，它代表了 Redis 中的一个哈希（hash）。
     * <p>
     * 当你开始使用这个 RMap 实例进行操作，比如添加键值对，那么 Redisson 会在 Redis 数据库中创建相应的 key，并将数据存储在这个 key 对应的哈希中。如果你只是获取了 RMap 实例而没有进行任何操作，那么在 Redis 数据库中是不会有任何变化的。
     * <p>
     * Map 方法返回的 RMap 对象是懒加载的，只有在你实际进行操作时，Redis 数据库中的数据结构才会被创建或修改。
     */
    @Override
    public void storeStrategyAwardSearchRateTable(String key, Integer rateRange, Map<Integer, Integer> strategyAwardSearchRateTable) {
        // 1. 存储抽奖策略范围值，如10000，用于生成1000以内的随机数
        redisService.setValue(Constants.RedisKey.STRATEGY_RATE_RANGE_KEY + key, rateRange);
        // 2. 存储概率查找表
        Map<Integer, Integer> cacheRateTable = redisService.getMap(Constants.RedisKey.STRATEGY_RATE_TABLE_KEY + key);
        cacheRateTable.putAll(strategyAwardSearchRateTable);
    }


    // @Override
    // public int getRateRange(Long strategyId) {
    //
    //     return redisService.getValue(String.valueOf(strategyId));
    // }

    @Override
    public int getRateRange(Long strategyId) {
        return getRateRange(String.valueOf(strategyId));
    }
    // @Override
    // public int getRateRange(String key) {
    //
    //     return redisService.getValue(Constants.RedisKey.STRATEGY_RATE_RANGE_KEY + key);
    // }

    @Override
    public int getRateRange(String key) {
        String cacheKey = Constants.RedisKey.STRATEGY_RATE_RANGE_KEY + key;
        if (!redisService.isExists(cacheKey)) {
            throw new AppException(UN_ASSEMBLED_STRATEGY_ARMORY.getCode(), cacheKey + Constants.COLON + UN_ASSEMBLED_STRATEGY_ARMORY.getInfo());
        }
        return redisService.getValue(cacheKey);
    }
    @Override
    public Integer getStrategyAwardAssemble(String key, int rateKey) {

        return redisService.getFromMap(Constants.RedisKey.STRATEGY_RATE_RANGE_KEY + key, rateKey);
    }

    @Override
    public StrategyEntity queryStrategyEntityByStrategyId(Long strategyId) {
        // 优先从缓存读取
        String cacheKey = Constants.RedisKey.STRATEGY_KEY+strategyId;
        StrategyEntity strategyEntity = redisService.getValue(cacheKey);
        if (null != strategyEntity) return strategyEntity;
        Strategy strategy = strategyDao.queryStrategyByStrategyId(strategyId);
        strategyEntity = StrategyEntity.builder()
                .strategyId(strategy.getStrategyId())
                .strategyDesc(strategy.getStrategyDesc())
                .ruleModels(strategy.getRuleModels())
                .build();
        redisService.setValue(cacheKey, strategyEntity);
        return strategyEntity;
    }

    @Override
    public StrategyRuleEntity queryStrategyRule(Long strategyId, String ruleModel) {
        StrategyRule strategyRuleReq = new StrategyRule();
        strategyRuleReq.setStrategyId(strategyId);
        strategyRuleReq.setRuleModel(ruleModel);
        StrategyRule strategyRuleRes = strategyRuleDao.queryStrategyRule(strategyRuleReq);
        if (null == strategyRuleRes) return null;
        return StrategyRuleEntity.builder()
                .strategyId(strategyRuleRes.getStrategyId())
                .awardId(strategyRuleRes.getAwardId())
                .ruleType(strategyRuleRes.getRuleType())
                .ruleModel(strategyRuleRes.getRuleModel())
                .ruleValue(strategyRuleRes.getRuleValue())
                .ruleDesc(strategyRuleRes.getRuleDesc())
                .build();
    }

    @Override
    public Boolean subtractionAwardStock(String cacheKey, Date endDateTime) {
        long surplus = redisService.decr(cacheKey);
        if (surplus < 0) {
            // 库存小于0，恢复为0个
            redisService.setAtomicLong(cacheKey, 0);
            return false;
        }
        // 1. 按照cacheKey decr 后的值，如 99、98、97 和 key 组成为库存锁的key进行使用。
        // 2. 加锁为了兜底，如果后续有恢复库存，手动处理等，也不会超卖。因为所有的可用库存key，都被加锁了。
        String lockKey = cacheKey + Constants.UNDERLINE + surplus;
        Boolean lock = false;
        if (null != endDateTime) {
            long expireMillis = endDateTime.getTime() - System.currentTimeMillis() + TimeUnit.DAYS.toMillis(1);
            lock = redisService.setNx(lockKey, expireMillis, TimeUnit.MILLISECONDS);
        } else {
            lock = redisService.setNx(lockKey);
        }
        if (!lock) {
            log.info("策略奖品库存加锁失败 {}", lockKey);
        }
        return lock;
    }

    @Override
    public void cacheStrategyAwardCount(String cacheKey, Integer awardCount) {
        if (redisService.isExists(cacheKey)) return;
        redisService.setAtomicLong(cacheKey, awardCount);
    }

    @Override
    public String queryStrategyRuleValue(Long strategyId, Integer awardId, String ruleModel) {
        StrategyRule strategyRule = new StrategyRule();
        strategyRule.setStrategyId(strategyId);
        strategyRule.setAwardId(awardId);
        strategyRule.setRuleModel(ruleModel);
        return strategyRuleDao.queryStrategyRuleValue(strategyRule);
    }


}

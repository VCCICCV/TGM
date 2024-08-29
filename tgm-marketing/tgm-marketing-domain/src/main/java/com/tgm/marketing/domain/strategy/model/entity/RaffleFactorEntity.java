package com.tgm.marketing.domain.strategy.model.entity;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

import java.util.Date;

/**
 * @PROJECT_NAME tgm
 * @AUTHOR VCCICCV
 * @DATE 2024/1/6 上午2:28
 * @DESCRIPTION
 */
@Builder
@Data
@AllArgsConstructor
@NoArgsConstructor
public class RaffleFactorEntity {
    /** 用户ID */
    private String userId;
    /** 策略ID */
    private Long strategyId;
    /** 结束时间 */
    private Date endDateTime;
}

package com.tgm.marketing.domain.strategy.service.annotation;

import com.tgm.marketing.domain.strategy.service.rule.factory.DefaultLogicFactory;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.Target;

/**
 * @AUTHOR CCI
 * @DATE 2024/2/6 上午5:25
 * @DESCRIPTION
 */
@Target ({ElementType.TYPE})
@Retention (RetentionPolicy.RUNTIME)
public @interface LogicStrategy {

    DefaultLogicFactory.LogicModel logicMode();

}

package com.tgm.marketing.config;

import cn.hutool.core.util.RandomUtil;
import io.swagger.v3.oas.models.OpenAPI;
import io.swagger.v3.oas.models.info.Info;
import io.swagger.v3.oas.models.info.License;
import org.springdoc.core.customizers.GlobalOpenApiCustomizer;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

import java.util.HashMap;
import java.util.Map;

/**
 * @AUTHOR CCI
 * @DATE 2024/8/7 上午4:18
 * @DESCRIPTION 创建Swagger配置
 */
@Configuration
public class SwaggerConfig {
    /**
     * 根据@Tag 上的排序，写入x-order
     *
     * @return the global open api customizer
     */
    @Bean
    public GlobalOpenApiCustomizer orderGlobalOpenApiCustomizer() {
        return openApi -> {
            if (openApi.getTags() != null) {
                openApi.getTags().forEach(tag -> {
                    Map<String, Object> map = new HashMap<>();
                    map.put("x-order", RandomUtil.randomInt(0, 100));
                    tag.setExtensions(map);
                });
            }
            if (openApi.getPaths() != null) {
                openApi.addExtension("x-test123", "333");
                openApi.getPaths().addExtension("x-abb", RandomUtil.randomInt(1, 100));
            }

        };
    }
    // 自定义swagger配置信息
    @Bean
    public OpenAPI customOpenAPI() {
        return new OpenAPI()
                .info(new Info()
                        .title("TGM Web3D 系统API")
                        .version("1.0")
                        .description("基于DDD设计实现的 Web3D 购车商城")
                        .termsOfService("145632.xyz")
                        .license(new License().name("Apache 2.0")
                                .url("145632.xyz")));
    }
}

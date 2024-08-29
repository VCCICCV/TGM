package com.tgm.marketing.types.exception;

import lombok.Data;
import lombok.EqualsAndHashCode;

/**
 * @PROJECT_NAME tgm
 * @AUTHOR VCCICCV
 * @DATE 2024/2/6 上午12:39
 * @DESCRIPTION
 */
@EqualsAndHashCode (callSuper = true)
@Data
public class AppException extends RuntimeException{
    private static final long serialVersionUID = 5317680961212299217L;

    /** 异常码 */
    private String code;

    /** 异常信息 */
    private String info;

    public AppException(String code) {
        this.code = code;
    }

    public AppException(String code, Throwable cause) {
        this.code = code;
        super.initCause(cause);
    }

    public AppException(String code, String message) {
        this.code = code;
        this.info = message;
        super.initCause(new Throwable(message));
    }

    public AppException(String code, String message, Throwable cause) {
        this.code = code;
        this.info = message;
        super.initCause(cause);
    }

    @Override
    public String toString() {
        return "cn.bugstack.types.exception.AppException{" +
                "code='" + code + '\'' +
                ", info='" + info + '\'' +
                '}';
    }
}

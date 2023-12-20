CREATE DATABASE /*!32312 IF NOT EXISTS*/ `rs_dbc` /*!40100 DEFAULT CHARACTER SET utf8mb4 */;
use rs_dbc;


DROP TABLE IF EXISTS `sys_user`;
CREATE TABLE `sys_user` (
    `id` bigint(20) NOT NULL COMMENT '主键',
    `username` varchar(20) NOT NULL COMMENT '用户名',
    `nickname` varchar(20) DEFAULT NULL COMMENT '昵称',
    `password` varchar(64) NOT NULL COMMENT '密码',
    `salt` varchar(32) NOT NULL COMMENT '盐值',
    `phone` varchar(20) DEFAULT NULL COMMENT '手机号码',
    `email` varchar(255) DEFAULT NULL COMMENT '邮箱',
    `gender` tinyint(4) DEFAULT '1' COMMENT '性别，0：女，1：男，默认1',
    `remark` varchar(200) DEFAULT NULL COMMENT '备注',
    `state` tinyint(4) NOT NULL DEFAULT '1' COMMENT '状态，0：禁用，1：启用，2：锁定',
    `create_user_id` bigint(20) NOT NULL COMMENT '创建人',
    `create_time` datetime NOT NULL COMMENT '创建时间',
    `update_user_id` bigint(20) NOT NULL COMMENT '修改人',
    `update_time` datetime NOT NULL COMMENT '修改时间',
    PRIMARY KEY (`id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='系统用户';
-- Your SQL goes here
CREATE TABLE IF NOT EXISTS `users`
(
    `id`         bigint  NOT NULL AUTO_INCREMENT COMMENT 'id',
    `name`       varchar(255)      NOT NULL COMMENT 'ユーザー名',
    `created_at` datetime          NOT NULL COMMENT '登録日時',
    `updated_at` datetime DEFAULT NULL COMMENT '更新日時',
    PRIMARY KEY (`id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8 COMMENT ='ユーザーテーブル';

create index idx_users_id on users (id);


CREATE TABLE IF NOT EXISTS `tasks`
(
    `id`         bigint  NOT NULL AUTO_INCREMENT COMMENT 'id',
    `user_id`    bigint  NOT NULL COMMENT 'user_id',
    `title`      varchar(255)      NOT NULL COMMENT 'タイトル',
    `is_done`    bool          NOT NULL DEFAULT false COMMENT '完了フラグ',
    `created_at` datetime          NOT NULL COMMENT '登録日時',
    `updated_at` datetime                   DEFAULT NULL COMMENT '更新日時',
    PRIMARY KEY (`id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8 COMMENT ='タスクテーブル';

CREATE INDEX `idx_tasks_id` ON `tasks` (`id`);
ALTER TABLE `tasks`
    ADD FOREIGN KEY `fk_tasks_users_id` (`user_id`) REFERENCES `users` (`id`) ON DELETE RESTRICT ON UPDATE NO ACTION;
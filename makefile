MAKEFILE_DIR:=$(dir $(abspath $(lastword $(MAKEFILE_LIST))))

# watch-run:
# 	cargo watch -x run &
	
# resource-server-start:
# 	php -S localhost:8000 -t ${MAKEFILE_DIR}resources &

kill-cargo:
	# このキルコマンドはパイプラインがうまくいかず動かない・・・
	zsh -c "kill `ps aux | grep cargo | awk '{print $$2}'`"

watch-run:
	cargo watch -x run

resource-server-start:
	php -S localhost:8000 -t ${MAKEFILE_DIR}resources

kill-php:
	# このキルコマンドはパイプラインがうまくいかず動かない・・・
	zsh -c "kill `ps aux | grep php | awk '{print $$2}'`"

all-start: watch-run resource-server-start;

all-kill: kill-cargo kill-php;

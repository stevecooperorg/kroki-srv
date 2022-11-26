test:
	cargo watch -x test

kroki:
	docker-compose up &

serve:
	cargo watch -x test -x run

start: kroki serve

stop:
	docker-compose down
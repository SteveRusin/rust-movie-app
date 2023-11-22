populate-db:
  cargo run --bin populate-db
fetch-mocks:
  seq 10 | parallel 'curl https://jsonfakery.com/movies/paginated\?page\=\{} | jq ".data" > src/bin/movies-mock/seed-{}.json'
dev:
  cargo watch -x run



# rising ware Docker

```
docker run -it --pull=always -p 4566:4566 -p 5691:5691 risingwavelabs/risingwave:latest playground
```





# redash docker安装

```
version: '3'
services:
  server:
    image: redash/redash:latest
    depends_on:
      - postgres
      - redis
    ports:
      - "5000:5000"
    environment:
      REDASH_DATABASE_URL: "postgresql://postgres:postgres@postgres/postgres"
      REDASH_REDIS_URL: "redis://redis:6379/0"
    command: server

  worker:
    image: redash/redash:latest
    depends_on:
      - server
      - redis
    environment:
      REDASH_DATABASE_URL: "postgresql://postgres:postgres@postgres/postgres"
      REDASH_REDIS_URL: "redis://redis:6379/0"
    command: worker

  scheduler:
    image: redash/redash:latest
    depends_on:
      - server
      - redis
    environment:
      REDASH_DATABASE_URL: "postgresql://postgres:postgres@postgres/postgres"
      REDASH_REDIS_URL: "redis://redis:6379/0"
    command: scheduler

  redis:
    image: redis:5.0-alpine

  postgres:
    image: postgres:12-alpine
    environment:
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: postgres
      POSTGRES_DB: postgres
    volumes:
      - postgres-data:/var/lib/postgresql/data

volumes:
  postgres-data:

```

```
docker-compose run --rm server create_db
docker-compose up -d

zhangsan
zhangsan113
```



![image-20241206094144677](risingwave.assets/image-20241206094144677.png)

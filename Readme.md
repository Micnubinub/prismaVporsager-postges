# Prisma vs Postgres lib by porsager

setup a postgres db and put the values in the respective positions in prisma/.env and postgres/index.js

Then run them by calling ```node postgres``` or ```node prisma```

Use your favorite benchmarking tool to test them by:

Run ```get lolcahost:8086/select``` to test the select speed

Run ```get lolcahost:8086/insert``` to test the insert speed

Don't forget to run ```prisma generate``` & ```prisma migrate dev --name init```, read the
setup [Setup Instructions in this link](https://www.prisma.io/docs/getting-started/setup-prisma/add-to-existing-project/relational-databases-typescript-postgres)

#### In my tests postgres vs ~2x faster while using half the RAM

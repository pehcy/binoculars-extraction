# OHLCV data streamer for binoculars

This repositories is a about the extraction of financial market data for various coins using Binance API calls. The data warehousing using Postgres SQL.

<b>Noted: Still work in progress, currently only support ada/usdt ohlcv data. 
</b>

## Setup Postgresql

```
psql -d <database_name> -h localhost
```

In case it prompted this error message:
```
psql: FATAL:  password authentication failed for user "postgres"
```

to fix this, you need to login to the database with password.

```sql
ALTER USER postgres PASSWORD 'your_password';
```

## View database tables

```
sudo -u postgres psql <database_name> 
```
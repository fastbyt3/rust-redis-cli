# Redis Client CLI

```bash
redis-cli [-h <hostname | 127.0.0.1>] [-p <port | 6379>] [-a <passwd>
```

CLI Commands
- `KEYS <pattern>` -> Search for keys matching a specific pattern
- `GET <key>` -> retrieves value for specified KEY
- `SET <key> <value>` -> set Key-value pair
- `SETEX <key> <value> <expiry>` -> Set Key-value pair with key expiry in **seconds**
- `DEL <key>` -> Delete key
- `TTL <key>` -> Get TTL (Time To Live) for key


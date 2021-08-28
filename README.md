# README

## API

POST /push
REQUIRED HEADER: X-Receive-Password

### Missing password header

400, text/plain, "Missing X-Receive-Password header"

### Bad password

401, text/plain, "Bad password in X-Receive-Password header"

### Good password

200, application/json

```
{
    "message": "string",
    "url": "asd","
}
```

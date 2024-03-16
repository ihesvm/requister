# requister


*simple cli app for send http request*  

---
Build Requister ...
```bash
$ cargo build --release
```
---
Example 
```shell
$ requister http://example.com/api/ [The default method is GET]

Object {
    "args": Object {},
    "headers": Object {
        "Accept": String("*/*"),
        "Host": String("example.com"),
        "X-Amzn-Trace-Id": String("Root=1-65f1a14d-6fb4feb639e9780461be4a33"),
    },
    "origin": String("1.1.1.1"),
    "url": String("https://example.com/get"),
}

```

---
### TODO
- [x] Send GET and DELETE method
- [x] Add ...
  - [x] POST
  - [x] PUT
  - [x] PATCH
- [x] Arguments are added to the POST request with key value data
- [x] Add another status code  
- [ ] Add help text and arguments switch
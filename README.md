# requister


*simple cli app for send http request*  

---
Example 
```shell
$ requister http://example.com/api/ [get is default method to send request]

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
- [ ] Add ...
  - [ ] POST
  - [ ] PUT
  - [ ] PATCH
- [ ] Arguments are added to the POST request with key value data

Backend for [DropPod](https://github.com/Epirius/DropPod)

### Setup
```cp ./configuration/example.secret.yaml ./configuration/secret.yaml ```

update `secret_key` in secret.yaml

-----
### Commands
Hot reload dev:
```cargo watch -q -c -w src/ -x run```

Test script during dev:
```cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"```

-----
### Docker
##### build 
```docker build -t drop:v1.0 .```
##### run
```docker run -p 4000:4000 drop:v1.0```


-----
This project backend is built on top of [rust-axum-course](https://github.com/jeremychone-channel/rust-axum-course)

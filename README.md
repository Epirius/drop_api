Backend for [DropPod](https://github.com/Epirius/DropPod)

### Setup
```cp ./configuration/example.secret.yaml ./configuration/secret.yaml ```

update `secret_key` in secret.yaml

-----
### Commands
Hot reload dev:
```cargo watch -q -c -w src/ -w .cargo/ -x run```

Test script during dev:
```cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"```

shuttle hot reload:
```cargo watch -q -c -w src/ -w .cargo/ -x "shuttle run```

-----
### Deploy
```cargo shuttle deploy```

-----
This project backend is built on top of [rust-axum-course](https://github.com/jeremychone-channel/rust-axum-course)

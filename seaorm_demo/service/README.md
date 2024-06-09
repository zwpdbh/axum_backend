## How to test service 

```sh 
cargo test -p service -F mock --test mock -- --nocapture
```

## How to enable vscode check mock code 

Edit VS Code configuration to recognize and analyze code under conditional compilation attributes:

```json
{
    "rust-analyzer.cargo.features": ["mock"]
}
```

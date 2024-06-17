# storage-cli

## devops-cli

下载项目

```shell
git clone https://github.com/yangbajing/technique-rust.git
cd technique-rust
```

执行示例：

```shell
# 查询文件元数据
cargo run -p storage-cli --bin devops-cli -- -f ./clis/storage-cli/.app.toml stat downloads/old/devops-cli

# 下载文件
cargo run -p storage-cli --bin devops-cli -- -f ./clis/storage-cli/.app.toml get java/demo/durian/durian-backend/durian-backend-service.jar ~/Downloads/durian-backend-service.jar

# 上传文件
cargo run -p storage-cli --bin devops-cli -- -f ./clis/storage-cli/.app.toml put ~/Downloads/durian-backend-service.jar java/demo/durian/durian-backend/durian-backend-service.jar
```

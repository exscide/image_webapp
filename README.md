> Random, minimalist "webapp" für @Psysician um docker zu lernen

## Setup
`image.jpg` und `Rocket.toml` müssen im cwd sein.

In der `Rocket.toml` können unter \[release\] IP, Port, usw angegeben werden. [Rocket.toml docs](https://rocket.rs/v0.5-rc/guide/configuration/#configuration)

#### Kompilieren und ausführen:
```bash
cargo run --release
```

#### Nur kompilieren:
```bash
cargo build --release
```

#### Nur ausführen:
```bash
./target/release/image_webapp
```

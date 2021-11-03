# What can this project do?

Given the name of a city and a server from command line, we will generate related information and store them into a json file.


# Which package will be used?

Some packages will be used in this project, they are listed as followed:

- `serde = 1`;
- `serde_json = 1`;
- `serde_derive = 1`;
- `chrono = 0.4`;
- `sysinfo = 0.16.4`;

# What does it look like?

```bash
# server-info-json on 🌱 main [?] is 📦 v0.1.0 via 🦀 v1.56.1 
❯ ls
 src   target   Cargo.lock   Cargo.toml   README.md

# server-info-json on 🌱 main [?] is 📦 v0.1.0 via 🦀 v1.56.1 
❯ cargo r Beijing www.bilibili.com hello.json
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/server-info-json Beijing www.bilibili.com hello.json`
	Done!

# server-info-json on 🌱 main [?] is 📦 v0.1.0 via 🦀 v1.56.1 
❯ ls
 src   target   Cargo.lock   Cargo.toml   README.md   hello.json

# server-info-json on 🌱 main [?] is 📦 v0.1.0 via 🦀 v1.56.1 
❯ cat hello.json 
{
  "city": "Beijing",
  "server": "www.bilibili.com",
  "time": "2021-11-03 04:50:09.470440122 UTC",
  "info": "GK-mini memory:2579690/7892733 processors:4"
}

```


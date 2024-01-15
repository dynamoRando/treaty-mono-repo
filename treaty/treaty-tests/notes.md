# Overview

This crate is for integration testing between the client, treaty, and proxy. 


# Etc

`cargo test --test (name of module)` example: `cargo test --test get_coop_hosts`
`cargo test --test (name of module)` example: `cargo test --test get_coop_hosts http`
`cargo test --test auth_for_token postgres`
`cargo test --test auth_for_token grpc::test`
`cargo test --test auth_for_token grpc::postgres`
`cargo test --test db_setup grpc::postgres`
`cargo test --test get_coop_hosts grpc::postgres`
`cargo test --test get_db_names grpc::postgres`
`cargo test --test db_setup_delete grpc::postgres`
`cargo test --test db_setup_insert grpc::postgres`
`cargo test --test get_host_info grpc::postgres`
`cargo test --test get_participants grpc::postgres`
`cargo test --test get_set_logical_storage_policy grpc::postgres`
`cargo test --test get_settings grpc::postgres`
`cargo test --test has_table grpc::postgres`
`cargo test --test read_write_host grpc::postgres`
`cargo test --test reject_host grpc::postgres`
`cargo test --test revoke_token grpc::postgres`
`cargo test --test validate_delete_behaviors grpc::postgres`
`cargo test --test validate_update_behaviors grpc::postgres`
`cargo test --test validate_update_with_log grpc::postgres`
`cargo test --test db_setup grpc::postgres_sqlite`

# Data Conversions from int

Where needed, convert to `Vec<u8>` using ` value.to_ne_bytes().to_vec()` or `value.to_string().as_bytes().to_vec()`

Other examples:

```
 pub async fn get_scalar_as_u64(&self, sql: &str) -> Result<Option<u64>, TreatyDbError> {
        let rows = self.query_db(sql).await?;

        trace!("[{}]: Rows: {rows:?}", function_name!());

        let row = &rows[0];
        trace!("[{}]: Row: {row:?}", function_name!());
        let result = row.try_get::<usize, Option<Vec<u8>>>(0);

        match result {
            Ok(item) => match item {
                Some(i) => Ok(Some(u64::from_ne_bytes(vec_to_array(i)))),
                None => Ok(None),
            },
            Err(e) => {
                error!("[{}]: {e:?}", function_name!());
                Err(TreatyDbError::General(
                    "Unable to get value as u32".to_string(),
                ))
            }
        }
    }
```

# Creating bundles

`git bundle create treaty.bundle --all`

Notes:

- https://stackoverflow.com/questions/5578270/fully-backup-a-git-repo
- https://stackoverflow.com/questions/11792671/how-to-git-bundle-a-complete-repo


# Using Mold

Taken from [mold](https://github.com/rui314/mold)

Create `.cargo/config.toml` in your project directory with the following:

```
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=/path/to/mold"]
```

where /path/to/mold is an absolute path to the mold executable. In the example above, we use clang as a linker driver since it always accepts the -fuse-ld option. If your GCC is recent enough to recognize the option, you may be able to remove the linker = "clang" line.

Note: mold is usually at:

```
/usr/local/bin/mold

# or 

/usr/bin/mold
```

If needed run `which mold` or `which clang` to get the proper locations from the environment. 

When running `treaty-dev` container in docker, you may run into compiler problems if VS Code is running (with rust-analyzer). To avoid this, you can run `cargo t --release`.

Note that this will build, but may take a long time.


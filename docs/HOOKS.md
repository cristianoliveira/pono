# Pono Hooks

Pono allows you to define hooks to run commands before enabling or disabling a pono. This is configured within the `pono.toml` file, under the `hooks` section of a specific pono definition.

#### Configuration

To define hooks, you must include a `hooks` table within the definition of a pono in your `pono.toml` file. This table can contain two optional fields: `pre_enable` and `pre_disable`.

-   `pre_enable`: A command to execute before enabling the pono.
-   `pre_disable`: A command to execute before disabling the pono.

#### Example

```toml
[ponos."with-hooks"]
  source = "./examples/from/hooks"
  target = "./examples/to/.hooks"

  [ponos."with-hooks".hooks]
  pre_enable = "echo 'Running pre_enable hook for with-hooks'"
  pre_disable = "echo 'Running pre_disable hook for with-hooks'"
```

#### Usage

When you run `pono enable <pono>` or `pono disable <pono>`, Pono will execute the corresponding hook (if defined) before performing the symlinking or unlinking operation.

##### Example

Enabling the `with-hooks` pono:

```bash
pono enable with-hooks
```

This will execute the `echo` command defined in `pre_enable` before creating the symlink.

Disabling the `with-hooks` pono:

```bash
pono disable with-hooks
```

This will execute the `echo` command defined in `pre_disable` before removing the symlink.


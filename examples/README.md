# xasm examples

To run these examples, test them with the following command.

```bash
xasm -r run EXAMPLE.x --packages xasm_std xasm_core
```

Some files may fail to build with the golang flag because either the `xasm_core` or `xasm_std` import is not used. If the example outputs nothing, either run it with the `--rs` attribute, or remove `xasm_core` from the imported foreign packages.
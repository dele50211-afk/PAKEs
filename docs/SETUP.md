# Setup and run

Run the appropriate setup script for your OS, then build and run the workspace.

Debian/Ubuntu:

```bash
sudo bash scripts/setup-debian.sh
make build
make test
make run
```

Alpine:

```sh
sudo sh scripts/setup-alpine.sh
make build
make test
make run
```

If you prefer a single script:

```bash
bash scripts/build_and_run.sh
```

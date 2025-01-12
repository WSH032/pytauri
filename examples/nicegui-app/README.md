# PyTauri + Vanilla TS

```bash
cd examples/nicegui-app
```

and follow the tutorial section in Documentation.

**NOTE**: **When using `tauri dev`, you must pass `--no-dev-server-wait`**, because the uvicorn server only starts after running the executable.

---

or you are hurry, just wanna see/run the demo:

> Make sure you have already installed `rust`, `uv`, `pnpm`, `node.js` and Tauri Prerequisites.
>
> > Because you have directly pulled the development repository, you need `pnpm` to manually build frontend assets.
> >
> > In actual use, this step has already been completed for `pytauri` on `crates.io`, which means only `rust` and `python` are required.
>
> You can find that in tutorial section in Documentation.

```bash
git clone https://github.com/WSH032/pytauri.git
cd pytauri

cd examples/nicegui-app

# build frontend assets
pnpm install
pnpm -r run build

# activate virtual environment
uv venv
source .venv/bin/activate
# or powershell: .venv\Scripts\Activate.ps1

# install the example package
# (need some time to compile rust code,
#  you can pass `--verbose` to see the progress)
uv pip install --reinstall -e .

# run the example
python -m nicegui_app
```

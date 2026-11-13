# BloxPM

BloxPM is the initial reference implementation for the BloxOS `.blox` package format. It uses only Python's standard library.

```bash
python package-manager/src/bloxpm.py create package-manager/examples/calculator Calculator-1.0.0.blox
python package-manager/src/bloxpm.py verify Calculator-1.0.0.blox
python package-manager/src/bloxpm.py info Calculator-1.0.0.blox
python package-manager/src/bloxpm.py install Calculator-1.0.0.blox --root ./apps
```

`install` extracts files only. It does not execute package contents or scripts.

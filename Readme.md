# Puzzles API

## Local live reload

setup: `pip install hupper pywatchman`

```bash
PYTHONPATH=src python -m run
```

## Updating puzzles repo

Assuming ../puzzles contains the repository:

```bash
git subtree pull --prefix puzzles ../puzzles master --squash
```

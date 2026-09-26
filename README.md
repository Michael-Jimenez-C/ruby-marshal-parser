# r_marshal

Implementación en Rust del parser/serializador Ruby Marshal 4.8, expuesta como módulo Python mediante PyO3.

## Requisitos

- Rust + cargo
- Python 3.x
- `maturin` instalado en el entorno virtual

## Desarrollo

Desde el directorio del proyecto:

```bash
uv pip install maturin
cd ruby-marshal-parser
source "$HOME/.cargo/env"
../.venv/bin/maturin develop
```

Esto compila e instala el módulo `r_marshal` directamente en el entorno virtual activo.

## Construir un wheel

```bash
cd ruby-marshal-parser
maturin build --release
```

El `.whl` se genera en `ruby-marshal-parser/target/wheels/`. Puedes instalarlo manualmente con pip:

```bash
pip install target/wheels/r_marshal-*.whl
```

## Uso desde Python

```python
import r_marshal

# Un solo objeto Marshaled
obj = r_marshal.load_file("data.rvdata2")
out = r_marshal.dump_file(obj, "data.rvdata2")

# Múltiples objetivos concatenados
objs = r_marshal.load_file_all("data.rvdata2")
rewr = r_marshal.dump_all(objs)

with open("data.rvdata2", "rb") as f:
    orig = f.read()

print(orig == rewr)  # True
```

## API expuesta

Funciones:

- `r_marshal.load(data: bytes) -> object`
- `r_marshal.load_file(path: str) -> object`
- `r_marshal.load_all(data: bytes) -> list`
- `r_marshal.load_file_all(path: str) -> list`
- `r_marshal.dump(obj) -> bytes`
- `r_marshal.dump_file(obj, path)`
- `r_marshal.dump_all(objects: list) -> bytes`
- `r_marshal.dump_file_all(objects: list, path)`

Clases principales devueltas por el parser:

- `RubySymbol`
- `RubyString`
- `RubyFloat`
- `RubyObject`
- `RubyStruct`
- `RubyUserClass`
- `RubyUserDefined`
- `RubyUserMarshal`
- `RubyData`
- `RubyExtended`
- `RubyClass`
- `RubyRegexp`
- `RubyList`
- `RubyDict`
- `RubyDictWithDefault`

Todas llevan `dict` habilitado, por lo que Python puede asignar atributos adicionales como `_extended_module` o `_user_class`.

## Notas

- Compatible con Marshal 4.8 (`\x04\x08`). Otros versiones no están soportados.

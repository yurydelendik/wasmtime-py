import wasmtime_py

def answer() -> 'i32':
    return 42

f = open("./one.wasm", "rb")
res = wasmtime_py.instantiate(f.read(), {"env": {"answer": answer}})
one = res.instance

f = open("./two.wasm", "rb")
res = wasmtime_py.instantiate(f.read(), { "one": one.exports })
two = res.instance

ask = two.exports["ask"]

print("answer() returned", ask())

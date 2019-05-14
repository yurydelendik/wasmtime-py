import wasmtime_py

f = open("./gcd.wasm", "rb")
res = wasmtime_py.instantiate(f.read(), {})
instance = res.instance
gcd = instance.exports["gcd"]
print("gcd(27, 6) = ", gcd(27, 6))

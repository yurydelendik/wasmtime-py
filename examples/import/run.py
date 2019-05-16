import wasmtime_py

def callback(msg_p: 'i32', msg_len: 'i32') -> 'i32':
    print("{} {}".format(msg_p, msg_len))

env = {
    "callback": callback
}
f = open("./import.wasm", "rb")
res = wasmtime_py.instantiate(f.read(), { "env": env })
instance = res.instance
instance.exports["test"]()

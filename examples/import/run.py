import wasmtime_py

def callback(msg_p, msg_len):
    print("{} {}".format(msg_p, msg_len))

env = {
    "callback": callback
}
f = open("./import.wasm", "rb")
res = wasmtime_py.instantiate(f.read(), { "env": env })
instance = res.instance
instance.exports["test"]()

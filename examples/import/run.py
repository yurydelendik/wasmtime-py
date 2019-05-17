import wasmtime_py

def callback(msg_p: 'i32', msg_len: 'i32') -> 'i32':
    global mv
    msg = bytes(mv[msg_p:(msg_p + msg_len)]).decode('utf-8')
    print(msg)
    return 42

env = {
    "callback": callback
}
f = open("./import.wasm", "rb")
res = wasmtime_py.instantiate(f.read(), { "env": env })
instance = res.instance

memory = instance.exports["memory"]
mv = memoryview(memory)
instance.exports["test"]()

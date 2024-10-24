import init, { WasmStorage } from "./pkg";

export async function run() {
    try {


        console.log("triggered wasm")
        await init()
        const storage = new WasmStorage();
        // console.log(storage.get("test"));
        // storage.set("test", "random");
        // console.log(storage.get("test"));
        storage.set_with_ttl("random", "test", 0n)
        console.log(storage.get("random"));
        setTimeout(() => {
            console.log(storage.get("random"));
        }, 11000)
    } catch (error) {
        console.log(error)
    }
}
// var Turbolinks = require("turbolinks")
// Turbolinks.start()

addEventListener('fetch', event => {
    event.respondWith(handleEvent(event))
})

async function handleEvent(event) {
    // bind wasm
    const { main } = wasm_bindgen
    await wasm_bindgen(wasm)

    try {
        // handle request
        const ev = await main(event)
        // console.log(ev)
        return ev
    } catch (e) {
        // debug err on failure
        return new Response(e.stack || e.message || e || "unknown error", {
            status: 500
        });
  }
}

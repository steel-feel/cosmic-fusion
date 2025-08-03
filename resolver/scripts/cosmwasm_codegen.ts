import codegen from "@cosmwasm/ts-codegen";

export async function codeGen() {
    console.log("Building cosmwasm codegen");

    await codegen({
        outPath: "./src/wasm-bindings",
        options: {
            types: {
                enabled: true,
            },
            client: {
                enabled: true,
            },
        },
        contracts: [
            {
                name: "limit_order_protocol",
                dir: "../contracts/limit-order-protocol/schema"
            },
            {
                name: "escrow_factory",
                dir: "../contracts/escrow-factory/schema"
            },
            {
                name: "escrow_dst",
                dir: "../contracts/escrow_dst/schema"
            },
            {
                name: "escrow_src",
                dir: "../contracts/escrow_src/schema"
            }
        ]
    })
    console.log("✨ all done!");
}
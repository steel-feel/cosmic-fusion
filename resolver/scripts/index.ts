import { codeGen } from "./cosmwasm_codegen";

async function main() {
   await codeGen()
}

main().catch(err => console.log(err) );


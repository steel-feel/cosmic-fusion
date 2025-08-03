import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing";
import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate"
import { getSigningCosmosClient } from "osmojs"
import { contracts } from "../wasm-bindings/bundle"

import { getExactDecimalsFromNumber } from "@injectivelabs/sdk-ts";
import { Decimal } from "@cosmjs/math";

export class CosmosManager {
    private wasmSigner: SigningCosmWasmClient
    private wallet: DirectSecp256k1HdWallet;
    private cosmosClient :any;
    private address: string;

    constructor() {
     
    }

    async init() {
        const mnemonic = Bun.env.SEED_PHRASE || ""
        this.wallet = await DirectSecp256k1HdWallet.fromMnemonic(mnemonic, {
            prefix: "xion",
        });
        const walletAccount = await this.wallet.getAccounts()
        this.address = walletAccount[0].address
        console.log(`Address is ${this.address}`);

        
        this.cosmosClient = await getSigningCosmosClient({ rpcEndpoint: Bun.env.RPC_URL || "", signer: this.wallet })
      
        this.wasmSigner = await SigningCosmWasmClient.connectWithSigner(Bun.env.RPC_URL || "", this.wallet, {
            gasPrice : { amount : Decimal.fromUserInput("0",1)   , denom : "stake" }
        })
    }

    async deployDstEscrow( deployParams : any  ) {
        console.log(`Using ${this.address} wallet`);
        
        const contractInstance = new contracts.EscrowFactory.EscrowFactoryClient(this.wasmSigner, this.address, Bun.env.XION_DEST_FACTORY_ADDRESS || "")
        /*
         deployParams : {
            hashlock: "0x",
            maker: "0x",
            orderHash: "0x",
            rescueDelay: 1,
            taker: "0x",
            timelocks: {
                "dest_cancellation": 1000,
                "dest_public_withdrawal": 2000,
                "dest_withdrawal": 3000,
                "src_cancellation": 4000,
                "src_public_cancellation": 5000,
                "src_public_withdrawal": 6000,
                "src_withdrawal": 7000
            },
            "token": { "amount": "10", "denom": "stake" },
            }, "auto", "", [{ "amount": "10", "denom": "stake" }]);
        }
        */

        const result =  await contractInstance.deployEscrow(deployParams,"auto","",[deployParams.token]  )  
        let escrowAddr =""
        for (let e of result.events ) {
                    if (e.type == "instantiate") {
                        escrowAddr =  e?.attributes[0]?.value
                        console.log(  e );
                    }   
            }
        console.log(`📝 Escrow contract ${escrowAddr}`);
            
        return escrowAddr
    }

    async withdrawMaker(escrowAddr:string,secret:string) {
        const contractInstance = new contracts.EscrowDst.EscrowDstClient(this.wasmSigner, this.address, escrowAddr || "")
        const result = await contractInstance.withdraw({secret})
        console.log({result});
    }


    async deploySrcEscrow() {

    }

    async withdrawTaker() {

    }



}
import {ethers, Interface, JsonRpcProvider, Signature, TransactionRequest} from 'ethers'
// import Sdk from '@1inch/cross-chain-sdk'
import Contract from '../eth_contracts/Resolver.json'

export class EthereumManager {
    private readonly iface = new Interface(Contract.abi)

    constructor(
        public readonly srcAddress: string,
        public readonly dstAddress: string
    ) {}

    // public deploySrc(
    //     chainId: number,
    //     order: Sdk.CrossChainOrder,
    //     signature: string,
    //     takerTraits: Sdk.TakerTraits,
    //     amount: bigint,
    //     hashLock = order.escrowExtension.hashLockInfo
    // ): TransactionRequest {
    //     const {r, yParityAndS: vs} = Signature.from(signature)
    //     const {args, trait} = takerTraits.encode()
    //     const immutables = order.toSrcImmutables(chainId, new Sdk.Address(this.srcAddress), amount, hashLock)

    //     return {
    //         to: this.srcAddress,
    //         data: this.iface.encodeFunctionData('deploySrc', [
    //             immutables.build(),
    //             order.build(),
    //             r,
    //             vs,
    //             amount,
    //             trait,
    //             args
    //         ]),
    //         value: order.escrowExtension.srcSafetyDeposit
    //     }
    // }

    // public deployDst(
    //     /**
    //      * Immutables from SrcEscrowCreated event with complement applied
    //      */
    //     immutables: Sdk.Immutables
    // ): TransactionRequest {
    //     return {
    //         to: this.dstAddress,
    //         data: this.iface.encodeFunctionData('deployDst', [
    //             immutables.build(),
    //             immutables.timeLocks.toSrcTimeLocks().privateCancellation
    //         ]),
    //         value: immutables.safetyDeposit
    //     }
    // }

    // public withdraw(
    //     side: 'src' | 'dst',
    //     escrow: Sdk.Address,
    //     secret: string,
    //     immutables: Sdk.Immutables
    // ): TransactionRequest {
    //     return {
    //         to: side === 'src' ? this.srcAddress : this.dstAddress,
    //         data: this.iface.encodeFunctionData('withdraw', [escrow.toString(), secret, immutables.build()])
    //     }
    // }

    // public cancel(side: 'src' | 'dst', escrow: Sdk.Address, immutables: Sdk.Immutables): TransactionRequest {
    //     return {
    //         to: side === 'src' ? this.srcAddress : this.dstAddress,
    //         data: this.iface.encodeFunctionData('cancel', [escrow.toString(), immutables.build()])
    //     }
    // }



    async withdrawDestination(amountToSend, recipientAddress) {
        const provider = new JsonRpcProvider(
             Bun.env.ARBITRUM_RPC || "",
        )
        // 1. Create a wallet instance from the private key
        const wallet = new ethers.Wallet(Bun.env.ETH_RESOLVER_PK , provider);

        // 2. Define the ABI for the transfer function
        const erc20Abi = [
            "function transfer(address to, uint256 amount) returns (bool)"
        ];
        const tokenAddress = "0x75faf114eafb1BDbe2F0316DF893fd58CE46AA4d"
        // 3. Create a contract instance
        const tokenContract = new ethers.Contract(tokenAddress, erc20Abi, wallet);

        // 4. Get the token's decimals to format the amount correctly
        // We assume 18 decimals for this example. For a robust script, you should fetch this from the contract.
        const decimals = 6; 
        const amount = ethers.parseUnits(amountToSend, decimals);
        
        // 5. Send the transaction
    
        const tx = await tokenContract.transfer(recipientAddress, amount);
        
        // 6. Wait for the transaction to be mined
        const receipt = await tx.wait();
        console.log(`Transaction confirmed in block: ${receipt.blockNumber}`);
    }
}
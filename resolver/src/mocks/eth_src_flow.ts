import { JsonRpcProvider, Signer, Wallet as SignerWallet, HDNodeWallet, Mnemonic, MaxUint256, randomBytes, parseUnits, ZeroAddress, parseEther } from "ethers"
import { Wallet } from './utils/wallet'
import factoryContract from "./utils/TestEscrowFactory.json"
import { ContractFactory } from "ethers"
import {uint8ArrayToHex, UINT_40_MAX } from '@1inch/byte-utils'
import { Resolver } from './utils/resolver'
import { EscrowFactory } from './utils/escrow-factory'
const Sdk = require('@1inch/cross-chain-sdk');

const limitOrderProtocol = '0x111111125421cA6dc452d289314280a0f8842A65'
const sepolia_USDC_Addr = "0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238"
const srcProvider = new JsonRpcProvider({
    url: Bun.env.SEPOLIA_RPC || "",
})
const srcChainUser = new Wallet(Bun.env.USER_PK, srcProvider)
let srcTimestamp;
srcChainUser.getAddress()

export async function startEth() {
    const mnemonic = Bun.env.SEED_PHRASE || ""
    const srcChainUser = new Wallet(Bun.env.USER_PK, srcProvider)
    // dstChainUser = new Wallet(userPk, dst.provider)
    const signer = HDNodeWallet.fromMnemonic(Mnemonic.fromPhrase(mnemonic))
    const srcChainResolver = new Wallet(Bun.env.ETH_RESOLVER_PK, srcProvider)
    // dstChainResolver = new Wallet(resolverPk, dst.provider)
    const srcFactory = new EscrowFactory(srcProvider, Bun.env.SEPOLIA_ESCROW_FACTORY)
    const srcResolverContract = await Wallet.fromAddress(await srcChainResolver.getAddress(), srcProvider)
   
    await srcChainUser.approveToken(
        "0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238",
        limitOrderProtocol,
        MaxUint256
    )
   const srcChainId= 11155111,
           dstChainId = 1212,
    srcTimestamp = BigInt((await srcProvider.getBlock('latest'))!.timestamp)
    const secret = uint8ArrayToHex(randomBytes(32))
    const order = CrossChainOrder.new(
        new Address(Bun.env.SEPOLIA_ESCROW_FACTORY),
        {
            salt: randBigInt(1000n),
            maker: new Address(await srcChainUser.getAddress()),
            makingAmount: parseUnits('2', 6),
            takingAmount: parseUnits('1', 6),
            makerAsset: new Address(sepolia_USDC_Addr),
            takerAsset: new Address(ZeroAddress)
        },
        {
            hashLock: HashLock.forSingleFill(secret),
            timeLocks: TimeLocks.new({
                srcWithdrawal: 10n, // 10sec finality lock for test
                srcPublicWithdrawal: 120n, // 2m for private withdrawal
                srcCancellation: 121n, // 1sec public withdrawal
                srcPublicCancellation: 122n, // 1sec private cancellation
                dstWithdrawal: 10n, // 10sec finality lock for test
                dstPublicWithdrawal: 100n, // 100sec private withdrawal
                dstCancellation: 101n // 1sec public withdrawal
            }),  
            srcChainId,
            dstChainId,
            srcSafetyDeposit: parseEther('0.000001'),
            dstSafetyDeposit: parseEther('0.000001')
        },
        {
            auction: new AuctionDetails({
                initialRateBump: 0,
                points: [],
                duration: 120n,
                startTime: srcTimestamp
            }),
            whitelist: [
                {
                    address: new Address(await srcChainResolver.getAddress()),
                    allowFrom: 0n
                }
            ],
            resolvingStartTime: 0n
        },
        {
            nonce: randBigInt(UINT_40_MAX),
            allowPartialFills: false,
            allowMultipleFills: false
        }
    )

    const signature = await srcChainUser.signOrder(11155111, order)
    const orderHash = order.getOrderHash(11155111)
    const resolverContractAddr = "0xec08b2C507295027607A5a299f13BE7e1b4079f1"
    const resolverContract = new Resolver(resolverContractAddr, resolverContractAddr)

    console.log(`Sepolia`, `Filling order ${orderHash}`)
    const fillAmount = order.makingAmount

    const {txHash: orderFillHash, blockHash: srcDeployBlock} = await srcChainResolver.send(
        resolverContract.deploySrc(
            srcChainId,
            order,
            signature,
            TakerTraits.default()
                .setExtension(order.extension)
                .setAmountMode(Sdk.AmountMode.maker)
                .setAmountThreshold(order.takingAmount),
            fillAmount
        )
    )

    console.log(`Sepolia`, `Order ${orderHash} filled for ${fillAmount} in tx ${orderFillHash}`)

}


startEth().catch(err => console.log(err));



async function deploy(
    json: { abi: any; bytecode: any },
    params: unknown[],
    provider: JsonRpcProvider,
    deployer: SignerWallet
): Promise<string> {
    const deployed = await new ContractFactory(json.abi, json.bytecode, deployer).deploy(...params)
    await deployed.waitForDeployment()

    return await deployed.getAddress()
}

export async function deploySrcFactory() {
    const deployer = new SignerWallet(Bun.env.USER_PK, srcProvider)

    await deploy(
        factoryContract,
        [
            limitOrderProtocol,
            "0x7b79995e5f793A07Bc00c21412e50Ecae098E7f9", // feeToken,
            Sdk.Address.fromBigInt(0n).toString(), // accessToken,
            deployer.address, // owner
            60 * 30, // src rescue delay
            60 * 30 // dst rescue delay
        ],
        srcProvider,
        deployer
    )
    //escrow factory 0x0f5e887d5bD488D0Fa29C801e839Bd262Da09CA3
    console.log(`Sepolia`, `Escrow factory contract deployed to`, escrowFactory)
}
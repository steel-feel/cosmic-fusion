import { CosmosManager } from "../classes/cosmos_manager";
import { EthereumManager } from "../classes/ethereum_manager";

export async function deployEscrows() {
    const cosmosRelayer = new CosmosManager()
    const ethResolver = new EthereumManager("","")
    console.log('Init Preparing order');

    await cosmosRelayer.init();
    console.log('Trigger Limit Order flow (Dutch Auction) on cosmos');

    const time = new Date()
    const startTime = parseInt(time.getTime() / 1000 + "") - 20
    
    const order = {
            auctionParams: {
                "duration": 3600,
                "start_time": startTime,
                "initial_rate_bump": 50000,
                "points": [],
                "gas_cost": {
                    "gas_bump_estimate": 0,
                    "gas_price_estimate": 0
                }
            },
            takerTraits: {
                "threshold_taking_price": "8"
            },
            "immutables": {
                rescue_delay: 60 * 30,
                "order_hash": "81ddf033cf9a8444d55d7a7c7275185893ccbc9da334d1f403b27c0d038ccd39",
                "hashlock": "65462b0520ef7d3df61b9992ed3bea0c56ead753be7c8b3614e0ce01e4cac41b",
                "maker": "xion10lvzwepm0044lyjv9xkgujmqk9853jxj05x4hp",
                "taker": "xion1zs43u36fujvlxsw90swpr65tdr5wskam0kghma",
                "timelocks": {
                    src_withdrawal: 1, // 10sec finality lock for test
                    src_public_withdrawal: 120, // 2m for private withdrawal
                    src_cancellation: 121, // 1sec public withdrawal
                    src_public_cancellation: 122, // 1sec private cancellation
                    dest_withdrawal: 1, // 10sec finality lock for test
                    dest_public_withdrawal: 100, // 100sec private withdrawal
                    dest_cancellation: 101 // 1sec public withdrawal
                }
            },
            makingAmount: {
                "denom": "ibc/6490A7EAB61059BFC1CDDEB05917DD70BDF3A611654162A1A47DB930D40D8AF4",
                "amount": "10"
            },
            takingAmount: {
                "denom": "ibc/6490A7EAB61059BFC1CDDEB05917DD70BDF3A611654162A1A47DB930D40D8AF4",
                "amount": "9"
            } 
    }

    console.log("Deploying escrows")

    const escrowAddr = await cosmosRelayer.deploySrcEscrow(order);
    console.log({escrowAddr});

    console.log("Escrow deployed, taking secret and completing order")
    await new Promise(resolve => setTimeout(resolve, 4_000));

    await cosmosRelayer.withdrawTaker(escrowAddr, "secret");

    await ethResolver.withdrawDestination("10", "0x7a8713E21e7434dC5441Fb666D252D13F380a97d" )
    
    console.log("🎉 Cosmic Fusion Completed.");

}
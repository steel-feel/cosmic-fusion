import { CosmosManager } from "../classes/cosmos_manager";

export async function startCosmos() {
    const cosmosRelayer = new CosmosManager()
    console.log('Init Preparing order');
    
    await cosmosRelayer.init();
    console.log('Trigger Limit Order flow (Dutch autciotn) on cosmos');

    //create cosmos source function

    const escrowDestParams = {
        rescueDelay:  60 * 30,
         "orderHash":"83ddf033cf9a8445d55d7a7c7275085893ccbc9da334d1f401b27c0d027ccd59",
         "hashlock":"65462b0520ef7d3df61b9992ed3bea0c56ead753be7c8b3614e0ce01e4cac41b",
         "maker":"xion1paesrlrpvkwneedn0mnzg0plncjh6cvls7hj2a",
         "taker":"xion1zs43u36fujvlxsw90swpr65tdr5wskam0kghma",
         "token": {  "amount" : "10", "denom" : "stake" },
          timelocks: {  
            src_withdrawal: 1, // 10sec finality lock for test
            src_public_withdrawal: 120, // 2m for private withdrawal
            src_cancellation: 121, // 1sec public withdrawal
            src_public_cancellation: 122, // 1sec private cancellation
            dest_withdrawal: 1, // 10sec finality lock for test
            dest_public_withdrawal: 100, // 100sec private withdrawal
            dest_cancellation: 101 // 1sec public withdrawal
        }
    }

  const escrowAddr = await cosmosRelayer.deployDstEscrow(escrowDestParams);

  console.log("⏲️ Waiting for timeout");
    //wait for 11 secs
  await new Promise(resolve => setTimeout(resolve, 11_000));

  await cosmosRelayer.withdrawMaker(escrowAddr ,"secret") 
  console.log("🏅 Maker got the funds!!");

}

startCosmos().catch(err => console.log(err));

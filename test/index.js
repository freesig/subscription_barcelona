/// NB: The try-o-rama config patterns are still not quite stabilized.
/// See the try-o-rama README [https://github.com/holochain/try-o-rama]
/// for a potentially more accurate example

const path = require('path')
const tape = require('tape')

const { Orchestrator, Config, tapeExecutor, singleConductor, combine  } = require('@holochain/try-o-rama')

process.on('unhandledRejection', error => {
  // Will print "unhandledRejection err is not defined"
  console.error('got unhandledRejection:', error);
});

const dnaPath = path.join(__dirname, "../dist/subscription_barcelona.dna.json")

const orchestrator = new Orchestrator({
  middleware: combine(
    // squash all instances from all conductors down into a single conductor,
    // for in-memory testing purposes.
    // Remove this middleware for other "real" network types which can actually
    // send messages across conductors
    singleConductor,

    // use the tape harness to run the tests, injects the tape API into each scenario
    // as the second argument
    tapeExecutor(require('tape'))
  ),

  globalConfig: {
    logger: true,
    network: 'memory',
    /*
    network: {
      type: 'sim2h',
      sim2h_url: 'wss://localhost:9002',
    },
    */
  },

  // the following are optional:

  waiter: {
    softTimeout: 5000,
    hardTimeout: 10000,
  },
})

const conductorConfig = {
  instances: {
    sub_instance: Config.dna(dnaPath, 'scaffold-test')
  }
}

orchestrator.registerScenario("description of example test", async (s, t) => {

  const {subscriber_alice, subscriber_bob} = await s.players({alice: conductorConfig, bob: conductorConfig})
  const {provider, host} = await s.players({provider: conductorConfig, host: conductorConfig})

  const add_result = await provider.call("sub_instance", "subscription", "add_content", {"content": "Hey Holochain" });
  t.ok(add_result.Ok);
  const alice_address = subscriber_alice.instance('sub_instance').agentAddress;
  const provider_address = provider.instance('sub_instance').agentAddress;
  const claim_address = await provider.call("sub_instance", "subscription", "request_subscription", {"agent_id": provider_address });
  t.ok(claim_address.Ok);


    /*
  // Make a call to a Zome function
  // indicating the function, and passing it an input
  const addr = await alice.call("myInstanceName", "my_zome", "create_my_entry", {"entry" : {"content":"sample content"}})

  // Wait for all network activity to
  await s.consistency()

  const result = await alice.call("myInstanceName", "my_zome", "get_my_entry", {"address": addr.Ok})

  // check for equality of the actual and expected results
  t.deepEqual(result, { Ok: { App: [ 'my_entry', '{"content":"sample content"}' ] } })
  */
})

orchestrator.run()

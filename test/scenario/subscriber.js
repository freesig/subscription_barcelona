module.exports = scenario => {

  const { config1, config2 } = require('../config')

  scenario('A subscriber can request a subscription and the claim address is returned', async (s, t) => {
    const {provider, subscriber} = await s.players({provider: config1, subscriber: config2}, true)
    let provider_agent_address = provider.info('happ').agentAddress
    const subscribe_result = await subscriber.call('happ', 'subscription', 'request_subscription', {provider_agent_id: provider_agent_address})
    console.log('Subscribing' + JSON.stringify(subscribe_result))
    t.deepEqual(subscribe_result.Ok.length, 46)
  })

  scenario.only('A subscriber can request a subscription and get premium content', async (s, t) => {
    const {provider, subscriber} = await s.players({provider: config1, subscriber: config2}, true)
    let provider_agent_address = provider.info('happ').agentAddress
    const claim_address_result = await subscriber.call('happ', 'subscription', 'request_subscription', {provider_agent_id: provider_agent_address})
    console.log('Subscribing' + JSON.stringify(claim_address_result))
    let claim_address = claim_address_result.Ok
    t.deepEqual(claim_address.length, 46)

    const content_result = await subscriber.call('happ', 'subscription', 'request_content', {provider_agent_id: provider_agent_address, claim_address: claim_address})
    console.log('getting content' + JSON.stringify(content_result))
    let content = content_result.Ok[0].content
    t.deepEqual(content, 'test')
    
  })
}

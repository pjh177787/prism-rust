let websocket = new WebSocket('ws://ec2-54-183-248-97.us-west-1.compute.amazonaws.com:8080', 'visualization');
websocket.onmessage = function (event) {
  const data = JSON.parse(event.data)
  if('VoterBlock' in data){
    const chain = data['VoterBlock']['chain']
    const votingBlockId = data['VoterBlock']['id']
    const sourceNodeId = data['VoterBlock']['miner']
    const parentId = chainsData[chain].blocks[chainsData[chain].blocks.length-1].blockId
    const votes = data['VoterBlock']['votes']
    mineVotingBlock(chain, votingBlockId, sourceNodeId, parentId, votes)
  }

  if('ProposerBlock' in data){
    const proposerBlockId = data['ProposerBlock']['id']
    const parent = proposerBlocks.find(el => el.blockId==data['ProposerBlock']['parent'])
    const sourceNodeId = data['ProposerBlock']['miner']
    let transactionBlockIds = data['ProposerBlock']['transaction_refs']
    addProposerBlock(proposerBlockId, parent, sourceNodeId, transactionBlockIds)
  }

  if('TransactionBlock' in data){
    const transactionBlockId = data['TransactionBlock']['id']
    const sourceNodeId = data['TransactionBlock']['miner']
    addTransactionBlock(transactionBlockId, sourceNodeId)
  }
  if('UpdatedLedger' in data){
    for(let i=0; i<data['UpdatedLedger']['added'].length; i++){
      const id = data['UpdatedLedger']['added'][i]
      const proposerBlock = proposerBlocks.find(el => el.blockId===id)
      confirmBlock(proposerBlock)
    }
  }
}
/* 
  Events:
  1) Add node
  Data: node id, node latitude, node longitude
  2) Add proposer block
  Data: source node id, block id, parent id, transaction block ids 
  3) Add transaction block
  Data: source node id, transaction block id 
  4) Add voting block
  Data: source node id, block id, voting chain number 
  5) Confirm proposer block
  Data: proposer block id
*/

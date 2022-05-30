import { createContractClient, instantiateContract } from '@stakeordie/griptape.js';
import { Context } from '@stakeordie/griptape.js';

export const contractDef = {
  messages: {
    scoreMember(
      Context,
      member,
      point
    ) {
      const handleMsg = {
        score: { member, point }
      };
      return { handleMsg };
    }
    
  },

  queries: {
    getMemberScore(
      Context,
      member
    ) {
      return { get_member_score: { member } };
    },
    getMember() {
      return { get_member: {} };
    }
  }
};

export const votingContract = createContractClient({
  id: 'contractClient',
  at: 'secret1ex02r2tw72tzfrgc86yeaedgzz9yz09y5fqmzm',
  definition: contractDef
});

export const getNewContract = async (groupName, memberList, memberNameList) => {
  console.log('getNewContract');
  let labelString = 'my new group scorer ' + groupName;
  let newContract = await instantiateContract({
    id:'newContractClient',
    codeId: 9460,
    definition: contractDef,
    label: labelString,
    initMsg: { group_name: groupName, addr: memberList, member: memberNameList }
  });
  console.log(newContract);
  // console.log(newContract.getMember());
  return newContract;
}



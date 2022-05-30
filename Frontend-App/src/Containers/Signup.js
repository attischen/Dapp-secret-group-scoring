import React, { useState } from "react";
import Button from '@mui/material/Button';
import Title from "../Components/Title";
import TextField from '@mui/material/TextField';
import { getNewContract, votingContract } from "../Contracts/contract"

const Signup = ({ contractClient, setContractClient }) => {
  const [groupNameValue, setGroupNameValue] = useState('');
  const [memberNameValue, setMemberNameValue] = useState('');
  const [memberAddressValue, setMemberAddressValue] = useState('');

  async function initGroup() {
    console.log('init group');
    if (groupNameValue === '') {
      alert('Please Enter Group Name!');
      return;
    } else if (memberAddressValue === '') {
      alert('Please Enter Member Addresses!');
      return;
    } else if (memberNameValue === '') {
      alert('Please Enter Member Names!');
      return;
    }

    let memberList = memberAddressValue.split(",").map(function (item) {
      return item.trim();
    });
    let memberNameList = memberNameValue.split(",").map(function (item) {
      return item.trim();
    });
    if (memberList.length !== memberNameList.length) {
      alert('The number of Addresses does not match the number of Names!');
      return;
    }
    console.log('getNewContract');
    let newContract = await getNewContract(groupNameValue, memberList, memberNameList)
    console.log(newContract);
    setContractClient(newContract);
  }

  return (
    <>
      <Title style={{ width: 1000, marginBottom: 20 }}>
        <h2>Please Sign Up Your Group to Secret Network Chain</h2>
      </Title>
      <div style={{ width: 400, margin: 10 }}>
        <div id="formGroupName" style={{ width: 400, marginBottom: 10, color: "white" }}>
          <TextField
            fullWidth
            required
            id="GroupName"
            label="Group Name"
            placeholder="Required"
            value={groupNameValue}
            onChange={(e) => { setGroupNameValue(e.target.value) }}
            inputProps={{
              style: { backgroundColor: 'white', margin: 10 },
            }}
          />
        </div>
        <div id="formGroupMember" style={{ width: 400, marginBottom: 5, color: "white" }}>
          <TextField
            fullWidth
            required
            id="GroupMember"
            label="Member Addresses (separated by commas)"
            placeholder="Required"
            value={memberAddressValue}
            onChange={(e) => { setMemberAddressValue(e.target.value) }}
            inputProps={{
              style: { backgroundColor: 'white', margin: 10 },
            }}
          />
        </div>
        <div id="formGroupMemberName" style={{ width: 400, marginBottom: 5, color: "white" }}>
          <TextField
            fullWidth
            required
            id="GroupMemberName"
            label="Member Names (separated by commas)"
            placeholder="Required"
            value={memberNameValue}
            onChange={(e) => { setMemberNameValue(e.target.value) }}
            inputProps={{
              style: { backgroundColor: 'white', margin: 10 },
            }}
          />
        </div>
      </div>
      <Button
        variant="contained"
        size="large" style={{ width: 200, margin: 10 }}
        onClick={() => { console.log('submit!'); initGroup(); }}
      > Sign Up </Button>
    </>
  );
}
export default Signup;
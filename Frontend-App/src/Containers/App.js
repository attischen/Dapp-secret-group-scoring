import React, { useState, useEffect } from "react";
import styled from 'styled-components';
import { bootstrap, onAccountAvailable, onAccountChange } from "@stakeordie/griptape.js";
import { votingContract }  from "../Contracts/contract"
import Login from './Login';
import Voting from './Voting';
import Signup from "./Signup";

const Wrapper = styled.div`
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100vh;
  width: 100vw;
  margin: auto;
`

function App() {
	const [isConnected, setIsConnected] = useState(false);
	const [isAccountChanged, setIsAccountChanged] = useState(false);
	const [contractClient, setContractClient] = useState(votingContract);

	useEffect(() => {
		const removeAccountAvailableListener = onAccountAvailable(() => {
      setIsConnected(true);
      console.log('connected!');
    });
		const removeAccountChangeListener = onAccountChange(() => {
      alert("You have changed your account, please refresh this page.")
      setIsAccountChanged(false);
    });

		return ()=> {
      removeAccountAvailableListener();
      removeAccountChangeListener();
    }
	}, []);

	return (
		<Wrapper>
			{
				!isConnected ? (
					<Login
						bootstrap={bootstrap}
					/>
				) : !contractClient ? (
					<Signup
						contractClient={contractClient}
						setContractClient={setContractClient}
					/>
				) : (
					<Voting
						contractClient={contractClient}
						setContractClient={setContractClient}
					/>
				)
			}
		</Wrapper>
	);
}

export default App;

import React from "react";
import Button from '@mui/material/Button';
import Title from "../Components/Title";

const Login = ({ bootstrap }) => (
  <>
    <Title style={{ width: 1000, margin: 10 }}>
      <h1>Welcome to Secret-Voting Service</h1>
    </Title>
    <Title style={{ width: 1000, marginBottom: 10 }}>
      <h2>Please Connect Your Wallet</h2>
    </Title>
    <Button
      variant="contained"
      size="large" style={{ width: 100, margin: 10 }}
      onClick={() => { bootstrap(); }}
    > Connect </Button>
  </>
);
export default Login;
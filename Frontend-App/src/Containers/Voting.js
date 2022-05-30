import React, { useState } from "react";
import Button from '@mui/material/Button';
import Paper from '@mui/material/Paper';
import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableCell from '@mui/material/TableCell';
import TableContainer from '@mui/material/TableContainer';
import TableHead from '@mui/material/TableHead';
import TablePagination from '@mui/material/TablePagination';
import TableRow from '@mui/material/TableRow';
import TextField from '@mui/material/TextField';
import Title from "../Components/Title";
import { getAddress } from "@stakeordie/griptape.js";

const columns = [
  { id: 'name', label: 'Name', minWidth: 150 },
  { id: 'score', label: 'Score State', minWidth: 150 },
  { id: 'finalScore', label: 'Final Score', minWidth: 150 },
  { id: 'action', label: 'Action', minWidth: 80, }
];

function createData(name, score, finalScore, action) {
  return { name, score, finalScore, action };
}

const initData = [
  createData('secret1vtrmdas7dsrwtuftwf8t85wd4dgup6ycq86nkr', 'Not Scored Yet', 'Not Scored Yet'),
  createData('China', 'Not Scored Yet', 'Not Scored Yet'),
  createData('Italy', 'Not Scored Yet', 'Not Scored Yet'),
  createData('United States', 'Not Scored Yet', 'Not Scored Yet')
];

const Voting = (contractClient, setContractClient) => {
  const [page, setPage] = useState(0);
  const [rowsPerPage, setRowsPerPage] = useState(10);
  const [readyToScore, setReadyToScore] = useState(false);
  const [score, setScore] = useState('');
  const [scoreAddress, setScoreAddress] = useState('');
  const [rows, setRows] = useState(null);
  const [groupName, setGroupName] = useState('');

  const handleChangePage = (event, newPage) => {
    setPage(newPage);
  };
  const handleChangeRowsPerPage = (event) => {
    setRowsPerPage(+event.target.value);
    setPage(0);
  };

  const contractVoteBTN = async () => {
    console.log('vote for', scoreAddress, ', score: ', score);
    try {
      await contractClient.contractClient.scoreMember(scoreAddress, parseInt(score));
    } catch (e) {
      console.error(e);
    }
    var tempRows = rows;
    for (var i = 0; i < rows.length; i++) {
      if (tempRows[i].name === scoreAddress) {
        var msg = 'Scored (' + score + ')';
        tempRows[i].score = msg;
        setRows(tempRows);
      }
    }
    setScore('');
  }

  const initMember = async () => {
    console.log(contractClient.contractClient);
    let initMsg = await contractClient.contractClient.getMember();
    let initMemberList = initMsg.members
    setGroupName(initMsg.group_name);

    // let initMemberList = ['secret1vtrmdas7dsrwtuftwf8t85wd4dgup6ycq86nkr', 'secret19cyceh0ekqhlg87r968snqyhkp8w4uwhpkx9wm'];
    console.log('init', initMemberList);
    let initTableRows = [];
    for (var i = 0; i < initMemberList.length; i++) {
      let initMemberScore;
      try {
        console.log(initMemberList[i]);
        initMemberScore = await contractClient.contractClient.getMemberScore(initMemberList[i]);
        initMemberScore = initMemberScore.member_score;
        console.log(initMemberScore);
      } catch (e) {
        console.log(e);
      }
      if (!initMemberScore) {
        initTableRows.push(createData(initMemberList[i], 'Not Scored Yet', 'Voting'));
      } else {
        initTableRows.push(createData(initMemberList[i], 'Scored (Encrypted)', initMemberScore));
      }
    }
    setRows(initTableRows);
  }

  const updateMemberScore = async (name) => {
    let tempRows = rows;
    console.log('get score:', name);
    for (var i = 0; i < tempRows.length; i++) {
      console.log(tempRows[i].name);
      let newMemberScore;
      try {
        newMemberScore = await contractClient.contractClient.getMemberScore(tempRows[i].name);
        newMemberScore = newMemberScore.member_score;
        console.log(newMemberScore);
      } catch (e) {
        console.log(e);
      }

      if (!newMemberScore) {
        alert("Voting is not ended yet.");
        return;
      } else {
        console.log(tempRows[i]);
        tempRows[i].finalScore = newMemberScore;
      }
    }
    console.log(tempRows);
    setRows(tempRows);
    alert("Final Score Updated.");
  }

  return (
    <> {
      rows == null ? (
        <div>
          <Title style={{ width: 1000, margin: 20 }}>
            <div>Welcome, {getAddress()}.</div>
          </Title>
          <Title style={{ width: 1000, margin: 20 }}>
            <h2>Ready to vote?</h2>
          </Title>
          <Title style={{ width: 1000, margin: 20 }}>
            <Button variant="contained" onClick={() => { initMember(); }}>
              Start
            </Button>
          </Title>
        </div>
      )
        : readyToScore ? (
          <div>
            <Title style={{ width: 1000, margin: 20 }}>
              <div>Please enter the score of {scoreAddress}.</div>
            </Title>
            <Title style={{ width: 1000, margin: 20 }}>
              <TextField
                id="scoreVoted"
                label="Please Enter Score"
                placeholder="Please Enter Score"
                value={score}
                onChange={(e) => { setScore(e.target.value) }}
                inputProps={{
                  style: { backgroundColor: 'white', margin: 10, width: 200, color: 'black' },
                }}
              />
            </Title>
            <Title style={{ width: 1000, margin: 20 }}>
              <Button variant="contained" style={{ margin: 5 }} onClick={() => {
                if (score != '') {
                  setReadyToScore(false);
                  contractVoteBTN();
                }
                else {
                  alert('Score must not be empty.');
                  return;
                }
              }}>Submit</Button>
              <Button variant="contained" style={{ margin: 5 }} onClick={() => {
                setReadyToScore(false);
              }}>Cancel</Button>
            </Title>
          </div>
        ) : (
          <>
            <Title style={{ width: 1000, margin: 0 }}>
              <h2>Secret-Voting Service</h2>
            </Title>
            <Title style={{ width: 1000, margin: 20 }}>
              <div style={{ marginRight: 20 }}>Welcome to Group {groupName}</div>
              <Button variant="contained" style={{ margin: 5 }} onClick={(e) => {
              e.stopPropagation();
              updateMemberScore();
            }}>Get Final Score</Button>
            </Title>
            <div style={{ width: 1000 }}>
              <Paper sx={{ width: '100%', overflow: 'hidden' }}>
                <TableContainer sx={{ maxHeight: 440 }}>
                  <Table stickyHeader aria-label="sticky table">
                    <TableHead>
                      <TableRow>
                        {columns.map((column) => (
                          <TableCell
                            key={column.id}
                            align={column.align}
                            style={{ minWidth: column.minWidth }}
                          >
                            {column.label}
                          </TableCell>
                        ))}
                      </TableRow>
                    </TableHead>
                    <TableBody>
                      {rows
                        .slice(page * rowsPerPage, page * rowsPerPage + rowsPerPage)
                        .map((row) => {
                          return (
                            <TableRow hover role="checkbox" tabIndex={-1} key={row.code}>
                              {columns.map((column) => {
                                const value = row[column.id];
                                return (
                                  <TableCell key={column.id} align={column.align}>
                                    {column.id === "action"
                                      ? <><Button variant="contained" style={{ margin: 10 }} onClick={(e) => {
                                        e.stopPropagation();
                                        setScoreAddress(row.name);
                                        setReadyToScore(true);

                                      }}>Vote</Button>
                                      </>
                                      : column.format && typeof value === 'number'
                                        ? column.format(value)
                                        : value}
                                  </TableCell>
                                );
                              })}
                            </TableRow>
                          );
                        })}
                    </TableBody>
                  </Table>
                </TableContainer>
                <TablePagination
                  rowsPerPageOptions={[10, 25, 100]}
                  component="div"
                  count={rows.length}
                  rowsPerPage={rowsPerPage}
                  page={page}
                  onPageChange={handleChangePage}
                  onRowsPerPageChange={handleChangeRowsPerPage}
                />
              </Paper>
            </div>
          </>
        )
    }
    </>
  );
}
export default Voting;
import React, { useState, useEffect } from 'react';
import { voting_system_backend } from 'declarations/voting_system_backend';

function App() {
  const [proposals, setProposals] = useState([]);
  const [newProposal, setNewProposal] = useState({ description: '', is_active: true });
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  // Fetch all proposals
  const fetchProposals = async () => {
    try {
      const count = await voting_system_backend.get_proposal_count();
      const proposalPromises = [];
      
      for (let i = 0n; i < count; i++) {
        proposalPromises.push(voting_system_backend.get_proposal(i));
      }
      
      const results = await Promise.all(proposalPromises);
      setProposals(results.map((p, index) => ({ ...p[0], id: index })).filter(p => p.description));
      setError(null);
    } catch (err) {
      setError('Failed to fetch proposals: ' + err.message);
    } finally {
      setLoading(false);
    }
  };

  // Create new proposal
  const handleCreateProposal = async (e) => {
    e.preventDefault();
    try {
      setLoading(true);
      const proposalCount = await voting_system_backend.get_proposal_count();
      await voting_system_backend.create_proposal(proposalCount, newProposal);
      setNewProposal({ description: '', is_active: true });
      await fetchProposals();
      setError(null);
    } catch (err) {
      setError('Failed to create proposal: ' + err.message);
    } finally {
      setLoading(false);
    }
  };

  // Vote on a proposal
  const handleVote = async (proposalId, choice) => {
    try {
      setLoading(true);
      await voting_system_backend.vote(BigInt(proposalId), { [choice]: null });
      await fetchProposals();
      setError(null);
    } catch (err) {
      setError('Failed to vote: ' + err.message);
    } finally {
      setLoading(false);
    }
  };

  // End a proposal
  const handleEndProposal = async (proposalId) => {
    try {
      setLoading(true);
      await voting_system_backend.end_proposal(BigInt(proposalId));
      await fetchProposals();
      setError(null);
    } catch (err) {
      setError('Failed to end proposal: ' + err.message);
    } finally {
      setLoading(false);
    }
  };

  // Load proposals on mount
  useEffect(() => {
    fetchProposals();
  }, []);

  if (loading && proposals.length === 0) {
    return <div className="container">Loading...</div>;
  }

  return (
    <div className="container">
      <h1>IC Voting System</h1>
      
      {error && <div className="error">{error}</div>}

      {/* Create New Proposal Form */}
      <div className="proposal">
        <h2>Create New Proposal</h2>
        <form onSubmit={handleCreateProposal}>
          <div className="form-group">
            <label>Description:</label>
            <textarea
              value={newProposal.description}
              onChange={(e) => setNewProposal({ ...newProposal, description: e.target.value })}
              required
            />
          </div>
          <button type="submit" className="primary-button">Create Proposal</button>
        </form>
      </div>

      {/* List of Proposals */}
      <h2>Proposals</h2>
      {proposals.map((proposal) => (
        <div key={proposal.id} className="proposal">
          <div className="proposal-header">
            <h3>Proposal #{proposal.id + 1}</h3>
            <span>{proposal.is_active ? 'Active' : 'Ended'}</span>
          </div>
          
          <p>{proposal.description}</p>
          
          <div className="stats">
            <span>👍 Approve: {proposal.approve.toString()}</span>
            <span>👎 Reject: {proposal.reject.toString()}</span>
            <span>⏭️ Pass: {proposal.pass.toString()}</span>
          </div>

          {proposal.is_active && (
            <>
              <div className="vote-buttons">
                <button 
                  className="primary-button"
                  onClick={() => handleVote(proposal.id, 'Approve')}
                >
                  Approve
                </button>
                <button 
                  className="secondary-button"
                  onClick={() => handleVote(proposal.id, 'Reject')}
                >
                  Reject
                </button>
                <button 
                  className="secondary-button"
                  onClick={() => handleVote(proposal.id, 'Pass')}
                >
                  Pass
                </button>
              </div>
              
              <button 
                className="secondary-button"
                onClick={() => handleEndProposal(proposal.id)}
                style={{ marginTop: '10px' }}
              >
                End Proposal
              </button>
            </>
          )}
        </div>
      ))}
    </div>
  );
}

export default App; 
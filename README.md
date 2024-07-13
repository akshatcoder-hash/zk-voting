# ZK Voting System

A privacy-preserving voting system built on Solana using zero-knowledge proofs.

## Table of Contents
- [Features](#features)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
- [Testing](#testing)
- [Contributing](#contributing)
- [License](#license)

## Features

- Create and manage DAOs
- Create proposals
- Cast encrypted votes
- Tally votes while preserving privacy
- Reward participants

## Prerequisites

Before you begin, ensure you have met the following requirements:

- Rust (latest stable version)
- Solana CLI tools (v1.10.0 or later)
- Node.js (v14 or later)
- Yarn

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/zk-voting.git
   cd zk-voting
   ```

2. Install dependencies:
   ```
   yarn install
   ```

3. Build the program:
   ```
   anchor build
   ```

## Usage

1. Start a local Solana cluster:
   ```
   solana-test-validator
   ```

2. Deploy the program:
   ```
   anchor deploy
   ```

3. Run the client:
   ```
   anchor run client
   ```

## Testing

To run the test suite:

```
anchor test
```

This will run through all the test cases, including initializing a DAO, creating a proposal, casting votes, and tallying results.

## Contributing

We welcome contributions to the ZK Voting System! Please see our [Contributing Guide](CONTRIBUTING.md) for more details.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

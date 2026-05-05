# WaveHub (The Orchestrator)

WaveHub is an administrative smart contract that defines and manages reward cycles called "Waves." Each Wave represents a fixed time window during which rewards are reserved for future distribution. The contract acts as the entry point for the ecosystem governor or core operator to initialize new reward windows and secure funds on-chain.

## Overview

WaveHub is designed for projects that need a controlled mechanism to create periodic reward pools and track the active window for distribution. It is not responsible for individual reward allocation or payout logic; instead, it orchestrates the lifecycle of reward Waves and stores the associated ETH pool.

Key responsibilities:
- Store reward pool funds for each Wave
- Record start and end timestamps for each window
- Restrict creation to the contract owner
- Ensure deposit integrity by requiring the owner to fund the pool at creation time

## Architecture

The WaveHub architecture is intentionally simple and modular:

1. `WaveHub` contract
   - Owner-managed governance via OpenZeppelin `Ownable`
   - Wave lifecycle state stored in a `mapping(uint256 => Wave)`
   - `createWave(...)` receives ETH and registers a new Wave

2. Wave storage
   - `Wave.totalPool` stores the reward fund in wei
   - `Wave.startTime` and `Wave.endTime` define the active window
   - `Wave.finalized` is reserved for future lifecycle state changes

3. External tooling
   - Hardhat for compilation, testing, and scripting
   - TypeScript-based tests for verifying access control and deposit behavior

### Logical flow

```
Owner -> WaveHub.createWave(duration, pool) + ETH
          |
          v
       validate inputs
          |
          v
       store Wave { totalPool, startTime, endTime, finalized }
          |
          v
      emit WaveCreated event
```

### Component map

- `contracts/WaveHub.sol`
- `scripts/deploy.ts`
- `test/WaveHub.test.ts`
- `hardhat.config.ts`

## Contract details

The main contract is `contracts/WaveHub.sol`. The core contract structure includes:

```solidity
contract WaveHub is Ownable {
    struct Wave {
        uint256 totalPool;
        uint32 startTime;
        uint32 endTime;
        bool finalized;
    }

    mapping(uint256 => Wave) public waves;
    uint256 public waveCount;

    event WaveCreated(uint256 indexed waveId, uint256 pool, uint32 start, uint32 end);

    constructor() Ownable() {}

    function createWave(uint32 _duration, uint256 _pool) external payable onlyOwner {
        require(_duration > 0, "Duration must be positive");
        require(msg.value == _pool, "Pool value must equal msg.value");
        require(block.timestamp + _duration <= type(uint32).max, "Duration too long");

        waveCount++;
        uint32 startTime = uint32(block.timestamp);
        uint32 endTime = uint32(block.timestamp + _duration);

        waves[waveCount] = Wave({
            totalPool: _pool,
            startTime: startTime,
            endTime: endTime,
            finalized: false
        });

        emit WaveCreated(waveCount, _pool, startTime, endTime);
    }
}
```

This contract uses OpenZeppelin's `Ownable` to ensure only the governor can create Waves. The `createWave` function requires the owner to deposit the exact `pool` amount in ETH to guarantee the contract is funded for future reward distribution.

## Usage

### Installation

```bash
npm install
```

### Compile

```bash
npm run compile
```

### Run tests

```bash
npm test
```

### Deploy locally

```bash
npx hardhat run scripts/deploy.ts --network localhost
```

## Testing

The `test/WaveHub.test.ts` file includes verification for:
- successful Wave creation by the owner
- correct `pool` deposit validation
- rejection of non-owner creation attempts
- invalid duration handling

## File structure

```
WaveHub (The Orchestrator)/
├── contracts/
│   └── WaveHub.sol
├── scripts/
│   └── deploy.ts
├── test/
│   └── WaveHub.test.ts
├── hardhat.config.ts
├── package.json
├── tsconfig.json
├── .gitignore
└── README.md
```

## Future improvements

Potential next steps for WaveHub:
- add `finalizeWave()` to complete a Wave lifecycle
- add withdrawal or distribution tools for the reward pool
- support ERC-20 deposits instead of native ETH
- add access roles beyond a single owner

---

This README is intended to provide a complete, developer-friendly introduction to the WaveHub Orchestrator project.

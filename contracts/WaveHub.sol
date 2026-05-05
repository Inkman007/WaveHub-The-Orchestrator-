// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/Ownable.sol";

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

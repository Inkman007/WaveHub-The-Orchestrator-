import { expect } from "chai";
import { ethers } from "hardhat";

describe("WaveHub", function () {
  it("should allow owner to create a wave with matching pool value", async function () {
    const [owner, other] = await ethers.getSigners();
    const WaveHub = await ethers.getContractFactory("WaveHub", owner);
    const waveHub = await WaveHub.deploy();
    await waveHub.deployed();

    const pool = ethers.parseEther("1");
    const duration = 3600; // 1 hour

    await expect(
      waveHub.connect(owner).createWave(duration, pool, { value: pool })
    ).to.not.be.reverted;

    const wave = await waveHub.waves(1);
    expect(wave.totalPool).to.equal(pool);
    expect(wave.finalized).to.equal(false);
    expect(wave.startTime).to.be.gt(0);
    expect(wave.endTime).to.equal(wave.startTime + duration);

    await expect(
      waveHub.connect(other).createWave(duration, pool, { value: pool })
    ).to.be.revertedWith("Ownable: caller is not the owner");
  });

  it("should reject invalid duration and mismatched pool deposit", async function () {
    const [owner] = await ethers.getSigners();
    const WaveHub = await ethers.getContractFactory("WaveHub", owner);
    const waveHub = await WaveHub.deploy();
    await waveHub.deployed();

    await expect(waveHub.createWave(0, 0, { value: 0 })).to.be.revertedWith(
      "Duration must be positive"
    );

    const pool = ethers.parseEther("1");
    await expect(waveHub.createWave(3600, pool, { value: 0 })).to.be.revertedWith(
      "Pool value must equal msg.value"
    );
  });
});

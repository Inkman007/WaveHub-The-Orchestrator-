import { ethers } from "hardhat";

async function main() {
  const WaveHub = await ethers.getContractFactory("WaveHub");
  const waveHub = await WaveHub.deploy();
  await waveHub.deployed();

  console.log("WaveHub deployed to:", waveHub.address);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});

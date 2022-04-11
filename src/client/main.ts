/**
 * Hello world
 */

import {
  establishConnection,
  establishPayer,
  checkProgram,
  uploadImage,
  reportImgData,
  chi
} from './prueba';

async function main() {
  //console.log("Let's say hello to a Solana account...");

  // Establish connection to the cluster
  await establishConnection();

  // Determine who pays for the fees
  await establishPayer();

  // Check if the program has been deployed
  await checkProgram();

  // Say hello to an account
  await uploadImage();

  // Find out how many times that account has been greeted
  await reportImgData();
  //
  //await chi();

  console.log('Success');
}

main().then(
  () => process.exit(),
  err => {
    console.error(err);
    process.exit(-1);
  },
);

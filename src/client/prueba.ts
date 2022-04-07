/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */

import {
  Keypair,
  Connection,
  PublicKey,
  LAMPORTS_PER_SOL,
  SystemProgram,
  TransactionInstruction,
  Transaction,
  sendAndConfirmTransaction,
} from '@solana/web3.js';
import fs from 'mz/fs';
import path from 'path';
import * as borsh from 'borsh';

import {getPayer, getRpcUrl, createKeypairFromFile} from './utils';
import { warn } from 'console';
import { wrap } from 'module';

/**
 * Connection to the network
 */
let connection: Connection;

/**
 * Keypair associated to the fees' payer
 */
let payer: Keypair;

/**
 * Hello world's program id
 */
let programId: PublicKey;

/**
 * The public key of the account we are saying hello to
 */
let imgDataPubkey: PublicKey;

/**
 * Path to program files
 */
const PROGRAM_PATH = path.resolve(__dirname, '../program-rust/target/deploy');

/**
 * Path to program shared object file which should be deployed on chain.
 * This file is created when running either:
 *   - `npm run build:program-c`
 *   - `npm run build:program-rust`
 */
const PROGRAM_SO_PATH = path.join(PROGRAM_PATH, 'img_test.so');

/**
 * Path to the keypair of the deployed program.
 * This file is created when running `solana program deploy dist/program/helloworld.so`
 */
const PROGRAM_KEYPAIR_PATH = path.join(PROGRAM_PATH, 'img_test-keypair.json');

/**
 * The state of a greeting account managed by the hello world program
 */

class ImgData {
    owner = Keypair.generate().publicKey.toString();
    cid = "00000000000000000000000000000000000000000000000000000000000";
    parent = "00000000000000000000000000000000000000000000000000000000000";
    child = 1;
    diff = 1;
    encrypted = 1;
    public = 1;
    editable = 1;
    constructor(fields: {owner: string, cid: string, parent: string, child: number, diff: number, encrypted: number, public: number, editable: number} | undefined = undefined) {
        if (fields) {
            this.owner = fields.owner;
            this.cid = fields.cid;
            this.parent = fields.parent;
            this.child = fields.child;
            this.diff = fields.diff;
            this.encrypted = fields.encrypted;
            this.public = fields.public;
            this.editable = fields.editable;
        }
    }
}
const ImgDataSchema = new Map([
    [
        ImgData,
        {
            kind: 'struct',
            fields: [
                ['owner', 'String'], 
                ['cid', 'String'],
                ['parent', 'String'], 
                ['child', 'u8'], 
                ['diff', 'u8'],
                ['encrypted', 'u8'], 
                ['public', 'u8'],
                ['editable', 'u8'] 
            ]
        }
    ],
]);

const IMGDATA_SIZE = borsh.serialize(
    ImgDataSchema, 
    new ImgData(),
).length;

console.log(IMGDATA_SIZE);
console.log(borsh.serialize(ImgDataSchema, new ImgData()));
const prueba = new ImgData();

export async function establishConnection(): Promise<void> {
    const rpcUrl = await getRpcUrl();
    connection = new Connection(rpcUrl, 'confirmed');
    const version = await connection.getVersion();
    console.log('Connection to cluster established:', rpcUrl, version);
}

export async function establishPayer(): Promise<void> {
    let fees = 0;
    if (!payer) {
        const { feeCalculator } = await connection.getRecentBlockhash();

        fees += await connection.getMinimumBalanceForRentExemption(IMGDATA_SIZE);

        fees += feeCalculator.lamportsPerSignature * 100;

        payer = await getPayer();
    }

    let lamports = await connection.getBalance(payer.publicKey);
    if (lamports < fees) {
        const sig = await connection.requestAirdrop(
            payer.publicKey, 
            fees - lamports,
        );

        await connection.confirmTransaction(sig);
        lamports = await connection.getBalance(payer.publicKey);
    }

    console.log(
        'Uploader account',
        payer.publicKey.toBase58(),
        'has',
        lamports / LAMPORTS_PER_SOL,
        'SOL to pay for fees',
    );
}

export async function checkProgram(): Promise<void> {
    try {
        const programKeypair = await createKeypairFromFile(PROGRAM_KEYPAIR_PATH);
        programId = programKeypair.publicKey;
    } catch (err) {
        const errMsg = (err as Error).message;
        throw new Error(
            `Failed to read program keypair at '${PROGRAM_KEYPAIR_PATH}' due to error: ${errMsg}. Program may need to be deployed with \`solana program deploy dist/program/helloworld.so\``,
        );
    }

    const programInfo = await connection.getAccountInfo(programId);
    if (programInfo === null) {
        if (fs.existsSync(PROGRAM_SO_PATH)) {
            throw new Error(
                'Program needs to be deployed with `solana program deploy dist/program/helloworld.so`',
            );
        } else {
            throw new Error('Program needs to be built and deployed');
        }
    }else if (!programInfo.executable) {
        throw new Error('Program is not executable');
    }
    console.log(`Using program ${programId.toBase58()}`);

    //const IMGDATA_SEED = 'bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzd';
    const IMGDATA_SEED = 'image';
    imgDataPubkey = await PublicKey.createWithSeed(
        payer.publicKey, 
        IMGDATA_SEED, 
        programId,
    );

    console.log(imgDataPubkey.toString());

    const imgDataAccount = await connection.getAccountInfo(imgDataPubkey);
    if (imgDataAccount === null) {
        console.log(
            'Creating account', 
            imgDataPubkey.toBase58(), 
            'to save image data',
        );
        const lamports = await connection.getMinimumBalanceForRentExemption(
            IMGDATA_SIZE,
        );
        console.log('Chi');
        const transaction = new Transaction().add(
            SystemProgram.createAccountWithSeed({ 
                basePubkey: payer.publicKey,
                fromPubkey: payer.publicKey,
                lamports: lamports,
                newAccountPubkey: imgDataPubkey,
                programId: programId,
                seed: IMGDATA_SEED,
                space: IMGDATA_SIZE, 
            }),
        );
        console.log(imgDataPubkey);
        console.log(transaction);
        console.log(transaction.instructions);

        await sendAndConfirmTransaction(connection, transaction, [payer]);
        console.log('Chi 2');

        console.log(imgDataAccount);
    }
}

export async function uploadImage(): Promise<void> {
    console.log('Uploading image and saving image data to', imgDataPubkey.toBase58());
    const instruction = new TransactionInstruction({
        keys: [{pubkey: payer.publicKey, isSigner: true, isWritable: false}, {pubkey: imgDataPubkey, isSigner: false, isWritable: true}],
        programId : programId,
        data: Buffer.from(Uint8Array.of(0, ...new TextEncoder().encode("bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi"),...new TextEncoder().encode("00000000000000000000000000000000000000000000000000000000000"),0,0,1,0,1)),
    });
    await sendAndConfirmTransaction(
        connection,
        new Transaction().add(instruction),
        [payer],
    );
}

export async function reportImgData(): Promise<void> {
    const accountInfo = await connection.getAccountInfo(imgDataPubkey);
    if (accountInfo === null) {
        throw 'Error: cannot find the image data account';
    }

    const imgData = borsh.deserialize(
        ImgDataSchema, 
        ImgData, 
        accountInfo.data
    );

    console.log(
        'Image',
        imgData.cid, 
        `and it's info have been saved`
    );
}

export async function chi(): Promise<void> {
    //console.log(IMGDATA_SIZE);
    //console.log(new ImgData());
    //let wi = borsh.serialize(ImgDataSchema, new ImgData());
    //console.log(wi);
    //let wo = borsh.deserialize(ImgDataSchema, ImgData, Buffer.from(wi));
    //console.log(wo);
    //let wa = Keypair.generate().publicKey;
    //console.log(wa.toBase58());
    //console.log(wa.toString());
    //console.log(typeof wa.toBase58());
    //console.log(typeof wa.toString());

    const infocuenta = await connection.getAccountInfo(imgDataPubkey);
    console.log(infocuenta);
}

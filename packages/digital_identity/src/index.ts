// Minimal client for interacting with the digital_identity contract.
// NOTE: This is a thin wrapper. Real usage requires a signer and the deployed contract ID.

import { Server, Networks, TransactionBuilder, Operation, Keypair, Account, xdr, Transaction } from "@stellar/soroban-client";

type Config = {
    rpcUrl?: string;
    network?: 'testnet' | 'futurenet' | 'local';
    contractId?: string; // must be set after deployment
};

export default function Client(cfg: Config) {
    const rpcUrl = cfg.rpcUrl || "https://soroban-testnet.stellar.org:443";
    const server = new Server(rpcUrl);
    const networkPassphrase = cfg.network === 'testnet' ? Networks.TESTNET : Networks.PUBLIC;
    let contractId = cfg.contractId || "";

    return {
        setContractId(id: string) { contractId = id; },

        // Prepare a transaction to set identity - requires submitting with a signer
        async setIdentity(pubkey: string, metadata: string) {
            if (!contractId) throw new Error("contractId not set");
            // In practice you need to build an invocation via soroban-xdr and sign with your keypair.
            // Return a prepared object for the caller to sign & submit.
            return { action: "set_identity", contractId, args: [pubkey, metadata] };
        },

        async getIdentity(pubkey: string) {
            if (!contractId) throw new Error("contractId not set");
            // For readonly calls you can use the RPC /call endpoint; here we return a placeholder showing required call.
            return { action: "get_identity", contractId, args: [pubkey] };
        },

        async registerItem(itemId: string, ownerPubKey: string, metadata: string) {
            if (!contractId) throw new Error("contractId not set");
            return { action: "register_item", contractId, args: [itemId, ownerPubKey, metadata] };
        },

        async getItem(itemId: string) {
            if (!contractId) throw new Error("contractId not set");
            return { action: "get_item", contractId, args: [itemId] };
        },

        // helper: set the network / rpc values for direct use
        raw: { server, networkPassphrase }
    };
}

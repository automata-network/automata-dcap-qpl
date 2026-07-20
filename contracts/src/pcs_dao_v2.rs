use ethers::contract::abigen;

// The V2 PCS DAO keeps the existing PCS ABI and adds completion-gated,
// sequential CRL indexing. Keep this binding intentionally small so existing
// generated PcsDao bindings remain usable by legacy deployments.
abigen!(
    PcsDaoV2,
    r#"[
        function indexStoredCrlBatch(uint8 ca, bytes32 expectedDerHash, uint256 maxEntries) external returns (uint256 indexedCount, bool complete)
    ]"#,
);

abigen!(
    X509CrlHelperV2,
    r#"[
        function getTbsAndSig(bytes der) external pure returns (bytes tbs, bytes sig)
        function indexedCrls(bytes32 derHash) external view returns (bool)
    ]"#,
);
